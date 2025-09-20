// Call Chain Refactoring for PythonVisitor
// This module contains a simplified and unified approach to handling call chains
// to replace the complex, duplicated logic spread across multiple methods

use crate::frame_c::ast::*;
use crate::frame_c::symbol_table::*;
use std::collections::HashSet;

/// Encapsulates the logic for determining how to generate a call
pub struct CallResolver<'a> {
    arcanium: &'a Arcanum,
    in_standalone_function: bool,
    in_class_method: bool,
    current_system_actions: &'a Vec<String>,
    current_system_operations: &'a Vec<String>,
}

impl<'a> CallResolver<'a> {
    pub fn new(
        arcanium: &'a Arcanum,
        in_standalone_function: bool,
        in_class_method: bool,
        current_system_actions: &'a Vec<String>,
        current_system_operations: &'a Vec<String>,
    ) -> Self {
        CallResolver {
            arcanium,
            in_standalone_function,
            in_class_method,
            current_system_actions,
            current_system_operations,
        }
    }

    /// Resolve how to generate code for a method call
    /// Returns the prefix and method name to use
    pub fn resolve_call(&self, method_call: &CallExprNode) -> CallResolution {
        let method_name = &method_call.identifier.name.lexeme;
        
        match &method_call.context {
            CallContextType::SelfCall => {
                self.resolve_self_call(method_name)
            }
            CallContextType::StaticCall(class_name) => {
                CallResolution {
                    prefix: format!("{}.", class_name),
                    method_name: method_name.to_string(),
                    needs_parameters: true,
                }
            }
            CallContextType::ExternalCall => {
                self.resolve_external_call(method_call)
            }
        }
    }

    /// Resolve a self.method() call
    fn resolve_self_call(&self, method_name: &str) -> CallResolution {
        // In a class method, self.method() is a regular method call
        if self.in_class_method {
            return CallResolution {
                prefix: "self.".to_string(),
                method_name: method_name.to_string(),
                needs_parameters: true,
            };
        }

        // Check if it's an action (actions have underscore prefix in Python)
        if let Some((_action_symbol, _system_symbol)) = 
            self.arcanium.lookup_action_in_all_systems(method_name) {
            return CallResolution {
                prefix: "self.".to_string(),
                method_name: format!("_{}", method_name),
                needs_parameters: true,
            };
        }

        // Check if it's an operation
        if let Some((_operation_symbol, system_symbol)) = 
            self.arcanium.lookup_operation_in_all_systems(method_name) {
            let prefix = if !self.in_standalone_function {
                "self.".to_string()
            } else {
                format!("{}.", system_symbol.borrow().name)
            };
            return CallResolution {
                prefix,
                method_name: method_name.to_string(),
                needs_parameters: true,
            };
        }

        // Default: treat as domain method
        CallResolution {
            prefix: "self.".to_string(),
            method_name: method_name.to_string(),
            needs_parameters: true,
        }
    }

    /// Resolve an external call (may still be an action/operation without explicit self)
    fn resolve_external_call(&self, method_call: &CallExprNode) -> CallResolution {
        let method_name = &method_call.identifier.name.lexeme;
        
        // If there's a call chain, handle the chain prefix
        let has_call_chain = method_call.call_chain
            .as_ref()
            .map_or(false, |chain| !chain.is_empty());

        if has_call_chain {
            // Call chain will be handled separately
            return CallResolution {
                prefix: String::new(),  // Chain handled elsewhere
                method_name: method_name.to_string(),
                needs_parameters: true,
            };
        }

        // No call chain - check if it's an internal action/operation
        let action_name = if method_name.starts_with('_') {
            &method_name[1..]
        } else {
            method_name
        };

        // Check for action
        if self.is_action(action_name) {
            let prefix = if !self.in_standalone_function {
                "self.".to_string()
            } else {
                String::new()
            };
            return CallResolution {
                prefix,
                method_name: format!("_{}", action_name),
                needs_parameters: true,
            };
        }

        // Check for operation
        if self.is_operation(action_name) {
            let prefix = if !self.in_standalone_function {
                "self.".to_string()
            } else if let Some((_op, system)) = 
                self.arcanium.lookup_operation_in_all_systems(action_name) {
                format!("{}.", system.borrow().name)
            } else {
                String::new()
            };
            return CallResolution {
                prefix,
                method_name: action_name.to_string(),
                needs_parameters: true,
            };
        }

        // External function call
        CallResolution {
            prefix: String::new(),
            method_name: method_name.to_string(),
            needs_parameters: true,
        }
    }

    /// Check if a name is an action in the current context
    fn is_action(&self, name: &str) -> bool {
        self.arcanium.lookup_action_in_all_systems(name).is_some() ||
        self.current_system_actions.iter().any(|s| s == name)
    }

    /// Check if a name is an operation in the current context
    fn is_operation(&self, name: &str) -> bool {
        self.arcanium.lookup_operation_in_all_systems(name).is_some() ||
        self.current_system_operations.iter().any(|s| s == name)
    }
}

/// Result of resolving a call
pub struct CallResolution {
    pub prefix: String,        // e.g., "self.", "ClassName.", or empty
    pub method_name: String,   // e.g., "_action", "operation", "function"
    pub needs_parameters: bool, // Whether to process call_expr_list
}

