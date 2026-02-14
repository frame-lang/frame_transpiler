//! Semantic validation pass for Frame V4
//!
//! Validates Frame semantics using the Arcanum symbol table:
//! - E400: Transition must be last statement in block
//! - E401: Frame statements not allowed in actions/operations
//! - E402: Unknown state in transition
//! - E403: Invalid parent forwards in HSM
//! - E404: Handler body must be inside a state block
//! - E405: State parameter arity mismatch
//! - E406: Invalid interface method calls

use crate::frame_c::v4::arcanum::Arcanum;
use crate::frame_c::v4::frame_ast::{
    FrameAst, SystemAst, StateAst, HandlerAst, HandlerBody,
    Statement, TransitionAst, ForwardAst, InterfaceMethod,
};
use crate::frame_c::v4::validation::pass::{ValidationContext, ValidationPass};
use crate::frame_c::v4::validation::types::ValidationIssue;
use std::collections::{HashMap, HashSet};

/// Semantic validation pass
///
/// Performs semantic validation using the Arcanum symbol table
/// to cross-reference Frame declarations.
pub struct SemanticPass;

impl ValidationPass for SemanticPass {
    fn name(&self) -> &'static str {
        "semantic"
    }

    fn run(
        &self,
        ast: &FrameAst,
        arcanum: &Arcanum,
        ctx: &mut ValidationContext,
    ) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        match ast {
            FrameAst::System(system) => {
                ctx.system_name = Some(system.name.clone());
                self.validate_system(system, arcanum, &mut issues);
            }
            FrameAst::Module(module) => {
                for system in &module.systems {
                    ctx.system_name = Some(system.name.clone());
                    self.validate_system(system, arcanum, &mut issues);
                }
            }
        }

        issues
    }
}

impl SemanticPass {
    /// Validate a system
    fn validate_system(&self, system: &SystemAst, arcanum: &Arcanum, issues: &mut Vec<ValidationIssue>) {
        // Build lookup tables
        let state_map = self.build_state_map(system);
        let interface_methods = self.build_interface_map(system);
        let actions = self.build_action_set(system);
        let operations = self.build_operation_set(system);

        // Validate machine if present
        if let Some(machine) = &system.machine {
            for state in &machine.states {
                self.validate_state(
                    &system.name,
                    state,
                    &state_map,
                    &interface_methods,
                    arcanum,
                    issues,
                );
            }
        }

        // E401 would be validated here if we tracked Frame statements in actions/operations
        // Currently the parser prevents this by not parsing Frame statements in those contexts
        let _ = (actions, operations);
    }

    /// Build map of state names to states
    fn build_state_map<'a>(&self, system: &'a SystemAst) -> HashMap<String, &'a StateAst> {
        let mut map = HashMap::new();
        if let Some(machine) = &system.machine {
            for state in &machine.states {
                map.insert(state.name.clone(), state);
            }
        }
        map
    }

    /// Build map of interface methods
    fn build_interface_map<'a>(&self, system: &'a SystemAst) -> HashMap<String, &'a InterfaceMethod> {
        let mut map = HashMap::new();
        for method in &system.interface {
            map.insert(method.name.clone(), method);
        }
        map
    }

    /// Build set of action names
    fn build_action_set(&self, system: &SystemAst) -> HashSet<String> {
        system.actions.iter().map(|a| a.name.clone()).collect()
    }

    /// Build set of operation names
    fn build_operation_set(&self, system: &SystemAst) -> HashSet<String> {
        system.operations.iter().map(|o| o.name.clone()).collect()
    }

    /// Validate a state
    fn validate_state(
        &self,
        system_name: &str,
        state: &StateAst,
        state_map: &HashMap<String, &StateAst>,
        interface_methods: &HashMap<String, &InterfaceMethod>,
        arcanum: &Arcanum,
        issues: &mut Vec<ValidationIssue>,
    ) {
        // E403: Validate parent state exists for HSM
        if let Some(parent_name) = &state.parent {
            if !state_map.contains_key(parent_name) {
                issues.push(
                    ValidationIssue::error(
                        "E403",
                        format!(
                            "State '{}' has invalid parent '{}'",
                            state.name, parent_name
                        )
                    )
                    .with_span(state.span.clone())
                    .with_note(format!(
                        "Available states: {}",
                        self.format_available_states(state_map)
                    ))
                    .with_fix(format!(
                        "Change parent to an existing state or remove the parent reference"
                    ))
                );
            }
        }

        // Validate handlers
        for handler in &state.handlers {
            self.validate_handler(
                system_name,
                state,
                handler,
                state_map,
                interface_methods,
                arcanum,
                issues,
            );
        }

        // Validate enter handler
        if let Some(enter) = &state.enter {
            self.validate_handler_body(system_name, state, &enter.body, state_map, arcanum, issues);
        }

        // Validate exit handler
        if let Some(exit) = &state.exit {
            self.validate_handler_body(system_name, state, &exit.body, state_map, arcanum, issues);
        }
    }

    /// Validate a handler
    fn validate_handler(
        &self,
        system_name: &str,
        state: &StateAst,
        handler: &HandlerAst,
        state_map: &HashMap<String, &StateAst>,
        interface_methods: &HashMap<String, &InterfaceMethod>,
        arcanum: &Arcanum,
        issues: &mut Vec<ValidationIssue>,
    ) {
        // E406: Check if handler corresponds to interface method
        if let Some(method) = interface_methods.get(&handler.event) {
            if handler.params.len() != method.params.len() {
                issues.push(
                    ValidationIssue::error(
                        "E406",
                        format!(
                            "Handler '{}' in state '{}' has {} parameters but interface method expects {}",
                            handler.event,
                            state.name,
                            handler.params.len(),
                            method.params.len()
                        )
                    )
                    .with_span(handler.span.clone())
                    .with_fix(format!(
                        "Update handler to match interface signature: {}({} params)",
                        method.name, method.params.len()
                    ))
                );
            }
        }

        self.validate_handler_body(system_name, state, &handler.body, state_map, arcanum, issues);
    }

    /// Validate handler body statements
    fn validate_handler_body(
        &self,
        system_name: &str,
        state: &StateAst,
        body: &HandlerBody,
        state_map: &HashMap<String, &StateAst>,
        arcanum: &Arcanum,
        issues: &mut Vec<ValidationIssue>,
    ) {
        // E400: Check that terminal statements are last
        self.validate_terminal_last(body, issues);

        for statement in &body.statements {
            match statement {
                Statement::Transition(transition) => {
                    self.validate_transition(system_name, state, transition, state_map, arcanum, issues);
                }
                Statement::Forward(forward) => {
                    self.validate_forward(state, forward, state_map, issues);
                }
                _ => {
                    // Other statements don't need validation yet
                }
            }
        }
    }

    /// E400: Validate terminal statements are last
    fn validate_terminal_last(&self, body: &HandlerBody, issues: &mut Vec<ValidationIssue>) {
        let statements = &body.statements;
        if statements.is_empty() {
            return;
        }

        // Find index of last terminal statement
        let mut terminal_index: Option<usize> = None;
        for (i, stmt) in statements.iter().enumerate() {
            if self.is_terminal_statement(stmt) {
                terminal_index = Some(i);
            }
        }

        // Check if terminal statement isn't last
        if let Some(idx) = terminal_index {
            let last_idx = statements.len() - 1;
            if idx != last_idx {
                // Check if remaining statements are non-trivial
                let has_non_trivial_after = statements[idx + 1..].iter().any(|s| {
                    !matches!(s, Statement::Return(_))
                });
                if has_non_trivial_after {
                    let span = match &statements[idx] {
                        Statement::Transition(t) => t.span.clone(),
                        Statement::Forward(f) => f.span.clone(),
                        _ => body.span.clone(),
                    };
                    issues.push(
                        ValidationIssue::error(
                            "E400",
                            "Transition/forward must be the last statement in its containing block"
                        )
                        .with_span(span)
                        .with_note("Code after a transition is unreachable")
                        .with_fix("Move the transition to the end of the block or remove code after it")
                    );
                }
            }
        }
    }

    /// Check if statement is terminal
    fn is_terminal_statement(&self, stmt: &Statement) -> bool {
        matches!(stmt, Statement::Transition(_) | Statement::Forward(_))
    }

    /// Validate transition using basic state map
    fn validate_transition(
        &self,
        system_name: &str,
        _state: &StateAst,
        transition: &TransitionAst,
        state_map: &HashMap<String, &StateAst>,
        arcanum: &Arcanum,
        issues: &mut Vec<ValidationIssue>,
    ) {
        // E402: Check target state exists (basic check)
        if !state_map.contains_key(&transition.target) {
            // Use Arcanum for "did you mean" suggestions
            let suggestion = arcanum.validate_transition(system_name, &transition.target)
                .err()
                .unwrap_or_else(|| format!("Unknown state '{}'", transition.target));

            issues.push(
                ValidationIssue::error("E402", suggestion)
                    .with_span(transition.span.clone())
                    .with_note(format!(
                        "Available states: {}",
                        self.format_available_states(state_map)
                    ))
                    .with_fix(format!(
                        "Add state ${}{{}} or correct the state name",
                        transition.target
                    ))
            );
        } else {
            // E405: Check STATE PARAMETER arity
            // Transition args like -> $State(a, b) are passed to state params $State(a, b)
            let target_state = state_map.get(&transition.target).unwrap();
            if target_state.params.len() != transition.args.len() {
                issues.push(
                    ValidationIssue::error(
                        "E405",
                        format!(
                            "State '{}' expects {} parameters but {} provided",
                            transition.target,
                            target_state.params.len(),
                            transition.args.len()
                        )
                    )
                    .with_span(transition.span.clone())
                    .with_note(format!(
                        "State '{}' parameters: {}",
                        transition.target,
                        self.format_params(target_state)
                    ))
                    .with_fix(format!(
                        "Provide {} argument(s) to the transition",
                        target_state.params.len()
                    ))
                );
            }
        }
    }

    /// Validate forward statement
    fn validate_forward(
        &self,
        state: &StateAst,
        forward: &ForwardAst,
        state_map: &HashMap<String, &StateAst>,
        issues: &mut Vec<ValidationIssue>,
    ) {
        // E403: Forward requires parent
        if state.parent.is_none() {
            issues.push(
                ValidationIssue::error(
                    "E403",
                    format!(
                        "State '{}' cannot forward event '{}' - no parent state defined",
                        state.name, forward.event
                    )
                )
                .with_span(forward.span.clone())
                .with_note("Forward (>>) is only valid in hierarchical state machines")
                .with_fix(format!(
                    "Add a parent state using '${}' => $ParentState {{ }}",
                    state.name
                ))
            );
        } else {
            // Check parent exists
            let parent_name = state.parent.as_ref().unwrap();
            if !state_map.contains_key(parent_name) {
                issues.push(
                    ValidationIssue::error(
                        "E403",
                        format!("Cannot forward to invalid parent state '{}'", parent_name)
                    )
                    .with_span(forward.span.clone())
                    .with_fix("Correct the parent state name")
                );
            }
        }
    }

    /// Format available states for error messages
    fn format_available_states(&self, state_map: &HashMap<String, &StateAst>) -> String {
        let mut states: Vec<String> = state_map.keys().cloned().collect();
        states.sort();
        if states.is_empty() {
            "(none)".to_string()
        } else {
            states.join(", ")
        }
    }

    /// Format state parameters
    fn format_params(&self, state: &StateAst) -> String {
        if state.params.is_empty() {
            "(none)".to_string()
        } else {
            state.params
                .iter()
                .map(|p| format!("{}: {:?}", p.name, p.param_type))
                .collect::<Vec<_>>()
                .join(", ")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame_c::v4::arcanum::build_arcanum_from_frame_ast;
    use crate::frame_c::v4::frame_parser::FrameParser;
    use crate::frame_c::v4::frame_ast::TargetLanguage;

    fn make_context() -> ValidationContext<'static> {
        static CONFIG: crate::frame_c::v4::validation::types::ValidationConfig =
            crate::frame_c::v4::validation::types::ValidationConfig {
                warnings_as_errors: false,
                suppress: Vec::new(),
                verbose: false,
                max_errors: 0,
            };
        ValidationContext::new(&CONFIG)
    }

    #[test]
    fn test_e402_unknown_state() {
        let source = r#"
@@system Test {
    machine:
        $Start {
            go() { -> $Unknown() }
        }
}"#;

        let mut parser = FrameParser::new(source.as_bytes(), TargetLanguage::Python3);
        let ast = parser.parse_module().unwrap();
        let arcanum = build_arcanum_from_frame_ast(&ast);
        let mut ctx = make_context();

        let pass = SemanticPass;
        let issues = pass.run(&ast, &arcanum, &mut ctx);

        assert!(issues.iter().any(|i| i.code == "E402"));
    }

    #[test]
    fn test_e403_invalid_parent() {
        let source = r#"
@@system Test {
    machine:
        $Child => $InvalidParent {
            event() { }
        }
        $ActualParent { }
}"#;

        let mut parser = FrameParser::new(source.as_bytes(), TargetLanguage::Python3);
        let ast = parser.parse_module().unwrap();
        let arcanum = build_arcanum_from_frame_ast(&ast);
        let mut ctx = make_context();

        let pass = SemanticPass;
        let issues = pass.run(&ast, &arcanum, &mut ctx);

        assert!(issues.iter().any(|i| i.code == "E403"));
    }

    #[test]
    fn test_e405_state_param_mismatch() {
        // Test: transition args go to STATE PARAMS
        // $Target(x: int) has 1 param, but we pass 3 args
        let source = r#"
@@system Test {
    machine:
        $Start {
            go() { -> $Target(1, 2, 3) }
        }
        $Target(x: int) { }
}"#;

        let mut parser = FrameParser::new(source.as_bytes(), TargetLanguage::Python3);
        let ast = parser.parse_module().unwrap();
        let arcanum = build_arcanum_from_frame_ast(&ast);
        let mut ctx = make_context();

        let pass = SemanticPass;
        let issues = pass.run(&ast, &arcanum, &mut ctx);

        // Should report state expects 1 param but 3 provided
        assert!(issues.iter().any(|i| i.code == "E405" && i.message.contains("expects 1")));
    }

    #[test]
    fn test_e405_state_no_params() {
        // Test: transition passes args but state has no params
        let source = r#"
@@system Test {
    machine:
        $Start {
            go() { -> $Target(1, 2, 3) }
        }
        $Target { }
}"#;

        let mut parser = FrameParser::new(source.as_bytes(), TargetLanguage::Python3);
        let ast = parser.parse_module().unwrap();
        let arcanum = build_arcanum_from_frame_ast(&ast);
        let mut ctx = make_context();

        let pass = SemanticPass;
        let issues = pass.run(&ast, &arcanum, &mut ctx);

        // Should report state expects 0 params but 3 provided
        assert!(issues.iter().any(|i| i.code == "E405" && i.message.contains("expects 0")));
    }

    #[test]
    fn test_valid_system() {
        let source = r#"
@@system Valid {
    machine:
        $Idle {
            start() { -> $Active() }
        }
        $Active {
            stop() { -> $Idle() }
        }
}"#;

        let mut parser = FrameParser::new(source.as_bytes(), TargetLanguage::Python3);
        let ast = parser.parse_module().unwrap();
        let arcanum = build_arcanum_from_frame_ast(&ast);
        let mut ctx = make_context();

        let pass = SemanticPass;
        let issues = pass.run(&ast, &arcanum, &mut ctx);

        assert!(issues.is_empty(), "Expected no issues but got: {:?}", issues);
    }
}
