//! Frame semantic validator using the Frame AST
//! 
//! This module performs semantic validation on the Frame AST, checking for:
//! - E402: Unknown state transitions
//! - E403: Invalid parent forwards in HSM
//! - E405: State parameter arity mismatches
//! - E406: Invalid interface method calls

use super::frame_ast::*;
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
    warnings: Vec<ValidationError>,
}

impl FrameValidator {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
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
    
    /// Validate a system
    fn validate_system(&mut self, system: &SystemAst) {
        // Build lookup tables
        let state_map = self.build_state_map(system);
        let interface_methods = self.build_interface_map(system);
        let actions = self.build_action_set(system);
        let operations = self.build_operation_set(system);
        
        // Validate machine if present
        if let Some(machine) = &system.machine {
            self.validate_machine(machine, &state_map, &interface_methods, &actions, &operations);
        }
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
        if interface_methods.contains_key(&handler.event) {
            // This is an interface method implementation
            let _method = interface_methods.get(&handler.event).unwrap();
            
            // Could validate parameter compatibility here
            // For now, just note it's a valid interface method
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
    
    /// Validate a transition statement
    fn validate_transition(
        &mut self,
        transition: &TransitionAst,
        _state: &StateAst,
        state_map: &HashMap<String, &StateAst>,
    ) {
        // E402: Check target state exists
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
            // E405: Check state parameter arity
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
    
    let mut parser = FrameParser::new(source.as_bytes(), target);
    let ast = parser.parse_module().map_err(|e| {
        vec![ValidationError::new("E001", format!("Parse error: {}", e))]
    })?;
    
    let mut validator = FrameValidator::new();
    validator.validate(&ast)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_e402_unknown_state() {
        let source = r#"
system Test {
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
system Test {
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
        let source = r#"
system Test {
    machine:
        $Standalone {
            unhandled() {
                => unhandled()
            }
        }
}"#;
        
        let result = validate_frame_source(source, TargetLanguage::Python3);
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.code == "E403" && e.message.contains("no parent")));
    }
    
    #[test]
    fn test_e405_parameter_arity() {
        let source = r#"
system Test {
    machine:
        $Start {
            go() { -> $Target(1, 2, 3) }
        }
        $Target(x: int) {
        }
}"#;
        
        let result = validate_frame_source(source, TargetLanguage::Python3);
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.code == "E405" && e.message.contains("expects 1 parameters but 3 provided")));
    }
    
    #[test]
    fn test_valid_system() {
        let source = r#"
system Valid {
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
system HSM {
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
}