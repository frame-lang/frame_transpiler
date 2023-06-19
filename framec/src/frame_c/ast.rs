#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case)]

use super::scanner::{Token, TokenType};
use super::symbol_table::{ActionDeclSymbol, EventSymbol, SymbolType};

use crate::frame_c::ast::OperatorType::{
    Divide, Greater, GreaterEqual, LessEqual, Minus, Multiply, Plus,
};
use crate::frame_c::symbol_table::{InterfaceMethodSymbol, ParameterSymbol};
use crate::frame_c::visitors::*;
use downcast_rs::__std::cell::RefCell;
use downcast_rs::*;
use std::collections::VecDeque;
use std::rc::Rc;
use std::fmt;
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
    fn accept_action_decl(&self, _ast_visitor: &mut dyn AstVisitor) {
        // no_op
    }
    fn accept_action_impl(&self, _ast_visitor: &mut dyn AstVisitor) {
        // no_op
    }

    // fn auto_pre_inc_dec_expr_type(&self, _ast_visitor: &mut dyn AstVisitor) {
    //     // no_op
    // }
    //
    // fn auto_post_inc_dec_expr_type(&self, _ast_visitor: &mut dyn AstVisitor) {
    //     // no_op
    // }
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
// See Rust attribute grammar spec:
// see https://doc.rust-lang.org/reference/attributes.html#attributes

pub enum AttributeNode {
    MetaNameValueStr { attr: AttributeMetaNameValueStr },
    MetaListIdents { attr: AttributeMetaListIdents },
}

impl AttributeNode {
    pub fn get_name(&self) -> String {
        match self {
            AttributeNode::MetaNameValueStr { attr } => attr.name.clone(),
            AttributeNode::MetaListIdents { attr } => attr.name.clone(),
        }
    }
}

// impl AttributeNode {
//     pub fn new(name: String, value: String) -> AttributeNode {
//         AttributeNode { name, value }
//     }
// }

pub struct AttributeMetaNameValueStr {
    pub name: String,
    pub value: String,
}

impl AttributeMetaNameValueStr {
    pub fn new(name: String, value: String) -> AttributeMetaNameValueStr {
        AttributeMetaNameValueStr { name, value }
    }
}

// e.g. macro_use(foo, bar)
pub struct AttributeMetaListIdents {
    pub name: String,
    pub idents: Vec<String>,
}

impl AttributeMetaListIdents {
    pub fn new(name: String, idents: Vec<String>) -> AttributeMetaListIdents {
        AttributeMetaListIdents { name, idents }
    }
}

//-----------------------------------------------------//

pub struct SystemNode {
    pub name: String,
    pub header: String,
    pub attributes_opt: Option<HashMap<String, AttributeNode>>,
    pub start_state_state_params_opt: Option<Vec<ParameterNode>>,
    pub start_state_enter_params_opt: Option<Vec<ParameterNode>>,
    pub domain_params_opt: Option<Vec<ParameterNode>>,
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
        start_state_state_params_opt: Option<Vec<ParameterNode>>,
        start_state_enter_params_opt: Option<Vec<ParameterNode>>,
        domain_params_opt: Option<Vec<ParameterNode>>,
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
            start_state_state_params_opt,
            start_state_enter_params_opt,
            domain_params_opt,
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

#[derive(Clone, PartialEq)]
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

// impl PartialEq for ParameterNode {
//     fn eq(&self, other: &Self) -> bool {
//         if self.param_name.ne(&other.param_name) {
//             return false;
//         }
//
//         return true;
//     }
// }
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
    fn accept_action_decl(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_action_decl_node(self);
    }
    fn accept_action_impl(&self, ast_visitor: &mut dyn AstVisitor) {
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


impl fmt::Display for VariableNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.id_node.to_string())
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
    pub fn get_first_state(&self) -> Option<&Rc<RefCell<StateNode>>> {
        self.states.get(0)
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

#[derive(Clone, PartialEq)]
pub enum FrameEventPart {
    Event {
        is_reference: bool,
    },
    Message {
        is_reference: bool,
    },
    Param {
        //        param_tok: Token,
        param_symbol_rcref: Rc<RefCell<ParameterSymbol>>,
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
//
// pub enum IncDecExprContext {
//     Pre,
//     Post,
// }
//
// pub enum IncDecExprType<'a> {
//     CallChainLiteralExprT {
//         call_chain_expr_node:  RefCell<&'a CallChainLiteralExprNode>,
//     },
// }
//
// impl<'a> IncDecExprType<'a> {
//     pub fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
//         match self {
//             IncDecExprType::CallChainLiteralExprT {mut call_chain_expr_node} => {
//                 call_chain_expr_node.borrow().accept_to_string(ast_visitor,output);
//             }
//         }
//     }
// }

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
    // LoopExprT {
    //     loop_types: LoopTypes,
    // }
}


pub enum RefExprType<'a> {
    AssignmentExprT {
        assignment_expr_node: &'a AssignmentExprNode,
    },
    // #[allow(dead_code)] // is used, don't know why I need this
    // ActionCallExprT {
    //     action_call_expr_node: ActionCallExprNode,
    // },
    CallChainLiteralExprT {
        call_chain_expr_node: &'a CallChainLiteralExprNode,
    },
    // #[allow(dead_code)] // is used, don't know why I need this
    // CallExprT {
    //     call_expr_node: CallExprNode,
    // },
    // #[allow(dead_code)] // is used, don't know why I need this
    // CallExprListT {
    //     call_expr_list_node: CallExprListNode,
    // },
    ExprListT {
        expr_list_node: &'a ExprListNode,
    },
    // VariableExprT {
    //     var_node: VariableNode,
    // },
    // LiteralExprT {
    //     literal_expr_node: LiteralExprNode,
    // },
    // StateStackOperationExprT {
    //     state_stack_op_node: StateStackOperationNode,
    // },
    // FrameEventExprT {
    //     frame_event_part: FrameEventPart,
    // },
    // UnaryExprT {
    //     unary_expr_node: UnaryExprNode,
    // },
    // BinaryExprT {
    //     binary_expr_node: BinaryExprNode,
    // },
    LoopStmtT {
        loop_types: &'a LoopTypes,
    }
}

impl fmt::Display for ExprType {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExprType::CallChainLiteralExprT {call_chain_expr_node} => {
                write!(f,"{}", call_chain_expr_node.to_string())
            },
            _ => {
                write!(f, "TODO")
            }
        }
    }
}

impl ExprType {
    /// Get the name of expression type we're looking at. Useful for debugging.
    pub fn expr_type_name(&self) -> &'static str {
        match self {
            ExprType::AssignmentExprT { .. } => "AssignmentExprT",
            ExprType::ActionCallExprT { .. } => "ActionCallExprT",
            ExprType::CallChainLiteralExprT { .. } => "CallChainLiteralExprT",
            ExprType::CallExprT { .. } => "CallExprT",
            ExprType::CallExprListT { .. } => "CallExprListT",
            ExprType::ExprListT { .. } => "ExprListT",
            ExprType::VariableExprT { .. } => "VariableExprT",
            ExprType::LiteralExprT { .. } => "LiteralExprT",
            ExprType::StateStackOperationExprT { .. } => "StateStackOperationExprT",
            ExprType::FrameEventExprT { .. } => "FrameEventExprT",
            ExprType::UnaryExprT { .. } => "UnaryExprT",
            ExprType::BinaryExprT { .. } => "BinaryExprT",
 //           ExprType::LoopExprT { .. } => "LoopT",
        }
    }

    pub fn auto_pre_inc_dec(&self, ast_visitor: &mut dyn AstVisitor) {
        match self {
            ExprType::CallChainLiteralExprT { call_chain_expr_node } => {
                let ref ref_expr_type = RefExprType::CallChainLiteralExprT { call_chain_expr_node };
                ast_visitor.visit_auto_pre_inc_dec_expr_node(ref_expr_type);
            }
            ExprType::ExprListT { expr_list_node } => {
                let ref ref_expr_type = RefExprType::ExprListT { expr_list_node };
                ast_visitor.visit_auto_pre_inc_dec_expr_node(ref_expr_type);
            }
            // ExprType::LoopExprT { loop_types } => {
            //     let ref ref_expr_type = RefExprType::LoopTypes { loop_types };
            //     ast_visitor.visit_auto_pre_inc_dec_expr_node(ref_expr_type);
            // }
            _ => {

            }
        }
    }

    pub fn auto_post_inc_dec(&self, ast_visitor: &mut dyn AstVisitor) {

        match self {
            ExprType::CallChainLiteralExprT { call_chain_expr_node } => {
                let ref ref_expr_type = RefExprType::CallChainLiteralExprT { call_chain_expr_node };
                ast_visitor.visit_auto_post_inc_dec_expr_node(ref_expr_type);
            }
            ExprType::ExprListT { expr_list_node } => {
                let ref ref_expr_type = RefExprType::ExprListT { expr_list_node };
                ast_visitor.visit_auto_post_inc_dec_expr_node(ref_expr_type);
            }
            _ => {

            }
        }
    }
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
  //           ExprType::LoopExprT { loop_types } => {
  //               match loop_types {
  //                   LoopTypes::LoopForExpr {loop_for_expr_node } => {
  //                       ast_visitor.visit_loop_for_expr_node(loop_for_expr_node);
  //                   }
  //                   LoopTypes::LoopInExpr{ loop_in_expr_node} => {
  //                       ast_visitor.visit_loop_in_expr_node(loop_in_expr_node);
  //                   }
  //               }
  // //              ast_visitor.visit_loop_expr_node(loop_types);
  //           }
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
            // ExprType::LoopExprT { loop_types } => {
            //     //ast_visitor.visit_loop_expr_node_to_string(loop_types, output);
            // }
        }
    }
    //
    // fn auto_inc_dec_expr_type(&self, ast_visitor: &mut dyn AstVisitor) {
    //     match self {
    //         ExprType::AssignmentExprT {
    //             assignment_expr_node,
    //         } => {
    //             ast_visitor.auto_inc_dec_assignment_expr_node(assignment_expr_node);
    //         }
    //         ExprType::CallChainLiteralExprT {
    //             call_chain_expr_node,
    //         } => {
    //             ast_visitor.auto_inc_dec_call_chain_literal_expr_node(call_chain_expr_node);
    //         }
    //         ExprType::CallExprT { call_expr_node } => {
    //         //    ast_visitor.visit_call_expression_node(call_expr_node);
    //         }
    //         ExprType::CallExprListT {
    //             call_expr_list_node,
    //         } => {
    //         //    ast_visitor.visit_call_expr_list_node(call_expr_list_node);
    //         }
    //         ExprType::ExprListT { expr_list_node } => {
    //          //   ast_visitor.auto_inc_dec_expression_list_node(expr_list_node);
    //         }
    //         ExprType::VariableExprT { var_node: id_node } => {
    //         //    ast_visitor.visit_variable_expr_node(id_node);
    //         }
    //         ExprType::LiteralExprT { literal_expr_node } => {
    //         //    ast_visitor.visit_literal_expression_node(literal_expr_node);
    //         }
    //         ExprType::StateStackOperationExprT {
    //             state_stack_op_node,
    //         } => {
    //         //    ast_visitor.visit_state_stack_operation_node(state_stack_op_node);
    //         }
    //         ExprType::FrameEventExprT { frame_event_part } => {
    //         //    ast_visitor.visit_frame_event_part(frame_event_part);
    //         }
    //         ExprType::ActionCallExprT {
    //             action_call_expr_node,
    //         } => {
    //         //    ast_visitor.visit_action_call_expression_node(action_call_expr_node);
    //         }
    //         ExprType::UnaryExprT { unary_expr_node } => {
    //        //     ast_visitor.visit_unary_expr_node(unary_expr_node);
    //         }
    //         ExprType::BinaryExprT { binary_expr_node } => {
    //             ast_visitor.auto_inc_dec_binary_expr_node(binary_expr_node);
    //         }
    //     }
    // }

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
    ExprListStmtT {
        expr_list_stmt_node: ExprListStmtNode,
    },
    // LoopStmtT {
    //     loop_stmt_node: LoopStmtNode,
    // }
}

#[allow(clippy::large_enum_variant)]
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
    LoopStmt {
        loop_stmt_node: LoopStmtNode,
    },
    #[allow(dead_code)] // is used, don't know why I need this
    NoStmt,
}

//-----------------------------------------------------//

// TODO: Adding a `Box` around `StmtT`'s argument would decreases the size of
// `VarDeclT` variant by nearly 400 bytes. However, this impacts every visitor.
#[allow(clippy::large_enum_variant)]
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
    pub is_decl: bool,
    pub line: usize,

}

impl AssignmentExprNode {
    pub fn new(l_value: ExprType, r_value: ExprType, is_decl:bool, line: usize) -> AssignmentExprNode {
        AssignmentExprNode {
            l_value_box: Box::new(l_value),
            r_value_box: Box::new(r_value),
            is_decl,
            line,
        }
    }
}

impl NodeElement for AssignmentExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_assignment_expr_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_assignment_expr_node_to_string(self, output);
    }

    // fn auto_inc_dec_expr_type(&self, ast_visitor: &mut dyn AstVisitor) {
    //     ast_visitor.auto_inc_dec_assignment_expr_node(self);
    // }
}

//-----------------------------------------------------//

// pub struct VariableStmtNode {
//     pub var_node: VariableNode,
// }
//
// impl VariableStmtNode {
//     pub fn new(var_node: VariableNode) -> VariableStmtNode {
//         VariableStmtNode { var_node }
//     }
//
//     pub fn get_line(&self) -> usize {
//         self.var_node.id_node.line
//     }
// }
//
// impl NodeElement for VariableStmtNode {
//     fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
//         ast_visitor.visit_variable_stmt_node(self);
//     }
// }


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

pub struct ExprListStmtNode {
    pub expr_list_node: ExprListNode,
}

impl ExprListStmtNode {
    pub fn new(expr_list_node: ExprListNode) -> ExprListStmtNode {
        ExprListStmtNode { expr_list_node }
    }

    // TODO
    // pub fn get_line(&self) -> usize {
    //     self.expr_list_node.id_node.line
    // }
}

impl NodeElement for ExprListStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        let ref ref_expr_type = RefExprType::ExprListT {expr_list_node: &self.expr_list_node };
        ast_visitor.visit_auto_pre_inc_dec_expr_node(ref_expr_type);
        ast_visitor.visit_expr_list_stmt_node(self);
        ast_visitor.visit_auto_post_inc_dec_expr_node(ref_expr_type);

    }
}

//-----------------------------------------------------//

pub struct LoopStmtNode {
    pub loop_types: LoopTypes,
}

impl LoopStmtNode {
    pub fn new(loop_types: LoopTypes) -> LoopStmtNode {
        LoopStmtNode { loop_types }
    }

    // TODO
    // pub fn get_line(&self) -> usize {
    //     self.expr_list_node.id_node.line
    // }
}

impl NodeElement for LoopStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
//        let ref ref_expr_type = RefExprType::LoopExprT {loop_expr_node: &self.loop_expr_node };
        ast_visitor.visit_loop_stmt_node(self);
    }
}

//-----------------------------------------------------//

pub struct TransitionStatementNode {
    pub target_state_context_t: StateContextType,
    pub exit_args_opt: Option<ExprListNode>,
    pub label_opt: Option<String>,
    pub forward_event: bool,
}

// TODO - why is new() commented out?
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


impl fmt::Display for InterfaceMethodCallExprNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.identifier.to_string())
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


impl fmt::Display for ActionCallExprNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.identifier.to_string())
    }
}

//-----------------------------------------------------//

pub enum LoopTypes {
    LoopForExpr { loop_for_expr_node: LoopForExprNode },
    LoopInExpr { loop_in_expr_node: LoopInExprNode },
}

pub struct LoopInExprNode {
    pub target_expr: Box<ExprType>,
    pub iterable_expr: Box<ExprType>,
}

impl LoopInExprNode {
    pub fn new (
        target_expr: Box<ExprType>,
        iterable_expr: Box<ExprType>,
    ) -> LoopInExprNode {
        LoopInExprNode {
            target_expr,
            iterable_expr,
        }
    }
}

impl NodeElement for LoopInExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_loop_in_expr_node(self);
    }
}

//-----------------------------------------------------//

pub struct LoopForExprNode {
    pub loop_init_expr_rcref_opt: Option<Rc<RefCell<ExprType>>>,
    pub test_expr_rcref_opt: Option<Rc<RefCell<ExprType>>>,
    pub inc_dec_expr_rcref_opt: Option<Rc<RefCell<ExprType>>>,
    pub statements: Vec<DeclOrStmtType>,
}

impl LoopForExprNode {
    pub fn new (
        loop_init_expr_opt: Option<ExprType>,
        test_expr_opt: Option<ExprType>,
        inc_dec_expr_opt: Option<ExprType>,
        statements: Vec<DeclOrStmtType>,
    ) -> LoopForExprNode {
        let mut lie_rcref_opt = Option::None;
        if let Some(expr_t) = loop_init_expr_opt {
            lie_rcref_opt = Some(Rc::new(RefCell::new(expr_t)));
        }
        let mut te_rcref_opt = Option::None;
        if let Some(expr_t) = test_expr_opt {
            te_rcref_opt = Some(Rc::new(RefCell::new(expr_t)));
        }
        let mut id_rcref_opt = Option::None;
        if let Some(expr_t) = inc_dec_expr_opt {
            id_rcref_opt = Some(Rc::new(RefCell::new(expr_t)));
        }
        LoopForExprNode {
            loop_init_expr_rcref_opt:lie_rcref_opt,
            test_expr_rcref_opt:te_rcref_opt,
            inc_dec_expr_rcref_opt:id_rcref_opt,
            statements,
        }
    }
}

impl NodeElement for LoopForExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_loop_for_expr_node(self);
    }
}

//-----------------------------------------------------//

// pub struct LoopExprNode {
//     // pub loop_init_expr_opt: Option<ExprType>,
//     // pub test_expr_opt: Option<ExprType>,
//     // pub inc_dec_expr_opt: Option<ExprType>,
//     // pub statements: Vec<DeclOrStmtType>,
//
// }
//
// impl LoopExprNode {
//     pub fn new (
//         loop_init_expr_opt: Option<ExprType>,
//         test_expr_opt: Option<ExprType>,
//         inc_dec_expr_opt: Option<ExprType>,
//         statements: Vec<DeclOrStmtType>,
//     ) -> LoopExprNode {
//         LoopExprNode {
//             loop_init_expr_opt,
//             test_expr_opt,
//             inc_dec_expr_opt,
//             statements,
//         }
//     }
// }
//-----------------------------------------------------//

#[derive(Clone)]
pub enum IncDecExpr {
    None,
    PreInc,
    PreDec,
    PostInc,
    PostDec,
}

//-----------------------------------------------------//

pub struct CallChainLiteralExprNode {
    pub call_chain: VecDeque<CallChainLiteralNodeType>,
    pub is_new_expr: bool,
    pub inc_dec: IncDecExpr,
}

impl CallChainLiteralExprNode {
    pub fn new(call_chain: VecDeque<CallChainLiteralNodeType>) -> CallChainLiteralExprNode {
        CallChainLiteralExprNode {
            call_chain,
            is_new_expr: false,
            inc_dec: IncDecExpr::None,
        }
    }
}

impl NodeElement for CallChainLiteralExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        let ref ref_expr_type = RefExprType::CallChainLiteralExprT {call_chain_expr_node: &self };
        ast_visitor.visit_auto_pre_inc_dec_expr_node( ref_expr_type);
        ast_visitor.visit_call_chain_literal_expr_node(self);
        ast_visitor.visit_auto_post_inc_dec_expr_node(ref_expr_type);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_call_chain_literal_expr_node_to_string(self, output);
    }
}

impl fmt::Display for CallChainLiteralExprNode {
    // This trait requires `fmt` with this exact signature.


    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        let mut separator = "";
        for node in &self.call_chain {
            output.push_str(separator);
            match &node {
                CallChainLiteralNodeType::IdentifierNodeT { id_node } => {
                    output.push_str(&*id_node.to_string());
                }
                CallChainLiteralNodeType::CallT { call } => {
                    output.push_str(&*call.to_string());
                }
                CallChainLiteralNodeType::InterfaceMethodCallT {
                    interface_method_call_expr_node,
                } => {
                    output.push_str(&*interface_method_call_expr_node.to_string());
                }
                CallChainLiteralNodeType::ActionCallT {
                    action_call_expr_node,
                } => {
                    output.push_str(&*action_call_expr_node.to_string());
                }
                CallChainLiteralNodeType::VariableNodeT { var_node } => {
                    output.push_str(&*var_node.to_string());
                }
            }
            separator = ".";
        }
        write!(f,"{}", output)
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
            TokenType::Plus => Plus,
            TokenType::Dash => Minus,
            TokenType::Star => Multiply,
            TokenType::ForwardSlash => Divide,
            TokenType::GT => Greater,
            TokenType::GreaterEqual => GreaterEqual,
            TokenType::LT => OperatorType::Less,
            TokenType::LessEqual => LessEqual,
            TokenType::Bang => OperatorType::Not,
            TokenType::EqualEqual => OperatorType::EqualEqual,
            TokenType::BangEqual => OperatorType::NotEqual,
            TokenType::LogicalAnd => OperatorType::LogicalAnd,
            TokenType::PipePipe => OperatorType::LogicalOr,
            TokenType::LogicalXor => OperatorType::LogicalXor,
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

impl fmt::Display for CallExprNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.identifier)
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
    pub inc_dec: IncDecExpr,
}

impl ExprListNode {
    pub fn new(exprs_t: Vec<ExprType>) -> ExprListNode {
        ExprListNode {
            exprs_t,
            inc_dec: IncDecExpr::None,
        }
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
// there is a misalignment of ParseScope vs IdentiferDeclScope?
// for instance States have a IdentiferDeclScope of None. Should be machine
#[derive(Clone, PartialEq)]
pub enum IdentifierDeclScope {
    //     Global,  TODO!
    System,
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


impl fmt::Display for IdentifierNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.name.lexeme)
    }
}

//-----------------------------------------------------//

#[derive(Clone)]
pub struct LiteralExprNode {
    pub token_t: TokenType,
    pub value: String,
    pub is_reference: bool,
    pub inc_dec: IncDecExpr,
}

impl LiteralExprNode {
    pub fn new(token_t: TokenType, value: String) -> LiteralExprNode {
        LiteralExprNode {
            token_t,
            value,
            is_reference: false,
            inc_dec: IncDecExpr::None,
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

#[derive(Clone, PartialEq)]
pub struct TypeNode {
    #[allow(dead_code)]
    is_superstring: bool,
    pub(crate) is_reference: bool,
    pub(crate) frame_event_part_opt: Option<FrameEventPart>,
    pub(crate) type_str: String,
}

// pub trait FrameEventPartFormatter {
//     fn formatFrameEventPart(frame_event_part:FrameEventPart) -> String;
// }

impl TypeNode {
    pub fn new(
        is_superstring: bool,
        is_reference: bool,
        frame_event_part_opt: Option<FrameEventPart>,
        type_str: String,
    ) -> TypeNode {
        TypeNode {
            is_superstring,
            is_reference,
            frame_event_part_opt,
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
