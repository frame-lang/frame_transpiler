// Semantic Analysis for Frame v0.62
// This module performs semantic analysis on the AST during the second parser pass
// Primary responsibility: Resolve call types to their actual semantic meaning

use crate::frame_c::ast::*;
use crate::frame_c::symbol_table::*;

pub struct SemanticAnalyzer<'a> {
    arcanum: &'a Arcanum,
    current_system: Option<&'a str>,
    in_standalone_function: bool,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new(arcanum: &'a Arcanum) -> Self {
        SemanticAnalyzer {
            arcanum,
            current_system: None,
            in_standalone_function: false,
        }
    }
    
    /// Resolve a call expression to its semantic type
    pub fn resolve_call(&self, call_expr: &CallExprNode) -> ResolvedCallType {
        let identifier = &call_expr.identifier.name.lexeme;
        
        // Handle self calls first
        if identifier.starts_with("self.") {
            let method_name = &identifier[5..]; // Strip "self."
            return self.resolve_self_call(method_name);
        }
        
        // Check if it's a call chain (e.g., Utils.add)
        if let Some(chain) = &call_expr.call_chain {
            if !chain.is_empty() {
                return self.resolve_chained_call(identifier, chain);
            }
        }
        
        // Check if it's an action (with or without underscore)
        let action_name = if identifier.starts_with('_') {
            &identifier[1..]
        } else {
            identifier
        };
        
        if let Some(system) = self.current_system {
            // Look up in current system
            if self.is_action_in_system(system, action_name) {
                return ResolvedCallType::Action(action_name.to_string());
            }
            
            if self.is_operation_in_system(system, action_name) {
                return ResolvedCallType::Operation(action_name.to_string());
            }
        }
        
        // Check if it's a known function
        if self.arcanum.lookup_function(identifier).is_some() {
            // This is a known function
            return ResolvedCallType::External(identifier.to_string());
        }
        
        // Default to external call
        ResolvedCallType::External(identifier.to_string())
    }
    
    fn resolve_self_call(&self, method_name: &str) -> ResolvedCallType {
        // Strip underscore if present
        let clean_name = if method_name.starts_with('_') {
            &method_name[1..]
        } else {
            method_name
        };
        
        if let Some(system) = self.current_system {
            if self.is_action_in_system(system, clean_name) {
                return ResolvedCallType::Action(clean_name.to_string());
            }
            
            if self.is_operation_in_system(system, clean_name) {
                return ResolvedCallType::Operation(clean_name.to_string());
            }
        }
        
        // Default to external (shouldn't happen for valid self calls)
        ResolvedCallType::External(method_name.to_string())
    }
    
    fn resolve_chained_call(&self, first: &str, _chain: &[Box<dyn CallableExpr>]) -> ResolvedCallType {
        // Check if first part references a system (by checking for its operations)
        // Since we can't directly lookup systems, we check if any operation exists in a system with this name
        // This is a temporary implementation until we have better system lookup
        
        // For now, treat all chains as external unless proven otherwise
        // TODO: Implement proper chain resolution when we have system registry
        
        ResolvedCallType::External(first.to_string())
    }
    
    fn resolve_module_call(&self, module_name: &str) -> ResolvedCallType {
        // This is a simplified version - need to look at the actual chain
        ResolvedCallType::ModuleFunction {
            module: module_name.to_string(),
            function: "unknown".to_string(),
        }
    }
    
    fn is_action_in_system(&self, _system_name: &str, action_name: &str) -> bool {
        // Use the arcanum's lookup methods
        self.arcanum.lookup_action(action_name).is_some() ||
        self.arcanum.lookup_action_in_all_systems(action_name).is_some()
    }
    
    fn is_operation_in_system(&self, _system_name: &str, operation_name: &str) -> bool {
        // Use the arcanum's lookup methods
        self.arcanum.lookup_operation(operation_name).is_some() ||
        self.arcanum.lookup_operation_in_all_systems(operation_name).is_some()
    }
    
    pub fn enter_system(&mut self, system_name: &'a str) {
        self.current_system = Some(system_name);
        self.in_standalone_function = false;
    }
    
    pub fn enter_function(&mut self) {
        self.current_system = None;
        self.in_standalone_function = true;
    }
    
    pub fn exit_scope(&mut self) {
        self.current_system = None;
        self.in_standalone_function = false;
    }
}