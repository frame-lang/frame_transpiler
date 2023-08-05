use std::convert::TryFrom;

/// An enumeration of the target languages currently supported by Frame.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum TargetLanguage {
    Cpp,
    CSharp,
    CSharpForBob,
    GdScript,
    GoLang,
    Java8,
    JavaScript,
    PlantUml,
    Python3,
    Rust,
    Smcat,
    // XState,
}

impl TargetLanguage {
    pub fn file_extension(&self) -> &'static str {
        match self {
            TargetLanguage::Cpp => "cpp",
            TargetLanguage::CSharp => "cs",
            TargetLanguage::CSharpForBob => "cs",
            TargetLanguage::GdScript => "gd",
            TargetLanguage::GoLang => "go",
            TargetLanguage::Java8 => "java",
            TargetLanguage::JavaScript => "js",
            TargetLanguage::PlantUml => "puml",
            TargetLanguage::Python3 => "py",
            TargetLanguage::Rust => "rs",
            TargetLanguage::Smcat => "smcat",
        }
    }
}

impl TryFrom<&str> for TargetLanguage {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "cpp" {
            Ok(TargetLanguage::Cpp)
        } else if value == "c_sharp" {
            Ok(TargetLanguage::CSharp)
        } else if value == "c_sharp_bob" {
            Ok(TargetLanguage::CSharpForBob)
        } else if value == "gdscript" {
            Ok(TargetLanguage::GdScript)
        } else if value == "golang" {
            Ok(TargetLanguage::GoLang)
        } else if value == "java_8" {
            Ok(TargetLanguage::Java8)
        } else if value == "javascript" {
            Ok(TargetLanguage::JavaScript)
        } else if value == "plantuml" {
            Ok(TargetLanguage::PlantUml)
        } else if value == "python_3" {
            Ok(TargetLanguage::Python3)
        } else if value == "rust" {
            Ok(TargetLanguage::Rust)
        } else if value == "smcat" {
            Ok(TargetLanguage::Smcat)
        // } else if value == "xstate" {
        //     Ok(TargetLanguage::XState)
        } else {
            Err(format!("Unrecognized target language {}", value))
        }
    }
}

impl TryFrom<String> for TargetLanguage {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

pub mod cpp_visitor;
pub mod cs_visitor;
pub mod cs_visitor_for_bob;
pub mod gdscript_3_2_visitor;
pub mod golang_visitor;
pub mod java_8_visitor;
pub mod javascript_visitor;
pub mod plantuml_visitor;
pub mod python_visitor;
pub mod rust_visitor;
pub mod smcat_visitor;
//pub mod xtate_visitor;

use super::ast::*;

#[rustfmt::skip]
pub trait AstVisitor {
    fn visit_system_node(&mut self, _node: &SystemNode) {}
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

    fn visit_call_chain_literal_expr_node(&mut self, _node: &CallChainLiteralExprNode) {}
   // fn auto_inc_dec_call_chain_literal_expr_node(&mut self, _node: &CallChainLiteralExprNode) {}
   // fn auto_post_inc_dec_call_chain_literal_expr_node(&mut self, _node: &CallChainLiteralExprNode) {}
    fn visit_call_chain_literal_expr_node_to_string(&mut self, _node: &CallChainLiteralExprNode, _output: &mut String) {}

    fn visit_call_chain_literal_statement_node(&mut self, _node: &CallChainLiteralStmtNode) {}
    fn visit_transition_statement_node(&mut self, _node: &TransitionStatementNode) {}
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
    fn visit_enum_match_test_node(&mut self, _node: &EnumMatchTestNode) {}
    fn visit_enum_match_test_match_branch_node(&mut self, _node: &EnumMatchTestMatchBranchNode) {}
    fn visit_enum_match_test_else_branch_node(&mut self, _node: &EnumMatchTestElseBranchNode) {}
    fn visit_enum_match_test_pattern_node(&mut self, _node: &EnumMatchTestPatternNode) {}

    fn visit_expression_list_node(&mut self, _expr_list: &ExprListNode) {}
    fn visit_expression_list_node_to_string(&mut self, _expr_list: &ExprListNode, _output: &mut String) {}
    fn visit_auto_pre_inc_dec_expr_node(&mut self, _expr_list: &RefExprType) {}
    fn visit_auto_post_inc_dec_expr_node(&mut self, _expr_list: &RefExprType) {}
 //   fn auto_post_inc_dec_expression_list_node(&mut self, _expr_list: &ExprListNode) {}

    fn visit_expr_list_stmt_node(&mut self, _expr_list: &ExprListStmtNode) {}

    fn visit_loop_stmt_node(&mut self, _expr_list: &LoopStmtNode) {}
    fn visit_loop_for_stmt_node(&mut self, _loop_for_stmt_node: &LoopForStmtNode) {}
    fn visit_loop_in_stmt_node(&mut self, _loop_in_stmt_node: &LoopInStmtNode) {}
    fn visit_loop_infinite_stmt_node(&mut self, _loop_infinite_stmt_node: &LoopInfiniteStmtNode) {}
    fn visit_break_stmt_node(&mut self, _break_expr_node: &BreakStmtNode) {}
    fn visit_continue_stmt_node(&mut self, _continue_expr_node: &ContinueStmtNode) {}
    fn visit_superstring_stmt_node(&mut self, _continue_expr_node: &SuperStringStmtNode) {}
    fn visit_block_stmt_node(&mut self, block_stmt_node: &BlockStmtNode) {}
    // fn visit_loop_expr_node(&mut self, _loop_types: &LoopTypes) {}
    // fn visit_loop_expr_node_to_string(&mut self, _loop_types: &LoopTypes, _output: &mut String) {}



    fn visit_literal_expression_node(&mut self, _node: &LiteralExprNode) {}
    fn visit_literal_expression_node_to_string(&mut self, _node: &LiteralExprNode, _output: &mut String) {}
    fn visit_identifier_node(&mut self, _node: &IdentifierNode) {}
    fn visit_identifier_node_to_string(&mut self, _node: &IdentifierNode, _output: &mut String) {}
    fn visit_state_stack_operation_node(&mut self, _node: &StateStackOperationNode) {}
    fn visit_state_stack_operation_node_to_string(&mut self, _node: &StateStackOperationNode, _output: &mut String) {}
    fn visit_state_stack_operation_statement_node(&mut self, _node: &StateStackOperationStatementNode) {}
    fn visit_state_context_node(&mut self, _node: &StateContextNode) {}
    fn visit_change_state_statement_node(&mut self, _node: &ChangeStateStatementNode) {}
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
}
