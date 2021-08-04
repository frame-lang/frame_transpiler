#![allow(non_snake_case)]

use super::scanner::{Token, TokenType};
use super::symbol_table::{ActionDeclSymbol, EventSymbol, SymbolType};

use crate::frame_c::ast::OperatorType::{
    Divide, Greater, GreaterEqual, LessEqual, Minus, Multiply, Plus,
};
use crate::frame_c::symbol_table::InterfaceMethodSymbol;
use crate::frame_c::visitors::*;
use downcast_rs::__std::cell::RefCell;
use downcast_rs::*;
use std::collections::VecDeque;
use std::rc::Rc;
use wasm_bindgen::__rt::std::collections::HashMap;

pub trait NodeElement {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor);
    // for Rust actions
    fn accept_rust_trait(&self, _ast_visitor: &mut dyn AstVisitor) {
        // no_op
    }
    fn accept_rust_impl(&self, _ast_visitor: &mut dyn AstVisitor) {
        // no_op
    }
    fn accept_rust_domain_var_decl(&self, _ast_visitor: &mut dyn AstVisitor) {
        // no_op
    }
    fn accept_frame_messages_enum(&self, _ast_visitor: &mut dyn AstVisitor) {
        // no_op
    }
    fn accept_frame_parameters(&self, _ast_visitor: &mut dyn AstVisitor) {
        // no_op
    }
    // fn accept_frame_message_enum(&self,  ast_visitor:&mut dyn AstVisitor) {
    //     // no_op
    // }
    fn accept_to_string(&self, _ast_visitor: &mut dyn AstVisitor, _output: &mut String) {
        // no_op
    }
}

// TODO: is this a good name for Identifier and Call expressions?

pub trait CallableExpr: Downcast {
    fn set_call_chain(&mut self, call_chain: Vec<Box<dyn CallableExpr>>);
    fn callable_accept(&self, ast_visitor: &mut dyn AstVisitor);
    fn callable_accept_to_string(&self, _ast_visitor: &mut dyn AstVisitor, _output: &mut String) {
        // no_op
    }
}
impl_downcast!(CallableExpr);

// TODO: note - exploring if this enum can replace the Callable : Downcast approach
pub enum CallChainLiteralNodeType {
    // TODO: should be differentiated parameter or variable? no funcitonal difference at this point though
    VariableNodeT {
        var_node: VariableNode,
    },
    IdentifierNodeT {
        id_node: IdentifierNode,
    }, // TODO: change IdentifierNode to VariableNode
    CallT {
        call: CallExprNode,
    },
    InterfaceMethodCallT {
        interface_method_call_expr_node: InterfaceMethodCallExprNode,
    },
    ActionCallT {
        action_call_expr_node: ActionCallExprNode,
    },
}

impl CallChainLiteralNodeType {
    pub fn setIsReference(&mut self, is_reference: bool) {
        match self {
            CallChainLiteralNodeType::VariableNodeT { var_node } => {
                var_node.id_node.is_reference = is_reference;
            }
            CallChainLiteralNodeType::IdentifierNodeT { id_node } => {
                id_node.is_reference = is_reference;
            }
            _ => {}
        }
    }
}

//-----------------------------------------------------//

pub struct AttributeNode {
    pub name: String,
    pub value: String,
}

impl AttributeNode {
    pub fn new(name: String, value: String) -> AttributeNode {
        AttributeNode { name, value }
    }
}

//-----------------------------------------------------//

pub struct SystemNode {
    pub name: String,
    pub header: String,
    pub attributes_opt: Option<HashMap<String, AttributeNode>>,
    pub interface_block_node_opt: Option<InterfaceBlockNode>,
    pub machine_block_node_opt: Option<MachineBlockNode>,
    pub actions_block_node_opt: Option<ActionsBlockNode>,
    pub domain_block_node_opt: Option<DomainBlockNode>,
    pub line: usize,
}

impl SystemNode {
    pub fn new(
        name: String,
        header: String,
        attributes_opt: Option<HashMap<String, AttributeNode>>,
        interface_block_node_opt: Option<InterfaceBlockNode>,
        machine_block_node_opt: Option<MachineBlockNode>,
        actions_block_node_opt: Option<ActionsBlockNode>,
        domain_block_node_opt: Option<DomainBlockNode>,
        line: usize,
    ) -> SystemNode {
        SystemNode {
            name,
            header,
            attributes_opt,
            interface_block_node_opt,
            machine_block_node_opt,
            actions_block_node_opt,
            domain_block_node_opt,
            line,
        }
    }

    pub fn get_first_state(&self) -> Option<&Rc<RefCell<StateNode>>> {
        match &self.machine_block_node_opt {
            Some(mb) => mb.states.get(0),
            None => None,
        }
    }
}

impl NodeElement for SystemNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_system_node(self);
    }
}

//-----------------------------------------------------//

pub struct InterfaceBlockNode {
    // pub interface_methods:Vec<InterfaceMethodNode>,
    pub interface_methods: Vec<Rc<RefCell<InterfaceMethodNode>>>,
}

impl InterfaceBlockNode {
    pub fn new(interface_methods: Vec<Rc<RefCell<InterfaceMethodNode>>>) -> InterfaceBlockNode {
        InterfaceBlockNode { interface_methods }
    }
}

impl NodeElement for InterfaceBlockNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_interface_block_node(self);
    }
    fn accept_frame_messages_enum(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_frame_messages_enum(self);
    }
    fn accept_frame_parameters(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_interface_parameters(self);
    }
}

//-----------------------------------------------------//

pub struct InterfaceMethodNode {
    pub name: String,
    pub params: Option<Vec<ParameterNode>>,
    pub return_type_opt: Option<TypeNode>,
    pub alias: Option<MessageNode>,
}

impl InterfaceMethodNode {
    pub fn new(
        name: String,
        params: Option<Vec<ParameterNode>>,
        return_type: Option<TypeNode>,
        alias: Option<MessageNode>,
    ) -> InterfaceMethodNode {
        InterfaceMethodNode {
            name,
            params,
            return_type_opt: return_type,
            alias,
        }
    }
}

impl NodeElement for InterfaceMethodNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_interface_method_node(self);
    }
}

//-----------------------------------------------------//

#[derive(Clone)]
pub struct ParameterNode {
    pub param_name: String,
    pub param_type_opt: Option<TypeNode>,
    pub scope: IdentifierDeclScope,
}

impl ParameterNode {
    pub fn new(
        param_name: String,
        param_type_opt: Option<TypeNode>,
        scope: IdentifierDeclScope,
    ) -> ParameterNode {
        ParameterNode {
            param_name,
            param_type_opt,
            scope,
            //           param_context,
        }
    }
}

impl NodeElement for ParameterNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_parameter_node(self);
    }
}

//-----------------------------------------------------//

pub struct ActionNode {
    pub name: String,
    pub params: Option<Vec<ParameterNode>>,
    pub type_opt: Option<TypeNode>,
    pub code_opt: Option<String>,
}

impl ActionNode {
    pub fn new(
        name: String,
        params: Option<Vec<ParameterNode>>,
        type_opt: Option<TypeNode>,
        code_opt: Option<String>,
    ) -> ActionNode {
        ActionNode {
            name,
            params,
            type_opt,
            code_opt,
        }
    }
}

impl NodeElement for ActionNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_action_decl_node(self);
    }
    fn accept_rust_impl(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_action_impl_node(self);
    }
}

//-----------------------------------------------------//

pub struct VariableDeclNode {
    pub name: String,
    pub type_opt: Option<TypeNode>,
    pub is_constant: bool,
    pub initializer_expr_t_opt: Option<ExprType>,
    pub identifier_decl_scope: IdentifierDeclScope,
}

impl VariableDeclNode {
    pub fn new(
        name: String,
        type_opt: Option<TypeNode>,
        is_constant: bool,
        initializer_expr_t_opt: Option<ExprType>,
        identifier_decl_scope: IdentifierDeclScope,
    ) -> VariableDeclNode {
        VariableDeclNode {
            name,
            type_opt,
            is_constant,
            initializer_expr_t_opt,
            identifier_decl_scope,
        }
    }
}

impl NodeElement for VariableDeclNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_variable_decl_node(self);
    }
    fn accept_rust_domain_var_decl(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_domain_variable_decl_node(self);
    }
}

//-----------------------------------------------------//

// TODO: consider call this a SystemVariableNode to differentiate
// from external variable references.

pub struct VariableNode {
    pub id_node: IdentifierNode,
    //   pub call_chain:Option<Vec<Box<dyn CallableExpr>>>,
    pub scope: IdentifierDeclScope,
    pub symbol_type_rcref_opt: Option<Rc<RefCell<SymbolType>>>, // TODO: consider a new enum for just variable types
}
impl VariableNode {
    pub fn new(
        id_node: IdentifierNode,
        scope: IdentifierDeclScope,
        symbol_type_rcref_opt: Option<Rc<RefCell<SymbolType>>>,
    ) -> VariableNode {
        VariableNode {
            id_node,
            scope, // TODO: consider accessor or moving out of IdentifierNode
            symbol_type_rcref_opt,
        }
    }
}

impl NodeElement for VariableNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_variable_expr_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_variable_expr_node_to_string(self, output);
    }
}

//-----------------------------------------------------//

pub struct MachineBlockNode {
    pub states: Vec<Rc<RefCell<StateNode>>>,
}

impl MachineBlockNode {
    pub fn new(states: Vec<Rc<RefCell<StateNode>>>) -> MachineBlockNode {
        MachineBlockNode { states }
    }
}

impl NodeElement for MachineBlockNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_machine_block_node(self);
    }
}

//-----------------------------------------------------//

pub struct ActionsBlockNode {
    pub actions: Vec<Rc<RefCell<ActionNode>>>,
}

impl ActionsBlockNode {
    pub fn new(actions: Vec<Rc<RefCell<ActionNode>>>) -> ActionsBlockNode {
        ActionsBlockNode { actions }
    }
}

impl NodeElement for ActionsBlockNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_actions_block_node(self);
    }
    fn accept_rust_trait(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_action_node_rust_trait(self);
    }
    fn accept_rust_impl(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_actions_node_rust_impl(self);
    }
}

//-----------------------------------------------------//

pub struct DomainBlockNode {
    pub member_variables: Vec<Rc<RefCell<VariableDeclNode>>>,
}

impl DomainBlockNode {
    pub fn new(member_variables: Vec<Rc<RefCell<VariableDeclNode>>>) -> DomainBlockNode {
        DomainBlockNode { member_variables }
    }
}

impl NodeElement for DomainBlockNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_domain_block_node(self);
    }
}

//-----------------------------------------------------//

pub struct StateNode {
    pub name: String,
    pub params_opt: Option<Vec<ParameterNode>>,
    pub vars_opt: Option<Vec<Rc<RefCell<VariableDeclNode>>>>,
    pub calls_opt: Option<Vec<CallChainLiteralExprNode>>,
    pub evt_handlers_rcref: Vec<Rc<RefCell<EventHandlerNode>>>,
    pub enter_event_handler_opt: Option<Rc<RefCell<EventHandlerNode>>>,
    pub exit_event_handler_opt: Option<Rc<RefCell<EventHandlerNode>>>,
    // pub transitions:Vec<Rc<RefCell<TransitionStatementNode>>>,
    pub dispatch_opt: Option<DispatchNode>,
    pub line: usize,
}

impl StateNode {
    pub fn new(
        name: String,
        params: Option<Vec<ParameterNode>>,
        vars: Option<Vec<Rc<RefCell<VariableDeclNode>>>>,
        calls: Option<Vec<CallChainLiteralExprNode>>,
        evt_handlers_rcref: Vec<Rc<RefCell<EventHandlerNode>>>,
        enter_event_handler_opt: Option<Rc<RefCell<EventHandlerNode>>>,
        exit_event_handler_opt: Option<Rc<RefCell<EventHandlerNode>>>,
        dispatch_opt: Option<DispatchNode>,
        line: usize,
    ) -> StateNode {
        StateNode {
            name,
            params_opt: params,
            vars_opt: vars,
            calls_opt: calls,
            evt_handlers_rcref,
            enter_event_handler_opt,
            exit_event_handler_opt,
            // transitions:Vec::new(),
            dispatch_opt,
            line,
        }
    }
}

impl NodeElement for StateNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_state_node(self);
    }
}

//-----------------------------------------------------//

// TODO: Dead code?

pub enum StateContextType {
    StateRef {
        state_context_node: StateContextNode,
    },
    StateStackPop {},
    // MethodCall { state_context_node:StateContextNode }, // TODO
}

pub struct StateContextNode {
    pub state_ref_node: StateRefNode,
    pub state_ref_args_opt: Option<ExprListNode>,
    pub enter_args_opt: Option<ExprListNode>,
}

impl StateContextNode {
    pub fn new(
        state_ref_node: StateRefNode,
        state_ref_args_opt: Option<ExprListNode>,
        enter_args_opt: Option<ExprListNode>,
    ) -> StateContextNode {
        StateContextNode {
            state_ref_node,
            state_ref_args_opt,
            enter_args_opt,
        }
    }
}

impl NodeElement for StateContextNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_state_context_node(self);
    }
}

//-----------------------------------------------------//

pub struct StateRefNode {
    pub name: String,
}

impl StateRefNode {
    pub fn new(name: String) -> StateRefNode {
        StateRefNode { name }
    }
}

impl NodeElement for StateRefNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_state_ref_node(self);
    }
}

//-----------------------------------------------------//

pub struct DispatchNode {
    pub target_state_ref: StateRefNode,
    pub line: usize,
}

impl DispatchNode {
    pub fn new(target_state_ref: StateRefNode, line: usize) -> DispatchNode {
        DispatchNode {
            target_state_ref,
            line,
        }
    }
}

impl NodeElement for DispatchNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_dispatch_node(self);
    }
}

//-----------------------------------------------------//

pub struct EventHandlerNode {
    //    pub event_handler_type:EventHandlerType,
    pub state_name: String,
    pub msg_t: MessageType,
    pub statements: Vec<DeclOrStmtType>,
    pub terminator_node: TerminatorExpr,
    pub event_symbol_rcref: Rc<RefCell<EventSymbol>>,
    // this is so we can know to declare a StateContext at the
    // top of the event handler.
    pub event_handler_has_transition: bool,
    pub line: usize,
}

impl EventHandlerNode {
    pub fn new(
        //event_handler_type:EventHandlerType,
        state_name: String,
        msg_t: MessageType,
        statements: Vec<DeclOrStmtType>,
        terminator_node: TerminatorExpr,
        event_symbol_rcref: Rc<RefCell<EventSymbol>>,
        event_handler_has_transition: bool,
        line: usize,
    ) -> EventHandlerNode {
        EventHandlerNode {
            //  event_handler_type,
            state_name,
            msg_t,
            statements,
            terminator_node,
            event_symbol_rcref,
            event_handler_has_transition,
            line,
        }
    }

    pub fn get_event_ret_type(&self) -> String {
        match &self.event_symbol_rcref.borrow().ret_type_opt {
            Some(c) => c.type_str.clone(),
            None => String::new(),
        }
    }
}

impl NodeElement for EventHandlerNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_event_handler_node(self);
    }
}

//-----------------------------------------------------//

// TODO: what is AnyMessge?
pub enum MessageType {
    CustomMessage { message_node: MessageNode },
    AnyMessage { line: usize },
}
//-----------------------------------------------------//

pub struct MessageNode {
    pub name: String,
    pub line: usize,
}

impl MessageNode {
    pub(crate) fn new(name: String, line: usize) -> MessageNode {
        MessageNode { name, line }
    }
}

//-----------------------------------------------------//

//pub struct AnyMessageNode;

//-----------------------------------------------------//

// TODO - reconcile the various terminator types

pub enum TerminatorType {
    Return,
    Continue,
}

pub struct TerminatorExpr {
    pub terminator_type: TerminatorType,
    pub return_expr_t_opt: Option<ExprType>,
    //    pub return_type_opt:Option<String>,
    pub line: usize,
}

impl TerminatorExpr {
    pub fn new(
        terminator_type: TerminatorType,
        return_expr_t_opt: Option<ExprType>,
        /*return_type_opt:Option<String>,*/ line: usize,
    ) -> TerminatorExpr {
        TerminatorExpr {
            terminator_type,
            return_expr_t_opt,
            line,
        }
    }
}

impl NodeElement for TerminatorExpr {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_event_handler_terminator_node(self);
    }
}

#[allow(dead_code)] // is used, don't know why I need this
pub enum StateCallType {
    ActionCallExprT {
        action_call_expr_node: ActionCallExprNode,
    },
    CallExprT {
        call_expr_node: CallExprNode,
    },
}

//-----------------------------------------------------//

pub enum FrameEventPart {
    Event {
        is_reference: bool,
    },
    Message {
        is_reference: bool,
    },
    Param {
        param_tok: Token,
        is_reference: bool,
    },
    Return {
        is_reference: bool,
    },
}

impl NodeElement for FrameEventPart {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_frame_event_part(self);
    }
}

//-----------------------------------------------------//
//                  -Expressions-

pub enum ExprType {
    AssignmentExprT {
        assignment_expr_node: AssignmentExprNode,
    },
    #[allow(dead_code)] // is used, don't know why I need this
    ActionCallExprT {
        action_call_expr_node: ActionCallExprNode,
    },
    CallChainLiteralExprT {
        call_chain_expr_node: CallChainLiteralExprNode,
    },
    #[allow(dead_code)] // is used, don't know why I need this
    CallExprT {
        call_expr_node: CallExprNode,
    },
    #[allow(dead_code)] // is used, don't know why I need this
    CallExprListT {
        call_expr_list_node: CallExprListNode,
    },
    ExprListT {
        expr_list_node: ExprListNode,
    },
    VariableExprT {
        var_node: VariableNode,
    },
    LiteralExprT {
        literal_expr_node: LiteralExprNode,
    },
    StateStackOperationExprT {
        state_stack_op_node: StateStackOperationNode,
    },
    FrameEventExprT {
        frame_event_part: FrameEventPart,
    },
    UnaryExprT {
        unary_expr_node: UnaryExprNode,
    },
    BinaryExprT {
        binary_expr_node: BinaryExprNode,
    },
}

impl NodeElement for ExprType {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        match self {
            ExprType::AssignmentExprT {
                assignment_expr_node,
            } => {
                ast_visitor.visit_assignment_expr_node(assignment_expr_node);
            }
            ExprType::CallChainLiteralExprT {
                call_chain_expr_node,
            } => {
                ast_visitor.visit_call_chain_literal_expr_node(call_chain_expr_node);
            }
            ExprType::CallExprT { call_expr_node } => {
                ast_visitor.visit_call_expression_node(call_expr_node);
            }
            ExprType::CallExprListT {
                call_expr_list_node,
            } => {
                ast_visitor.visit_call_expr_list_node(call_expr_list_node);
            }
            ExprType::ExprListT { expr_list_node } => {
                ast_visitor.visit_expression_list_node(expr_list_node);
            }
            ExprType::VariableExprT { var_node: id_node } => {
                ast_visitor.visit_variable_expr_node(id_node);
            }
            ExprType::LiteralExprT { literal_expr_node } => {
                ast_visitor.visit_literal_expression_node(literal_expr_node);
            }
            ExprType::StateStackOperationExprT {
                state_stack_op_node,
            } => {
                ast_visitor.visit_state_stack_operation_node(state_stack_op_node);
            }
            ExprType::FrameEventExprT { frame_event_part } => {
                ast_visitor.visit_frame_event_part(frame_event_part);
            }
            ExprType::ActionCallExprT {
                action_call_expr_node,
            } => {
                ast_visitor.visit_action_call_expression_node(action_call_expr_node);
            }
            ExprType::UnaryExprT { unary_expr_node } => {
                ast_visitor.visit_unary_expr_node(unary_expr_node);
            }
            ExprType::BinaryExprT { binary_expr_node } => {
                ast_visitor.visit_binary_expr_node(binary_expr_node);
            }
        }
    }

    // TODO: make sure this is the proper subset (think I need all)
    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        match self {
            ExprType::AssignmentExprT {
                assignment_expr_node,
            } => {
                ast_visitor.visit_assignment_expr_node_to_string(assignment_expr_node, output);
            }
            ExprType::CallChainLiteralExprT {
                call_chain_expr_node,
            } => {
                ast_visitor
                    .visit_call_chain_literal_expr_node_to_string(call_chain_expr_node, output);
            }
            ExprType::CallExprT { call_expr_node } => {
                ast_visitor.visit_call_expression_node_to_string(call_expr_node, output);
            }
            ExprType::CallExprListT {
                call_expr_list_node,
            } => {
                ast_visitor.visit_call_expr_list_node_to_string(call_expr_list_node, output);
            }
            ExprType::ExprListT { expr_list_node } => {
                ast_visitor.visit_expression_list_node_to_string(expr_list_node, output);
            }
            ExprType::VariableExprT { var_node: id_node } => {
                ast_visitor.visit_variable_expr_node_to_string(id_node, output);
            }
            ExprType::LiteralExprT { literal_expr_node } => {
                ast_visitor.visit_literal_expression_node_to_string(literal_expr_node, output);
            }
            ExprType::StateStackOperationExprT {
                state_stack_op_node,
            } => {
                ast_visitor.visit_state_stack_operation_node_to_string(state_stack_op_node, output);
            }
            ExprType::FrameEventExprT { frame_event_part } => {
                ast_visitor.visit_frame_event_part_to_string(frame_event_part, output);
            }
            ExprType::ActionCallExprT {
                action_call_expr_node,
            } => {
                ast_visitor
                    .visit_action_call_expression_node_to_string(action_call_expr_node, output);
            }
            ExprType::BinaryExprT { binary_expr_node } => {
                ast_visitor.visit_binary_expr_node_to_string(binary_expr_node, output);
            }
            ExprType::UnaryExprT { unary_expr_node } => {
                ast_visitor.visit_unary_expr_node_to_string(unary_expr_node, output);
            }
        }
    }
}

//-----------------------------------------------------//
//                  -Statements-

pub enum ExprStmtType {
    CallStmtT {
        call_stmt_node: CallStmtNode,
    },
    ActionCallStmtT {
        action_call_stmt_node: ActionCallStmtNode,
    },
    CallChainLiteralStmtT {
        call_chain_literal_stmt_node: CallChainLiteralStmtNode,
    },
    AssignmentStmtT {
        assignment_stmt_node: AssignmentStmtNode,
    },
    VariableStmtT {
        variable_stmt_node: VariableStmtNode,
    },
}

pub enum StatementType {
    ExpressionStmt {
        expr_stmt_t: ExprStmtType,
    },
    TransitionStmt {
        transition_statement: TransitionStatementNode,
    },
    ChangeStateStmt {
        change_state_stmt: ChangeStateStatementNode,
    },
    TestStmt {
        test_stmt_node: TestStatementNode,
    },
    StateStackStmt {
        state_stack_operation_statement_node: StateStackOperationStatementNode,
    },
    #[allow(dead_code)] // is used, don't know why I need this
    NoStmt,
}

//-----------------------------------------------------//

pub enum DeclOrStmtType {
    VarDeclT {
        var_decl_t_rc_ref: Rc<RefCell<VariableDeclNode>>,
    },
    StmtT {
        stmt_t: StatementType,
    },
}

//-----------------------------------------------------//

pub struct CallStmtNode {
    pub call_expr_node: CallExprNode,
}

impl CallStmtNode {
    pub fn new(call_expr_node: CallExprNode) -> CallStmtNode {
        CallStmtNode { call_expr_node }
    }
}

impl NodeElement for CallStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_call_statement_node(self);
    }
}

//-----------------------------------------------------//

pub struct ActionCallStmtNode {
    pub action_call_expr_node: ActionCallExprNode,
}

impl ActionCallStmtNode {
    pub fn new(action_call_expr_node: ActionCallExprNode) -> ActionCallStmtNode {
        ActionCallStmtNode {
            action_call_expr_node,
        }
    }
}

impl NodeElement for ActionCallStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_action_call_statement_node(self);
    }
}

//-----------------------------------------------------//

pub struct CallChainLiteralStmtNode {
    pub call_chain_literal_expr_node: CallChainLiteralExprNode,
}

impl CallChainLiteralStmtNode {
    pub fn new(call_chain_literal_expr_node: CallChainLiteralExprNode) -> CallChainLiteralStmtNode {
        CallChainLiteralStmtNode {
            call_chain_literal_expr_node,
        }
    }
}

impl NodeElement for CallChainLiteralStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_call_chain_literal_statement_node(self);
    }
}

//-----------------------------------------------------//

pub struct AssignmentStmtNode {
    pub assignment_expr_node: AssignmentExprNode,
}

impl AssignmentStmtNode {
    pub fn new(assignment_expr_node: AssignmentExprNode) -> AssignmentStmtNode {
        AssignmentStmtNode {
            assignment_expr_node,
        }
    }

    pub fn get_line(&self) -> usize {
        self.assignment_expr_node.line
    }
}

impl NodeElement for AssignmentStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_assignment_statement_node(self);
    }
}

//-----------------------------------------------------//

pub struct AssignmentExprNode {
    pub l_value_box: Box<ExprType>,
    pub r_value_box: Box<ExprType>,
    pub line: usize,
}

impl AssignmentExprNode {
    pub fn new(l_value: ExprType, r_value: ExprType, line: usize) -> AssignmentExprNode {
        AssignmentExprNode {
            l_value_box: Box::new(l_value),
            r_value_box: Box::new(r_value),
            line,
        }
    }
}

impl NodeElement for AssignmentExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_assignment_expr_node(self);
    }
}

//-----------------------------------------------------//

pub struct VariableStmtNode {
    pub var_node: VariableNode,
}

impl VariableStmtNode {
    pub fn new(var_node: VariableNode) -> VariableStmtNode {
        VariableStmtNode { var_node }
    }

    pub fn get_line(&self) -> usize {
        self.var_node.id_node.line
    }
}

impl NodeElement for VariableStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_variable_stmt_node(self);
    }
}

//-----------------------------------------------------//

pub struct TransitionStatementNode {
    pub target_state_context_t: StateContextType,
    pub exit_args_opt: Option<ExprListNode>,
    pub label_opt: Option<String>,
}

impl TransitionStatementNode {
    // pub fn new(target_state_context_t:StateContextType,
    //            exit_args_opt:Option<ExprListNode>,
    //            label_opt:Option<String>) -> TransitionStatementNode {
    //     TransitionStatementNode {
    //         target_state_context_t,
    //         exit_args_opt,
    //         label_opt,
    //
    //     }
    // }
}

impl NodeElement for TransitionStatementNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_transition_statement_node(self);
    }
}

//-----------------------------------------------------//

pub struct ChangeStateStatementNode {
    pub state_context_t: StateContextType,
    pub label_opt: Option<String>,
}

impl ChangeStateStatementNode {
    // pub fn new( state_context_t:StateContextType,
    //             label_opt:Option<String>) -> ChangeStateStatementNode {
    //     ChangeStateStatementNode {
    //         state_context_t,
    //         label_opt,
    //
    //     }
    // }
}

impl NodeElement for ChangeStateStatementNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_change_state_statement_node(self);
    }
}

//-----------------------------------------------------//

pub struct TestStatementNode {
    pub test_t: TestType,
}

impl TestStatementNode {
    pub fn new(test_t: TestType) -> TestStatementNode {
        TestStatementNode { test_t }
    }
}

impl NodeElement for TestStatementNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_test_statement_node(self);
    }
}

//-----------------------------------------------------//
//
pub struct StateStackOperationStatementNode {
    pub state_stack_operation_node: StateStackOperationNode,
}

impl StateStackOperationStatementNode {
    pub fn new(
        state_stack_operation_node: StateStackOperationNode,
    ) -> StateStackOperationStatementNode {
        StateStackOperationStatementNode {
            state_stack_operation_node,
        }
    }
}

impl NodeElement for StateStackOperationStatementNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_state_stack_operation_statement_node(self);
    }
}

//-----------------------------------------------------//

pub struct InterfaceMethodCallExprNode {
    pub identifier: IdentifierNode,
    pub call_expr_list: CallExprListNode,
    pub interface_symbol_rcref_opt: Option<Rc<RefCell<InterfaceMethodSymbol>>>,
}

impl InterfaceMethodCallExprNode {
    // Harvest the id and arguments from the CallExpressionNode.
    // It will be discarded.

    pub fn new(call_expr_node: CallExprNode) -> InterfaceMethodCallExprNode {
        InterfaceMethodCallExprNode {
            identifier: call_expr_node.identifier,
            call_expr_list: call_expr_node.call_expr_list,
            interface_symbol_rcref_opt: None,
        }
    }

    pub fn set_interface_symbol(
        &mut self,
        interface_method_symbol: &Rc<RefCell<InterfaceMethodSymbol>>,
    ) {
        self.interface_symbol_rcref_opt = Some(Rc::clone(interface_method_symbol));
    }
}

impl NodeElement for InterfaceMethodCallExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_interface_method_call_expression_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_interface_method_call_expression_node_to_string(self, output);
    }
}

//-----------------------------------------------------//

pub struct ActionCallExprNode {
    pub identifier: IdentifierNode,
    pub call_expr_list: CallExprListNode,
    pub action_symbol_rcref_opt: Option<Rc<RefCell<ActionDeclSymbol>>>,
}

impl ActionCallExprNode {
    // Harvest the id and arguments from the CallExpressionNode.
    // It will be discarded.

    pub fn new(call_expr_node: CallExprNode) -> ActionCallExprNode {
        ActionCallExprNode {
            identifier: call_expr_node.identifier,
            call_expr_list: call_expr_node.call_expr_list,
            action_symbol_rcref_opt: None,
        }
    }

    pub fn set_action_symbol(&mut self, action_symbol: &Rc<RefCell<ActionDeclSymbol>>) {
        self.action_symbol_rcref_opt = Some(Rc::clone(action_symbol));
    }
}

impl NodeElement for ActionCallExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_action_call_expression_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_action_call_expression_node_to_string(self, output);
    }
}

//-----------------------------------------------------//

pub struct CallChainLiteralExprNode {
    pub call_chain: VecDeque<CallChainLiteralNodeType>,
}

impl CallChainLiteralExprNode {
    pub fn new(call_chain: VecDeque<CallChainLiteralNodeType>) -> CallChainLiteralExprNode {
        CallChainLiteralExprNode { call_chain }
    }
}

impl NodeElement for CallChainLiteralExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_call_chain_literal_expr_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_call_chain_literal_expr_node_to_string(self, output);
    }
}

//-----------------------------------------------------//
#[derive(PartialEq)]
pub enum OperatorType {
    Plus,
    Minus,
    Multiply,
    Divide,
    Greater,
    GreaterEqual,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Not,
    LogicalAnd,
    LogicalOr,
    LogicalXor,
    Negated,
}

impl NodeElement for OperatorType {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_operator_type(self);
    }

    // TODO: make sure this is the proper subset (think I need all)
    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_operator_type_to_string(self, output);
    }
}

impl OperatorType {
    pub fn get_operator_type(token_type: &TokenType) -> OperatorType {
        match token_type {
            TokenType::PlusTok => Plus,
            TokenType::DashTok => Minus,
            TokenType::StarTok => Multiply,
            TokenType::ForwardSlashTok => Divide,
            TokenType::GTTok => Greater,
            TokenType::GreaterEqualTok => GreaterEqual,
            TokenType::LTTok => OperatorType::Less,
            TokenType::LessEqualTok => LessEqual,
            TokenType::BangTok => OperatorType::Not,
            TokenType::EqualEqualTok => OperatorType::EqualEqual,
            TokenType::BangEqualTok => OperatorType::NotEqual,
            TokenType::LogicalAndTok => OperatorType::LogicalAnd,
            TokenType::PipePipeTok => OperatorType::LogicalOr,
            TokenType::LogicalXorTok => OperatorType::LogicalXor,
            _ => panic!("Invalid token for operator."),
        }
    }
}

//-----------------------------------------------------//

pub struct UnaryExprNode {
    pub operator: OperatorType,
    pub right_rcref: Rc<RefCell<ExprType>>,
}

impl UnaryExprNode {
    pub fn new(operator: OperatorType, right: ExprType) -> UnaryExprNode {
        UnaryExprNode {
            operator,
            right_rcref: Rc::new(RefCell::new(right)),
        }
    }
}

impl NodeElement for UnaryExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_unary_expr_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_unary_expr_node_to_string(self, output);
    }
}

//-----------------------------------------------------//

pub struct BinaryExprNode {
    pub left_rcref: Rc<RefCell<ExprType>>,
    pub operator: OperatorType,
    pub right_rcref: Rc<RefCell<ExprType>>,
}

impl BinaryExprNode {
    pub fn new(left: ExprType, operator: OperatorType, right: ExprType) -> BinaryExprNode {
        BinaryExprNode {
            left_rcref: Rc::new(RefCell::new(left)),
            operator,
            right_rcref: Rc::new(RefCell::new(right)),
        }
    }
}

impl NodeElement for BinaryExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_binary_expr_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_binary_expr_node_to_string(self, output);
    }
}

//-----------------------------------------------------//

pub struct CallExprNode {
    pub identifier: IdentifierNode,
    pub call_expr_list: CallExprListNode,
    pub call_chain: Option<Vec<Box<dyn CallableExpr>>>,
}

impl CallExprNode {
    pub fn new(
        identifier: IdentifierNode,
        call_expr_list: CallExprListNode,
        call_chain: Option<Vec<Box<dyn CallableExpr>>>,
    ) -> CallExprNode {
        CallExprNode {
            identifier,
            call_expr_list,
            call_chain,
        }
    }
}

impl NodeElement for CallExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_call_expression_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_call_expression_node_to_string(self, output);
    }
}

impl CallableExpr for CallExprNode {
    fn set_call_chain(&mut self, call_chain: Vec<Box<dyn CallableExpr>>) {
        self.call_chain = Some(call_chain);
    }

    fn callable_accept(&self, ast_visitor: &mut dyn AstVisitor) {
        self.accept(ast_visitor);
    }
}

//-----------------------------------------------------//

// #[derive(Clone)]
pub struct CallExprListNode {
    pub exprs_t: Vec<ExprType>,
}

impl CallExprListNode {
    pub fn new(exprs_t: Vec<ExprType>) -> CallExprListNode {
        CallExprListNode { exprs_t }
    }
}

impl NodeElement for CallExprListNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_call_expr_list_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_call_expr_list_node_to_string(self, output);
    }
}

//-----------------------------------------------------//

// #[derive(Clone)]
pub struct ExprListNode {
    pub exprs_t: Vec<ExprType>,
}

impl ExprListNode {
    pub fn new(exprs_t: Vec<ExprType>) -> ExprListNode {
        ExprListNode { exprs_t }
    }
}

impl NodeElement for ExprListNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_expression_list_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_expression_list_node_to_string(self, output);
    }
}

//-----------------------------------------------------//

// TODO!!: why aren't there MachineBlock, InterfaceBlock etc?
// there is a misalighment of ParseScope vs IdentiferDeclScope?
// for instance States have a IdentiferDeclScope of None. Should be machine
#[derive(Clone, PartialEq)]
pub enum IdentifierDeclScope {
    //     Global,  TODO!
    InterfaceBlock,
    DomainBlock,
    ActionsBlock,
    StateParam,
    StateVar,
    EventHandlerParam,
    EventHandlerVar,
    None,
}

// #[derive(Clone)]
pub struct IdentifierNode {
    pub name: Token,
    pub call_chain: Option<Vec<Box<dyn CallableExpr>>>,
    pub scope: IdentifierDeclScope,
    pub is_reference: bool,
    pub line: usize,
}

impl IdentifierNode {
    pub fn new(
        name: Token,
        call_chain: Option<Vec<Box<dyn CallableExpr>>>,
        scope: IdentifierDeclScope,
        is_reference: bool,
        line: usize,
    ) -> IdentifierNode {
        IdentifierNode {
            name,
            call_chain,
            scope,
            is_reference,
            line,
        }
    }
}

impl NodeElement for IdentifierNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_identifier_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_identifier_node_to_string(self, output);
    }
}

impl CallableExpr for IdentifierNode {
    fn set_call_chain(&mut self, call_chain: Vec<Box<dyn CallableExpr>>) {
        self.call_chain = Some(call_chain);
    }
    fn callable_accept(&self, ast_visitor: &mut dyn AstVisitor) {
        self.accept(ast_visitor);
    }
}

//-----------------------------------------------------//

#[derive(Clone)]
pub struct LiteralExprNode {
    pub token_t: TokenType,
    pub value: String,
    pub is_reference: bool,
}

impl LiteralExprNode {
    pub fn new(token_t: TokenType, value: String) -> LiteralExprNode {
        LiteralExprNode {
            token_t,
            value,
            is_reference: false,
        }
    }
}

impl NodeElement for LiteralExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_literal_expression_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_literal_expression_node_to_string(self, output);
    }
}

//-----------------------------------------------------//

// &String | &str | Widget<int> | `& mut String` | &`mut String` | *x

#[derive(Clone)]
pub struct TypeNode {
    is_superstring: bool,
    is_reference: bool,
    type_str: String,
}

impl TypeNode {
    pub fn new(is_superstring: bool, is_reference: bool, type_str: String) -> TypeNode {
        TypeNode {
            is_superstring,
            is_reference,
            type_str,
        }
    }

    pub fn get_type_str(&self) -> String {
        let mut s = String::new();

        if self.is_reference {
            s.push('&');
        }

        s.push_str(&*self.type_str);
        s
    }
}

impl NodeElement for TypeNode {
    fn accept(&self, _ast_visitor: &mut dyn AstVisitor) {
        panic!("TODO?");
        //ast_visitor.visit_type_node(self);
    }
    //
    // fn accept_to_string(&self, ast_visitor:&mut dyn AstVisitor,output:&mut String, ) {
    //     ast_visitor.visit_typedef_node(self, output);
    // }
}

//-----------------------------------------------------//

pub enum TestType {
    BoolTest {
        bool_test_node: BoolTestNode,
    },
    StringMatchTest {
        string_match_test_node: StringMatchTestNode,
    },
    NumberMatchTest {
        number_match_test_node: NumberMatchTestNode,
    },
}

pub struct BoolTestNode {
    pub conditional_branch_nodes: Vec<BoolTestConditionalBranchNode>,
    pub else_branch_node_opt: Option<BoolTestElseBranchNode>,
}

impl BoolTestNode {
    pub fn new(
        conditional_branch_nodes: Vec<BoolTestConditionalBranchNode>,
        else_branch_node_opt: Option<BoolTestElseBranchNode>,
    ) -> BoolTestNode {
        BoolTestNode {
            conditional_branch_nodes,
            else_branch_node_opt,
        }
    }
}

impl NodeElement for BoolTestNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_bool_test_node(self);
    }
}

//-----------------------------------------------------//

pub struct BoolTestConditionalBranchNode {
    pub is_negated: bool,
    pub expr_t: ExprType,
    pub statements: Vec<DeclOrStmtType>,
    pub branch_terminator_expr_opt: Option<TerminatorExpr>,
}

impl BoolTestConditionalBranchNode {
    pub fn new(
        is_negated: bool,
        expr_t: ExprType,
        statements: Vec<DeclOrStmtType>,
        branch_terminator_t_opt: Option<TerminatorExpr>,
    ) -> BoolTestConditionalBranchNode {
        BoolTestConditionalBranchNode {
            is_negated,
            expr_t,
            statements,
            branch_terminator_expr_opt: branch_terminator_t_opt,
        }
    }
}

impl NodeElement for BoolTestConditionalBranchNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_bool_test_conditional_branch_node(self);
    }
}

//-----------------------------------------------------//

pub struct BoolTestElseBranchNode {
    pub statements: Vec<DeclOrStmtType>,
    pub branch_terminator_expr_opt: Option<TerminatorExpr>,
}

impl BoolTestElseBranchNode {
    pub fn new(
        statements: Vec<DeclOrStmtType>,
        branch_terminator_t_opt: Option<TerminatorExpr>,
    ) -> BoolTestElseBranchNode {
        BoolTestElseBranchNode {
            statements,
            branch_terminator_expr_opt: branch_terminator_t_opt,
        }
    }
}

impl NodeElement for BoolTestElseBranchNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_bool_test_else_branch_node(self);
    }
}

//-----------------------------------------------------//

pub struct StringMatchTestNode {
    pub expr_t: ExprType,
    pub match_branch_nodes: Vec<StringMatchTestMatchBranchNode>,
    pub else_branch_node_opt: Option<StringMatchTestElseBranchNode>,
}

impl StringMatchTestNode {
    pub fn new(
        expr_t: ExprType,
        match_branch_nodes: Vec<StringMatchTestMatchBranchNode>,
        else_branch_node_opt: Option<StringMatchTestElseBranchNode>,
    ) -> StringMatchTestNode {
        StringMatchTestNode {
            expr_t,
            match_branch_nodes,
            else_branch_node_opt,
        }
    }
}

impl NodeElement for StringMatchTestNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_string_match_test_node(self);
    }
}

//-----------------------------------------------------//

pub struct StringMatchTestMatchBranchNode {
    pub string_match_pattern_node: StringMatchTestPatternNode,
    pub statements: Vec<DeclOrStmtType>,
    pub branch_terminator_expr_opt: Option<TerminatorExpr>,
}

impl StringMatchTestMatchBranchNode {
    pub fn new(
        string_match_pattern_node: StringMatchTestPatternNode,
        statements: Vec<DeclOrStmtType>,
        branch_terminator_t_opt: Option<TerminatorExpr>,
    ) -> StringMatchTestMatchBranchNode {
        StringMatchTestMatchBranchNode {
            string_match_pattern_node,
            statements,
            branch_terminator_expr_opt: branch_terminator_t_opt,
        }
    }
}

impl NodeElement for StringMatchTestMatchBranchNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_string_match_test_match_branch_node(self);
    }
}

//-----------------------------------------------------//

pub struct StringMatchTestElseBranchNode {
    pub statements: Vec<DeclOrStmtType>,
    pub branch_terminator_expr_opt: Option<TerminatorExpr>,
}

impl StringMatchTestElseBranchNode {
    pub fn new(
        statements: Vec<DeclOrStmtType>,
        branch_terminator_t_opt: Option<TerminatorExpr>,
    ) -> StringMatchTestElseBranchNode {
        StringMatchTestElseBranchNode {
            statements,
            branch_terminator_expr_opt: branch_terminator_t_opt,
        }
    }
}

impl NodeElement for StringMatchTestElseBranchNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_string_match_test_else_branch_node(self);
    }
}

//-----------------------------------------------------//

pub struct StringMatchTestPatternNode {
    pub match_pattern_strings: Vec<String>,
}

impl StringMatchTestPatternNode {
    pub fn new(match_pattern_strings: Vec<String>) -> StringMatchTestPatternNode {
        StringMatchTestPatternNode {
            match_pattern_strings,
        }
    }
}

impl NodeElement for StringMatchTestPatternNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_string_match_test_pattern_node(self);
    }
}

//-----------------------------------------------------//

pub struct NumberMatchTestNode {
    pub expr_t: ExprType,
    pub match_branch_nodes: Vec<NumberMatchTestMatchBranchNode>,
    pub else_branch_node_opt: Option<NumberMatchTestElseBranchNode>,
}

impl NumberMatchTestNode {
    pub fn new(
        expr_t: ExprType,
        match_branch_nodes: Vec<NumberMatchTestMatchBranchNode>,
        else_branch_node_opt: Option<NumberMatchTestElseBranchNode>,
    ) -> NumberMatchTestNode {
        NumberMatchTestNode {
            expr_t,
            match_branch_nodes,
            else_branch_node_opt,
        }
    }
}

impl NodeElement for NumberMatchTestNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_number_match_test_node(self);
    }
}

//-----------------------------------------------------//

pub struct NumberMatchTestMatchBranchNode {
    pub number_match_pattern_nodes: Vec<NumberMatchTestPatternNode>,
    pub statements: Vec<DeclOrStmtType>,
    pub branch_terminator_expr_opt: Option<TerminatorExpr>,
}

impl NumberMatchTestMatchBranchNode {
    pub fn new(
        number_match_pattern_nodes: Vec<NumberMatchTestPatternNode>,
        statements: Vec<DeclOrStmtType>,
        branch_terminator_t_opt: Option<TerminatorExpr>,
    ) -> NumberMatchTestMatchBranchNode {
        NumberMatchTestMatchBranchNode {
            number_match_pattern_nodes,
            statements,
            branch_terminator_expr_opt: branch_terminator_t_opt,
        }
    }
}

impl NodeElement for NumberMatchTestMatchBranchNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_number_match_test_match_branch_node(self);
    }
}

//-----------------------------------------------------//

pub struct NumberMatchTestElseBranchNode {
    pub statements: Vec<DeclOrStmtType>,
    pub branch_terminator_expr_opt: Option<TerminatorExpr>,
}

impl NumberMatchTestElseBranchNode {
    pub fn new(
        statements: Vec<DeclOrStmtType>,
        branch_terminator_t_opt: Option<TerminatorExpr>,
    ) -> NumberMatchTestElseBranchNode {
        NumberMatchTestElseBranchNode {
            statements,
            branch_terminator_expr_opt: branch_terminator_t_opt,
        }
    }
}

impl NodeElement for NumberMatchTestElseBranchNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_number_match_test_else_branch_node(self);
    }
}

//-----------------------------------------------------//

pub struct NumberMatchTestPatternNode {
    pub match_pattern_number: String,
}

impl NumberMatchTestPatternNode {
    pub fn new(match_pattern_number: String) -> NumberMatchTestPatternNode {
        NumberMatchTestPatternNode {
            match_pattern_number,
        }
    }
}

impl NodeElement for NumberMatchTestPatternNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_number_match_test_pattern_node(self);
    }
}

//-----------------------------------------------------//

#[derive(Clone)]
pub enum StateStackOperationType {
    Push,
    Pop,
}

#[derive(Clone)]
pub struct StateStackOperationNode {
    pub operation_t: StateStackOperationType,
}

impl StateStackOperationNode {
    pub fn new(operation_t: StateStackOperationType) -> StateStackOperationNode {
        StateStackOperationNode { operation_t }
    }
}

impl NodeElement for StateStackOperationNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_state_stack_operation_node(self);
    }
}
