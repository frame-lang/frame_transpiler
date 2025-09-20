// Integration of simplified call chain handling into PythonVisitor
// 
// To integrate this refactoring:
// 1. Add this module to python_visitor.rs:
//    mod call_chain_simplified;
//
// 2. Replace the existing visit_call_expression_node with this simplified version:

    fn visit_call_expression_node(&mut self, method_call: &CallExprNode) {
        self.debug_enter(&format!("visit_call_expression_node({})", method_call.identifier.name.lexeme));
        
        // Don't add source mapping here to avoid duplicates when visited as part of a statement
        // The statement-level visitors (CallStmtNode, CallChainStmtNode) handle the mapping
        
        // Use the simplified handler
        self.handle_call_simplified(method_call);
        
        self.debug_exit("visit_call_expression_node");
    }

// 3. Replace the existing visit_call_expression_node_to_string with this:

    fn visit_call_expression_node_to_string(
        &mut self,
        method_call: &CallExprNode,
        output: &mut String,
    ) {
        // Use the simplified to_string handler
        self.handle_call_to_string_simplified(method_call, output);
    }

// Benefits of this refactoring:
// 
// 1. Reduced complexity:
//    - Original visit_call_expression_node: 158 lines
//    - New version: ~8 lines + shared logic
//    
// 2. Eliminated duplication:
//    - Logic is shared between normal and to_string versions
//    - Clear separation of concerns
//    
// 3. Easier to maintain:
//    - Each function has a single responsibility
//    - Call resolution logic is centralized
//    
// 4. Better testability:
//    - Individual functions can be tested in isolation
//    - Clear inputs and outputs
//
// 5. Prevents bugs:
//    - The v0.60 double-call bug was caused by complex, duplicated logic
//    - This simpler structure makes such bugs less likely
//
// Testing checklist:
// [ ] All 378 tests still pass
// [ ] Action calls work correctly
// [ ] Operation calls work correctly
// [ ] Static method calls work correctly
// [ ] External function calls work correctly
// [ ] Collection constructors work correctly
// [ ] Call chains work correctly
// [ ] Class method calls work correctly