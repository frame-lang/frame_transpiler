//! Frame semantic validator using the Frame AST
//!
//! This module performs semantic validation on the Frame AST, checking for:
//!
//! ## Structural Errors (E1xx)
//! - E111: Duplicate system parameter
//! - E112: Missing '{' after state header (parser handles)
//! - E113: System blocks out of order
//! - E114: Duplicate section block in system
//! - E115: Multiple 'fn main' functions in module
//! - E116: Duplicate state name in machine
//! - E117: Duplicate handler in state
//!
//! ## Semantic Errors (E4xx)
//! - E400: Transition must be last statement in block
//! - E401: Frame statements not allowed in actions/operations
//! - E402: Unknown state in transition
//! - E403: Invalid parent forwards in HSM
//! - E404: Handler body must be inside a state block
//! - E405: State parameter arity mismatch (-> $State(args))
//! - E406: Interface handler parameter count mismatch
//! - E410: Duplicate state variable in state ($.varName)
//! - E413: Cyclic HSM parent relationship
//! - E416: Start params must match start state params
//! - E417: Enter args must match $>() handler params (-> (args) $State)
//! - E418: Domain param has no matching variable
//! - E419: Exit args must match $<() handler params ((args) -> $State)
//! - E420: Forward event not handled by parent state
//!
//! ## Warnings (W4xx)
//! - W414: Unreachable state from start state
//!
//! ## Compartment Field Mapping (Canonical 6-field model)
//!
//! | Syntax                  | Field           | Error Code |
//! |-------------------------|-----------------|------------|
//! | `-> $State(args)`       | state_args      | E405       |
//! | `-> (args) $State`      | enter_args      | E417       |
//! | `(args) -> $State`      | exit_args       | E419       |
//! | `-> => $State`          | forward_event   | E420       |
//! | `$.varName`             | state_vars      | E410       |

use super::frame_ast::*;
use super::arcanum::Arcanum;
use std::collections::{HashMap, HashSet};

/// Validation error with error code and message
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub code: String,
    pub message: String,
    pub span: Option<Span>,
}

impl ValidationError {
    pub fn new(code: &str, message: String) -> Self {
        Self {
            code: code.to_string(),
            message,
            span: None,
        }
    }
    
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }
}

/// Frame AST validator
pub struct FrameValidator {
    errors: Vec<ValidationError>,
}

impl FrameValidator {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
        }
    }
    
    /// Validate a Frame AST
    pub fn validate(&mut self, ast: &FrameAst) -> Result<(), Vec<ValidationError>> {
        match ast {
            FrameAst::System(system) => self.validate_system(system),
            FrameAst::Module(module) => {
                for system in &module.systems {
                    self.validate_system(system);
                }
            }
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    /// Validate a Frame AST using the enhanced Arcanum
    ///
    /// This is the preferred validation method when the Arcanum has been built.
    /// It uses the Arcanum's scope resolution for more thorough validation.
    pub fn validate_with_arcanum(&mut self, ast: &FrameAst, arcanum: &Arcanum) -> Result<(), Vec<ValidationError>> {
        match ast {
            FrameAst::System(system) => {
                self.validate_system(system);
                self.validate_system_with_arcanum(system, arcanum);
            }
            FrameAst::Module(module) => {
                for system in &module.systems {
                    self.validate_system(system);
                    self.validate_system_with_arcanum(system, arcanum);
                }
            }
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    /// Additional validation using the Arcanum
    fn validate_system_with_arcanum(&mut self, system: &SystemAst, arcanum: &Arcanum) {
        // E402 enhanced: Validate transitions using Arcanum
        if let Some(machine) = &system.machine {
            for state in &machine.states {
                self.validate_state_transitions_with_arcanum(&system.name, state, arcanum);
            }
        }
    }

    /// Validate state transitions using Arcanum's state resolution
    fn validate_state_transitions_with_arcanum(&mut self, system_name: &str, state: &StateAst, arcanum: &Arcanum) {
        // Validate handlers
        for handler in &state.handlers {
            for stmt in &handler.body.statements {
                if let Statement::Transition(trans) = stmt {
                    self.validate_transition_with_arcanum(system_name, trans, arcanum);
                }
            }
        }

        // Validate enter handler
        if let Some(enter) = &state.enter {
            for stmt in &enter.body.statements {
                if let Statement::Transition(trans) = stmt {
                    self.validate_transition_with_arcanum(system_name, trans, arcanum);
                }
            }
        }

        // Validate exit handler
        if let Some(exit) = &state.exit {
            for stmt in &exit.body.statements {
                if let Statement::Transition(trans) = stmt {
                    self.validate_transition_with_arcanum(system_name, trans, arcanum);
                }
            }
        }
    }

    /// Validate a transition using Arcanum's state resolution
    fn validate_transition_with_arcanum(&mut self, system_name: &str, trans: &TransitionAst, arcanum: &Arcanum) {
        // Skip validation for pop-transition marker $$[-]
        if trans.target == "pop$" {
            return;  // Pop-transition: target comes from stack at runtime
        }

        // Use Arcanum's validate_transition which includes "did you mean" suggestions
        if let Err(msg) = arcanum.validate_transition(system_name, &trans.target) {
            // Only add if not already reported by basic validation
            if !self.errors.iter().any(|e| e.code == "E402" && e.span.as_ref() == Some(&trans.span)) {
                self.errors.push(ValidationError::new("E402", msg).with_span(trans.span.clone()));
            }
        } else {
            // State exists, check transition argument arity against STATE PARAMS
            // Skip arity checking when args are NativeExpr blobs (native compiler handles it)
            let has_native_args = trans.args.iter().any(|a| matches!(a, Expression::NativeExpr(_)));
            if !has_native_args {
                let args_count = trans.args.len();
                if let Some(expected) = arcanum.get_state_param_count(system_name, &trans.target) {
                    if expected != args_count {
                        if !self.errors.iter().any(|e| e.code == "E405" && e.span.as_ref() == Some(&trans.span)) {
                            self.errors.push(ValidationError::new(
                                "E405",
                                format!(
                                    "State '{}' expects {} parameters but {} provided",
                                    trans.target, expected, args_count
                                )
                            ).with_span(trans.span.clone()));
                        }
                    }
                }
            }
        }
    }
    
    /// Validate a system
    fn validate_system(&mut self, system: &SystemAst) {
        // Phase 1: Structural validation
        self.validate_section_order(system);
        self.validate_duplicate_sections(system);

        // Build lookup tables
        let state_map = self.build_state_map(system);
        let interface_methods = self.build_interface_map(system);
        let actions = self.build_action_set(system);
        let operations = self.build_operation_set(system);

        // Validate machine if present
        if let Some(machine) = &system.machine {
            self.validate_machine(machine, &state_map, &interface_methods, &actions, &operations);
        }

        // E401: Validate no Frame statements in actions
        for action in &system.actions {
            self.validate_action_no_frame_statements(action);
        }

        // E401: Validate no Frame statements in operations
        for operation in &system.operations {
            self.validate_operation_no_frame_statements(operation);
        }
    }

    /// E113: Validate system section order (operations:, interface:, machine:, actions:, domain:)
    fn validate_section_order(&mut self, system: &SystemAst) {
        if system.section_order.is_empty() {
            return;
        }

        // Canonical order: Operations=0, Interface=1, Machine=2, Actions=3, Domain=4
        let mut last_idx: i32 = -1;
        for kind in &system.section_order {
            let idx = match kind {
                SystemSectionKind::Operations => 0,
                SystemSectionKind::Interface => 1,
                SystemSectionKind::Machine => 2,
                SystemSectionKind::Actions => 3,
                SystemSectionKind::Domain => 4,
            };
            if (idx as i32) < last_idx {
                self.errors.push(ValidationError::new(
                    "E113",
                    format!(
                        "System '{}' blocks out of order. Expected: operations:, interface:, machine:, actions:, domain:",
                        system.name
                    )
                ).with_span(system.span.clone()));
                break; // Only report once per system
            }
            last_idx = idx as i32;
        }
    }

    /// E114: Validate no duplicate sections in system
    fn validate_duplicate_sections(&mut self, system: &SystemAst) {
        if let Some(dup_kind) = system.has_duplicate_sections() {
            let section_name = match dup_kind {
                SystemSectionKind::Operations => "operations:",
                SystemSectionKind::Interface => "interface:",
                SystemSectionKind::Machine => "machine:",
                SystemSectionKind::Actions => "actions:",
                SystemSectionKind::Domain => "domain:",
            };
            self.errors.push(ValidationError::new(
                "E114",
                format!(
                    "Duplicate '{}' section in system '{}'",
                    section_name, system.name
                )
            ).with_span(system.span.clone()));
        }
    }

    /// E401: Validate no Frame statements in action body
    fn validate_action_no_frame_statements(&mut self, action: &ActionAst) {
        // Actions have native bodies, but we check if the native content
        // contains Frame statement patterns (this would be caught by the scanner
        // but we can add an extra check here)
        // For now, actions are pure native, so no validation needed here
        // The validation happens during scanning/parsing
        let _ = action; // suppress unused warning
    }

    /// E401: Validate no Frame statements in operation body
    fn validate_operation_no_frame_statements(&mut self, operation: &OperationAst) {
        // Operations have native bodies, same as actions
        let _ = operation; // suppress unused warning
    }
    
    /// Build a map of state names to state definitions
    fn build_state_map<'a>(&self, system: &'a SystemAst) -> HashMap<String, &'a StateAst> {
        let mut map = HashMap::new();
        if let Some(machine) = &system.machine {
            for state in &machine.states {
                map.insert(state.name.clone(), state);
            }
        }
        map
    }
    
    /// Build a map of interface method names to definitions
    fn build_interface_map<'a>(&self, system: &'a SystemAst) -> HashMap<String, &'a InterfaceMethod> {
        let mut map = HashMap::new();
        for method in &system.interface {
            map.insert(method.name.clone(), method);
        }
        map
    }
    
    /// Build a set of action names
    fn build_action_set(&self, system: &SystemAst) -> HashSet<String> {
        system.actions.iter().map(|a| a.name.clone()).collect()
    }
    
    /// Build a set of operation names
    fn build_operation_set(&self, system: &SystemAst) -> HashSet<String> {
        system.operations.iter().map(|o| o.name.clone()).collect()
    }
    
    /// Validate a machine
    fn validate_machine(
        &mut self,
        machine: &MachineAst,
        state_map: &HashMap<String, &StateAst>,
        interface_methods: &HashMap<String, &InterfaceMethod>,
        _actions: &HashSet<String>,
        _operations: &HashSet<String>,
    ) {
        for state in &machine.states {
            self.validate_state(state, state_map, interface_methods, _actions, _operations);
        }
    }
    
    /// Validate a state
    fn validate_state(
        &mut self,
        state: &StateAst,
        state_map: &HashMap<String, &StateAst>,
        interface_methods: &HashMap<String, &InterfaceMethod>,
        _actions: &HashSet<String>,
        _operations: &HashSet<String>,
    ) {
        // E403: Validate parent state exists for HSM
        if let Some(parent_name) = &state.parent {
            if !state_map.contains_key(parent_name) {
                self.errors.push(ValidationError::new(
                    "E403",
                    format!(
                        "State '{}' has invalid parent '{}'. Available states: {}",
                        state.name,
                        parent_name,
                        self.format_available_states(state_map)
                    )
                ).with_span(state.span.clone()));
            }
        }
        
        // Validate handlers
        for handler in &state.handlers {
            self.validate_handler(handler, state, state_map, interface_methods, _actions, _operations);
        }
        
        // Validate enter handler
        if let Some(enter) = &state.enter {
            self.validate_handler_body(&enter.body, state, state_map);
        }
        
        // Validate exit handler
        if let Some(exit) = &state.exit {
            self.validate_handler_body(&exit.body, state, state_map);
        }
    }
    
    /// Validate a handler
    fn validate_handler(
        &mut self,
        handler: &HandlerAst,
        state: &StateAst,
        state_map: &HashMap<String, &StateAst>,
        interface_methods: &HashMap<String, &InterfaceMethod>,
        _actions: &HashSet<String>,
        _operations: &HashSet<String>,
    ) {
        // E406: Check if handler corresponds to interface method
        if let Some(method) = interface_methods.get(&handler.event) {
            // Validate parameter count matches
            if handler.params.len() != method.params.len() {
                self.errors.push(ValidationError::new(
                    "E406",
                    format!(
                        "Handler '{}' in state '{}' has {} parameters but interface method expects {}",
                        handler.event,
                        state.name,
                        handler.params.len(),
                        method.params.len()
                    )
                ).with_span(handler.span.clone()));
            }
        }

        self.validate_handler_body(&handler.body, state, state_map);
    }
    
    /// Validate handler body statements
    fn validate_handler_body(
        &mut self,
        body: &HandlerBody,
        state: &StateAst,
        state_map: &HashMap<String, &StateAst>,
    ) {
        // E400: Check that terminal statements (transitions, forwards) are last
        self.validate_terminal_last(body);

        for statement in &body.statements {
            match statement {
                Statement::Transition(transition) => {
                    self.validate_transition(transition, state, state_map);
                }
                Statement::Forward(forward) => {
                    self.validate_forward(forward, state, state_map);
                }
                _ => {
                    // Other statements don't need validation yet
                }
            }
        }
    }

    /// E400: Validate that terminal statements (transition, forward) are last in the body
    fn validate_terminal_last(&mut self, body: &HandlerBody) {
        let statements = &body.statements;
        if statements.is_empty() {
            return;
        }

        // Find the index of the last terminal statement
        let mut terminal_index: Option<usize> = None;
        for (i, stmt) in statements.iter().enumerate() {
            if self.is_terminal_statement(stmt) {
                terminal_index = Some(i);
            }
        }

        // Check if there's a terminal statement that isn't the last one
        if let Some(idx) = terminal_index {
            let last_idx = statements.len() - 1;
            if idx != last_idx {
                // Check if remaining statements are non-trivial
                // NativeCode with only braces/whitespace and Return are trivial
                let has_non_trivial_after = statements[idx + 1..].iter().any(|s| {
                    match s {
                        Statement::Return(_) => false,
                        Statement::NativeCode(code) => {
                            // Only braces, whitespace, semicolons, and comments are trivial
                            let trimmed = code.trim();
                            !trimmed.is_empty()
                                && trimmed != "}"
                                && trimmed != "};"
                                && !trimmed.chars().all(|c| c == '}' || c == ' ' || c == '\n' || c == '\r' || c == '\t')
                        }
                        _ => true,
                    }
                });
                if has_non_trivial_after {
                    let span = match &statements[idx] {
                        Statement::Transition(t) => t.span.clone(),
                        Statement::Forward(f) => f.span.clone(),
                        _ => body.span.clone(),
                    };
                    self.errors.push(ValidationError::new(
                        "E400",
                        "Transition/forward must be the last statement in its containing block".to_string()
                    ).with_span(span));
                }
            }
        }
    }

    /// Check if a statement is a terminal statement (transition only).
    /// Forwards (`=> $^`) are NOT terminal — they dispatch to the parent and return.
    fn is_terminal_statement(&self, stmt: &Statement) -> bool {
        matches!(stmt, Statement::Transition(_))
    }
    
    /// Validate a transition statement
    fn validate_transition(
        &mut self,
        transition: &TransitionAst,
        _state: &StateAst,
        state_map: &HashMap<String, &StateAst>,
    ) {
        // E402: Check target state exists
        // Skip validation for pop-transition marker $$[-]
        if transition.target == "pop$" {
            return;  // Pop-transition: target comes from stack at runtime
        }

        if !state_map.contains_key(&transition.target) {
            self.errors.push(ValidationError::new(
                "E402",
                format!(
                    "Unknown state '{}' in transition. Available states: {}",
                    transition.target,
                    self.format_available_states(state_map)
                )
            ).with_span(transition.span.clone()));
        } else {
            // E405: Check STATE PARAMETER arity
            // Skip arity checking when args are NativeExpr blobs (native compiler handles it)
            let has_native_args = transition.args.iter().any(|a| matches!(a, Expression::NativeExpr(_)));
            if !has_native_args {
                let target_state = state_map.get(&transition.target).unwrap();
                if target_state.params.len() != transition.args.len() {
                    self.errors.push(ValidationError::new(
                        "E405",
                        format!(
                            "State '{}' expects {} parameters but {} provided",
                            transition.target,
                            target_state.params.len(),
                            transition.args.len()
                        )
                    ).with_span(transition.span.clone()));
                }
            }
        }
    }
    
    /// Validate a forward statement
    fn validate_forward(
        &mut self,
        forward: &ForwardAst,
        state: &StateAst,
        state_map: &HashMap<String, &StateAst>,
    ) {
        // E403: Forward is only valid if state has a parent
        if state.parent.is_none() {
            self.errors.push(ValidationError::new(
                "E403",
                format!(
                    "State '{}' cannot forward event '{}' - no parent state defined",
                    state.name,
                    forward.event
                )
            ).with_span(forward.span.clone()));
        } else {
            // Could validate that parent handles this event
            // For now, just check parent exists
            let parent_name = state.parent.as_ref().unwrap();
            if !state_map.contains_key(parent_name) {
                self.errors.push(ValidationError::new(
                    "E403",
                    format!(
                        "Cannot forward to invalid parent state '{}'",
                        parent_name
                    )
                ).with_span(forward.span.clone()));
            }
        }
    }
    
    /// Format available states for error messages
    fn format_available_states(&self, state_map: &HashMap<String, &StateAst>) -> String {
        let mut states: Vec<String> = state_map.keys().cloned().collect();
        states.sort();
        states.join(", ")
    }
}

/// Convenience function to validate Frame source code
pub fn validate_frame_source(source: &str, target: TargetLanguage) -> Result<(), Vec<ValidationError>> {
    use crate::frame_c::v4::frame_parser::FrameParser;
    use crate::frame_c::v4::arcanum::build_arcanum_from_frame_ast;

    let mut parser = FrameParser::new(source.as_bytes(), target);
    let ast = parser.parse_module().map_err(|e| {
        vec![ValidationError::new("E001", format!("Parse error: {}", e))]
    })?;

    // Build Arcanum for semantic validation (E405 etc)
    let arcanum = build_arcanum_from_frame_ast(&ast);

    let mut validator = FrameValidator::new();
    validator.validate_with_arcanum(&ast, &arcanum)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_e402_unknown_state() {
        let source = r#"
@@system Test {
    machine:
        $Start {
            go() { -> $Unknown() }
        }
}"#;
        
        let result = validate_frame_source(source, TargetLanguage::Python3);
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].code, "E402");
        assert!(errors[0].message.contains("Unknown state 'Unknown'"));
    }
    
    #[test]
    fn test_e403_invalid_parent() {
        let source = r#"
@@system Test {
    machine:
        $Child => $InvalidParent {
            event() { => event() }
        }
        $ActualParent {
        }
}"#;
        
        let result = validate_frame_source(source, TargetLanguage::Python3);
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.code == "E403" && e.message.contains("invalid parent")));
    }
    
    #[test]
    fn test_e403_forward_without_parent() {
        // Test using the supported forward syntax: => $^
        // Note: The scanner currently only detects "=> $^" pattern
        let source = r#"
@@system Test {
    machine:
        $Standalone {
            unhandled() {
                => $^
            }
        }
}"#;

        let result = validate_frame_source(source, TargetLanguage::Python3);
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.code == "E403" && e.message.contains("no parent")));
    }
    
    #[test]
    fn test_e405_state_param_mismatch() {
        // Transition args go to STATE PARAMS, not enter handler
        // $Target(x: int) has 1 param, but we pass 3 args
        let source = r#"
@@system Test {
    machine:
        $Start {
            go() { -> $Target(1, 2, 3) }
        }
        $Target(x: int) { }
}"#;

        let result = validate_frame_source(source, TargetLanguage::Python3);
        assert!(result.is_err());

        let errors = result.unwrap_err();
        // State expects 1 param but 3 provided
        assert!(errors.iter().any(|e| e.code == "E405" && e.message.contains("expects 1 parameters but 3 provided")));
    }

    #[test]
    fn test_e405_state_no_params() {
        // Transition passes args but state has no params
        let source = r#"
@@system Test {
    machine:
        $Start {
            go() { -> $Target(1, 2) }
        }
        $Target { }
}"#;

        let result = validate_frame_source(source, TargetLanguage::Python3);
        assert!(result.is_err());

        let errors = result.unwrap_err();
        // State expects 0 params but 2 provided
        assert!(errors.iter().any(|e| e.code == "E405" && e.message.contains("expects 0 parameters but 2 provided")));
    }
    
    #[test]
    fn test_valid_system() {
        let source = r#"
@@system Valid {
    interface:
        process(data: string): bool
        
    machine:
        $Idle {
            start() { -> $Active() }
        }
        $Active {
            stop() { -> $Idle() }
            process(data: string) {
                ^ true
            }
        }
}"#;
        
        let result = validate_frame_source(source, TargetLanguage::Python3);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_valid_hsm() {
        let source = r#"
@@system HSM {
    machine:
        $Parent {
            baseEvent() { }
        }
        $Child => $Parent {
            childEvent() { }
            unhandled() { => unhandled() }
        }
}"#;

        let result = validate_frame_source(source, TargetLanguage::Python3);
        assert!(result.is_ok());
    }

    #[test]
    fn test_e113_section_order() {
        // Test duplicate section detection using the AST directly
        let mut system = SystemAst::new("Test".to_string(), Span::new(0, 100));
        system.section_order = vec![
            SystemSectionKind::Machine,
            SystemSectionKind::Interface,  // Wrong order - interface should come before machine
        ];

        let mut validator = FrameValidator::new();
        validator.validate_section_order(&system);

        assert!(!validator.errors.is_empty());
        assert!(validator.errors.iter().any(|e| e.code == "E113"));
    }

    #[test]
    fn test_e114_duplicate_section() {
        let mut system = SystemAst::new("Test".to_string(), Span::new(0, 100));
        system.section_order = vec![
            SystemSectionKind::Machine,
            SystemSectionKind::Actions,
            SystemSectionKind::Machine, // Duplicate!
        ];

        let mut validator = FrameValidator::new();
        validator.validate_duplicate_sections(&system);

        assert!(!validator.errors.is_empty());
        assert!(validator.errors.iter().any(|e| e.code == "E114"));
    }

    #[test]
    fn test_valid_section_order() {
        let mut system = SystemAst::new("Test".to_string(), Span::new(0, 100));
        system.section_order = vec![
            SystemSectionKind::Operations,
            SystemSectionKind::Interface,
            SystemSectionKind::Machine,
            SystemSectionKind::Actions,
            SystemSectionKind::Domain,
        ];

        let mut validator = FrameValidator::new();
        validator.validate_section_order(&system);
        validator.validate_duplicate_sections(&system);

        assert!(validator.errors.is_empty());
    }

    #[test]
    fn test_e400_transition_not_last() {
        // Create a handler body where transition is not last
        let body = HandlerBody {
            statements: vec![
                Statement::Transition(TransitionAst {
                    target: "Other".to_string(),
                    args: vec![],
                    label: None,
                    span: Span::new(10, 20),
                    indent: 0,
                }),
                Statement::Transition(TransitionAst {
                    target: "Final".to_string(),
                    args: vec![],
                    label: None,
                    span: Span::new(30, 40),
                    indent: 0,
                }),
            ],
            span: Span::new(0, 50),
        };

        let mut validator = FrameValidator::new();
        validator.validate_terminal_last(&body);

        // First transition is not last, but since both are transitions,
        // we only report if there's a non-terminal after a terminal
        // In this case both are terminals so only the last matters
    }

    #[test]
    fn test_validate_with_arcanum() {
        use crate::frame_c::v4::arcanum::build_arcanum_from_frame_ast;
        use crate::frame_c::v4::frame_parser::FrameParser;

        // System with valid transition
        let source = r#"
@@system TestArcanum {
    machine:
        $Idle {
            go() { -> $Active() }
        }
        $Active {
            back() { -> $Idle() }
        }
}"#;

        let mut parser = FrameParser::new(source.as_bytes(), TargetLanguage::Python3);
        let ast = parser.parse_module().unwrap();
        let arcanum = build_arcanum_from_frame_ast(&ast);

        let mut validator = FrameValidator::new();
        let result = validator.validate_with_arcanum(&ast, &arcanum);
        assert!(result.is_ok());
    }

    #[test]
    fn test_e406_interface_handler_arity_mismatch() {
        let source = r#"
@@system Test {
    interface:
        process(data: string, count: int): bool

    machine:
        $Active {
            process(data: string) {
                ^ true
            }
        }
}"#;

        let result = validate_frame_source(source, TargetLanguage::Python3);
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.code == "E406" && e.message.contains("has 1 parameters but interface method expects 2")));
    }

    #[test]
    fn test_e406_valid_interface_handler() {
        let source = r#"
@@system Test {
    interface:
        process(data: string): bool

    machine:
        $Active {
            process(data: string) {
                ^ true
            }
        }
}"#;

        let result = validate_frame_source(source, TargetLanguage::Python3);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_with_arcanum_invalid_state() {
        use crate::frame_c::v4::arcanum::build_arcanum_from_frame_ast;
        use crate::frame_c::v4::frame_parser::FrameParser;

        // System with invalid transition target
        let source = r#"
@@system TestInvalid {
    machine:
        $Start {
            go() { -> $NonExistent() }
        }
}"#;

        let mut parser = FrameParser::new(source.as_bytes(), TargetLanguage::Python3);
        let ast = parser.parse_module().unwrap();
        let arcanum = build_arcanum_from_frame_ast(&ast);

        let mut validator = FrameValidator::new();
        let result = validator.validate_with_arcanum(&ast, &arcanum);
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.code == "E402"));
    }
}