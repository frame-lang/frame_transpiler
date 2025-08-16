#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case)]

use super::scanner::{Token, TokenType};
use super::symbol_table::{ActionScopeSymbol, EventSymbol, SymbolType};

use crate::frame_c::ast::OperatorType::{
    Divide, Greater, GreaterEqual, LessEqual, Minus, Multiply, Plus,
};
use crate::frame_c::symbol_table::{InterfaceMethodSymbol, OperationScopeSymbol, ParameterSymbol};
use crate::frame_c::visitors::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
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
    fn accept_action_decl(&self, _ast_visitor: &mut dyn AstVisitor) {
        // no_op
    }
    fn accept_action_impl(&self, _ast_visitor: &mut dyn AstVisitor) {
        // no_op
    }
    fn accept_enums(&self, _ast_visitor: &mut dyn AstVisitor) {
        // no_op
    }
}

pub struct Module {
    pub module_elements: Vec<ModuleElement>,
}

impl Module {
    pub fn new(module_elements: Vec<ModuleElement>) -> Module {
        Module { module_elements }
    }
}

impl NodeElement for Module {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_module(self);
    }

    // fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
    //     ast_visitor.visit_system_instance_expr_node_to_string(self, output);
    // }
}

pub enum ModuleElement {
    CodeBlock { code_block: String },
    ModuleAttribute { attribute_node: AttributeNode },
}

// TODO: is this a good name for Identifier and Call expressions?

pub trait CallableExpr {
    fn callable_accept(&self, ast_visitor: &mut dyn AstVisitor);
    fn callable_accept_to_string(&self, _ast_visitor: &mut dyn AstVisitor, _output: &mut String) {
        // no_op
    }
}

// TODO: note - exploring if this enum can replace the Callable : Downcast approach
pub enum CallChainNodeType {
    // Declared identifier types
    VariableNodeT {
        var_node: VariableNode,
    },
    // TODO
    // ParameterNodeT {
    //     param_node: ParameterNode,
    // },
    // FunctionCallNodeT {
    //     var_node: VariableNode,
    // },
    InterfaceMethodCallT {
        interface_method_call_expr_node: InterfaceMethodCallExprNode,
    },
    OperationCallT {
        operation_call_expr_node: OperationCallExprNode,
    },
    OperationRefT {
        operation_ref_expr_node: OperationRefExprNode,
    },
    ActionCallT {
        action_call_expr_node: ActionCallExprNode,
    },
    ListElementNodeT {
        list_elem_node: ListElementNode,
    },
    // Undeclared identifier types
    UndeclaredIdentifierNodeT {
        id_node: IdentifierNode,
    },
    UndeclaredCallT {
        call_node: CallExprNode,
    },
    UndeclaredListElementT {
        list_elem_node: ListElementNode,
    },
}

impl CallChainNodeType {
    pub fn setIsReference(&mut self, is_reference: bool) {
        match self {
            CallChainNodeType::VariableNodeT { var_node } => {
                var_node.id_node.is_reference = is_reference;
            }
            CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
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
    MetaWord { attr: AttributeMetaWord },
    MetaNameValueStr { attr: AttributeMetaNameValueStr },
    MetaListIdents { attr: AttributeMetaListIdents },
}

impl AttributeNode {
    pub fn get_name(&self) -> String {
        match self {
            AttributeNode::MetaWord { attr } => attr.name.clone(),
            AttributeNode::MetaNameValueStr { attr } => attr.name.clone(),
            AttributeNode::MetaListIdents { attr } => attr.name.clone(),
        }
    }
}

// Enum indicating if an attribute applies to the entity
// it is inside of or the next entity it comes before
// in the parse.

#[derive(Clone)]
pub enum AttributeAffinity {
    Inner,
    Outer,
}

// e.g. generate_frame_event
pub struct AttributeMetaWord {
    pub name: String,
    pub affinity: AttributeAffinity,
}

impl AttributeMetaWord {
    pub fn new(name: String, affinity: AttributeAffinity) -> AttributeMetaWord {
        AttributeMetaWord { name, affinity }
    }

    // fn get_affinity(&self) -> AttributeAffinity {
    //     self.affinity.clone()
    // }
}

// e.g. name="foo"
pub struct AttributeMetaNameValueStr {
    pub name: String,
    pub value: String,
    pub affinity: AttributeAffinity,
}

impl AttributeMetaNameValueStr {
    pub fn new(
        name: String,
        value: String,
        affinity: AttributeAffinity,
    ) -> AttributeMetaNameValueStr {
        AttributeMetaNameValueStr {
            name,
            value,
            affinity,
        }
    }

    // fn get_affinity(&self) -> AttributeAffinity {
    //     self.affinity.clone()
    // }
}

// e.g. macro_use(foo, bar)
pub struct AttributeMetaListIdents {
    pub name: String,
    pub idents: Vec<String>,
    pub affinity: AttributeAffinity,
}

impl AttributeMetaListIdents {
    pub fn new(
        name: String,
        idents: Vec<String>,
        affinity: AttributeAffinity,
    ) -> AttributeMetaListIdents {
        AttributeMetaListIdents {
            name,
            idents,
            affinity,
        }
    }

    // fn get_affinity(&self) -> AttributeAffinity {
    //     self.affinity.clone()
    // }
}

//-----------------------------------------------------//

pub struct SystemNode {
    pub name: String,
    pub module: Module,
    // TODO - module attributes need to move to a program "module"
    //    pub module_attributes_opt: Option<HashMap<String, AttributeNode>>,
    pub system_attributes_opt: Option<HashMap<String, AttributeNode>>,
    pub start_state_state_params_opt: Option<Vec<ParameterNode>>,
    pub start_state_enter_params_opt: Option<Vec<ParameterNode>>,
    pub domain_params_opt: Option<Vec<ParameterNode>>,
    pub interface_block_node_opt: Option<InterfaceBlockNode>,
    pub machine_block_node_opt: Option<MachineBlockNode>,
    pub actions_block_node_opt: Option<ActionsBlockNode>,
    pub operations_block_node_opt: Option<OperationsBlockNode>,
    pub domain_block_node_opt: Option<DomainBlockNode>,
    pub line: usize,
    // TODO - move this int a module node
    pub functions_opt: Option<Vec<Rc<RefCell<FunctionNode>>>>,
}

impl SystemNode {
    pub fn new(
        name: String,
        module: Module,
        //        module_attributes_opt: Option<HashMap<String, AttributeNode>>,
        system_attributes_opt: Option<HashMap<String, AttributeNode>>,
        start_state_state_params_opt: Option<Vec<ParameterNode>>,
        start_state_enter_params_opt: Option<Vec<ParameterNode>>,
        domain_params_opt: Option<Vec<ParameterNode>>,
        interface_block_node_opt: Option<InterfaceBlockNode>,
        machine_block_node_opt: Option<MachineBlockNode>,
        actions_block_node_opt: Option<ActionsBlockNode>,
        operations_block_node_opt: Option<OperationsBlockNode>,
        domain_block_node_opt: Option<DomainBlockNode>,
        line: usize,
        functions_node_opt: Option<Vec<Rc<RefCell<FunctionNode>>>>,
    ) -> SystemNode {
        SystemNode {
            name,
            module,
            system_attributes_opt,
            start_state_state_params_opt,
            start_state_enter_params_opt,
            domain_params_opt,
            interface_block_node_opt,
            machine_block_node_opt,
            actions_block_node_opt,
            operations_block_node_opt,
            domain_block_node_opt,
            line,
            functions_opt: functions_node_opt,
        }
    }

    pub fn get_first_state(&self) -> Option<&Rc<RefCell<StateNode>>> {
        match &self.machine_block_node_opt {
            Some(mb) => mb.states.get(0),
            None => None,
        }
    }

    pub fn get_state_node(&self, state_name: &String) -> Option<Rc<RefCell<StateNode>>> {
        match &self.machine_block_node_opt {
            Some(mb) => mb.get_state_node(state_name),
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
    pub return_init_expr_opt: Option<ExprType>,
    pub alias: Option<MessageNode>,
}

impl InterfaceMethodNode {
    pub fn new(
        name: String,
        params: Option<Vec<ParameterNode>>,
        return_type: Option<TypeNode>,
        return_init_expr_opt: Option<ExprType>,
        alias: Option<MessageNode>,
    ) -> InterfaceMethodNode {
        InterfaceMethodNode {
            name,
            params,
            return_type_opt: return_type,
            return_init_expr_opt,
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

pub struct FunctionNode {
    pub name: String,
    pub params: Option<Vec<ParameterNode>>,
    pub is_implemented: bool,
    pub statements: Vec<DeclOrStmtType>,
    pub terminator_expr: TerminatorExpr,
    pub type_opt: Option<TypeNode>,
    pub line: usize,
}

impl FunctionNode {
    pub fn new(
        name: String,
        params: Option<Vec<ParameterNode>>,
        is_implemented: bool,
        statements: Vec<DeclOrStmtType>,
        terminator_node: TerminatorExpr,
        type_opt: Option<TypeNode>,
        line: usize,
    ) -> FunctionNode {
        FunctionNode {
            name,
            params,
            is_implemented,
            statements,
            terminator_expr: terminator_node,
            type_opt,
            line,
        }
    }
}

impl NodeElement for FunctionNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_function_node(self);
    }

    // fn accept_action_decl(&self, ast_visitor: &mut dyn AstVisitor) {
    //     ast_visitor.visit_action_node(self);
    // }
    // fn accept_action_impl(&self, ast_visitor: &mut dyn AstVisitor) {
    //     ast_visitor.visit_action_impl_node(self);
    // }
}

//-----------------------------------------------------//

pub struct ActionNode {
    pub name: String,
    pub params: Option<Vec<ParameterNode>>,
    pub is_implemented: bool,
    pub statements: Vec<DeclOrStmtType>,
    pub terminator_expr: TerminatorExpr,
    pub type_opt: Option<TypeNode>,
    pub code_opt: Option<String>, // TODO - remove
}

impl ActionNode {
    pub fn new(
        name: String,
        params: Option<Vec<ParameterNode>>,
        is_implemented: bool,
        statements: Vec<DeclOrStmtType>,
        terminator_node: TerminatorExpr,
        type_opt: Option<TypeNode>,
        code_opt: Option<String>,
    ) -> ActionNode {
        ActionNode {
            name,
            params,
            is_implemented,
            statements,
            terminator_expr: terminator_node,
            type_opt,
            code_opt,
        }
    }
}

impl NodeElement for ActionNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_action_node(self);
    }
    fn accept_rust_impl(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_action_impl_node(self);
    }
    fn accept_action_decl(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_action_node(self);
    }
    fn accept_action_impl(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_action_impl_node(self);
    }
}

//-----------------------------------------------------//

pub struct OperationNode {
    pub name: String,
    pub params: Option<Vec<ParameterNode>>,
    pub attributes_opt: Option<HashMap<String, AttributeNode>>,
    pub is_implemented: bool,
    pub statements: Vec<DeclOrStmtType>,
    pub terminator_expr: TerminatorExpr,
    pub type_opt: Option<TypeNode>,
    pub code_opt: Option<String>, // TODO - remove
}

impl OperationNode {
    pub fn new(
        name: String,
        params: Option<Vec<ParameterNode>>,
        attributes_opt: Option<HashMap<String, AttributeNode>>,
        is_implemented: bool,
        statements: Vec<DeclOrStmtType>,
        terminator_node: TerminatorExpr,
        type_opt: Option<TypeNode>,
        code_opt: Option<String>,
    ) -> OperationNode {
        OperationNode {
            name,
            params,
            attributes_opt,
            is_implemented,
            statements,
            terminator_expr: terminator_node,
            type_opt,
            code_opt,
        }
    }

    pub fn is_static(&self) -> bool {
        if let Some(attributes_map) = &self.attributes_opt {
            let is_static = attributes_map.get("static");
            is_static.is_some()
        } else {
            false
        }
    }
}

impl NodeElement for OperationNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_operation_node(self);
    }
    // fn accept_rust_impl(&self, ast_visitor: &mut dyn AstVisitor) {
    //     ast_visitor.visit_action_impl_node(self);
    // }
    // fn accept_action_decl(&self, ast_visitor: &mut dyn AstVisitor) {
    //     ast_visitor.visit_action_node(self);
    // }
    // fn accept_action_impl(&self, ast_visitor: &mut dyn AstVisitor) {
    //     ast_visitor.visit_action_impl_node(self);
    // }
}

//-----------------------------------------------------//

pub struct VariableDeclNode {
    pub name: String,
    pub type_opt: Option<TypeNode>,
    pub is_constant: bool,
    initializer_value_rc: Rc<ExprType>,
    pub value_rc: Rc<ExprType>,
    pub identifier_decl_scope: IdentifierDeclScope,
}

impl VariableDeclNode {
    pub fn new(
        name: String,
        type_opt: Option<TypeNode>,
        is_constant: bool,
        initializer_value_rc: Rc<ExprType>,
        value_rc: Rc<ExprType>,
        identifier_decl_scope: IdentifierDeclScope,
    ) -> VariableDeclNode {
        VariableDeclNode {
            name,
            type_opt,
            is_constant,
            initializer_value_rc,
            value_rc,
            identifier_decl_scope,
        }
    }
}

impl VariableDeclNode {
    pub fn get_initializer_value_rc(&self) -> Rc<ExprType> {
        self.initializer_value_rc.clone()
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

pub struct LoopVariableDeclNode {
    pub name: String,
    pub type_opt: Option<TypeNode>,
    pub initializer_expr_t_opt: Option<ExprType>,
    pub identifier_decl_scope: IdentifierDeclScope,
}

impl LoopVariableDeclNode {
    pub fn new(
        name: String,
        type_opt: Option<TypeNode>,
        initializer_expr_t_opt: Option<ExprType>,
        identifier_decl_scope: IdentifierDeclScope,
    ) -> LoopVariableDeclNode {
        LoopVariableDeclNode {
            name,
            type_opt,
            initializer_expr_t_opt,
            identifier_decl_scope,
        }
    }
}

impl NodeElement for LoopVariableDeclNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_loop_variable_decl_node(self);
    }
}

//-----------------------------------------------------//

// TODO: consider call this a SystemVariableNode to differentiate
// from external variable references.

pub struct VariableNode {
    pub id_node: IdentifierNode,
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

    pub fn get_name(&self) -> &str {
        self.id_node.name.lexeme.as_str()
    }

    pub fn get_value(&self) -> Rc<ExprType> {
        let m = &self.symbol_type_rcref_opt.as_ref().unwrap();
        let mut y = m.borrow_mut();
        match y.get_ast_node() {
            Ok(Some(variable_decl_node_rcref)) => {
                variable_decl_node_rcref.borrow().value_rc.clone()
            }
            Ok(None) => {
                // NilExprT is a new ExprType used atm to hack around
                // differences between variables and parameters. Parmenters
                // can't be assigned values atm so this patches that
                // gap until they can be.
                Rc::new(ExprType::NilExprT)
            }
            Err(str) => {
                // TODO review this
                panic!("get_value() found invalid SymbolType::{}", str);
            }
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
        write!(f, "{}", self.id_node.to_string())
    }
}

//-----------------------------------------------------//

pub struct EnumDeclNode {
    pub name: String,
    pub enums: Vec<Rc<EnumeratorDeclNode>>,
}

impl EnumDeclNode {
    pub fn new(identifier: String, enums: Vec<Rc<EnumeratorDeclNode>>) -> EnumDeclNode {
        EnumDeclNode {
            name: identifier,
            enums,
        }
    }
}

impl NodeElement for EnumDeclNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_enum_decl_node(self);
    }
}

pub struct EnumeratorDeclNode {
    pub name: String,
    pub value: i32,
}

impl EnumeratorDeclNode {
    pub fn new(name: String, value: i32) -> EnumeratorDeclNode {
        EnumeratorDeclNode { name, value }
    }
}

impl NodeElement for EnumeratorDeclNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_enumerator_decl_node(self);
    }
}

pub struct EnumeratorExprNode {
    pub enum_type: String,
    pub enumerator: String,
}

impl EnumeratorExprNode {
    pub fn new(enum_type: String, enumerator: String) -> EnumeratorExprNode {
        EnumeratorExprNode {
            enum_type,
            enumerator,
        }
    }
}

impl NodeElement for EnumeratorExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_enumerator_expr_node(self);
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
    pub fn get_state_node(&self, name: &String) -> Option<Rc<RefCell<StateNode>>> {
        for state_node_rcref in &self.states {
            if state_node_rcref.borrow().name == *name {
                return Some(state_node_rcref.clone());
            }
        }

        None
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

pub struct OperationsBlockNode {
    pub operations: Vec<Rc<RefCell<OperationNode>>>,
}

impl OperationsBlockNode {
    pub fn new(operations: Vec<Rc<RefCell<OperationNode>>>) -> OperationsBlockNode {
        OperationsBlockNode { operations }
    }
}

impl NodeElement for OperationsBlockNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_operations_block_node(self);
    }
    // fn accept_rust_trait(&self, ast_visitor: &mut dyn AstVisitor) {
    //     ast_visitor.visit_operations_node_rust_trait(self);
    // }
    // fn accept_rust_impl(&self, ast_visitor: &mut dyn AstVisitor) {
    //     ast_visitor.visit_operations_node_rust_impl(self);
    // }
}

//-----------------------------------------------------//

pub struct DomainBlockNode {
    pub member_variables: Vec<Rc<RefCell<VariableDeclNode>>>,
    pub enums: Vec<Rc<RefCell<EnumDeclNode>>>,
}

impl DomainBlockNode {
    pub fn new(
        member_variables: Vec<Rc<RefCell<VariableDeclNode>>>,
        enums: Vec<Rc<RefCell<EnumDeclNode>>>,
    ) -> DomainBlockNode {
        DomainBlockNode {
            member_variables,
            enums,
        }
    }
}

impl NodeElement for DomainBlockNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_domain_block_node(self);
    }

    fn accept_enums(&self, ast_visitor: &mut dyn AstVisitor) {
        for enum_decl_node_rcref in &self.enums {
            let enum_decl_node = enum_decl_node_rcref.borrow();
            ast_visitor.visit_enum_decl_node(&*enum_decl_node);
        }
    }
}

//-----------------------------------------------------//

pub struct StateNode {
    pub name: String,
    pub params_opt: Option<Vec<ParameterNode>>,
    pub vars_opt: Option<Vec<Rc<RefCell<VariableDeclNode>>>>,
    pub calls_opt: Option<Vec<CallChainExprNode>>,
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
        calls: Option<Vec<CallChainExprNode>>,
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

impl StateNode {
    pub fn get_enter_param_count(&self) -> usize {
        match &self.enter_event_handler_opt {
            Some(event_handler_node_rcref) => {
                let event_handler_node = event_handler_node_rcref.borrow();
                let size = event_handler_node
                    .event_symbol_rcref
                    .borrow()
                    .get_param_count();
                size
            }
            None => 0,
        }
    }
    pub fn get_exit_param_count(&self) -> usize {
        match &self.exit_event_handler_opt {
            Some(event_handler_node_rcref) => {
                let event_handler_node = event_handler_node_rcref.borrow();
                let size = event_handler_node
                    .event_symbol_rcref
                    .borrow()
                    .get_param_count();
                size
            }
            None => 0,
        }
    }
}
impl NodeElement for StateNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_state_node(self);
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
    None,
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
    DispatchToParentState,
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
    OperationCallExprT {
        operation_call_expr_node: OperationCallExprNode,
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
    CallChainExprT {
        call_chain_expr_node: CallChainExprNode,
    },
    #[allow(dead_code)] // is used, don't know why I need this
    CallExprT {
        call_expr_node: CallExprNode,
    },
    #[allow(dead_code)] // is used, don't know why I need this
    CallExprListT {
        call_expr_list_node: CallExprListNode,
    },
    ListT {
        list_node: ListNode,
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
    // Expression for default literal for type
    DefaultLiteralValueForTypeExprT,
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
    EnumeratorExprT {
        enum_expr_node: EnumeratorExprNode,
    },
    TransitionExprT {
        transition_expr_node: TransitionExprNode,
    },
    SystemInstanceExprT {
        system_instance_expr_node: SystemInstanceExprNode,
    },
    SystemTypeExprT {
        system_type_expr_node: SystemTypeExprNode,
    },
    SelfExprT {
        self_expr_node: SelfExprNode,
    },
    // TODO:
    // NilExprT is a new ExprType used atm to hack around
    // differences between variables and parameters. Parameters
    // can't be assigned values atm so this patches that
    // gap until they can be.
    NilExprT,
}

impl fmt::Display for ExprType {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExprType::CallChainExprT {
                call_chain_expr_node,
            } => {
                write!(f, "{}", call_chain_expr_node.to_string())
            }
            _ => {
                write!(f, "TODO")
            }
        }
    }
}

impl ExprType {
    pub fn is_valid_binary_expr_type(&self) -> bool {
        match self {
            ExprType::AssignmentExprT { .. } => false,
            ExprType::TransitionExprT { .. } => false,
            ExprType::StateStackOperationExprT { .. } => false,
            ExprType::CallExprListT { .. } => false, // this shouldn't happen
            _ => true,
        }
    }
    pub fn is_valid_assignment_rvalue_expr_type(&self) -> bool {
        match self {
            ExprType::AssignmentExprT { .. } => false,
            ExprType::TransitionExprT { .. } => false,
            ExprType::StateStackOperationExprT { .. } => false,
            ExprType::CallExprListT { .. } => false, // this shouldn't happen
            _ => true,
        }
    }

    pub fn get_name(&self) -> Option<String> {
        match self {
            ExprType::VariableExprT { var_node } => {
                let name = var_node.id_node.name.lexeme.clone();
                Some(name)
            }
            ExprType::CallChainExprT {
                call_chain_expr_node,
            } => {
                let call_chain_node_type_opt = call_chain_expr_node.call_chain.get(0);
                match call_chain_node_type_opt {
                    Some(call_chain_node_type) => match call_chain_node_type {
                        CallChainNodeType::VariableNodeT { var_node } => {
                            let name = var_node.id_node.name.lexeme.clone();
                            Some(name)
                        }
                        _ => None,
                    },
                    None => None,
                }
            }
            _ => None,
        }
    }

    /// Get the name of expression type we're looking at. Useful for debugging.
    pub fn expr_type_name(&self) -> &'static str {
        match self {
            ExprType::AssignmentExprT { .. } => "AssignmentExprT",
            ExprType::ActionCallExprT { .. } => "ActionCallExprT",
            ExprType::CallChainExprT { .. } => "CallChainExprT",
            ExprType::CallExprT { .. } => "CallExprT",
            ExprType::CallExprListT { .. } => "CallExprListT",
            ExprType::ListT { .. } => "ListT",
            ExprType::ExprListT { .. } => "ExprListT",
            ExprType::VariableExprT { .. } => "VariableExprT",
            ExprType::LiteralExprT { .. } => "LiteralExprT",
            ExprType::StateStackOperationExprT { .. } => "StateStackOperationExprT",
            ExprType::FrameEventExprT { .. } => "FrameEventExprT",
            ExprType::UnaryExprT { .. } => "UnaryExprT",
            ExprType::BinaryExprT { .. } => "BinaryExprT",
            ExprType::EnumeratorExprT { .. } => "EnumExprT",
            ExprType::SystemInstanceExprT { .. } => "SystemInstanceExprT",
            ExprType::SystemTypeExprT { .. } => "SystemTypeExprT",
            ExprType::DefaultLiteralValueForTypeExprT { .. } => "DefaultLiteralValueForTypeExprT",
            ExprType::TransitionExprT { .. } => "TransitionExprT",
            ExprType::NilExprT { .. } => "NilExprT",
            ExprType::SelfExprT { .. } => "SelfExprT",
        }
    }

    pub fn debug_print(&self) {
        match self {
            ExprType::VariableExprT { var_node } => {
                let name = var_node.id_node.name.lexeme.clone();
                println!("VariableNode: {}", name);
            }
            ExprType::CallChainExprT {
                call_chain_expr_node,
            } => {
                let mut separator = "";
                for call_chain_node_type in &call_chain_expr_node.call_chain {
                    match call_chain_node_type {
                        CallChainNodeType::VariableNodeT { var_node } => {
                            let name = var_node.id_node.name.lexeme.clone();
                            print!("{}{}", name, separator);
                        }
                        CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                            let name = id_node.name.lexeme.clone();
                            print!("{}{}", name, separator);
                        }
                        _ => {
                            print!("Unknown ExprType");
                        }
                    }
                    separator = ".";
                }
            }
            _ => {}
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
            ExprType::CallChainExprT {
                call_chain_expr_node,
            } => {
                ast_visitor.visit_call_chain_expr_node(call_chain_expr_node);
            }
            ExprType::SystemInstanceExprT {
                system_instance_expr_node,
            } => {
                ast_visitor.visit_system_instance_expr_node(system_instance_expr_node);
            }
            ExprType::SystemTypeExprT {
                system_type_expr_node,
            } => {
                ast_visitor.visit_system_type_expr_node(system_type_expr_node);
            }
            ExprType::CallExprT { call_expr_node } => {
                ast_visitor.visit_call_expression_node(call_expr_node);
            }
            ExprType::CallExprListT {
                call_expr_list_node,
            } => {
                ast_visitor.visit_call_expr_list_node(call_expr_list_node);
            }
            ExprType::ListT { list_node } => {
                ast_visitor.visit_list_node(list_node);
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
            ExprType::EnumeratorExprT { enum_expr_node } => {
                ast_visitor.visit_enumerator_expr_node(enum_expr_node);
            }
            ExprType::SelfExprT { self_expr_node } => {
                ast_visitor.visit_self_expr_node(self_expr_node);
            }
            ExprType::TransitionExprT {
                transition_expr_node,
            } => {
                ast_visitor.visit_transition_expr_node(transition_expr_node);
            }
            ExprType::NilExprT => {
                panic!("Unexpect use of ExprType::NilExprT");
            }
            ExprType::DefaultLiteralValueForTypeExprT => {
                panic!("Unexpect use of ExprType::DefaultLiteralValueForTypeExprT");
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
            ExprType::CallChainExprT {
                call_chain_expr_node,
            } => {
                ast_visitor.visit_call_chain_expr_node_to_string(call_chain_expr_node, output);
            }
            ExprType::SystemInstanceExprT {
                system_instance_expr_node,
            } => {
                ast_visitor
                    .visit_system_instance_expr_node_to_string(system_instance_expr_node, output);
            }
            ExprType::SystemTypeExprT {
                system_type_expr_node,
            } => {
                ast_visitor.visit_system_type_expr_node_to_string(system_type_expr_node, output);
            }
            ExprType::CallExprT { call_expr_node } => {
                ast_visitor.visit_call_expression_node_to_string(call_expr_node, output);
            }
            ExprType::CallExprListT {
                call_expr_list_node,
            } => {
                ast_visitor.visit_call_expr_list_node_to_string(call_expr_list_node, output);
            }
            ExprType::ListT { list_node } => {
                ast_visitor.visit_list_node_to_string(list_node, output);
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
            ExprType::EnumeratorExprT { enum_expr_node } => {
                ast_visitor.visit_enumerator_expr_node_to_string(enum_expr_node, output);
            }
            ExprType::TransitionExprT {
                transition_expr_node,
            } => {
                ast_visitor.visit_transition_expr_node_to_string(transition_expr_node, output);
            }
            ExprType::SelfExprT { self_expr_node } => {
                ast_visitor.visit_self_expr_node_to_string(self_expr_node,output);
            }
            ExprType::NilExprT => {
                panic!("Unexpect use of ExprType::NilExprT");
            }
            ExprType::DefaultLiteralValueForTypeExprT => {
                panic!("Unexpect use of ExprType::DefaultLiteralValueForTypeExprT");
            }
        }
    }
}

// TODO - need to create new types for permitted expressions
//        inside functions as opposed to systems. This is a start.

//-----------------------------------------------------//
//                  -Function Arg Expressions-

// TODO v0.12 - use this for restricting the expressions allowed in functions.
//
// pub enum FunctionArgExprType {
//     CallChainLiteralExprT {
//         call_chain_expr_node: CallChainExprNode,
//     },
//     #[allow(dead_code)] // is used, don't know why I need this
//     CallExprT {
//         call_expr_node: CallExprNode,
//     },
//     VariableExprT {
//         var_node: VariableNode,
//     },
//     LiteralExprT {
//         literal_expr_node: LiteralExprNode,
//     },
//
//     // TODO
//     // - FunctionBinaryExprNode
//     // - FunctionAssignment
// }

// pub enum RefExprType<'a> {
//     AssignmentExprT {
//         assignment_expr_node: &'a AssignmentExprNode,
//     },
//     // #[allow(dead_code)] // is used, don't know why I need this
//     // ActionCallExprT {
//     //     action_call_expr_node: ActionCallExprNode,
//     // },
//     CallChainLiteralExprT {
//         call_chain_expr_node: &'a CallChainExprNode,
//     },
//     // #[allow(dead_code)] // is used, don't know why I need this
//     CallExprT {
//         call_expr_node: &'a CallExprNode,
//     },
//     // #[allow(dead_code)] // is used, don't know why I need this
//     // CallExprListT {
//     //     call_expr_list_node: CallExprListNode,
//     // },
//     ExprListT {
//         expr_list_node: &'a ExprListNode,
//     },
//     // VariableExprT {
//     //     var_node: VariableNode,
//     // },
//     // LiteralExprT {
//     //     literal_expr_node: LiteralExprNode,
//     // },
//     // StateStackOperationExprT {
//     //     state_stack_op_node: StateStackOperationNode,
//     // },
//     // FrameEventExprT {
//     //     frame_event_part: FrameEventPart,
//     // },
//     // UnaryExprT {
//     //     unary_expr_node: UnaryExprNode,
//     // },
//     BinaryExprT {
//         binary_expr_node: &'a BinaryExprNode,
//     },
//     LoopStmtT {
//         loop_types: &'a LoopStmtTypes,
//     },
// }

//-----------------------------------------------------//
//                  -Statements-

pub enum ExprStmtType {
    SystemInstanceStmtT {
        system_instance_stmt_node: SystemInstanceStmtNode,
    },
    SystemTypeStmtT {
        system_type_stmt_node: SystemTypeStmtNode,
    },
    CallStmtT {
        call_stmt_node: CallStmtNode,
    },
    ActionCallStmtT {
        action_call_stmt_node: ActionCallStmtNode,
    },
    CallChainStmtT {
        call_chain_literal_stmt_node: CallChainStmtNode,
    },
    AssignmentStmtT {
        assignment_stmt_node: AssignmentStmtNode,
    },
    VariableStmtT {
        variable_stmt_node: VariableStmtNode,
    },
    ListStmtT {
        list_stmt_node: ListStmtNode,
    },
    ExprListStmtT {
        expr_list_stmt_node: ExprListStmtNode,
    },
    EnumeratorStmtT {
        enumerator_stmt_node: EnumeratorStmtNode,
    },
    BinaryStmtT {
        binary_stmt_node: BinaryStmtNode,
    },
    TransitionStmtT {
        transition_statement_node: TransitionStatementNode,
    },
    // SuperStringStmtT {
    //     super_string_stmt_node: SuperStringStmtNode,
    // }
}

#[allow(clippy::large_enum_variant)]
pub enum StatementType {
    ExpressionStmt {
        expr_stmt_t: ExprStmtType,
    },
    TransitionStmt {
        transition_statement_node: TransitionStatementNode,
    },
    // ChangeStateStmt {
    //     change_state_stmt_node: ChangeStateStatementNode,
    // },
    TestStmt {
        test_stmt_node: TestStatementNode,
    },
    StateStackStmt {
        state_stack_operation_statement_node: StateStackOperationStatementNode,
    },
    IfStmt {
        if_stmt_node: IfStmtNode,
    },
    ForStmt {
        for_stmt_node: ForStmtNode,
    },
    WhileStmt {
        while_stmt_node: WhileStmtNode,
    },
    LoopStmt {
        loop_stmt_node: LoopStmtNode,
    },
    ContinueStmt {
        continue_stmt_node: ContinueStmtNode,
    },
    BreakStmt {
        break_stmt_node: BreakStmtNode,
    },
    SuperStringStmt {
        super_string_stmt_node: SuperStringStmtNode,
    },
    BlockStmt {
        block_stmt_node: BlockStmtNode,
    },
    ReturnAssignStmt {
        return_assign_stmt_node: ReturnAssignStmtNode,
    },
    ReturnStmt {
        return_stmt_node: ReturnStmtNode,
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
        var_decl_t_rcref: Rc<RefCell<VariableDeclNode>>,
    },
    StmtT {
        stmt_t: StatementType,
    },
}

//-----------------------------------------------------//

pub struct SystemInstanceStmtNode {
    pub system_instance_expr_node: SystemInstanceExprNode,
}

impl SystemInstanceStmtNode {
    pub fn new(system_instance_expr_node: SystemInstanceExprNode) -> SystemInstanceStmtNode {
        SystemInstanceStmtNode {
            system_instance_expr_node,
        }
    }
}

impl NodeElement for SystemInstanceStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_system_instance_statement_node(self);
    }
}

//-----------------------------------------------------//

pub struct SystemTypeStmtNode {
    pub system_type_expr_node: SystemTypeExprNode,
}

impl SystemTypeStmtNode {
    pub fn new(system_type_expr_node: SystemTypeExprNode) -> SystemTypeStmtNode {
        SystemTypeStmtNode {
            system_type_expr_node,
        }
    }
}

impl NodeElement for SystemTypeStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_system_type_statement_node(self);
    }
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

pub struct EnumeratorStmtNode {
    pub enumerator_expr_node: EnumeratorExprNode,
}

impl EnumeratorStmtNode {
    pub fn new(enumerator_expr_node: EnumeratorExprNode) -> EnumeratorStmtNode {
        EnumeratorStmtNode {
            enumerator_expr_node,
        }
    }
}

impl NodeElement for EnumeratorStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_enumerator_statement_node(self);
    }
}

//-----------------------------------------------------//

pub struct CallChainStmtNode {
    pub call_chain_literal_expr_node: CallChainExprNode,
}

impl CallChainStmtNode {
    pub fn new(call_chain_literal_expr_node: CallChainExprNode) -> CallChainStmtNode {
        CallChainStmtNode {
            call_chain_literal_expr_node,
        }
    }
}

impl NodeElement for CallChainStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_call_chain_statement_node(self);
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
    pub r_value_rc: Rc<ExprType>,
    //    pub is_decl: bool,
    pub line: usize,
}

impl AssignmentExprNode {
    pub fn new(l_value: ExprType, r_value: Rc<ExprType>, line: usize) -> AssignmentExprNode {
        AssignmentExprNode {
            l_value_box: Box::new(l_value),
            r_value_rc: r_value.clone(),
            //            is_decl,
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

pub struct ListStmtNode {
    pub list_node: ListNode,
}

impl ListStmtNode {
    pub fn new(list_node: ListNode) -> ListStmtNode {
        ListStmtNode { list_node }
    }

    // TODO
    // pub fn get_line(&self) -> usize {
    //     self.expr_list_node.id_node.line
    // }
}

impl NodeElement for ListStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_list_stmt_node(self);
    }
}
//-----------------------------------------------------//

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
        ast_visitor.visit_expr_list_stmt_node(self);
    }
}

//-----------------------------------------------------//

pub struct BinaryStmtNode {
    pub binary_expr_node: BinaryExprNode,
}

impl BinaryStmtNode {
    pub fn new(binary_expr_node: BinaryExprNode) -> BinaryStmtNode {
        BinaryStmtNode { binary_expr_node }
    }

    // TODO
    // pub fn get_line(&self) -> usize {
    //     self.expr_list_node.id_node.line
    // }
}

impl NodeElement for BinaryStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_binary_stmt_node(self);
    }
}


//-----------------------------------------------------//

pub struct IfStmtNode {
    pub condition: ExprType,
    pub if_block: BlockStmtNode,
    pub elif_clauses: Vec<ElifClause>,
    pub else_block: Option<BlockStmtNode>,
}

pub struct ElifClause {
    pub condition: ExprType,
    pub block: BlockStmtNode,
}

impl IfStmtNode {
    pub fn new(
        condition: ExprType,
        if_block: BlockStmtNode,
        elif_clauses: Vec<ElifClause>,
        else_block: Option<BlockStmtNode>,
    ) -> IfStmtNode {
        IfStmtNode {
            condition,
            if_block,
            elif_clauses,
            else_block,
        }
    }
}

impl NodeElement for IfStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_if_stmt_node(self);
    }
}

//-----------------------------------------------------//

pub struct ForStmtNode {
    pub variable: Option<VariableNode>, // for var x in items
    pub identifier: Option<IdentifierNode>, // for x in items
    pub iterable: ExprType,
    pub block: BlockStmtNode,
}

impl ForStmtNode {
    pub fn new(
        variable: Option<VariableNode>,
        identifier: Option<IdentifierNode>,
        iterable: ExprType,
        block: BlockStmtNode,
    ) -> ForStmtNode {
        ForStmtNode {
            variable,
            identifier,
            iterable,
            block,
        }
    }
}

impl NodeElement for ForStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_for_stmt_node(self);
    }
}

//-----------------------------------------------------//

pub struct WhileStmtNode {
    pub condition: ExprType,
    pub block: BlockStmtNode,
}

impl WhileStmtNode {
    pub fn new(condition: ExprType, block: BlockStmtNode) -> WhileStmtNode {
        WhileStmtNode { condition, block }
    }
}

impl NodeElement for WhileStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_while_stmt_node(self);
    }
}

//-----------------------------------------------------//

pub struct LoopStmtNode {
    pub loop_types: LoopStmtTypes,
}

impl LoopStmtNode {
    pub fn new(loop_types: LoopStmtTypes) -> LoopStmtNode {
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

pub enum TargetStateContextType {
    StateRef {
        state_context_node: TargetStateContextNode,
    },
    StateStackPop {},
    // MethodCall { state_context_node:StateContextNode }, // TODO
}

pub struct TargetStateContextNode {
    pub state_ref_node: StateRefNode,
    pub state_ref_args_opt: Option<ExprListNode>,
    pub enter_args_opt: Option<ExprListNode>,
}

impl TargetStateContextNode {
    pub fn new(
        state_ref_node: StateRefNode,
        state_ref_args_opt: Option<ExprListNode>,
        enter_args_opt: Option<ExprListNode>,
    ) -> TargetStateContextNode {
        TargetStateContextNode {
            state_ref_node,
            state_ref_args_opt,
            enter_args_opt,
        }
    }
}

impl NodeElement for TargetStateContextNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_state_context_node(self);
    }
}

//-----------------------------------------------------//

pub struct TransitionExprNode {
    pub target_state_context_t: TargetStateContextType,
    pub label_opt: Option<String>,
    pub forward_event: bool,
}

impl TransitionExprNode {
    pub fn new(
        target_state_context_t: TargetStateContextType,
        label_opt: Option<String>,
        forward_event: bool,
    ) -> TransitionExprNode {
        TransitionExprNode {
            target_state_context_t,
            label_opt,
            forward_event,
        }
    }
}

impl NodeElement for TransitionExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_transition_expr_node(self);
    }
}

//-----------------------------------------------------//

pub struct TransitionStatementNode {
    pub transition_expr_node: TransitionExprNode,
    pub exit_args_opt: Option<ExprListNode>,
}

// TODO - why is new() commented out?
impl TransitionStatementNode {
    pub fn new(
        transition_expr_node: TransitionExprNode,
        exit_args_opt: Option<ExprListNode>,
    ) -> TransitionStatementNode {
        TransitionStatementNode {
            transition_expr_node,
            exit_args_opt,
        }
    }
}

impl NodeElement for TransitionStatementNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_transition_statement_node(self);
    }
}

//-----------------------------------------------------//
//
// pub struct ChangeStateStatementNode {
//     pub state_context_t: TargetStateContextType,
//     pub label_opt: Option<String>,
// }
//
// impl ChangeStateStatementNode {}
//
// impl NodeElement for ChangeStateStatementNode {
//     fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
//         ast_visitor.visit_change_state_statement_node(self);
//     }
// }

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

#[derive(PartialEq)]
pub enum CallOrigin {
    External,
    Internal,
}

pub struct InterfaceMethodCallExprNode {
    pub identifier: IdentifierNode,
    pub call_expr_list: CallExprListNode,
    pub call_origin: CallOrigin,
    pub interface_symbol_rcref_opt: Option<Rc<RefCell<InterfaceMethodSymbol>>>,
}

impl InterfaceMethodCallExprNode {
    // Harvest the id and arguments from the CallExpressionNode.
    // It will be discarded.

    pub fn new(
        call_expr_node: CallExprNode,
        interface_method_call_t: CallOrigin,
    ) -> InterfaceMethodCallExprNode {
        InterfaceMethodCallExprNode {
            identifier: call_expr_node.identifier,
            call_expr_list: call_expr_node.call_expr_list,
            interface_symbol_rcref_opt: None,
            call_origin: interface_method_call_t,
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
        write!(f, "{}", self.identifier.to_string())
    }
}

//-----------------------------------------------------//

pub struct ActionCallExprNode {
    pub identifier: IdentifierNode,
    pub call_expr_list: CallExprListNode,
    pub action_symbol_rcref_opt: Option<Rc<RefCell<ActionScopeSymbol>>>,
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

    pub fn set_action_symbol(&mut self, action_symbol: &Rc<RefCell<ActionScopeSymbol>>) {
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
        write!(f, "{}", self.identifier.to_string())
    }
}

//-----------------------------------------------------//

pub struct OperationCallExprNode {
    pub identifier: IdentifierNode,
    pub call_expr_list: CallExprListNode,
    pub operation_symbol_rcref_opt: Option<Rc<RefCell<OperationScopeSymbol>>>,
}

impl OperationCallExprNode {
    // Harvest the id and arguments from the CallExpressionNode.
    // It will be discarded.

    pub fn new(call_expr_node: CallExprNode) -> OperationCallExprNode {
        OperationCallExprNode {
            identifier: call_expr_node.identifier,
            call_expr_list: call_expr_node.call_expr_list,
            operation_symbol_rcref_opt: None,
        }
    }

    pub fn set_operation_symbol(&mut self, operation_symbol: &Rc<RefCell<OperationScopeSymbol>>) {
        self.operation_symbol_rcref_opt = Some(Rc::clone(operation_symbol));
    }
}

impl NodeElement for OperationCallExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_operation_call_expression_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_operation_call_expression_node_to_string(self, output);
    }
}

impl fmt::Display for OperationCallExprNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.identifier.to_string())
    }
}

//-----------------------------------------------------//

pub struct OperationRefExprNode {
    pub name: String,
}

impl OperationRefExprNode {
    pub fn new(name: String) -> OperationRefExprNode {
        OperationRefExprNode { name }
    }
}

impl NodeElement for OperationRefExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_operation_ref_expression_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_operation_ref_expression_node_to_string(self, output);
    }
}

impl fmt::Display for OperationRefExprNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name.to_string())
    }
}

//-----------------------------------------------------//

pub enum LoopStmtTypes {
    LoopInfiniteStmt {
        loop_infinite_stmt_node: LoopInfiniteStmtNode,
    },
    LoopForStmt {
        loop_for_stmt_node: LoopForStmtNode,
    },
    LoopInStmt {
        loop_in_stmt_node: LoopInStmtNode,
    },
}

//-----------------------------------------------------//

pub struct LoopInfiniteStmtNode {
    pub statements: Vec<DeclOrStmtType>,
}

impl LoopInfiniteStmtNode {
    pub fn new(statements: Vec<DeclOrStmtType>) -> LoopInfiniteStmtNode {
        LoopInfiniteStmtNode { statements }
    }
}

impl NodeElement for LoopInfiniteStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_loop_infinite_stmt_node(self);
    }
}

//-----------------------------------------------------//

pub struct LoopInStmtNode {
    pub loop_first_stmt: LoopFirstStmt,
    pub iterable_expr: Box<ExprType>,
    pub statements: Vec<DeclOrStmtType>,
}

impl LoopInStmtNode {
    pub fn new(
        loop_first_stmt: LoopFirstStmt,
        iterable_expr: Box<ExprType>,
        statements: Vec<DeclOrStmtType>,
    ) -> LoopInStmtNode {
        LoopInStmtNode {
            loop_first_stmt,
            iterable_expr,
            statements,
        }
    }
}

impl NodeElement for LoopInStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_loop_in_stmt_node(self);
    }
}

//-----------------------------------------------------//

pub enum LoopFirstStmt {
    // x
    Var {
        var_node: VariableNode,
    },
    // x
    CallChain {
        call_chain_expr_node: CallChainExprNode,
    },
    // x = 0
    VarAssign {
        assign_expr_node: AssignmentExprNode,
    },
    // The semantics of it being a decl are in the enum type name.
    // var x
    VarDecl {
        var_decl_node_rcref: Rc<RefCell<VariableDeclNode>>,
    },
    // var x:int = 0
    // var x = 0
    VarDeclAssign {
        var_decl_node_rcref: Rc<RefCell<VariableDeclNode>>,
    },

    None,
}

impl NodeElement for LoopFirstStmt {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        match self {
            LoopFirstStmt::Var { var_node } => {
                ast_visitor.visit_variable_expr_node(var_node);
            }
            LoopFirstStmt::CallChain {
                call_chain_expr_node,
            } => {
                ast_visitor.visit_call_chain_expr_node(call_chain_expr_node);
            }
            LoopFirstStmt::VarAssign { assign_expr_node } => {
                ast_visitor.visit_assignment_expr_node(assign_expr_node);
            }
            LoopFirstStmt::VarDecl {
                var_decl_node_rcref,
            } => ast_visitor.visit_variable_decl_node(&*var_decl_node_rcref.borrow()),
            LoopFirstStmt::VarDeclAssign {
                var_decl_node_rcref,
            } => ast_visitor.visit_variable_decl_node(&*var_decl_node_rcref.borrow()),

            LoopFirstStmt::None => {}
        }
        // ast_visitor.visit_loop_for_stmt_node(self);
    }
}

//-----------------------------------------------------//

pub struct LoopForStmtNode {
    pub loop_init_expr_rcref_opt: Option<Rc<RefCell<LoopFirstStmt>>>,
    pub test_expr_rcref_opt: Option<Rc<RefCell<ExprType>>>,
    pub post_expr_rcref_opt: Option<Rc<RefCell<ExprType>>>,
    pub statements: Vec<DeclOrStmtType>,
}

impl LoopForStmtNode {
    pub fn new(
        loop_init_expr_opt: Option<LoopFirstStmt>,
        test_expr_opt: Option<ExprType>,
        inc_dec_expr_opt: Option<ExprType>,
        statements: Vec<DeclOrStmtType>,
    ) -> LoopForStmtNode {
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
        LoopForStmtNode {
            loop_init_expr_rcref_opt: lie_rcref_opt,
            test_expr_rcref_opt: te_rcref_opt,
            post_expr_rcref_opt: id_rcref_opt,
            statements,
        }
    }
}

impl NodeElement for LoopForStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_loop_for_stmt_node(self);
    }
}

//-----------------------------------------------------//

pub struct BlockStmtNode {
    pub statements: Vec<DeclOrStmtType>,
}

impl BlockStmtNode {
    pub fn new(statements: Vec<DeclOrStmtType>) -> BlockStmtNode {
        BlockStmtNode { statements }
    }
}

impl NodeElement for BlockStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_block_stmt_node(self);
    }
}

//-----------------------------------------------------//

pub struct ContinueStmtNode {}

impl ContinueStmtNode {
    pub fn new() -> ContinueStmtNode {
        ContinueStmtNode {}
    }
}

impl NodeElement for ContinueStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_continue_stmt_node(self);
    }
}

//-----------------------------------------------------//

pub struct BreakStmtNode {}

impl BreakStmtNode {
    pub fn new() -> BreakStmtNode {
        BreakStmtNode {}
    }
}

impl NodeElement for BreakStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_break_stmt_node(self);
    }
}

//-----------------------------------------------------//

pub struct SuperStringStmtNode {
    pub literal_expr_node: LiteralExprNode,
}

impl SuperStringStmtNode {
    pub fn new(literal_expr_node: LiteralExprNode) -> SuperStringStmtNode {
        SuperStringStmtNode { literal_expr_node }
    }
}

impl NodeElement for SuperStringStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_superstring_stmt_node(self);
    }
}

//-----------------------------------------------------//

#[derive(Clone, PartialEq)]
pub enum IncDecExpr {
    None,
    PreInc,
    PreDec,
    PostInc,
    PostDec,
}

//-----------------------------------------------------//

// TODO - I think this should be renmed to CallChainExprNode.
// No idea why I thought this was a literal.

pub struct CallChainExprNode {
    pub call_chain: VecDeque<CallChainNodeType>,
    pub is_new_expr: bool,
    pub inc_dec: IncDecExpr,
}

impl CallChainExprNode {
    pub fn new(call_chain: VecDeque<CallChainNodeType>) -> CallChainExprNode {
        CallChainExprNode {
            call_chain,
            is_new_expr: false,
            inc_dec: IncDecExpr::None,
        }
    }
}

// impl CallChainExprNode {
//     fn get_name(&self) {
//         match self {
//
//         }
//     }
// }

impl NodeElement for CallChainExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        // let ref ref_expr_type = RefExprType::CallChainLiteralExprT {
        //     call_chain_expr_node: &self,
        // };

        //    ast_visitor.visit_auto_pre_inc_dec_expr_node(ref_expr_type);

        // search for pre autoupdated  parameters
        // for a in &self.call_chain {
        //     if let CallChainLiteralNodeType::CallT { call } = a {
        //         for b in &call.call_expr_list.exprs_t {
        //             match &b {
        //                 ExprType::CallChainLiteralExprT {
        //                     call_chain_expr_node,
        //                 } => {
        //                     let ref ref_expr_type = RefExprType::CallChainLiteralExprT {
        //                         call_chain_expr_node,
        //                     };
        //                     ast_visitor.visit_auto_pre_inc_dec_expr_node(ref_expr_type);
        //                 }
        //                 ExprType::BinaryExprT { binary_expr_node } => {
        //                     let ref ref_expr_type = RefExprType::BinaryExprT { binary_expr_node };
        //                     ast_visitor.visit_auto_pre_inc_dec_expr_node(ref_expr_type);
        //                 }
        //                 _ => {}
        //             }
        //         }
        //     }
        // }

        ast_visitor.visit_call_chain_expr_node(self);

        //     ast_visitor.visit_auto_post_inc_dec_expr_node(ref_expr_type);

        // search for post autoupdated  parameters
        // for a in &self.call_chain {
        //     if let CallChainLiteralNodeType::CallT { call } = a {
        //         for b in &call.call_expr_list.exprs_t {
        //             match &b {
        //                 ExprType::CallChainLiteralExprT {
        //                     call_chain_expr_node,
        //                 } => {
        //                     let ref ref_expr_type = RefExprType::CallChainLiteralExprT {
        //                         call_chain_expr_node,
        //                     };
        //                     ast_visitor.visit_auto_post_inc_dec_expr_node(ref_expr_type);
        //                 }
        //                 ExprType::BinaryExprT { binary_expr_node } => {
        //                     let ref ref_expr_type = RefExprType::BinaryExprT { binary_expr_node };
        //                     ast_visitor.visit_auto_post_inc_dec_expr_node(ref_expr_type);
        //                 }
        //                 _ => {}
        //             }
        //         }
        //     }
        // }
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_call_chain_expr_node_to_string(self, output);
    }
}

impl fmt::Display for CallChainExprNode {
    // This trait requires `fmt` with this exact signature.

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        let mut separator = "";
        for node in &self.call_chain {
            output.push_str(separator);
            match &node {
                CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                    output.push_str(&*id_node.to_string());
                }
                CallChainNodeType::UndeclaredCallT { call_node: call } => {
                    output.push_str(&*call.to_string());
                }
                CallChainNodeType::InterfaceMethodCallT {
                    interface_method_call_expr_node,
                } => {
                    output.push_str(&*interface_method_call_expr_node.to_string());
                }
                CallChainNodeType::OperationCallT {
                    operation_call_expr_node,
                } => {
                    output.push_str(&*operation_call_expr_node.to_string());
                }
                CallChainNodeType::OperationRefT {
                    operation_ref_expr_node,
                } => {
                    output.push_str(&*operation_ref_expr_node.to_string());
                }
                CallChainNodeType::ActionCallT {
                    action_call_expr_node,
                } => {
                    output.push_str(&*action_call_expr_node.to_string());
                }
                CallChainNodeType::VariableNodeT { var_node } => {
                    output.push_str(&*var_node.to_string());
                }
                CallChainNodeType::ListElementNodeT { .. } => {
                    // output.push_str(&*var_node.to_string());
                }
                CallChainNodeType::UndeclaredListElementT { .. } => {
                    // output.push_str(&*var_node.to_string());
                }
            }
            separator = ".";
        }
        write!(f, "{}", output)
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
    Percent,
    Unknown,
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
            TokenType::Percent => OperatorType::Percent,
            _ => OperatorType::Unknown,
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

impl CallExprNode {
    pub fn get_name(&self) -> &str {
        self.identifier.name.lexeme.as_str()
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
    fn callable_accept(&self, ast_visitor: &mut dyn AstVisitor) {
        self.accept(ast_visitor);
    }
}

impl fmt::Display for CallExprNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.identifier)
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
        // for x in &self.exprs_t {
        //     x.auto_pre_inc_dec(ast_visitor);
        // }
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
    //    pub inc_dec: IncDecExpr,
}

impl ExprListNode {
    pub fn new(exprs_t: Vec<ExprType>) -> ExprListNode {
        ExprListNode {
            exprs_t,
            //            inc_dec: IncDecExpr::None,
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
    //     GlobalScope,  TODO!
    UnknownScope, // TODO - should this module or global scope?
    SystemScope,
    InterfaceBlockScope,
    DomainBlockScope,
    ActionsBlockScope,
    ActionVarScope,
    OperationsBlockScope,
    StateParamScope,
    StateVarScope,
    EventHandlerParamScope,
    EventHandlerVarScope,
    LoopVarScope,
    BlockVarScope,
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
    fn callable_accept(&self, ast_visitor: &mut dyn AstVisitor) {
        self.accept(ast_visitor);
    }
}

impl fmt::Display for IdentifierNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name.lexeme)
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

//-----------------------------------------------------//

// &String | &str | Widget<int> | `& mut String` | &`mut String` | *x

#[derive(Clone, PartialEq)]
pub struct TypeNode {
    #[allow(dead_code)]
    pub is_superstring: bool,
    pub is_system: bool,
    pub is_enum: bool,
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
        is_system: bool,
        is_reference: bool,
        is_enum: bool,
        frame_event_part_opt: Option<FrameEventPart>,
        type_str: String,
    ) -> TypeNode {
        TypeNode {
            is_superstring,
            is_system,
            is_enum,
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
    EnumMatchTest {
        enum_match_test_node: EnumMatchTestNode,
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
    pub string_match_type: StringMatchType,
    pub statements: Vec<DeclOrStmtType>,
    pub branch_terminator_expr_opt: Option<TerminatorExpr>,
}

impl StringMatchTestMatchBranchNode {
    pub fn new(
        string_match_type: StringMatchType,
        statements: Vec<DeclOrStmtType>,
        branch_terminator_t_opt: Option<TerminatorExpr>,
    ) -> StringMatchTestMatchBranchNode {
        StringMatchTestMatchBranchNode {
            string_match_type,
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

pub enum StringMatchType {
    MatchString {
        string_match_test_pattern_node: StringMatchTestPatternNode,
    },
    MatchEmptyString,
    MatchNullString,
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

pub struct EnumMatchTestNode {
    pub enum_type_name: String,
    pub expr_t: ExprType,
    pub match_branch_nodes: Vec<EnumMatchTestMatchBranchNode>,
    pub else_branch_node_opt: Option<EnumMatchTestElseBranchNode>,
}

impl EnumMatchTestNode {
    pub fn new(
        enum_type_name: String,
        expr_t: ExprType,
        match_branch_nodes: Vec<EnumMatchTestMatchBranchNode>,
        else_branch_node_opt: Option<EnumMatchTestElseBranchNode>,
    ) -> EnumMatchTestNode {
        EnumMatchTestNode {
            enum_type_name,
            expr_t,
            match_branch_nodes,
            else_branch_node_opt,
        }
    }
}

impl NodeElement for EnumMatchTestNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_enum_match_test_node(self);
    }
}

//-----------------------------------------------------//

pub struct EnumMatchTestMatchBranchNode {
    pub enum_type_name: String,
    pub enum_match_pattern_node: Vec<EnumMatchTestPatternNode>,
    pub statements: Vec<DeclOrStmtType>,
    pub branch_terminator_t_opt: Option<TerminatorExpr>,
}

impl EnumMatchTestMatchBranchNode {
    pub fn new(
        enum_type_name: String,
        enum_match_pattern_node: Vec<EnumMatchTestPatternNode>,
        statements: Vec<DeclOrStmtType>,
        branch_terminator_t_opt: Option<TerminatorExpr>,
    ) -> EnumMatchTestMatchBranchNode {
        EnumMatchTestMatchBranchNode {
            enum_type_name,
            enum_match_pattern_node,
            statements,
            branch_terminator_t_opt,
        }
    }
}

impl NodeElement for EnumMatchTestMatchBranchNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_enum_match_test_match_branch_node(self);
    }
}

//-----------------------------------------------------//

pub struct EnumMatchTestElseBranchNode {
    pub statements: Vec<DeclOrStmtType>,
    pub branch_terminator_expr_opt: Option<TerminatorExpr>,
}

impl EnumMatchTestElseBranchNode {
    pub fn new(
        statements: Vec<DeclOrStmtType>,
        branch_terminator_t_opt: Option<TerminatorExpr>,
    ) -> EnumMatchTestElseBranchNode {
        EnumMatchTestElseBranchNode {
            statements,
            branch_terminator_expr_opt: branch_terminator_t_opt,
        }
    }
}

impl NodeElement for EnumMatchTestElseBranchNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_enum_match_test_else_branch_node(self);
    }
}

//-----------------------------------------------------//

pub struct EnumMatchTestPatternNode {
    pub match_pattern: String,
}

impl EnumMatchTestPatternNode {
    pub fn new(match_pattern_strings: String) -> EnumMatchTestPatternNode {
        EnumMatchTestPatternNode {
            match_pattern: match_pattern_strings,
        }
    }
}

impl NodeElement for EnumMatchTestPatternNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_enum_match_test_pattern_node(self);
    }
}

//-----------------------------------------------------//

// Built-in Types

//-----------------------------------------------------//

pub struct SystemInstanceExprNode {
    pub identifier: IdentifierNode,
    pub start_state_state_args_opt: Option<ExprListNode>,
    pub start_state_enter_args_opt: Option<ExprListNode>,
    pub domain_args_opt: Option<ExprListNode>,
}

impl SystemInstanceExprNode {
    pub fn new(
        identifier: IdentifierNode,
        start_state_state_args_opt: Option<ExprListNode>,
        start_state_enter_args_opt: Option<ExprListNode>,
        domain_args_opt: Option<ExprListNode>,
    ) -> SystemInstanceExprNode {
        SystemInstanceExprNode {
            identifier,
            start_state_state_args_opt,
            start_state_enter_args_opt,
            domain_args_opt,
        }
    }
}

impl NodeElement for SystemInstanceExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_system_instance_expr_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_system_instance_expr_node_to_string(self, output);
    }
}

impl fmt::Display for SystemInstanceExprNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}

//-----------------------------------------------------//

pub struct SelfExprNode {
}

impl SelfExprNode {
    pub fn new() -> SelfExprNode {
        SelfExprNode {}
    }
}

impl NodeElement for SelfExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_self_expr_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_self_expr_node_to_string(self, output);
    }
}

//-----------------------------------------------------//

pub struct SystemTypeExprNode {
    pub identifier: IdentifierNode,
    pub call_chain_opt: Box<Option<ExprType>>,
}

impl SystemTypeExprNode {
    pub fn new(
        identifier: IdentifierNode,
        call_chain_opt: Box<Option<ExprType>>,
    ) -> SystemTypeExprNode {
        SystemTypeExprNode {
            identifier,
            call_chain_opt,
        }
    }
}

impl NodeElement for SystemTypeExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_system_type_expr_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_system_type_expr_node_to_string(self, output);
    }
}

impl fmt::Display for SystemTypeExprNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}

//-----------------------------------------------------//

// #[derive(Clone)]
pub struct ListNode {
    pub exprs_t: Vec<ExprType>,
    //    pub inc_dec: IncDecExpr,
}

impl ListNode {
    pub fn new(exprs_t: Vec<ExprType>) -> ListNode {
        ListNode { exprs_t }
    }
}

impl NodeElement for ListNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_list_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_list_node_to_string(self, output);
    }
}

//-----------------------------------------------------//

// ListElementNode captures list elements such as
// x[0], zoo["lion"], bar[foo()] etc.
pub struct ListElementNode {
    pub identifier: IdentifierNode,
    pub scope: IdentifierDeclScope,
    pub expr_t: ExprType,
}

impl ListElementNode {
    pub fn new(
        identifier: IdentifierNode,
        scope: IdentifierDeclScope,
        expr_t: ExprType,
    ) -> ListElementNode {
        ListElementNode {
            identifier,
            scope,
            expr_t,
        }
    }
}

impl NodeElement for ListElementNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_list_elem_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_list_elem_node_to_string(self, output);
    }
}

//-----------------------------------------------------//

// ReturnAssignExprNode captures "^= expr" statements.

pub struct ReturnAssignStmtNode {
    pub expr_t: ExprType,
}

impl ReturnAssignStmtNode {
    pub fn new(expr_t: ExprType) -> ReturnAssignStmtNode {
        ReturnAssignStmtNode { expr_t }
    }
}

impl NodeElement for ReturnAssignStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_return_assign_stmt_node(self);
    }

    // fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
    //     ast_visitor.visit_return_assign_stmt_node_to_string(self, output);
    // }
}

//-----------------------------------------------------//

pub struct ReturnStmtNode {
    pub expr_t_opt: Option<ExprType>,
}

impl ReturnStmtNode {
    pub fn new(expr_t_opt: Option<ExprType>) -> ReturnStmtNode {
        ReturnStmtNode { expr_t_opt }
    }
}

impl NodeElement for ReturnStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_return_stmt_node(self);
    }
}

//
// pub enum ListElementExprType {
//     ActionCallExprT {
//         action_call_expr_node: ActionCallExprNode,
//     },
//     CallChainExprT {
//         call_chain_expr_node: CallChainExprNode,
//     },
//     #[allow(dead_code)] // is used, don't know why I need this
//     CallExprT {
//         call_expr_node: CallExprNode,
//     },
//     VariableExprT {
//         var_node: VariableNode,
//     },
//     LiteralExprT {
//         literal_expr_node: LiteralExprNode,
//     },
// }
