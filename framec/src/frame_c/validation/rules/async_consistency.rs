use crate::frame_c::ast::*;
use crate::frame_c::validation::*;

pub struct AsyncConsistencyRule {
    name: String,
}

impl AsyncConsistencyRule {
    pub fn new() -> Self {
        Self {
            name: "async_consistency".to_string(),
        }
    }

    fn handler_contains_await<'a>(&self, handler: &EventHandlerNode) -> bool {
        // Prefer MixedBody; fallback to parsed_target_blocks or target regions
        if let Some(items) = &handler.mixed_body {
            for it in items {
                match it {
                    MixedBodyItem::NativeText { target: _, text, .. } => {
                        if text.contains("await ") || text.trim_start().starts_with("await ") {
                            return true;
                        }
                    }
                    MixedBodyItem::NativeAst { ast, .. } => {
                        let code = ast.to_source();
                        if code.contains("await ") || code.trim_start().starts_with("await ") {
                            return true;
                        }
                    }
                    _ => {}
                }
            }
        } else if !handler.parsed_target_blocks.is_empty() {
            for block in &handler.parsed_target_blocks {
                let code = block.ast.to_source();
                if code.contains("await ") || code.trim_start().starts_with("await ") {
                    return true;
                }
            }
        } else if !handler.target_specific_regions.is_empty() {
            // Slice raw source lines using frame line spans
            // Note: validation context provides source_code at top-level; here we conservatively
            // fallback to scanning handler.target_specific_regions frame spans from the original source
            // via an environment variable hook disabled in this build.
            // If unavailable, return false to avoid false positives.
            let Some(source) = std::option::Option::None::<&str> else { return false };
            for region in &handler.target_specific_regions {
                let start = region.frame_start_line.saturating_sub(1);
                let end = region.frame_end_line.saturating_sub(1);
                let mut raw = String::new();
                let lines: Vec<&str> = source.lines().collect();
                let end_idx = end.min(lines.len().saturating_sub(1));
                for i in start..=end_idx {
                    if let Some(l) = lines.get(i) {
                        raw.push_str(l);
                        raw.push('\n');
                    }
                }
                if raw.contains("await ") || raw.trim_start().starts_with("await ") {
                    return true;
                }
            }
        }
        false
    }

    fn interface_method_is_async(&self, system: &SystemNode, name: &str) -> Option<bool> {
        let iface = system.interface_block_node_opt.as_ref()?;
        for m in &iface.interface_methods {
            let mm = m.borrow();
            if mm.name == name {
                return Some(mm.is_async);
            }
        }
        None
    }
}

impl ValidationRule for AsyncConsistencyRule {
    fn name(&self) -> &str {
        &self.name
    }

    fn level(&self) -> ValidationLevel {
        ValidationLevel::Semantic
    }

    fn validate(&self, context: &ValidationContext) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        let system = context.ast;
        let source_path = context.file_path.to_string_lossy().to_string();

        let machine = match &system.machine_block_node_opt {
            Some(m) => m,
            None => return issues,
        };

        for state_r in &machine.states {
            let state = state_r.borrow();
            for eh_r in &state.evt_handlers_rcref {
                let handler = eh_r.borrow();

                let contains_await = self.handler_contains_await(&handler);
                if !contains_await {
                    continue;
                }

                let evt = handler.event_symbol_rcref.borrow();

                // Lifecycle/internal handler must be explicitly async if it awaits
                if evt.is_enter_msg || evt.is_exit_msg || evt.interface_name_opt.is_none() {
                    if !handler.is_async {
                        issues.push(ValidationIssue {
                            severity: Severity::Error,
                            category: Category::Semantic,
                            rule_name: self.name.clone(),
                            message: "Handler uses 'await' but is not marked async. Mark the lifecycle/internal handler as 'async'.".to_string(),
                            location: SourceLocation {
                                line: handler.line as u32,
                                column: 1,
                                offset: 0,
                                length: 0,
                                file_path: Some(source_path.clone()),
                            },
                            suggestion: Some("Prefix the handler with 'async' (e.g., async $>() { ... })".to_string()),
                            help_url: None,
                        });
                    }
                    continue;
                }

                // Interface handler: interface method must be async to allow await
                if let Some(is_async_iface) = self.interface_method_is_async(system, &evt.msg) {
                    if !is_async_iface {
                        issues.push(ValidationIssue {
                            severity: Severity::Error,
                            category: Category::Semantic,
                            rule_name: self.name.clone(),
                            message: format!(
                                "Interface method '{}' must be declared async to use 'await' in its handler.",
                                evt.msg
                            ),
                            location: SourceLocation {
                                line: handler.line as u32,
                                column: 1,
                                offset: 0,
                                length: 0,
                                file_path: Some(source_path.clone()),
                            },
                            suggestion: Some(format!("Declare 'async {}(...)' in the interface block.", evt.msg)),
                            help_url: None,
                        });
                    }
                }
            }
        }

        issues
    }
}
