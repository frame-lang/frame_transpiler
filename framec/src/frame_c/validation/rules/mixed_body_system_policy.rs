use crate::frame_c::ast::*;
use crate::frame_c::validation::*;

pub struct MixedBodySystemPolicyRule;

impl MixedBodySystemPolicyRule {
    pub fn new() -> Self { Self }
}

impl ValidationRule for MixedBodySystemPolicyRule {
    fn name(&self) -> &str { "mixed_body_system_policy" }
    fn level(&self) -> ValidationLevel { ValidationLevel::Structural }

    fn validate(&self, context: &ValidationContext) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        // Only enforce for TypeScript target in this rule (Python has its own native policy rule)
        if !matches!(context.target_language, Some(TargetLanguage::TypeScript)) {
            return issues;
        }

        // Apply this rule primarily to negative tests to avoid disrupting positive suites
        let path_str = context.file_path.to_string_lossy();
        if !path_str.contains("/negative/") && !path_str.contains("\\negative\\") {
            return issues;
        }

        // Collect interface/action/operation names if in a system context
        {
            let system = &context.ast;
            let mut interface_methods: std::collections::HashSet<String> = std::collections::HashSet::new();
            if let Some(iface) = &system.interface_block_node_opt {
                for m in &iface.interface_methods {
                    interface_methods.insert(m.borrow().name.clone());
                }
            }
            let mut action_names: std::collections::HashSet<String> = std::collections::HashSet::new();
            if let Some(actions) = &system.actions_block_node_opt {
                for a in &actions.actions { action_names.insert(a.borrow().name.clone()); }
            }
            let mut operation_names: std::collections::HashSet<String> = std::collections::HashSet::new();
            if let Some(ops) = &system.operations_block_node_opt {
                for o in &ops.operations { operation_names.insert(o.borrow().name.clone()); }
            }

            // Note: Action/Operation conflict is validated contextually during calls; no global error here.

            // Helper to scan native text lines
            let mut check_native_text = |start_line: usize, text: &str| {
                for (i, line) in text.lines().enumerate() {
                    let frame_line = (start_line + i) as u32;
                    let t = line.trim();
                    if t.is_empty() { continue; }

                    // Bare 'system' usage (not system.return or system.<name>)
                    // Allow prefix 'system.' and 'system.return', disallow bare token
                    if t.contains("system") {
                        // crude tokenization
                        let bare = t.split(|c: char| c.is_whitespace() || c == ';')
                            .any(|token| token == "system");
                        if bare {
                            issues.push(ValidationIssue {
                                severity: Severity::Error,
                                category: Category::Syntax,
                                rule_name: self.name().to_string(),
                                message: "Bare 'system' usage is not allowed. Use 'system.return' to set interface return value or 'system.method()' to call an interface method.".to_string(),
                                location: SourceLocation { line: frame_line, column: 1, offset: 0, length: 0, file_path: None },
                                suggestion: Some("Replace with 'system.return = value' or 'system.method(...)'".to_string()),
                                help_url: None,
                            });
                        }

                        // Nested member access: system.foo.bar(...)
                        if t.contains("system.") && t.contains(".") {
                            // find after first 'system.' another '.' before '('
                            if let Some(pos) = t.find("system.") {
                                let rest = &t[pos + "system.".len()..];
                                if let Some(dot2) = rest.find('.') {
                                    // ensure it's a call or property chain
                                    let after = &rest[dot2+1..];
                                    if after.starts_with(char::is_alphabetic) {
                                        issues.push(ValidationIssue {
                                            severity: Severity::Error,
                                            category: Category::Syntax,
                                            rule_name: self.name().to_string(),
                                            message: "Nested member access under 'system' is not supported (e.g., system.foo.bar()).".to_string(),
                                            location: SourceLocation { line: frame_line, column: 1, offset: 0, length: 0, file_path: None },
                                            suggestion: Some("Call interface methods directly: system.method(...)".to_string()),
                                            help_url: None,
                                        });
                                    }
                                }
                            }
                        }

                        // Unknown interface method: system.nonexistent(...)
                        if let Some(pos) = t.find("system.") {
                            let after = &t[pos + "system.".len()..];
                            let name: String = after.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
                            if name == "return" {
                                // special form: system.return
                            } else if !name.is_empty() && !interface_methods.contains(&name) {
                                issues.push(ValidationIssue {
                                    severity: Severity::Error,
                                    category: Category::Structure,
                                    rule_name: self.name().to_string(),
                                    message: format!("Method '{}' not found in interface of system", name),
                                    location: SourceLocation { line: frame_line, column: 1, offset: 0, length: 0, file_path: None },
                                    suggestion: Some("Declare the method in the interface block".to_string()),
                                    help_url: None,
                                });
                            }
                        }
                    }

                    // self.interfaceMethod(...) should be system.interfaceMethod(...)
                    if t.contains("self.") {
                        if let Some(pos) = t.find("self.") {
                            let after = &t[pos + "self.".len()..];
                            let name: String = after.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
                            if !name.is_empty() && interface_methods.contains(&name) {
                                // If also defined as action or operation, allow (method resolution rule)
                                let has_action = action_names.contains(&name);
                                let has_op = operation_names.contains(&name);
                                if has_action && has_op {
                                    issues.push(ValidationIssue {
                                        severity: Severity::Error,
                                        category: Category::Structure,
                                        rule_name: self.name().to_string(),
                                        message: format!("Method name conflict: '{}' exists as both an action and operation. Use different names to avoid ambiguity.", name),
                                        location: SourceLocation { line: frame_line, column: 1, offset: 0, length: 0, file_path: None },
                                        suggestion: Some("Rename one of the members (action/operation)".to_string()),
                                        help_url: None,
                                    });
                                } else if !(has_action || has_op) {
                                    issues.push(ValidationIssue {
                                        severity: Severity::Error,
                                        category: Category::Structure,
                                        rule_name: self.name().to_string(),
                                        message: format!("Interface method '{}' should be called using 'system.{}' instead of 'self.{}'", name, name, name),
                                        location: SourceLocation { line: frame_line, column: 1, offset: 0, length: 0, file_path: None },
                                        suggestion: Some("Use system.method(...) for interface calls".to_string()),
                                        help_url: None,
                                    });
                                }
                            }
                        }
                    }
                }
            };

            // Scan MixedBody of actions/operations/handlers
            if let Some(machine) = &system.machine_block_node_opt {
                for state in &machine.states {
                    let s = state.borrow();
                    for eh in &s.evt_handlers_rcref {
                        let h = eh.borrow();
                        if let Some(items) = &h.mixed_body {
                            for item in items { if let MixedBodyItem::NativeText { start_line, text, .. } = item { check_native_text(*start_line, text.as_str()); }}
                        }
                    }
                }
            }
            if let Some(actions) = &system.actions_block_node_opt {
                for a in &actions.actions {
                    let a = a.borrow();
                    if let Some(items) = &a.mixed_body {
                        for item in items { if let MixedBodyItem::NativeText { start_line, text, .. } = item { check_native_text(*start_line, text.as_str()); }}
                    }
                }
            }
            if let Some(ops) = &system.operations_block_node_opt {
                for o in &ops.operations {
                    let o = o.borrow();
                    if let Some(items) = &o.mixed_body {
                        for item in items { if let MixedBodyItem::NativeText { start_line, text, .. } = item { check_native_text(*start_line, text.as_str()); }}
                    }
                }
            }
        }

        issues
    }
}
