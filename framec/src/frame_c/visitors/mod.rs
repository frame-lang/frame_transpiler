#![allow(dead_code)] // Visitor trait methods are part of the API

use std::convert::TryFrom;

/// An enumeration of the target languages currently supported by Frame.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum TargetLanguage {
    Python3,
    TypeScript,
    Graphviz,
    LLVM,
}

impl TargetLanguage {
    pub fn file_extension(&self) -> &'static str {
        match self {
            TargetLanguage::Python3 => "py",
            TargetLanguage::TypeScript => "ts",
            TargetLanguage::Graphviz => "graphviz",
            TargetLanguage::LLVM => "ll",
        }
    }
}

impl TryFrom<&str> for TargetLanguage {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let normalized = value.to_ascii_lowercase();
        if normalized == "python_3" || normalized == "python" {
            Ok(TargetLanguage::Python3)
        } else if normalized == "typescript" || normalized == "ts" {
            Ok(TargetLanguage::TypeScript)
        } else if normalized == "graphviz" {
            Ok(TargetLanguage::Graphviz)
        } else if normalized == "llvm" {
            Ok(TargetLanguage::LLVM)
        } else {
            Err(format!(
                "Unrecognized target language: {}. Supported languages are: python_3 (python), typescript (ts), graphviz, llvm",
                normalized
            ))
        }
    }
}

impl TryFrom<String> for TargetLanguage {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

pub mod graphviz_visitor;
pub mod python_visitor; // v0.75+: Complete implementation using CodeBuilder
pub mod typescript_visitor; // v0.82: TypeScript support

use super::ast::*;

#[rustfmt::skip]
pub trait AstVisitor {
    fn visit_frame_module(&mut self, _frame_module: &FrameModule) {}
    fn visit_module(&mut self, _node: &Module) {}
    fn visit_module_node(&mut self, _node: &ModuleNode) {}
    fn visit_import_node(&mut self, _node: &ImportNode) {}
    fn visit_type_alias_node(&mut self, _node: &TypeAliasNode) {}
    fn visit_native_module_decl_node(&mut self, _node: &NativeModuleDeclNode) {}
    fn visit_native_function_decl_node(&mut self, _node: &NativeFunctionDeclNode) {}
    fn visit_native_type_decl_node(&mut self, _node: &NativeTypeDeclNode) {}
    fn visit_class_node(&mut self, _node: &ClassNode) {}
    fn visit_method_node(&mut self, _node: &MethodNode) {}
    fn visit_function_node(&mut self, _node: &FunctionNode) {}
    fn visit_system_node(&mut self, _node: &SystemNode) {}
    fn visit_system_instance_statement_node(&mut self, _system_instance_stmt_node: &SystemInstanceStmtNode) {}
    fn visit_system_instance_statement_node_to_string(&mut self, _system_instance_stmt_node: &SystemInstanceStmtNode, _output: &mut String) {}
    fn visit_system_instance_expr_node(&mut self, _system_instance_expr_node: &SystemInstanceExprNode) {}
    fn visit_system_instance_expr_node_to_string(&mut self, _system_instance_expr_node: &SystemInstanceExprNode, _output: &mut String) {}
    fn visit_system_type_statement_node(&mut self, _system_type_stmt_node: &SystemTypeStmtNode) {}
    fn visit_system_type_statement_node_to_string(&mut self, _system_type_stmt_node: &SystemTypeStmtNode, _output: &mut String) {}
    fn visit_system_type_expr_node(&mut self, _system_type_expr_node: &SystemTypeExprNode) {}
    fn visit_system_type_expr_node_to_string(&mut self, _system_type_expr_node: &SystemTypeExprNode, _output: &mut String) {}
    fn visit_interface_block_node(&mut self, _node: &InterfaceBlockNode) {}
    fn visit_interface_method_node(&mut self, _node: &InterfaceMethodNode) {}
    fn visit_machine_block_node(&mut self, _node: &MachineBlockNode) {}
    fn visit_state_node(&mut self, _node: &StateNode) {}
    fn visit_event_handler_node(&mut self, _node: &EventHandlerNode) {}
    fn visit_event_handler_terminator_node(&mut self, _node: &TerminatorExpr) {}
    fn visit_call_statement_node(&mut self, _node: &CallStmtNode) {}
    fn visit_frame_messages_enum(&mut self, _node: &InterfaceBlockNode) {}
    fn visit_interface_parameters(&mut self, _node: &InterfaceBlockNode) {}
    fn visit_interface_method_call_expression_node(&mut self, _node: &InterfaceMethodCallExprNode) {}
    fn visit_interface_method_call_expression_node_to_string(&mut self, _node: &InterfaceMethodCallExprNode, _output: &mut String) {}
    fn visit_call_expression_node(&mut self, _node: &CallExprNode) {}
    fn visit_call_expression_node_to_string(&mut self, _node: &CallExprNode, _output: &mut String) {}

    fn visit_call_expr_list_node(&mut self, _node: &CallExprListNode) {}
    fn visit_call_expr_list_node_to_string(&mut self, _node: &CallExprListNode, _output: &mut String) {}

    fn visit_call_chain_expr_node(&mut self, _node: &CallChainExprNode) {}
    fn visit_call_chain_expr_node_to_string(&mut self, _node: &CallChainExprNode, _output: &mut String) {}

    fn visit_call_chain_statement_node(&mut self, _node: &CallChainStmtNode) {}
    fn visit_transition_statement_node(&mut self, _node: &TransitionStatementNode) {}
    fn visit_transition_expr_node(&mut self, _node: &TransitionExprNode) {}
    fn visit_transition_expr_node_to_string(&mut self, _expr_list: &TransitionExprNode, _output: &mut String) {}
    fn visit_state_ref_node(&mut self, _node: &StateRefNode) {}
    fn visit_parameter_node(&mut self, _node: &ParameterNode) {}
    fn visit_dispatch_node(&mut self, _node: &DispatchNode) {}
    fn visit_test_statement_node(&mut self, _node: &TestStatementNode) {}
    fn visit_bool_test_node(&mut self, _node: &BoolTestNode) {}
    fn visit_bool_test_conditional_branch_node(&mut self, _node: &BoolTestConditionalBranchNode) {}
    fn visit_bool_test_else_branch_node(&mut self, _node: &BoolTestElseBranchNode) {}
    fn visit_string_match_test_node(&mut self, _node: &StringMatchTestNode) {}
    fn visit_string_match_test_match_branch_node(&mut self, _node: &StringMatchTestMatchBranchNode) {}
    fn visit_string_match_test_else_branch_node(&mut self, _node: &StringMatchTestElseBranchNode) {}
    fn visit_string_match_test_pattern_node(&mut self, _node: &StringMatchTestPatternNode) {}
    fn visit_number_match_test_node(&mut self, _node: &NumberMatchTestNode) {}
    fn visit_number_match_test_match_branch_node(&mut self, _node: &NumberMatchTestMatchBranchNode) {}
    fn visit_number_match_test_else_branch_node(&mut self, _node: &NumberMatchTestElseBranchNode) {}
    fn visit_number_match_test_pattern_node(&mut self, _node: &NumberMatchTestPatternNode) {}
    fn visit_enum_match_test_node(&mut self, _enum_match_test_node: &EnumMatchTestNode) {}
    fn visit_enum_match_test_match_branch_node(&mut self, _enum_match_test_match_branch_node: &EnumMatchTestMatchBranchNode) {}
    fn visit_enum_match_test_else_branch_node(&mut self, _enum_match_test_else_branch_node: &EnumMatchTestElseBranchNode) {}
    fn visit_enum_match_test_pattern_node(&mut self, _enum_match_test_pattern_node: &EnumMatchTestPatternNode) {}

    fn visit_expression_list_node(&mut self, _expr_list: &ExprListNode) {}
    fn visit_expression_list_node_to_string(&mut self, _expr_list: &ExprListNode, _output: &mut String) {}

    fn visit_list_node(&mut self, _list: &ListNode) {}
    fn visit_list_node_to_string(&mut self, _list: &ListNode, _output: &mut String) {}
    fn visit_dict_literal_node(&mut self, _dict: &DictLiteralNode) {}
    fn visit_dict_literal_node_to_string(&mut self, _dict: &DictLiteralNode, _output: &mut String) {}
    fn visit_set_literal_node(&mut self, _set: &SetLiteralNode) {}
    fn visit_set_literal_node_to_string(&mut self, _set: &SetLiteralNode, _output: &mut String) {}
    fn visit_tuple_literal_node(&mut self, _tuple: &TupleLiteralNode) {}
    fn visit_tuple_literal_node_to_string(&mut self, _tuple: &TupleLiteralNode, _output: &mut String) {}

    fn visit_list_elem_node(&mut self, _list_elem: &ListElementNode) {}
    fn visit_list_elem_node_to_string(&mut self, _list_elem: &ListElementNode, _output: &mut String) {}
    fn visit_slice_node(&mut self, _slice_node: &SliceNode) {}
    fn visit_slice_node_to_string(&mut self, _slice_node: &SliceNode, _output: &mut String) {}
    
    // v0.34: Unpacking and comprehensions
    fn visit_unpack_expr_node(&mut self, _unpack_expr: &UnpackExprNode) {}
    fn visit_unpack_expr_node_to_string(&mut self, _unpack_expr: &UnpackExprNode, _output: &mut String) {}
    // v0.54: Star expression for unpacking
    fn visit_star_expr_node(&mut self, _star_expr: &StarExprNode) {}
    fn visit_star_expr_node_to_string(&mut self, _star_expr: &StarExprNode, _output: &mut String) {}
    fn visit_list_comprehension_node(&mut self, _comprehension: &ListComprehensionNode) {}
    fn visit_list_comprehension_node_to_string(&mut self, _comprehension: &ListComprehensionNode, _output: &mut String) {}
    fn visit_dict_comprehension_node(&mut self, _comprehension: &DictComprehensionNode) {}
    fn visit_dict_comprehension_node_to_string(&mut self, _comprehension: &DictComprehensionNode, _output: &mut String) {}
    fn visit_set_comprehension_node(&mut self, _comprehension: &SetComprehensionNode) {}
    fn visit_set_comprehension_node_to_string(&mut self, _comprehension: &SetComprehensionNode, _output: &mut String) {}
    
    // v0.38: Dict unpacking
    fn visit_dict_unpack_expr_node(&mut self, _dict_unpack_expr: &DictUnpackExprNode) {}
    fn visit_dict_unpack_expr_node_to_string(&mut self, _dict_unpack_expr: &DictUnpackExprNode, _output: &mut String) {}
    
    // v0.35: Async/await support
    fn visit_await_expr_node(&mut self, _await_expr: &AwaitExprNode) {}
    fn visit_await_expr_node_to_string(&mut self, _await_expr: &AwaitExprNode, _output: &mut String) {}
    
    // v0.38: Lambda expressions
    fn visit_lambda_expr_node(&mut self, _lambda_expr: &LambdaExprNode) {}
    fn visit_lambda_expr_node_to_string(&mut self, _lambda_expr: &LambdaExprNode, _output: &mut String) {}
    
    // v0.38: Function references for first-class functions
    fn visit_function_ref_node(&mut self, _name: &str) {}
    fn visit_function_ref_node_to_string(&mut self, _name: &str, _output: &mut String) {}
    
    // v0.42: Generator features
    fn visit_yield_expr_node(&mut self, _yield_expr: &YieldExprNode) {}
    fn visit_yield_expr_node_to_string(&mut self, _yield_expr: &YieldExprNode, _output: &mut String) {}
    fn visit_yield_from_expr_node(&mut self, _yield_from_expr: &YieldFromExprNode) {}
    fn visit_yield_from_expr_node_to_string(&mut self, _yield_from_expr: &YieldFromExprNode, _output: &mut String) {}
    fn visit_generator_expr_node(&mut self, _generator_expr: &GeneratorExprNode) {}
    fn visit_generator_expr_node_to_string(&mut self, _generator_expr: &GeneratorExprNode, _output: &mut String) {}

    fn visit_list_stmt_node(&mut self, _list: &ListStmtNode) {}

    fn visit_expr_list_stmt_node(&mut self, _expr_list: &ExprListStmtNode) {}

    fn visit_if_stmt_node(&mut self, _if_stmt_node: &IfStmtNode) {}
    fn visit_for_stmt_node(&mut self, _for_stmt_node: &ForStmtNode) {}
    fn visit_while_stmt_node(&mut self, _while_stmt_node: &WhileStmtNode) {}
    fn visit_loop_stmt_node(&mut self, _expr_list: &LoopStmtNode) {}
    fn visit_loop_for_stmt_node(&mut self, _loop_for_stmt_node: &LoopForStmtNode) {}
    fn visit_loop_in_stmt_node(&mut self, _loop_in_stmt_node: &LoopInStmtNode) {}
    fn visit_loop_infinite_stmt_node(&mut self, _loop_infinite_stmt_node: &LoopInfiniteStmtNode) {}
    fn visit_break_stmt_node(&mut self, _break_expr_node: &BreakStmtNode) {}
    fn visit_continue_stmt_node(&mut self, _continue_expr_node: &ContinueStmtNode) {}
    fn visit_assert_stmt_node(&mut self, _assert_stmt_node: &AssertStmtNode) {}
    fn visit_del_stmt_node(&mut self, _del_stmt_node: &DelStmtNode) {}  // v0.50
    fn visit_try_stmt_node(&mut self, _try_stmt_node: &TryStmtNode) {}
    fn visit_except_clause_node(&mut self, _except_clause_node: &ExceptClauseNode) {}
    fn visit_raise_stmt_node(&mut self, _raise_stmt_node: &RaiseStmtNode) {}
    fn visit_with_stmt_node(&mut self, _with_stmt_node: &WithStmtNode) {}
    fn visit_match_stmt_node(&mut self, _match_stmt_node: &MatchStmtNode) {}
    fn visit_case_node(&mut self, _case_node: &CaseNode) {}
    fn visit_pattern_node(&mut self, _pattern_node: &PatternNode) {}
    // visit_superstring_stmt_node removed - backticks no longer supported
    fn visit_block_stmt_node(&mut self, _block_stmt_node: &BlockStmtNode) {}
    fn visit_literal_expression_node(&mut self, _node: &LiteralExprNode) {}
    fn visit_literal_expression_node_to_string(&mut self, _node: &LiteralExprNode, _output: &mut String) {}
    fn visit_identifier_node(&mut self, _node: &IdentifierNode) {}
    fn visit_identifier_node_to_string(&mut self, _node: &IdentifierNode, _output: &mut String) {}
    fn visit_state_stack_operation_node(&mut self, _node: &StateStackOperationNode) {}
    fn visit_state_stack_operation_node_to_string(&mut self, _node: &StateStackOperationNode, _output: &mut String) {}
    fn visit_state_stack_operation_statement_node(&mut self, _node: &StateStackOperationStatementNode) {}
    fn visit_state_context_node(&mut self, _node: &TargetStateContextNode) {}
    fn visit_frame_event_part(&mut self, _event_part: &FrameEventPart) {}
    fn visit_frame_event_part_to_string(&mut self, _event_part: &FrameEventPart, _output: &mut String) {}
    fn visit_actions_block_node(&mut self, _node: &ActionsBlockNode) {}
    fn visit_action_node_rust_trait(&mut self, _node: &ActionsBlockNode) {}
    fn visit_actions_node_rust_impl(&mut self, _node: &ActionsBlockNode) {}

    fn visit_action_node(&mut self, _node: &ActionNode) {}
    fn visit_action_impl_node(&mut self, _node: &ActionNode) {}

    fn visit_action_call_expression_node(&mut self, _node: &ActionCallExprNode) {}
    fn visit_action_call_expression_node_to_string(&mut self, _node: &ActionCallExprNode, _output: &mut String) {}
    fn visit_action_call_statement_node(&mut self, _node: &ActionCallStmtNode) {}


    // ---
    fn visit_operations_block_node(&mut self, _node: &OperationsBlockNode) {}

    fn visit_operation_node(&mut self, _node: &OperationNode) {}
    fn visit_operation_call_expression_node(&mut self, _node: &OperationCallExprNode) {}
    fn visit_operation_call_expression_node_to_string(&mut self, _node: &OperationCallExprNode, _output: &mut String) {}
    fn visit_operation_call_statement_node(&mut self, _node: &OperationCallExprNode) {}

    fn visit_operation_ref_expression_node(&mut self, _node: &OperationRefExprNode) {}
    fn visit_operation_ref_expression_node_to_string(&mut self, _node: &OperationRefExprNode, _output: &mut String) {}
    fn visit_operation_ref_statement_node(&mut self, _node: &OperationRefExprNode) {}

    // ---


    fn visit_domain_block_node(&mut self, _node: &DomainBlockNode) {}
    fn visit_domain_variable_decl_node(&mut self, _node: &VariableDeclNode) {}
    fn visit_variable_decl_node(&mut self, _node: &VariableDeclNode) {}
    fn visit_variable_expr_node(&mut self, _node: &VariableNode) {}
    fn visit_variable_expr_node_to_string(&mut self, _node: &VariableNode, _output: &mut String) {}
    fn visit_variable_stmt_node(&mut self, _node: &VariableStmtNode) {}

    fn visit_loop_variable_decl_node(&mut self, _node: &LoopVariableDeclNode) {}

    fn visit_enum_decl_node(&mut self, _node: &EnumDeclNode) {}
    fn visit_enumerator_decl_node(&mut self, _node: &EnumeratorDeclNode) {}
    fn visit_enumerator_expr_node(&mut self, _node: &EnumeratorExprNode) {}
    fn visit_enumerator_expr_node_to_string(&mut self, _node: &EnumeratorExprNode, _output: &mut String) {}
    fn visit_enumerator_statement_node(&mut self, _node: &EnumeratorStmtNode) {}

    fn visit_assignment_expr_node(&mut self, _node: &AssignmentExprNode) {}
    fn visit_assignment_expr_node_to_string(&mut self, _node: &AssignmentExprNode, _output: &mut String) {}
    fn visit_walrus_expr_node(&mut self, _node: &AssignmentExprNode) {}
    fn visit_walrus_expr_node_to_string(&mut self, _node: &AssignmentExprNode, _output: &mut String) {}
    fn auto_inc_dec_assignment_expr_node(&mut self, _node: &AssignmentExprNode) {}

    fn visit_assignment_statement_node(&mut self, _node: &AssignmentStmtNode) {}
    fn visit_unary_expr_node(&mut self, _node: &UnaryExprNode) {}
    fn visit_unary_expr_node_to_string(&mut self, _node: &UnaryExprNode, _output: &mut String) {}

    fn visit_binary_stmt_node(&mut self, _node: &BinaryStmtNode) {}
    fn visit_binary_expr_node(&mut self, _node: &BinaryExprNode) {}
    fn visit_binary_expr_node_to_string(&mut self, _node: &BinaryExprNode, _output: &mut String) {}
    fn auto_inc_dec_binary_expr_node(&mut self, _node: &BinaryExprNode) {}

    fn visit_operator_type(&mut self, _operator_type: &OperatorType) {}
    fn visit_operator_type_to_string(&mut self, _operator_type: &OperatorType, _output: &mut String) {}

    fn visit_return_assign_stmt_node(&mut self, _return_assign_stmt_node: &ReturnAssignStmtNode) {}
    fn visit_return_stmt_node(&mut self, _return_stmt_node: &ReturnStmtNode) {}
    fn visit_parent_dispatch_stmt_node(&mut self, _parent_dispatch_stmt_node: &ParentDispatchStmtNode) {}

    fn visit_self_expr_node(&mut self, _self_expr_node: &SelfExprNode) {}
    fn visit_self_expr_node_to_string(&mut self, _self_expr_node: &SelfExprNode, _output: &mut String) {}

}
