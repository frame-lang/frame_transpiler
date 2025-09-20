// BACKUP: Complex call chain methods from python_visitor.rs
// These are being replaced with simplified versions in v0.61
// Keep this backup until we verify the refactoring works correctly

// Original visit_call_expression_node (lines 6367-6525, ~158 lines)
/*
    fn visit_call_expression_node(&mut self, method_call: &CallExprNode) {
        self.debug_enter(&format!("visit_call_expression_node({})", method_call.identifier.name.lexeme));
        
        // Don't add source mapping here to avoid duplicates when visited as part of a statement
        // The statement-level visitors (CallStmtNode, CallChainStmtNode) handle the mapping
        // self.add_source_mapping(method_call.identifier.line);
        
        // Debug: log the call chain to understand what's happening
        if let Some(call_chain) = &method_call.call_chain {
            debug_print!("DEBUG visit_call_expression_node: method={}, call_chain length={}, context={:?}", 
                method_call.identifier.name.lexeme, call_chain.len(), method_call.context);
            for (i, _callable) in call_chain.iter().enumerate() {
                debug_print!("  Call chain[{}]: <callable>", i);
            }
        } else {
            debug_print!("DEBUG visit_call_expression_node: method={}, NO call_chain, context={:?}", 
                method_call.identifier.name.lexeme, method_call.context);
        }
        
        // Frame v0.31: Handle explicit self/system context
        match &method_call.context {
            CallContextType::SelfCall => {
                self.handle_self_call(method_call);
                return;
            }
            CallContextType::StaticCall(class_name) => {
                self.handle_static_call(method_call, class_name);
                return;
            }
            CallContextType::ExternalCall => {
                // ... 100+ lines of complex nested logic ...
            }
        }
    }
*/

// Original visit_call_expression_node_to_string (lines 6529-6673, ~144 lines)
/*
    fn visit_call_expression_node_to_string(
        &mut self,
        method_call: &CallExprNode,
        output: &mut String,
    ) {
        // Handle SelfCall first (for class methods)
        if let CallContextType::SelfCall = &method_call.context {
            // ... complex logic ...
        }
        
        // Handle call context first - but still check for actions/operations even for ExternalCall
        if let CallContextType::ExternalCall = &method_call.context {
            // ... 100+ lines of complex nested logic ...
        }
    }
*/