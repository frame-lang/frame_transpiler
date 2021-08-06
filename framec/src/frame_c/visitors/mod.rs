pub mod cpp_visitor;
pub mod cs_visitor;
pub mod cs_visitor_for_bob;
pub mod gdscript_3_2_visitor;
pub mod java_8_visitor;
pub mod javascript_visitor;
pub mod plantuml_visitor;
pub mod python_visitor;
pub mod rust_visitor;
pub mod smcat_visitor;
//pub mod xtate_visitor;

use super::ast::*;

pub enum AstVisitorReturnType {
    SystemNode,
    InterfaceBlockNode,
    InterfaceMethodNode,
    InterfaceMethodCallExpressionNode,
    MachineBlockNode,
    ActionBlockNode,
    DomainBlockNode,
    StateNode,
    EventHandlerNode,
    EventHandlerTerminatorNode,
    CallExpressionNode,
    CallChainLiteralExprNode,
    CallChainLiteralStmtNode,
    CallStatementNode,
    ActionCallExpressionNode,
    ActionCallStatementNode,
    TestStatementNode,
    StateRefNode,
    ParameterNode,
    DispatchNode,
    ParentheticalExpressionNode,
    IdentifierNode,
    BoolTestNode,
    BoolTestConditionalBranchNode,
    BoolTestElseBranchNode,
    StringMatchTestNode,
    StringMatchTestMatchBranchNode,
    StringMatchElseBranchNode,
    StringMatchTestPatternNode,
    NumberMatchTestNode,
    NumberMatchTestMatchBranchNode,
    NumberMatchElseBranchNode,
    NumberMatchTestPatternNode,
    StateStackOperationNode,
    StateStackOperationStatementNode,
    StateContextNode,
    ChangeStateStmtNode,
    FrameEventExprType,
    ActionDeclNode,
    AssignmentExprNode,
    VariableDeclNode,
    UnaryExprNode,
    BinaryExprNode,
    OperatorType,
    CallExprListNode,
}

pub trait AstVisitor {
    fn visit_system_node(&mut self, machine_node: &SystemNode) -> AstVisitorReturnType;
    fn visit_interface_block_node(
        &mut self,
        interface_block_node: &InterfaceBlockNode,
    ) -> AstVisitorReturnType;
    fn visit_interface_method_node(
        &mut self,
        interface_method_node: &InterfaceMethodNode,
    ) -> AstVisitorReturnType;
    fn visit_machine_block_node(
        &mut self,
        machine_block_node: &MachineBlockNode,
    ) -> AstVisitorReturnType;
    fn visit_state_node(&mut self, state_node: &StateNode) -> AstVisitorReturnType;
    fn visit_event_handler_node(
        &mut self,
        evt_handler_node: &EventHandlerNode,
    ) -> AstVisitorReturnType;
    fn visit_event_handler_terminator_node(
        &mut self,
        evt_handler_node: &TerminatorExpr,
    ) -> AstVisitorReturnType;
    fn visit_call_statement_node(
        &mut self,
        method_call_statement_node: &CallStmtNode,
    ) -> AstVisitorReturnType;
    fn visit_frame_messages_enum(
        &mut self,
        interface_block_node: &InterfaceBlockNode,
    ) -> AstVisitorReturnType;
    fn visit_interface_parameters(
        &mut self,
        interface_block_node: &InterfaceBlockNode,
    ) -> AstVisitorReturnType;
    fn visit_interface_method_call_expression_node(
        &mut self,
        interface_method_call_expr_node: &InterfaceMethodCallExprNode,
    ) -> AstVisitorReturnType;
    fn visit_interface_method_call_expression_node_to_string(
        &mut self,
        interface_method_call_expr_node: &InterfaceMethodCallExprNode,
        output: &mut String,
    ) -> AstVisitorReturnType;

    // NOTE: the difference between call_expression and call_expr is that
    // the former is for the method itself and the latter is the
    // (expr_list) clause of it.
    fn visit_call_expression_node(
        &mut self,
        method_call_expression_node: &CallExprNode,
    ) -> AstVisitorReturnType;
    fn visit_call_expression_node_to_string(
        &mut self,
        method_call_expression_node: &CallExprNode,
        output: &mut String,
    ) -> AstVisitorReturnType;
    fn visit_call_expr_list_node(
        &mut self,
        call_expr_list_node: &CallExprListNode,
    ) -> AstVisitorReturnType;
    fn visit_call_expr_list_node_to_string(
        &mut self,
        assignment_expr_list_node: &CallExprListNode,
        output: &mut String,
    ) -> AstVisitorReturnType;

    fn visit_call_chain_literal_expr_node(
        &mut self,
        method_call_chain_expression_node: &CallChainLiteralExprNode,
    ) -> AstVisitorReturnType;
    fn visit_call_chain_literal_expr_node_to_string(
        &mut self,
        method_call_chain_expression_node: &CallChainLiteralExprNode,
        output: &mut String,
    ) -> AstVisitorReturnType;
    fn visit_call_chain_literal_statement_node(
        &mut self,
        method_call_chain_literal_stmt_node: &CallChainLiteralStmtNode,
    ) -> AstVisitorReturnType;
    fn visit_transition_statement_node(
        &mut self,
        transition_node: &TransitionStatementNode,
    ) -> AstVisitorReturnType;
    fn visit_state_ref_node(&mut self, state_ref_node: &StateRefNode) -> AstVisitorReturnType;
    //    fn visit_argument_node(&mut self, argument:&Argument) -> AstVisitorReturnType;
    fn visit_parameter_node(&mut self, parameter_node: &ParameterNode) -> AstVisitorReturnType;
    fn visit_dispatch_node(&mut self, dispatch_node: &DispatchNode) -> AstVisitorReturnType;
    fn visit_test_statement_node(
        &mut self,
        test_statement_node: &TestStatementNode,
    ) -> AstVisitorReturnType;
    fn visit_bool_test_node(&mut self, bool_test_body_node: &BoolTestNode) -> AstVisitorReturnType;
    fn visit_bool_test_conditional_branch_node(
        &mut self,
        bool_test_true_branch_node: &BoolTestConditionalBranchNode,
    ) -> AstVisitorReturnType;
    fn visit_bool_test_else_branch_node(
        &mut self,
        bool_test_else_branch_node: &BoolTestElseBranchNode,
    ) -> AstVisitorReturnType;
    fn visit_string_match_test_node(
        &mut self,
        string_match_test_node: &StringMatchTestNode,
    ) -> AstVisitorReturnType;
    fn visit_string_match_test_match_branch_node(
        &mut self,
        string_match_test_match_branch_node: &StringMatchTestMatchBranchNode,
    ) -> AstVisitorReturnType;
    fn visit_string_match_test_else_branch_node(
        &mut self,
        string_match_test_else_branch_node: &StringMatchTestElseBranchNode,
    ) -> AstVisitorReturnType;
    fn visit_string_match_test_pattern_node(
        &mut self,
        match_pattern_node: &StringMatchTestPatternNode,
    ) -> AstVisitorReturnType;
    fn visit_number_match_test_node(
        &mut self,
        number_match_test_node: &NumberMatchTestNode,
    ) -> AstVisitorReturnType;
    fn visit_number_match_test_match_branch_node(
        &mut self,
        number_match_test_match_branch_node: &NumberMatchTestMatchBranchNode,
    ) -> AstVisitorReturnType;
    fn visit_number_match_test_else_branch_node(
        &mut self,
        number_match_test_else_branch_node: &NumberMatchTestElseBranchNode,
    ) -> AstVisitorReturnType;
    fn visit_number_match_test_pattern_node(
        &mut self,
        match_pattern_node: &NumberMatchTestPatternNode,
    ) -> AstVisitorReturnType;
    //    fn visit_expression_node(&mut self, expression_node:&ExpressionNode) -> AstVisitorReturnType;
    fn visit_expression_list_node(&mut self, expr_list: &ExprListNode) -> AstVisitorReturnType;
    fn visit_expression_list_node_to_string(
        &mut self,
        expr_list: &ExprListNode,
        output: &mut String,
    ) -> AstVisitorReturnType;
    fn visit_literal_expression_node(
        &mut self,
        literal_expression_node: &LiteralExprNode,
    ) -> AstVisitorReturnType;
    fn visit_literal_expression_node_to_string(
        &mut self,
        literal_expression_node: &LiteralExprNode,
        output: &mut String,
    ) -> AstVisitorReturnType;
    fn visit_identifier_node(&mut self, identifier_node: &IdentifierNode) -> AstVisitorReturnType;
    fn visit_identifier_node_to_string(
        &mut self,
        identifier_node: &IdentifierNode,
        output: &mut String,
    ) -> AstVisitorReturnType;
    fn visit_state_stack_operation_node(
        &mut self,
        state_stack_op_node: &StateStackOperationNode,
    ) -> AstVisitorReturnType;
    fn visit_state_stack_operation_node_to_string(
        &mut self,
        state_stack_op_node: &StateStackOperationNode,
        output: &mut String,
    ) -> AstVisitorReturnType;
    fn visit_state_stack_operation_statement_node(
        &mut self,
        state_stack_op_statement_node: &StateStackOperationStatementNode,
    ) -> AstVisitorReturnType;
    fn visit_state_context_node(
        &mut self,
        state_context_node: &StateContextNode,
    ) -> AstVisitorReturnType;
    fn visit_change_state_statement_node(
        &mut self,
        change_state_stmt_node: &ChangeStateStatementNode,
    ) -> AstVisitorReturnType;
    fn visit_frame_event_part(&mut self, frame_event_part: &FrameEventPart)
        -> AstVisitorReturnType;
    fn visit_frame_event_part_to_string(
        &mut self,
        frame_event_part: &FrameEventPart,
        output: &mut String,
    ) -> AstVisitorReturnType;
    fn visit_actions_block_node(
        &mut self,
        actions_block_node: &ActionsBlockNode,
    ) -> AstVisitorReturnType;
    fn visit_action_node_rust_trait(
        &mut self,
        actions_block_node: &ActionsBlockNode,
    ) -> AstVisitorReturnType;
    fn visit_actions_node_rust_impl(
        &mut self,
        actions_block_node: &ActionsBlockNode,
    ) -> AstVisitorReturnType;
    fn visit_action_decl_node(&mut self, action_decl_node: &ActionNode) -> AstVisitorReturnType;
    fn visit_action_impl_node(&mut self, action_decl_node: &ActionNode) -> AstVisitorReturnType;

    fn visit_action_call_expression_node(
        &mut self,
        action_call_expr_node: &ActionCallExprNode,
    ) -> AstVisitorReturnType;
    fn visit_action_call_expression_node_to_string(
        &mut self,
        action_call_expr_node: &ActionCallExprNode,
        output: &mut String,
    ) -> AstVisitorReturnType;
    fn visit_action_call_statement_node(
        &mut self,
        action_call_stmt_node: &ActionCallStmtNode,
    ) -> AstVisitorReturnType;
    fn visit_domain_block_node(
        &mut self,
        domain_block_node: &DomainBlockNode,
    ) -> AstVisitorReturnType;
    fn visit_domain_variable_decl_node(
        &mut self,
        variable_decl_node: &VariableDeclNode,
    ) -> AstVisitorReturnType;
    fn visit_variable_decl_node(
        &mut self,
        member_variable_node: &VariableDeclNode,
    ) -> AstVisitorReturnType;
    fn visit_variable_expr_node(
        &mut self,
        variable_stmt_node: &VariableNode,
    ) -> AstVisitorReturnType;
    fn visit_variable_expr_node_to_string(
        &mut self,
        variable_stmt_node: &VariableNode,
        output: &mut String,
    ) -> AstVisitorReturnType;
    fn visit_variable_stmt_node(
        &mut self,
        variable_stmt_node: &VariableStmtNode,
    ) -> AstVisitorReturnType;
    fn visit_assignment_expr_node(
        &mut self,
        assignment_expr_node: &AssignmentExprNode,
    ) -> AstVisitorReturnType;
    fn visit_assignment_expr_node_to_string(
        &mut self,
        assignment_expr_node: &AssignmentExprNode,
        output: &mut String,
    ) -> AstVisitorReturnType;
    fn visit_assignment_statement_node(
        &mut self,
        assignment_stmt_node: &AssignmentStmtNode,
    ) -> AstVisitorReturnType;
    fn visit_unary_expr_node(&mut self, unary_expr_node: &UnaryExprNode) -> AstVisitorReturnType;
    fn visit_unary_expr_node_to_string(
        &mut self,
        unary_expr_node: &UnaryExprNode,
        output: &mut String,
    ) -> AstVisitorReturnType;
    fn visit_binary_expr_node(&mut self, binary_expr_node: &BinaryExprNode)
        -> AstVisitorReturnType;
    fn visit_binary_expr_node_to_string(
        &mut self,
        binary_expr_node: &BinaryExprNode,
        output: &mut String,
    ) -> AstVisitorReturnType;
    fn visit_operator_type(&mut self, operator_type: &OperatorType) -> AstVisitorReturnType;
    fn visit_operator_type_to_string(
        &mut self,
        operator_type: &OperatorType,
        output: &mut String,
    ) -> AstVisitorReturnType;
}
