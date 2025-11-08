// Validation Rule: Transitions must be terminal in handler bodies
// Ensures that if a MixedBody contains a terminal Frame statement (Transition, ParentForward,
// StackPush, StackPop), then no subsequent executable native content appears after it within
// the same handler body. We allow syntactic headers (except/else/finally) to appear for Python
// to close blocks, but they must not contain additional executable content beyond a 'pass' stub.

use crate::frame_c::ast::*;
use crate::frame_c::validation::*;

pub struct TransitionsTerminalRule;

impl TransitionsTerminalRule {
    pub fn new() -> Self { Self }

    fn is_terminal(stmt: &MirStatement) -> bool {
        match stmt {
            MirStatement::Transition { .. } |
            MirStatement::ParentForward |
            MirStatement::StackPush |
            MirStatement::StackPop => true,
            MirStatement::Return(_) => true, // treat MIR return as terminal within MixedBody
        }
    }

    fn item_is_permitted_after_terminal(item: &MixedBodyItem) -> bool {
        match item {
            MixedBodyItem::Frame { .. } => false, // another MIR after terminal is invalid
            MixedBodyItem::NativeText { text, .. } => {
                // Permit only structural headers (except/else/finally) and blank/comment lines
                for line in text.lines() {
                    let trimmed = line.trim_start();
                    if trimmed.is_empty() { continue; }
                    if trimmed.starts_with('#') { continue; }
                    if trimmed.starts_with("except ") || trimmed == "except:" || trimmed == "else:" || trimmed == "finally:" {
                        continue;
                    }
                    // Any other native content is not permitted after terminal
                    return false;
                }
                true
            }
            MixedBodyItem::NativeAst { ast, .. } => {
                // Conservatively disallow any AST content after terminal
                // (we do not parse down to headers here)
                let s = ast.to_source();
                for line in s.lines() {
                    let trimmed = line.trim_start();
                    if trimmed.is_empty() { continue; }
                    if trimmed.starts_with('#') { continue; }
                    if trimmed.starts_with("except ") || trimmed == "except:" || trimmed == "else:" || trimmed == "finally:" {
                        continue;
                    }
                    return false;
                }
                true
            }
        }
    }
}

impl ValidationRule for TransitionsTerminalRule {
    fn name(&self) -> &str { "transitions_terminal" }
    fn level(&self) -> ValidationLevel { ValidationLevel::Structural }

    fn validate(&self, context: &ValidationContext) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        // Walk all states/handlers and inspect MixedBody where present
        if let Some(machine) = &context.ast.machine_block_node_opt {
            for state_rcref in &machine.states {
                let state = state_rcref.borrow();
                for handler_rcref in &state.evt_handlers_rcref {
                    let handler = handler_rcref.borrow();
                    if let Some(mixed) = &handler.mixed_body {
                        // Find first terminal directive position
                        let mut terminal_index: Option<usize> = None;
                        for (i, item) in mixed.iter().enumerate() {
                            if let MixedBodyItem::Frame { stmt, .. } = item {
                                if Self::is_terminal(&stmt) {
                                    terminal_index = Some(i);
                                    break;
                                }
                            }
                        }
                        if let Some(idx) = terminal_index {
                            // Ensure that all subsequent items are permitted headers/blank/comment only
                            for j in (idx + 1)..mixed.len() {
                                let item = &mixed[j];
                                if !Self::item_is_permitted_after_terminal(item) {
                                    // Compute a reasonable location (use frame_line if available)
                                    let (line, file_path) = match item {
                                        MixedBodyItem::Frame { frame_line, .. } => (*frame_line as u32, Some(context.file_path.to_string_lossy().to_string())),
                                        MixedBodyItem::NativeText { start_line, .. } => (*start_line as u32, Some(context.file_path.to_string_lossy().to_string())),
                                        MixedBodyItem::NativeAst { start_line, .. } => (*start_line as u32, Some(context.file_path.to_string_lossy().to_string())),
                                    };
                                    issues.push(ValidationIssue {
                                        severity: Severity::Error,
                                        category: Category::Semantic,
                                        rule_name: self.name().to_string(),
                                        message: "Frame transition/terminal directive must be the last statement in a handler block".to_string(),
                                        location: SourceLocation { line, column: 1, offset: 0, length: 0, file_path },
                                        suggestion: Some("Move additional code before the transition, or wrap it in separate control-flow prior to the terminal directive".to_string()),
                                        help_url: None,
                                    });
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        issues
    }
}
