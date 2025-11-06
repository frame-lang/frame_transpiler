use regex::Regex;

use crate::frame_c::ast::{ActionNode, EventHandlerNode, MixedBodyItem, OperationNode};
use crate::frame_c::validation::{
    Category, Severity, SourceLocation, ValidationContext, ValidationIssue, ValidationLevel,
    ValidationRule,
};

pub struct PythonNativePolicyRule;

impl PythonNativePolicyRule {
    pub fn new() -> Self {
        Self
    }
}

impl ValidationRule for PythonNativePolicyRule {
    fn name(&self) -> &str {
        "python_native_policy"
    }

    fn level(&self) -> ValidationLevel {
        // Structural: run after basic syntax checks
        ValidationLevel::Structural
    }

    fn validate(&self, context: &ValidationContext) -> Vec<ValidationIssue> {
        // Only applies to Python target
        if !matches!(context.target_language, Some(super::super::TargetLanguage::Python)) {
            return Vec::new();
        }

        let mut issues = Vec::new();

        let var_decl = Regex::new(r"^\s*var\b").unwrap();
        // Legacy braced control-flow in Python bodies (must use ':')
        let braced_cf = Regex::new(
            r"^\s*(if|elif|else|for|while|try|except|finally|with|async\s+with|async\s+for)\b.*\{\s*$",
        )
        .unwrap();
        // Closing brace starting a legacy 'else' chain (e.g., `} else {`)
        let brace_else = Regex::new(r"^\s*}\s*(else\b.*\{\s*)?$").unwrap();

        // Helper to scan a body of MixedBody items
        let mut scan_mixed = |_name: &str, items_opt: &Option<Vec<MixedBodyItem>>| {
            if let Some(items) = items_opt {
                for item in items {
                    if let MixedBodyItem::NativeText {
                        target,
                        text,
                        start_line,
                        ..
                    } = item
                    {
                        // Only enforce for Python native text
                        if format!("{:?}", target).to_lowercase() != "python3" {
                            continue;
                        }
                        for (i, line) in text.lines().enumerate() {
                            if var_decl.is_match(line)
                                || braced_cf.is_match(line)
                                || brace_else.is_match(line)
                            {
                                let frame_line = (*start_line as u32) + (i as u32);
                                issues.push(ValidationIssue {
                                    severity: Severity::Error,
                                    category: Category::Syntax,
                                    rule_name: self.name().to_string(),
                                    message: format!(
                                        "Legacy Frame syntax in Python native body: '{}'",
                                        line.trim()
                                    ),
                                    location: SourceLocation {
                                        line: frame_line,
                                        column: 1,
                                        offset: 0,
                                        length: 0,
                                        file_path: Some(
                                            context.file_path.to_string_lossy().to_string(),
                                        ),
                                    },
                                    suggestion: Some(
                                        "Use Python-native syntax (':' and indentation) and drop 'var'"
                                            .to_string(),
                                    ),
                                    help_url: None,
                                });
                            }
                        }
                    }
                }
            }
        };

        // Walk states/handlers
        if let Some(machine) = &context.ast.machine_block_node_opt {
            for state_r in &machine.states {
                let state = state_r.borrow();
                // normal event handlers
                for eh_r in &state.evt_handlers_rcref {
                    let eh: &EventHandlerNode = &eh_r.borrow();
                    scan_mixed(&state.name, &eh.mixed_body);
                }
                // enter/exit if present
                if let Some(eh_r) = &state.enter_event_handler_opt {
                    let eh = eh_r.borrow();
                    scan_mixed(&state.name, &eh.mixed_body);
                }
                if let Some(eh_r) = &state.exit_event_handler_opt {
                    let eh = eh_r.borrow();
                    scan_mixed(&state.name, &eh.mixed_body);
                }
            }
        }

        // Actions
        if let Some(actions_block) = &context.ast.actions_block_node_opt {
            for a_r in &actions_block.actions {
                let a: &ActionNode = &a_r.borrow();
                scan_mixed(&a.name, &a.mixed_body);
            }
        }
        // Operations
        if let Some(ops_block) = &context.ast.operations_block_node_opt {
            for o_r in &ops_block.operations {
                let o: &OperationNode = &o_r.borrow();
                scan_mixed(&o.name, &o.mixed_body);
            }
        }

        issues
    }
}
