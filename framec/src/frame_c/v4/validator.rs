// Frame v4 Validator - Semantic validation for Frame structures
//
// Validates Frame-specific rules like:
// - System parameter flow
// - State references
// - Interface implementation
// - Block ordering

use super::ast::*;
use super::error::ErrorsAcc;
use super::mir::MirItem;

pub fn validate(ast: &SystemAst) -> Result<(), ErrorsAcc> {
    let mut validator = Validator::new();
    validator.validate_system(ast)
}

struct Validator {
    errors: ErrorsAcc,
}

impl Validator {
    fn new() -> Self {
        Self {
            errors: ErrorsAcc::new(),
        }
    }

    fn validate_system(&mut self, ast: &SystemAst) -> Result<(), ErrorsAcc> {
        // Validate system parameter flow
        self.validate_system_params(ast);
        
        // Validate state references
        self.validate_state_references(ast);
        
        // Validate interface implementation
        self.validate_interface_implementation(ast);
        
        // Validate domain variables for domain params
        self.validate_domain_params(ast);
        
        if self.errors.has_errors() {
            Err(self.errors.clone())
        } else {
            Ok(())
        }
    }

    fn validate_system_params(&mut self, ast: &SystemAst) {
        // Check that start state params match initial state
        if !ast.params.start_state_params.is_empty() {
            if let Some(initial_state) = ast.initial_state() {
                // Check parameter count matches
                if initial_state.params.len() != ast.params.start_state_params.len() {
                    self.error(&format!(
                        "Start state parameter count mismatch: system declares {}, state {} expects {}",
                        ast.params.start_state_params.len(),
                        initial_state.name,
                        initial_state.params.len()
                    ));
                }
                
                // Check parameter names match
                for (sys_param, state_param) in ast.params.start_state_params.iter()
                    .zip(initial_state.params.iter()) {
                    if sys_param.name != state_param.name {
                        self.error(&format!(
                            "Start state parameter name mismatch: system has '{}', state has '{}'",
                            sys_param.name, state_param.name
                        ));
                    }
                }
            } else {
                self.error("System has start state parameters but no initial state defined");
            }
        }
        
        // Check that enter params match initial state's enter handler
        if !ast.params.enter_params.is_empty() {
            if let Some(initial_state) = ast.initial_state() {
                if let Some(enter_handler) = initial_state.enter_handler() {
                    // Check parameter count
                    if enter_handler.params.len() != ast.params.enter_params.len() {
                        self.error(&format!(
                            "Enter parameter count mismatch: system declares {}, handler expects {}",
                            ast.params.enter_params.len(),
                            enter_handler.params.len()
                        ));
                    }
                    
                    // Check parameter names
                    for (sys_param, handler_param) in ast.params.enter_params.iter()
                        .zip(enter_handler.params.iter()) {
                        if sys_param.name != handler_param.name {
                            self.error(&format!(
                                "Enter parameter name mismatch: system has '{}', handler has '{}'",
                                sys_param.name, handler_param.name
                            ));
                        }
                    }
                } else {
                    self.error("System has enter parameters but initial state has no enter handler");
                }
            } else {
                self.error("System has enter parameters but no initial state defined");
            }
        }
    }

    fn validate_state_references(&mut self, ast: &SystemAst) {
        // Check all state transitions reference valid states
        if let Some(machine) = &ast.machine {
            let state_names: Vec<&str> = machine.states.iter()
                .map(|s| s.name.as_str())
                .collect();
            
            for state in &machine.states {
                for handler in &state.handlers {
                    // Extract Frame statements from MIR
                    for stmt in handler.mir_block.frame_statements() {
                        if let MirItem::Transition { target, .. } = &stmt {
                            if !state_names.contains(&target.as_str()) {
                                self.error(&format!(
                                    "State '{}' referenced in transition does not exist",
                                    target
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    fn validate_interface_implementation(&mut self, ast: &SystemAst) {
        // Check that all interface methods are implemented in at least one state
        if let Some(interface) = &ast.interface {
            if let Some(machine) = &ast.machine {
                for method in &interface.methods {
                    let mut found = false;
                    
                    for state in &machine.states {
                        if state.find_handler(&method.name).is_some() {
                            found = true;
                            break;
                        }
                    }
                    
                    if !found {
                        self.error(&format!(
                            "Interface method '{}' is not implemented in any state",
                            method.name
                        ));
                    }
                }
            } else if !interface.methods.is_empty() {
                self.error("Interface methods defined but no machine block present");
            }
        }
    }

    fn validate_domain_params(&mut self, ast: &SystemAst) {
        // Check that all domain params have matching domain variables
        for param in &ast.params.domain_params {
            if ast.find_domain_var(&param.name).is_none() {
                self.error(&format!(
                    "Domain parameter '{}' has no matching domain variable",
                    param.name
                ));
            }
        }
    }

    fn error(&mut self, message: &str) {
        self.errors.push_error(message.to_string());
    }
}