// Simplified call chain handling for PythonVisitor
// This is a more conservative refactoring that simplifies without major restructuring

use super::*;

impl PythonVisitor {
    /// Simplified method to handle any call expression
    /// This consolidates the logic from visit_call_expression_node
    pub fn handle_call_simplified(&mut self, method_call: &CallExprNode) {
        self.debug_enter(&format!("handle_call_simplified({})", method_call.identifier.name.lexeme));
        
        // v0.65: Simplified - always handle as external call
        // The semantic resolution now determines the correct prefix
        self.handle_external_call(method_call);
        
        self.debug_exit("handle_call_simplified");
    }
    
    /// Handle external calls with simplified logic
    fn handle_external_call(&mut self, method_call: &CallExprNode) {
        // Process call chain first (for object.method() syntax)
        let has_call_chain = self.process_call_chain(method_call);
        
        // If there's a call chain, it's definitely external
        if has_call_chain {
            self.generate_external_method_call(method_call);
            return;
        }
        
        // No call chain - check if it's actually internal (action/operation)
        if self.try_handle_internal_call(method_call) {
            return;
        }
        
        // True external call
        self.generate_external_method_call(method_call);
    }
    
    /// Process call chain if present, returns true if chain exists
    fn process_call_chain(&mut self, method_call: &CallExprNode) -> bool {
        if let Some(call_chain) = &method_call.call_chain {
            if !call_chain.is_empty() {
                for callable in call_chain {
                    callable.callable_accept(self);
                    self.add_code(".");
                }
                return true;
            }
        }
        false
    }
    
    /// Try to handle as internal action/operation, returns true if handled
    fn try_handle_internal_call(&mut self, method_call: &CallExprNode) -> bool {
        let method_name = &method_call.identifier.name.lexeme;
        
        // Strip underscore for action lookup
        let action_name = if method_name.starts_with('_') {
            &method_name[1..]
        } else {
            method_name
        };
        
        // Check for action
        if self.is_action(action_name) {
            self.generate_action_call(action_name, method_call);
            return true;
        }
        
        // Check for operation
        if self.is_operation(action_name) {
            self.generate_operation_call(action_name, method_call);
            return true;
        }
        
        false
    }
    
    /// Check if a name is an action
    fn is_action(&self, name: &str) -> bool {
        self.arcanium.lookup_action_in_all_systems(name).is_some()
    }
    
    /// Check if a name is an operation
    fn is_operation(&self, name: &str) -> bool {
        self.arcanium.lookup_operation_in_all_systems(name).is_some()
    }
    
    /// Generate an action call
    fn generate_action_call(&mut self, action_name: &str, method_call: &CallExprNode) {
        if !self.in_standalone_function {
            self.add_code("self.");
        }
        self.add_code(&format!("_{}", action_name));
        method_call.call_expr_list.accept(self);
    }
    
    /// Generate an operation call
    fn generate_operation_call(&mut self, operation_name: &str, method_call: &CallExprNode) {
        if !self.in_standalone_function {
            self.add_code("self.");
        }
        // Don't add system prefix in standalone functions - already handled by call chain
        self.add_code(operation_name);
        method_call.call_expr_list.accept(self);
    }
    
    /// Generate an external method call (including collection constructors)
    fn generate_external_method_call(&mut self, method_call: &CallExprNode) {
        let method_name = &method_call.identifier.name.lexeme;
        
        // Special handling for Python collection constructors
        match method_name.as_ref() {
            "set" | "list" | "tuple" => {
                self.handle_sequence_constructor(method_call);
            }
            "dict" => {
                self.handle_collection_constructor(method_call);
            }
            _ => {
                self.add_code(method_name);
                method_call.call_expr_list.accept(self);
            }
        }
    }
    
    /// Handle sequence constructors (set, list, tuple)
    fn handle_sequence_constructor(&mut self, method_call: &CallExprNode) {
        let method_name = &method_call.identifier.name.lexeme;
        self.add_code(method_name);
        self.add_code("(");
        
        let expr_count = method_call.call_expr_list.exprs_t.len();
        
        if expr_count > 1 {
            // Multiple arguments: wrap in list
            self.add_code("[");
            let mut separator = "";
            for expr in &method_call.call_expr_list.exprs_t {
                self.add_code(separator);
                expr.accept(self);
                separator = ",";
            }
            self.add_code("]");
        } else if expr_count == 1 {
            // Single argument: pass as-is
            method_call.call_expr_list.exprs_t[0].accept(self);
        }
        
        self.add_code(")");
    }
    
    /// Simplified version for to_string
    pub fn handle_call_to_string_simplified(
        &mut self,
        method_call: &CallExprNode,
        output: &mut String,
    ) {
        // v0.65: Use unified external call handling
        let saved_code = self.code.clone();
        self.code.clear();
        self.handle_call_simplified(method_call);
        output.push_str(&self.code);
        self.code = saved_code;
    }
}