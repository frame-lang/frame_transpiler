#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case)]
#![allow(dead_code)] // Many AST nodes are part of the API even if not currently used

use super::scanner::{TargetRegion, Token, TokenLiteral, TokenType};
use super::symbol_table::{ActionScopeSymbol, EventSymbol, SymbolType};

// Removed unused OperatorType imports
use crate::frame_c::native_region_segmenter::BodySegment;
use crate::frame_c::symbol_table::{InterfaceMethodSymbol, OperationScopeSymbol, ParameterSymbol};
use crate::frame_c::target_parsers::{ParsedTargetBlock, TargetAst};
use crate::frame_c::visitors::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;
use std::sync::Arc;
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

// v0.30: Top-level Frame module containing systems and functions
// v0.31: Added module-level variables and statements
// v0.57: Added imports for multi-file module system
pub struct FrameModule {
    pub module: Module,
    pub target_language: Option<TargetLanguage>,
    pub target_regions: Arc<Vec<TargetRegion>>,
    pub imports: Vec<ImportNode>, // v0.57: Track imports for multi-file system
    pub functions: Vec<Rc<RefCell<FunctionNode>>>,
    pub systems: Vec<SystemNode>,
    pub classes: Vec<Rc<RefCell<ClassNode>>>, // v0.45: Classes
    pub variables: Vec<Rc<RefCell<VariableDeclNode>>>,
    pub enums: Vec<Rc<RefCell<EnumDeclNode>>>,
    pub modules: Vec<Rc<RefCell<ModuleNode>>>, // v0.34: Nested modules
    pub native_modules: Vec<Rc<RefCell<NativeModuleDeclNode>>>, // v0.90: Native runtime declarations
    pub statements: Vec<DeclOrStmtType>,
}

impl FrameModule {
    pub fn new(
        module: Module,
        target_language: Option<TargetLanguage>,
        target_regions: Arc<Vec<TargetRegion>>,
        imports: Vec<ImportNode>, // v0.57: Added imports parameter
        functions: Vec<Rc<RefCell<FunctionNode>>>,
        systems: Vec<SystemNode>,
        classes: Vec<Rc<RefCell<ClassNode>>>,
        variables: Vec<Rc<RefCell<VariableDeclNode>>>,
        enums: Vec<Rc<RefCell<EnumDeclNode>>>,
        modules: Vec<Rc<RefCell<ModuleNode>>>,
        native_modules: Vec<Rc<RefCell<NativeModuleDeclNode>>>,
        statements: Vec<DeclOrStmtType>,
    ) -> FrameModule {
        FrameModule {
            module,
            target_language,
            target_regions,
            imports,
            functions,
            systems,
            classes,
            variables,
            enums,
            modules,
            native_modules,
            statements,
        }
    }

    // v0.30: Get primary system for single-system compatibility
    pub fn get_primary_system(&self) -> SystemNode {
        if self.systems.is_empty() {
            // Create empty system if no systems exist (function-only modules)
            SystemNode::new(
                String::new(),
                self.module.clone(),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                1,
            )
        } else {
            // TODO: Fix when Clone is resolved - for now create new system
            // Cannot clone systems without Clone trait
            SystemNode::new(
                self.systems[0].name.clone(),
                self.module.clone(),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                1,
            )
        }
    }
}

impl Module {
    pub fn new(module_elements: Vec<ModuleElement>) -> Module {
        Module { module_elements }
    }
}

impl Clone for Module {
    fn clone(&self) -> Self {
        Module {
            module_elements: vec![], // Simplified - only clone structure not contents
        }
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
    CodeBlock {
        code_block: String,
    },
    Target {
        language: TargetLanguage,
    },
    ModuleAttribute {
        attribute_node: AttributeNode,
    },
    // v0.30: Multi-entity support
    Function {
        function_node: Rc<RefCell<FunctionNode>>,
    },
    System {
        system_node: SystemNode,
    },
    // v0.31: Import support
    Import {
        import_node: ImportNode,
    },
    // v0.31: Module-scope variables and statements
    Variable {
        var_decl_node: Rc<RefCell<VariableDeclNode>>,
    },
    Statement {
        stmt_node: DeclOrStmtType,
    },
    // v0.32: Module-scope enums
    Enum {
        enum_decl_node: Rc<RefCell<EnumDeclNode>>,
    },
    // v0.34: Nested modules
    Module {
        module_node: Rc<RefCell<ModuleNode>>,
    },
    // v0.90: Native module declarations
    NativeModule {
        native_module_node: Rc<RefCell<NativeModuleDeclNode>>,
    },
    // v0.56: Type aliases
    TypeAlias {
        type_alias_node: TypeAliasNode,
    },
}

//-----------------------------------------------------//
// v0.90: Native declaration support

#[derive(Clone, Debug)]
pub struct NativeModuleDeclNode {
    pub qualified_name: Vec<String>,
    pub line: usize,
    pub column: usize,
    pub items: Vec<NativeModuleItem>,
}

impl NativeModuleDeclNode {
    pub fn new(
        qualified_name: Vec<String>,
        line: usize,
        column: usize,
        items: Vec<NativeModuleItem>,
    ) -> NativeModuleDeclNode {
        NativeModuleDeclNode {
            qualified_name,
            line,
            column,
            items,
        }
    }

    pub fn path(&self) -> String {
        self.qualified_name.join("::")
    }

    pub fn name(&self) -> Option<&str> {
        self.qualified_name.last().map(|s| s.as_str())
    }
}

impl NodeElement for NativeModuleDeclNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_native_module_decl_node(self);
    }
}

#[derive(Clone, Debug)]
pub enum NativeModuleItem {
    Type(NativeTypeDeclNode),
    Function(NativeFunctionDeclNode),
}

#[derive(Clone, Debug)]
pub struct NativeTypeDeclNode {
    pub name: String,
    pub aliased_type: Option<String>,
    pub line: usize,
    pub column: usize,
}

impl NativeTypeDeclNode {
    pub fn new(name: String, aliased_type: Option<String>, line: usize, column: usize) -> Self {
        NativeTypeDeclNode {
            name,
            aliased_type,
            line,
            column,
        }
    }
}

#[derive(Clone, Debug)]
pub struct NativeFunctionDeclNode {
    pub name: String,
    pub parameters: Vec<NativeFunctionParameterNode>,
    pub return_type: Option<String>,
    pub is_async: bool,
    pub line: usize,
    pub column: usize,
}

impl NativeFunctionDeclNode {
    pub fn new(
        name: String,
        parameters: Vec<NativeFunctionParameterNode>,
        return_type: Option<String>,
        is_async: bool,
        line: usize,
        column: usize,
    ) -> Self {
        NativeFunctionDeclNode {
            name,
            parameters,
            return_type,
            is_async,
            line,
            column,
        }
    }
}

#[derive(Clone, Debug)]
pub struct NativeFunctionParameterNode {
    pub name: String,
    pub type_annotation: Option<String>,
    pub line: usize,
    pub column: usize,
}

impl NativeFunctionParameterNode {
    pub fn new(name: String, type_annotation: Option<String>, line: usize, column: usize) -> Self {
        NativeFunctionParameterNode {
            name,
            type_annotation,
            line,
            column,
        }
    }
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
    // Special node for 'self' reference (v0.37)
    SelfT {
        self_expr_node: SelfExprNode,
    },
    // Special node for literal expressions with method calls (v0.41)
    CallChainLiteralExprT {
        call_chain_literal_expr_node: CallChainLiteralExprNode,
    },
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
    SliceNodeT {
        slice_node: SliceNode,
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
    UndeclaredSliceT {
        slice_node: SliceNode,
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
            CallChainNodeType::CallChainLiteralExprT { .. } => {
                // Literals don't need reference marking
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionBody {
    Empty,
    Frame,
    TargetSpecific,
    Mixed,
}

/// Frame MIR (minimal intermediate representation) used inside mixed bodies
/// to capture Frame directives alongside native target code.
#[derive(Debug, Clone)]
pub enum MirStatement {
    /// -> $StateName(args...)
    Transition {
        state: String,
        // Future: carry parsed args once available (ExprType). For now, capture as strings.
        args: Vec<String>,
    },
    /// => $^
    ParentForward,
    /// $$[+]
    StackPush,
    /// $$[-]
    StackPop,
    /// return [expr]
    Return(Option<String>),
}

/// MixedBodyItem captures an ordered sequence of native target code and
/// embedded Frame directives (as MIR). This is target-agnostic at the AST level.
#[derive(Clone)]
pub enum MixedBodyItem {
    /// Verbatim native text for the active target with source span
    NativeText {
        target: TargetLanguage,
        text: String,
        start_line: usize,
        end_line: usize,
    },
    /// Parsed native AST slice for the active target with source span
    NativeAst {
        target: TargetLanguage,
        start_line: usize,
        end_line: usize,
        ast: Arc<dyn TargetAst>,
    },
    /// A Frame statement occurring inside a native body
    Frame {
        frame_line: usize,
        indent: usize, // leading whitespace count on the directive line
        stmt: MirStatement,
    },
}

#[derive(Debug, Clone)]
pub struct UnrecognizedStatementNode {
    pub frame_line: usize,
    pub target: TargetLanguage,
    pub region_index: usize,
}

//-----------------------------------------------------//
// v0.30 Improved Call Chain Architecture
// These new types provide a cleaner, more maintainable
// approach to handling call chains

#[derive(Debug, Clone)]
pub enum IdentifierScope {
    Unknown,   // External identifiers
    Variable,  // Local variables
    Parameter, // Parameters
    Domain,    // Domain variables
    System,    // System names
    Function,  // Function names
}

#[derive(Debug, Clone)]
pub enum CallTargetType {
    Unknown,   // External functions like print()
    Interface, // Interface methods
    Operation, // Operations
    Action,    // Actions
    Function,  // Functions
}

// Context for method/variable access in Frame v0.31
#[derive(Debug, Clone, PartialEq)]
pub enum CallContextType {
    SelfCall,           // self.method() - calls action or operation
    StaticCall(String), // For static method calls: System.operation() or Class.method() with @staticmethod
    // TODO v0.62: Parser should detect and set this for static call patterns
    ExternalCall, // function() - external function or local (default)
}

// v0.62: Semantic call resolution (to replace CallContextType)
// This represents the actual semantic meaning of a call, determined during parsing
#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedCallType {
    Action(String),    // Internal action call (needs _ prefix)
    Operation(String), // Internal operation call
    SystemInterface {
        // Interface method call from within system
        system: String,
        method: String,
    },
    SystemOperation {
        // Qualified system operation call
        system: String,
        operation: String,
        is_static: bool, // True if marked with @staticmethod
    },
    ClassMethod {
        // Qualified class method call
        class: String,
        method: String,
        is_static: bool, // True if marked with @staticmethod
    },
    ModuleFunction {
        // Qualified module function call
        module: String,
        function: String,
    },
    NativeFunction {
        module: String,
        function: String,
    },
    External(String), // True external function call
}

// New simplified call chain node types
// These will eventually replace CallChainNodeType
pub enum CallChainNodeTypeV2 {
    // BASE TYPES
    Identifier {
        name: String,
        scope: IdentifierScope,
        line: usize,
    },
    Call {
        expr: CallExprNode,
        target_type: CallTargetType,
    },
    ListAccess {
        expr: ListElementNode,
    },

    // SPECIALIZED TYPES (for optimization/validation)
    Variable {
        var_node: VariableNode,
    },
    InterfaceMethod {
        method_node: InterfaceMethodCallExprNode,
    },
    Operation {
        op_node: OperationCallExprNode,
    },
    Action {
        action_node: ActionCallExprNode,
    },
}

impl CallChainNodeTypeV2 {
    pub fn set_is_reference(&mut self, is_reference: bool) {
        match self {
            CallChainNodeTypeV2::Variable { var_node } => {
                var_node.id_node.is_reference = is_reference;
            }
            CallChainNodeTypeV2::Identifier { .. } => {
                // For now, we'll handle this in the conversion phase
            }
            _ => {
                // Other types don't support reference semantics yet
            }
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

// Assignment operators for compound assignments (v0.39)
#[derive(Clone, Debug, PartialEq)]
pub enum AssignmentOperator {
    Equals,            // =
    PlusEquals,        // +=
    MinusEquals,       // -=
    StarEquals,        // *=
    SlashEquals,       // /=
    FloorDivideEquals, // //=
    PercentEquals,     // %=
    PowerEquals,       // **=
    AndEquals,         // &=
    OrEquals,          // |=
    LeftShiftEquals,   // <<=
    RightShiftEquals,  // >>=
    XorEquals,         // ^= (v0.40)
    MatMulEquals,      // @= (v0.40)
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
    pub runtime_info: Option<RuntimeInfo>, // v0.37: Runtime async requirements
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
            runtime_info: None, // v0.37: Will be populated during semantic analysis
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

impl Clone for SystemNode {
    fn clone(&self) -> Self {
        SystemNode {
            name: self.name.clone(),
            module: self.module.clone(),
            system_attributes_opt: None, // Simplified clone
            start_state_state_params_opt: None,
            start_state_enter_params_opt: None,
            domain_params_opt: None,
            interface_block_node_opt: None,
            machine_block_node_opt: None,
            actions_block_node_opt: None,
            operations_block_node_opt: None,
            domain_block_node_opt: None,
            line: self.line,
            runtime_info: None, // v0.37: Runtime info not cloned
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
    pub line: usize, // v0.77: source map support for interface definitions
    pub name: String,
    pub params: Option<Vec<ParameterNode>>,
    pub return_type_opt: Option<TypeNode>,
    pub return_init_expr_opt: Option<ExprType>,
    pub alias: Option<MessageNode>,
    pub is_async: bool, // v0.35: async interface method support
}

impl InterfaceMethodNode {
    pub fn new(
        name: String,
        params: Option<Vec<ParameterNode>>,
        return_type: Option<TypeNode>,
        return_init_expr_opt: Option<ExprType>,
        alias: Option<MessageNode>,
        is_async: bool,
        line: usize,
    ) -> InterfaceMethodNode {
        InterfaceMethodNode {
            name,
            params,
            return_type_opt: return_type,
            return_init_expr_opt,
            alias,
            is_async,
            line,
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

// v0.31: Import support (Python modules)
// v0.57: Extended for Frame file imports
#[derive(Clone, Debug)]
pub enum ImportType {
    // Python imports (existing v0.31)
    // import math
    Simple {
        module: String,
    },
    // import numpy as np
    Aliased {
        module: String,
        alias: String,
    },
    // from math import sqrt, pi
    FromImport {
        module: String,
        items: Vec<String>,
    },
    // from math import *
    FromImportAll {
        module: String,
    },

    // Frame file imports (new v0.57)
    // import Utils from "./utils.frm"
    FrameModule {
        module_name: String,
        file_path: String,
    },
    // import Utils from "./utils.frm" as U
    FrameModuleAliased {
        module_name: String,
        file_path: String,
        alias: String,
    },
    // import { add, multiply } from "./math.frm"
    FrameSelective {
        items: Vec<String>,
        file_path: String,
    },
    Native {
        target: TargetLanguage,
        code: String,
    },
}

#[derive(Clone, Debug)]
pub struct ImportNode {
    pub import_type: ImportType,
    pub line: usize,
}

impl ImportNode {
    pub fn new(import_type: ImportType, line: usize) -> ImportNode {
        ImportNode { import_type, line }
    }
}

impl NodeElement for ImportNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_import_node(self);
    }
}

//-----------------------------------------------------//
// v0.56: Type alias support

#[derive(Clone, Debug)]
pub struct TypeAliasNode {
    pub name: String,
    pub type_expr: String, // The type expression as a string (e.g., "tuple[float, float]")
    pub line: usize,
}

impl TypeAliasNode {
    pub fn new(name: String, type_expr: String, line: usize) -> TypeAliasNode {
        TypeAliasNode {
            name,
            type_expr,
            line,
        }
    }
}

impl NodeElement for TypeAliasNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_type_alias_node(self);
    }
}

//-----------------------------------------------------//
// v0.34: Nested module support

pub struct ModuleNode {
    pub name: String,
    pub functions: Vec<Rc<RefCell<FunctionNode>>>,
    pub systems: Vec<SystemNode>,
    pub variables: Vec<Rc<RefCell<VariableDeclNode>>>,
    pub enums: Vec<Rc<RefCell<EnumDeclNode>>>,
    pub modules: Vec<Rc<RefCell<ModuleNode>>>, // Nested modules
}

impl ModuleNode {
    pub fn new(
        name: String,
        functions: Vec<Rc<RefCell<FunctionNode>>>,
        systems: Vec<SystemNode>,
        variables: Vec<Rc<RefCell<VariableDeclNode>>>,
        enums: Vec<Rc<RefCell<EnumDeclNode>>>,
        modules: Vec<Rc<RefCell<ModuleNode>>>,
    ) -> ModuleNode {
        ModuleNode {
            name,
            functions,
            systems,
            variables,
            enums,
            modules,
        }
    }
}

impl NodeElement for ModuleNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_module_node(self);
    }
}

//-----------------------------------------------------//

// v0.45: Class support
pub struct ClassNode {
    pub name: String,
    pub parent: Option<String>,  // Parent class name for inheritance
    pub decorators: Vec<String>, // v0.58: Class decorators (pass-through to Python)
    pub methods: Vec<Rc<RefCell<MethodNode>>>,
    pub static_methods: Vec<Rc<RefCell<MethodNode>>>,
    pub class_methods: Vec<Rc<RefCell<MethodNode>>>, // @classmethod methods
    pub properties: Vec<Rc<RefCell<PropertyNode>>>,  // @property definitions
    pub instance_vars: Vec<Rc<RefCell<VariableDeclNode>>>,
    pub static_vars: Vec<Rc<RefCell<VariableDeclNode>>>,
    pub constructor: Option<Rc<RefCell<MethodNode>>>, // implicit init() or @init methods
    pub line: usize,
}

impl ClassNode {
    pub fn new(
        name: String,
        parent: Option<String>,
        decorators: Vec<String>,
        methods: Vec<Rc<RefCell<MethodNode>>>,
        static_methods: Vec<Rc<RefCell<MethodNode>>>,
        class_methods: Vec<Rc<RefCell<MethodNode>>>,
        properties: Vec<Rc<RefCell<PropertyNode>>>,
        instance_vars: Vec<Rc<RefCell<VariableDeclNode>>>,
        static_vars: Vec<Rc<RefCell<VariableDeclNode>>>,
        constructor: Option<Rc<RefCell<MethodNode>>>,
        line: usize,
    ) -> ClassNode {
        ClassNode {
            name,
            parent,
            decorators,
            methods,
            static_methods,
            class_methods,
            properties,
            instance_vars,
            static_vars,
            constructor,
            line,
        }
    }
}

impl NodeElement for ClassNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_class_node(self);
    }
}

// v0.46: Property node for @property definitions
pub struct PropertyNode {
    pub name: String,
    pub getter: Option<Rc<RefCell<MethodNode>>>,
    pub setter: Option<Rc<RefCell<MethodNode>>>,
    pub deleter: Option<Rc<RefCell<MethodNode>>>,
}

impl PropertyNode {
    pub fn new(name: String) -> PropertyNode {
        PropertyNode {
            name,
            getter: None,
            setter: None,
            deleter: None,
        }
    }
}

// v0.45: Method node for class methods
pub struct MethodNode {
    pub name: String,
    pub params: Option<Vec<ParameterNode>>,
    pub statements: Vec<DeclOrStmtType>,
    pub terminator_expr: TerminatorExpr,
    pub type_opt: Option<TypeNode>,
    pub is_constructor: bool, // true if this is an init method
    pub is_static: bool,      // true if @staticmethod
    pub is_class: bool,       // true if @classmethod
    pub line: usize,
}

impl MethodNode {
    pub fn new(
        name: String,
        params: Option<Vec<ParameterNode>>,
        statements: Vec<DeclOrStmtType>,
        terminator_expr: TerminatorExpr,
        type_opt: Option<TypeNode>,
        is_constructor: bool,
        is_static: bool,
        is_class: bool,
        line: usize,
    ) -> MethodNode {
        MethodNode {
            name,
            params,
            statements,
            terminator_expr,
            type_opt,
            is_constructor,
            is_static,
            is_class,
            line,
        }
    }
}

impl NodeElement for MethodNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_method_node(self);
    }
}

//-----------------------------------------------------//

pub struct FunctionNode {
    pub line: usize,
    pub name: String,
    pub params: Option<Vec<ParameterNode>>,
    pub is_implemented: bool,
    pub statements: Vec<DeclOrStmtType>,
    pub terminator_expr: TerminatorExpr,
    pub type_opt: Option<TypeNode>,
    pub is_async: bool, // v0.35: async function support
    pub target_specific_regions: Vec<TargetSpecificRegionRef>,
    pub parsed_target_blocks: Vec<ParsedTargetBlock>,
    pub unrecognized_statements: Vec<UnrecognizedStatementNode>,
    pub body: ActionBody,
}

impl FunctionNode {
    pub fn new(
        name: String,
        params: Option<Vec<ParameterNode>>,
        is_implemented: bool,
        statements: Vec<DeclOrStmtType>,
        terminator_node: TerminatorExpr,
        type_opt: Option<TypeNode>,
        is_async: bool,
        line: usize,
    ) -> FunctionNode {
        FunctionNode {
            name,
            params,
            is_implemented,
            statements,
            terminator_expr: terminator_node,
            type_opt,
            is_async,
            line,
            target_specific_regions: Vec::new(),
            parsed_target_blocks: Vec::new(),
            unrecognized_statements: Vec::new(),
            body: ActionBody::Empty,
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
    pub line: usize, // v0.78.7: source map support
    pub name: String,
    pub params: Option<Vec<ParameterNode>>,
    pub is_implemented: bool,
    pub statements: Vec<DeclOrStmtType>,
    pub terminator_expr: TerminatorExpr,
    pub type_opt: Option<TypeNode>,
    pub is_async: bool,           // v0.37: async action support
    pub code_opt: Option<String>, // TODO - remove
    pub target_specific_regions: Vec<TargetSpecificRegionRef>,
    pub parsed_target_blocks: Vec<ParsedTargetBlock>,
    pub unrecognized_statements: Vec<UnrecognizedStatementNode>,
    pub body: ActionBody,
    pub segmented_body: Option<Vec<BodySegment>>, // NativeRegion segments for native bodies
    pub mixed_body: Option<Vec<MixedBodyItem>>,   // Unified sequence (native + MIR directives)
}

impl ActionNode {
    pub fn new(
        line: usize,
        name: String,
        params: Option<Vec<ParameterNode>>,
        is_implemented: bool,
        statements: Vec<DeclOrStmtType>,
        terminator_node: TerminatorExpr,
        type_opt: Option<TypeNode>,
        is_async: bool,
        code_opt: Option<String>,
    ) -> ActionNode {
        ActionNode {
            line,
            name,
            params,
            is_implemented,
            statements,
            terminator_expr: terminator_node,
            type_opt,
            is_async,
            code_opt,
            target_specific_regions: Vec::new(),
            parsed_target_blocks: Vec::new(),
            unrecognized_statements: Vec::new(),
            body: ActionBody::Empty,
            segmented_body: None,
            mixed_body: None,
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
    pub is_async: bool,           // v0.35: async operation support
    pub code_opt: Option<String>, // TODO - remove
    pub line: usize,              // v0.78.2: source map support for operations
    pub target_specific_regions: Vec<TargetSpecificRegionRef>,
    pub parsed_target_blocks: Vec<ParsedTargetBlock>,
    pub unrecognized_statements: Vec<UnrecognizedStatementNode>,
    pub body: ActionBody,
    pub segmented_body: Option<Vec<BodySegment>>, // NativeRegion segments for native bodies
    pub mixed_body: Option<Vec<MixedBodyItem>>,   // Unified sequence (native + MIR directives)
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
        is_async: bool,
        code_opt: Option<String>,
        line: usize,
    ) -> OperationNode {
        OperationNode {
            name,
            params,
            attributes_opt,
            is_implemented,
            statements,
            terminator_expr: terminator_node,
            type_opt,
            is_async,
            code_opt,
            line,
            target_specific_regions: Vec::new(),
            parsed_target_blocks: Vec::new(),
            unrecognized_statements: Vec::new(),
            body: ActionBody::Empty,
            segmented_body: None,
            mixed_body: None,
        }
    }

    pub fn is_static(&self) -> bool {
        if let Some(attributes_map) = &self.attributes_opt {
            // Check for both "static" (legacy) and "staticmethod" (v0.20 Python-style)
            let is_static = attributes_map
                .get("staticmethod")
                .or_else(|| attributes_map.get("static"));
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
    pub line: usize,
    pub name: String,
    pub type_opt: Option<TypeNode>,
    pub is_constant: bool,
    pub initializer_value_rc: Rc<ExprType>,
    pub value_rc: Rc<ExprType>,
    pub identifier_decl_scope: IdentifierDeclScope,
}

impl VariableDeclNode {
    pub fn new(
        line: usize,
        name: String,
        type_opt: Option<TypeNode>,
        is_constant: bool,
        initializer_value_rc: Rc<ExprType>,
        value_rc: Rc<ExprType>,
        identifier_decl_scope: IdentifierDeclScope,
    ) -> VariableDeclNode {
        VariableDeclNode {
            line,
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
    pub line: usize,
    pub id_node: IdentifierNode,
    pub scope: IdentifierDeclScope,
    pub symbol_type_rcref_opt: Option<Rc<RefCell<SymbolType>>>, // TODO: consider a new enum for just variable types
    pub is_self: bool, // NEW: true for self.variable access
}
impl VariableNode {
    pub fn new(
        line: usize,
        id_node: IdentifierNode,
        scope: IdentifierDeclScope,
        symbol_type_rcref_opt: Option<Rc<RefCell<SymbolType>>>,
    ) -> VariableNode {
        VariableNode {
            line,
            id_node,
            scope, // TODO: consider accessor or moving out of IdentifierNode
            symbol_type_rcref_opt,
            is_self: false, // Default to false
        }
    }

    pub fn new_with_self(
        line: usize,
        id_node: IdentifierNode,
        scope: IdentifierDeclScope,
        symbol_type_rcref_opt: Option<Rc<RefCell<SymbolType>>>,
        is_self: bool,
    ) -> VariableNode {
        VariableNode {
            line,
            id_node,
            scope,
            symbol_type_rcref_opt,
            is_self,
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

#[derive(Clone, Debug, PartialEq)]
pub enum EnumType {
    Integer,
    String,
}

#[derive(Clone, Debug)]
pub enum EnumValue {
    Integer(i32),
    String(String),
    Auto, // Compiler determines based on enum type
}

pub struct EnumDeclNode {
    pub line: usize, // v0.78.8: source map support
    pub name: String,
    pub enum_type: EnumType,
    pub enums: Vec<Rc<EnumeratorDeclNode>>,
}

impl EnumDeclNode {
    pub fn new(
        line: usize,
        identifier: String,
        enum_type: EnumType,
        enums: Vec<Rc<EnumeratorDeclNode>>,
    ) -> EnumDeclNode {
        EnumDeclNode {
            line,
            name: identifier,
            enum_type,
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
    pub line: usize, // v0.78.9: source map support
    pub name: String,
    pub value: EnumValue,
}

impl EnumeratorDeclNode {
    pub fn new(line: usize, name: String, value: EnumValue) -> EnumeratorDeclNode {
        EnumeratorDeclNode { line, name, value }
    }
}

impl NodeElement for EnumeratorDeclNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_enumerator_decl_node(self);
    }
}

#[derive(Clone)]
pub struct EnumeratorExprNode {
    pub line: usize,
    pub enum_type: String,
    pub enumerator: String,
}

impl EnumeratorExprNode {
    pub fn new(line: usize, enum_type: String, enumerator: String) -> EnumeratorExprNode {
        EnumeratorExprNode {
            line,
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

#[derive(Clone)]
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
    pub terminator_node: Option<TerminatorExpr>,
    pub event_symbol_rcref: Rc<RefCell<EventSymbol>>,
    // this is so we can know to declare a StateContext at the
    // top of the event handler.
    pub event_handler_has_transition: bool,
    pub line: usize,
    pub return_init_expr_opt: Option<ExprType>, // Default return value for event handler
    pub is_async: bool,                         // v0.37: async event handler support
    pub target_specific_regions: Vec<TargetSpecificRegionRef>,
    pub parsed_target_blocks: Vec<ParsedTargetBlock>,
    pub unrecognized_statements: Vec<UnrecognizedStatementNode>,
    pub body: ActionBody,
    pub segmented_body: Option<Vec<crate::frame_c::native_region_segmenter::BodySegment>>, // NativeRegion segments for native TS bodies
    pub mixed_body: Option<Vec<MixedBodyItem>>, // Unified sequence (native + MIR directives)
}

impl EventHandlerNode {
    pub fn new(
        //event_handler_type:EventHandlerType,
        state_name: String,
        msg_t: MessageType,
        statements: Vec<DeclOrStmtType>,
        terminator_node: Option<TerminatorExpr>,
        event_symbol_rcref: Rc<RefCell<EventSymbol>>,
        event_handler_has_transition: bool,
        line: usize,
        return_init_expr_opt: Option<ExprType>,
        is_async: bool,
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
            return_init_expr_opt,
            is_async,
            target_specific_regions: Vec::new(),
            parsed_target_blocks: Vec::new(),
            unrecognized_statements: Vec::new(),
            body: ActionBody::Empty,
            segmented_body: None,
            mixed_body: None,
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

#[derive(Debug, Clone)]
pub struct TargetSpecificRegionRef {
    pub target: TargetLanguage,
    pub region_index: usize,
    pub frame_start_line: usize,
    pub frame_end_line: usize,
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
    WalrusExprT {
        assignment_expr_node: AssignmentExprNode, // Walrus operator (:=) - assignment that returns value
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
    DictLiteralT {
        dict_literal_node: DictLiteralNode,
    },
    SetLiteralT {
        set_literal_node: SetLiteralNode,
    },
    TupleLiteralT {
        tuple_literal_node: TupleLiteralNode,
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
    // Unpacking operator (v0.34)
    UnpackExprT {
        unpack_expr_node: UnpackExprNode,
    },
    // Dict unpacking operator (v0.38)
    DictUnpackExprT {
        dict_unpack_expr_node: DictUnpackExprNode,
    },
    // List comprehension (v0.34)
    ListComprehensionExprT {
        list_comprehension_node: ListComprehensionNode,
    },
    // Dictionary comprehension (v0.38)
    DictComprehensionExprT {
        dict_comprehension_node: DictComprehensionNode,
    },
    // Set comprehension (v0.41)
    SetComprehensionExprT {
        set_comprehension_node: SetComprehensionNode,
    },
    // Await expression (v0.35)
    AwaitExprT {
        await_expr_node: AwaitExprNode,
    },
    // Lambda expression (v0.38)
    LambdaExprT {
        lambda_expr_node: LambdaExprNode,
    },
    // Function reference (v0.38) - for first-class functions
    FunctionRefT {
        name: String,
    },
    // Star expression (v0.54) - for unpacking with rest (*var)
    StarExprT {
        star_expr_node: StarExprNode,
    },
    // Yield expression (v0.42)
    YieldExprT {
        yield_expr_node: YieldExprNode,
    },
    // Yield from expression (v0.42)
    YieldFromExprT {
        yield_from_expr_node: YieldFromExprNode,
    },
    // Generator expression (v0.42)
    GeneratorExprT {
        generator_expr_node: GeneratorExprNode,
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
            ExprType::WalrusExprT { .. } => true, // Walrus can be used in binary expressions
            ExprType::TransitionExprT { .. } => false,
            ExprType::StateStackOperationExprT { .. } => false,
            ExprType::CallExprListT { .. } => false, // this shouldn't happen
            _ => true,
        }
    }
    pub fn is_valid_assignment_rvalue_expr_type(&self) -> bool {
        match self {
            ExprType::AssignmentExprT { .. } => false,
            ExprType::WalrusExprT { .. } => true, // Walrus can be used as rvalue (it returns a value)
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
            ExprType::WalrusExprT { .. } => "WalrusExprT",
            ExprType::ActionCallExprT { .. } => "ActionCallExprT",
            ExprType::CallChainExprT { .. } => "CallChainExprT",
            ExprType::CallExprT { .. } => "CallExprT",
            ExprType::CallExprListT { .. } => "CallExprListT",
            ExprType::ListT { .. } => "ListT",
            ExprType::DictLiteralT { .. } => "DictLiteralT",
            ExprType::SetLiteralT { .. } => "SetLiteralT",
            ExprType::TupleLiteralT { .. } => "TupleLiteralT",
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
            ExprType::UnpackExprT { .. } => "UnpackExprT",
            ExprType::DictUnpackExprT { .. } => "DictUnpackExprT",
            ExprType::ListComprehensionExprT { .. } => "ListComprehensionExprT",
            ExprType::DictComprehensionExprT { .. } => "DictComprehensionExprT",
            ExprType::SetComprehensionExprT { .. } => "SetComprehensionExprT",
            ExprType::AwaitExprT { .. } => "AwaitExprT",
            ExprType::LambdaExprT { .. } => "LambdaExprT",
            ExprType::FunctionRefT { .. } => "FunctionRefT",
            ExprType::YieldExprT { .. } => "YieldExprT",
            ExprType::YieldFromExprT { .. } => "YieldFromExprT",
            ExprType::GeneratorExprT { .. } => "GeneratorExprT",
            ExprType::StarExprT { .. } => "StarExprT",
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
            ExprType::UnpackExprT { unpack_expr_node } => {
                print!("*");
                unpack_expr_node.expr.debug_print();
            }
            ExprType::DictUnpackExprT {
                dict_unpack_expr_node,
            } => {
                print!("**");
                dict_unpack_expr_node.expr.debug_print();
            }
            ExprType::ListComprehensionExprT {
                list_comprehension_node,
            } => {
                print!("[");
                list_comprehension_node.expr.debug_print();
                print!(" for {} in ", list_comprehension_node.target);
                list_comprehension_node.iter.debug_print();
                if let Some(ref cond) = list_comprehension_node.condition {
                    print!(" if ");
                    cond.debug_print();
                }
                print!("]");
            }
            ExprType::DictComprehensionExprT {
                dict_comprehension_node,
            } => {
                print!("{{");
                dict_comprehension_node.key_expr.debug_print();
                print!(": ");
                dict_comprehension_node.value_expr.debug_print();
                print!(" for {} in ", dict_comprehension_node.target);
                dict_comprehension_node.iter.debug_print();
                if let Some(ref cond) = dict_comprehension_node.condition {
                    print!(" if ");
                    cond.debug_print();
                }
                print!("}}");
            }
            ExprType::SetComprehensionExprT {
                set_comprehension_node,
            } => {
                print!("{{");
                set_comprehension_node.expr.debug_print();
                print!(" for {} in ", set_comprehension_node.target);
                set_comprehension_node.iter.debug_print();
                if let Some(ref cond) = set_comprehension_node.condition {
                    print!(" if ");
                    cond.debug_print();
                }
                print!("}}");
            }
            ExprType::AwaitExprT { await_expr_node } => {
                print!("await ");
                await_expr_node.expr.debug_print();
            }
            ExprType::WalrusExprT {
                assignment_expr_node,
            } => {
                print!("(");
                assignment_expr_node.l_value_box.debug_print();
                print!(" := ");
                assignment_expr_node.r_value_rc.debug_print();
                print!(")");
            }
            ExprType::YieldExprT { yield_expr_node } => {
                print!("yield");
                if let Some(ref expr) = yield_expr_node.expr {
                    print!(" ");
                    expr.debug_print();
                }
            }
            ExprType::YieldFromExprT {
                yield_from_expr_node,
            } => {
                print!("yield from ");
                yield_from_expr_node.expr.debug_print();
            }
            ExprType::GeneratorExprT {
                generator_expr_node,
            } => {
                print!("(");
                generator_expr_node.expr.debug_print();
                print!(" for {} in ", generator_expr_node.target);
                generator_expr_node.iter.debug_print();
                if let Some(ref condition) = generator_expr_node.condition {
                    print!(" if ");
                    condition.debug_print();
                }
                print!(")");
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
            ExprType::WalrusExprT {
                assignment_expr_node,
            } => {
                ast_visitor.visit_walrus_expr_node(assignment_expr_node);
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
            ExprType::DictLiteralT { dict_literal_node } => {
                ast_visitor.visit_dict_literal_node(dict_literal_node);
            }
            ExprType::SetLiteralT { set_literal_node } => {
                ast_visitor.visit_set_literal_node(set_literal_node);
            }
            ExprType::TupleLiteralT { tuple_literal_node } => {
                ast_visitor.visit_tuple_literal_node(tuple_literal_node);
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
            ExprType::UnpackExprT { unpack_expr_node } => {
                ast_visitor.visit_unpack_expr_node(unpack_expr_node);
            }
            ExprType::DictUnpackExprT {
                dict_unpack_expr_node,
            } => {
                ast_visitor.visit_dict_unpack_expr_node(dict_unpack_expr_node);
            }
            ExprType::ListComprehensionExprT {
                list_comprehension_node,
            } => {
                ast_visitor.visit_list_comprehension_node(list_comprehension_node);
            }
            ExprType::DictComprehensionExprT {
                dict_comprehension_node,
            } => {
                ast_visitor.visit_dict_comprehension_node(dict_comprehension_node);
            }
            ExprType::SetComprehensionExprT {
                set_comprehension_node,
            } => {
                ast_visitor.visit_set_comprehension_node(set_comprehension_node);
            }
            ExprType::AwaitExprT { await_expr_node } => {
                ast_visitor.visit_await_expr_node(await_expr_node);
            }
            ExprType::LambdaExprT { lambda_expr_node } => {
                ast_visitor.visit_lambda_expr_node(lambda_expr_node);
            }
            ExprType::FunctionRefT { name } => {
                ast_visitor.visit_function_ref_node(name);
            }
            ExprType::YieldExprT { yield_expr_node } => {
                ast_visitor.visit_yield_expr_node(yield_expr_node);
            }
            ExprType::YieldFromExprT {
                yield_from_expr_node,
            } => {
                ast_visitor.visit_yield_from_expr_node(yield_from_expr_node);
            }
            ExprType::GeneratorExprT {
                generator_expr_node,
            } => {
                ast_visitor.visit_generator_expr_node(generator_expr_node);
            }
            ExprType::StarExprT { star_expr_node } => {
                ast_visitor.visit_star_expr_node(star_expr_node);
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
            ExprType::DictLiteralT { dict_literal_node } => {
                ast_visitor.visit_dict_literal_node_to_string(dict_literal_node, output);
            }
            ExprType::SetLiteralT { set_literal_node } => {
                ast_visitor.visit_set_literal_node_to_string(set_literal_node, output);
            }
            ExprType::TupleLiteralT { tuple_literal_node } => {
                ast_visitor.visit_tuple_literal_node_to_string(tuple_literal_node, output);
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
                ast_visitor.visit_self_expr_node_to_string(self_expr_node, output);
            }
            ExprType::NilExprT => {
                panic!("Unexpect use of ExprType::NilExprT");
            }
            ExprType::DefaultLiteralValueForTypeExprT => {
                panic!("Unexpect use of ExprType::DefaultLiteralValueForTypeExprT");
            }
            ExprType::UnpackExprT { unpack_expr_node } => {
                ast_visitor.visit_unpack_expr_node_to_string(unpack_expr_node, output);
            }
            ExprType::DictUnpackExprT {
                dict_unpack_expr_node,
            } => {
                ast_visitor.visit_dict_unpack_expr_node_to_string(dict_unpack_expr_node, output);
            }
            ExprType::ListComprehensionExprT {
                list_comprehension_node,
            } => {
                ast_visitor
                    .visit_list_comprehension_node_to_string(list_comprehension_node, output);
            }
            ExprType::DictComprehensionExprT {
                dict_comprehension_node,
            } => {
                ast_visitor
                    .visit_dict_comprehension_node_to_string(dict_comprehension_node, output);
            }
            ExprType::SetComprehensionExprT {
                set_comprehension_node,
            } => {
                ast_visitor.visit_set_comprehension_node_to_string(set_comprehension_node, output);
            }
            ExprType::AwaitExprT { await_expr_node } => {
                ast_visitor.visit_await_expr_node_to_string(await_expr_node, output);
            }
            ExprType::WalrusExprT {
                assignment_expr_node,
            } => {
                ast_visitor.visit_walrus_expr_node_to_string(assignment_expr_node, output);
            }
            ExprType::LambdaExprT { lambda_expr_node } => {
                ast_visitor.visit_lambda_expr_node_to_string(lambda_expr_node, output);
            }
            ExprType::FunctionRefT { name } => {
                ast_visitor.visit_function_ref_node_to_string(name, output);
            }
            ExprType::YieldExprT { yield_expr_node } => {
                ast_visitor.visit_yield_expr_node_to_string(yield_expr_node, output);
            }
            ExprType::YieldFromExprT {
                yield_from_expr_node,
            } => {
                ast_visitor.visit_yield_from_expr_node_to_string(yield_from_expr_node, output);
            }
            ExprType::GeneratorExprT {
                generator_expr_node,
            } => {
                ast_visitor.visit_generator_expr_node_to_string(generator_expr_node, output);
            }
            ExprType::StarExprT { star_expr_node } => {
                ast_visitor.visit_star_expr_node_to_string(star_expr_node, output);
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
    DelStmt {
        del_stmt_node: DelStmtNode,
    },
    AssertStmt {
        assert_stmt_node: AssertStmtNode,
    },
    TryStmt {
        try_stmt_node: TryStmtNode,
    },
    RaiseStmt {
        raise_stmt_node: RaiseStmtNode,
    },
    WithStmt {
        with_stmt_node: WithStmtNode,
    },
    MatchStmt {
        match_stmt_node: MatchStmtNode,
    },
    // SuperStringStmt removed - backticks no longer supported
    BlockStmt {
        block_stmt_node: BlockStmtNode,
    },
    ReturnAssignStmt {
        return_assign_stmt_node: ReturnAssignStmtNode,
    },
    ReturnStmt {
        return_stmt_node: ReturnStmtNode,
    },
    ParentDispatchStmt {
        parent_dispatch_stmt_node: ParentDispatchStmtNode,
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
    pub line: usize,
    pub system_instance_expr_node: SystemInstanceExprNode,
}

impl SystemInstanceStmtNode {
    pub fn new(
        line: usize,
        system_instance_expr_node: SystemInstanceExprNode,
    ) -> SystemInstanceStmtNode {
        SystemInstanceStmtNode {
            line,
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
    pub line: usize,
    pub system_type_expr_node: SystemTypeExprNode,
}

impl SystemTypeStmtNode {
    pub fn new(line: usize, system_type_expr_node: SystemTypeExprNode) -> SystemTypeStmtNode {
        SystemTypeStmtNode {
            line,
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
    pub line: usize,
    pub call_expr_node: CallExprNode,
}

impl CallStmtNode {
    pub fn new(line: usize, call_expr_node: CallExprNode) -> CallStmtNode {
        CallStmtNode {
            line,
            call_expr_node,
        }
    }
}

impl NodeElement for CallStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_call_statement_node(self);
    }
}

//-----------------------------------------------------//

pub struct ActionCallStmtNode {
    pub line: usize,
    pub action_call_expr_node: ActionCallExprNode,
}

impl ActionCallStmtNode {
    pub fn new(line: usize, action_call_expr_node: ActionCallExprNode) -> ActionCallStmtNode {
        ActionCallStmtNode {
            line,
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

#[derive(Clone)]
pub struct EnumeratorStmtNode {
    pub line: usize,
    pub enumerator_expr_node: EnumeratorExprNode,
}

impl EnumeratorStmtNode {
    pub fn new(line: usize, enumerator_expr_node: EnumeratorExprNode) -> EnumeratorStmtNode {
        EnumeratorStmtNode {
            line,
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
    pub line: usize,
    pub call_chain_literal_expr_node: CallChainExprNode,
}

impl CallChainStmtNode {
    pub fn new(line: usize, call_chain_literal_expr_node: CallChainExprNode) -> CallChainStmtNode {
        CallChainStmtNode {
            line,
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
    pub line: usize,
    pub assignment_expr_node: AssignmentExprNode,
}

impl AssignmentStmtNode {
    pub fn new(line: usize, assignment_expr_node: AssignmentExprNode) -> AssignmentStmtNode {
        AssignmentStmtNode {
            line,
            assignment_expr_node,
        }
    }

    pub fn get_line(&self) -> usize {
        self.line
    }
}

impl NodeElement for AssignmentStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_assignment_statement_node(self);
    }
}

//-----------------------------------------------------//

pub struct AssignmentExprNode {
    pub line: usize,
    pub l_value_box: Box<ExprType>,
    pub r_value_rc: Rc<ExprType>,
    pub assignment_op: AssignmentOperator,
    //    pub is_decl: bool,
    // v0.52: Support for multiple assignment targets
    pub is_multiple_assignment: bool,
    pub l_values: Vec<ExprType>, // For x, y, z = ...
}

impl AssignmentExprNode {
    pub fn new(l_value: ExprType, r_value: Rc<ExprType>, line: usize) -> AssignmentExprNode {
        AssignmentExprNode {
            l_value_box: Box::new(l_value),
            r_value_rc: r_value.clone(),
            assignment_op: AssignmentOperator::Equals, // Default to simple assignment
            //            is_decl,
            line,
            is_multiple_assignment: false,
            l_values: Vec::new(),
        }
    }

    pub fn new_with_op(
        l_value: ExprType,
        r_value: Rc<ExprType>,
        op: AssignmentOperator,
        line: usize,
    ) -> AssignmentExprNode {
        AssignmentExprNode {
            l_value_box: Box::new(l_value),
            r_value_rc: r_value.clone(),
            assignment_op: op,
            line,
            is_multiple_assignment: false,
            l_values: Vec::new(),
        }
    }

    // v0.52: Constructor for multiple assignment
    pub fn new_multiple(
        l_values: Vec<ExprType>,
        r_value: Rc<ExprType>,
        line: usize,
    ) -> AssignmentExprNode {
        // For multiple assignment, l_value_box is not used (we use l_values instead)
        // Create a dummy for compatibility
        let dummy_l_value = ExprType::VariableExprT {
            var_node: VariableNode::new(
                line,
                IdentifierNode::new(
                    Token::new(
                        TokenType::Identifier,
                        "_multi".to_string(),
                        TokenLiteral::None,
                        line,
                        0,
                        6,
                    ),
                    None,
                    IdentifierDeclScope::UnknownScope,
                    false,
                    line,
                ),
                IdentifierDeclScope::UnknownScope,
                None,
            ),
        };

        AssignmentExprNode {
            l_value_box: Box::new(dummy_l_value),
            r_value_rc: r_value.clone(),
            assignment_op: AssignmentOperator::Equals,
            line,
            is_multiple_assignment: true,
            l_values,
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
    pub line: usize,
    pub var_node: VariableNode,
}

impl VariableStmtNode {
    pub fn new(line: usize, var_node: VariableNode) -> VariableStmtNode {
        VariableStmtNode { line, var_node }
    }

    pub fn get_line(&self) -> usize {
        self.line
    }
}

impl NodeElement for VariableStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_variable_stmt_node(self);
    }
}

//-----------------------------------------------------//

pub struct ListStmtNode {
    pub line: usize,
    pub list_node: ListNode,
}

impl ListStmtNode {
    pub fn new(line: usize, list_node: ListNode) -> ListStmtNode {
        ListStmtNode { line, list_node }
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
    pub line: usize,
    pub expr_list_node: ExprListNode,
}

impl ExprListStmtNode {
    pub fn new(line: usize, expr_list_node: ExprListNode) -> ExprListStmtNode {
        ExprListStmtNode {
            line,
            expr_list_node,
        }
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
    pub line: usize,
    pub binary_expr_node: BinaryExprNode,
}

impl BinaryStmtNode {
    pub fn new(line: usize, binary_expr_node: BinaryExprNode) -> BinaryStmtNode {
        BinaryStmtNode {
            line,
            binary_expr_node,
        }
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
    pub line: usize,
    pub condition: ExprType,
    pub if_block: BlockStmtNode,
    pub elif_clauses: Vec<ElifClause>,
    pub else_block: Option<BlockStmtNode>,
}

pub struct ElifClause {
    pub line: usize,
    pub condition: ExprType,
    pub block: BlockStmtNode,
}

impl IfStmtNode {
    pub fn new(
        line: usize,
        condition: ExprType,
        if_block: BlockStmtNode,
        elif_clauses: Vec<ElifClause>,
        else_block: Option<BlockStmtNode>,
    ) -> IfStmtNode {
        IfStmtNode {
            line,
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
    pub line: usize,
    pub variable: Option<VariableNode>,     // for var x in items
    pub identifier: Option<IdentifierNode>, // for x in items
    pub iterable: ExprType,
    pub block: BlockStmtNode,
    pub else_block: Option<BlockStmtNode>, // v0.51: else clause for loops
    pub is_enum_iteration: bool,           // v0.32: Track if iterating over enum
    pub enum_type_name: Option<String>,    // v0.32: Name of enum being iterated
}

impl ForStmtNode {
    pub fn new(
        line: usize,
        variable: Option<VariableNode>,
        identifier: Option<IdentifierNode>,
        iterable: ExprType,
        block: BlockStmtNode,
    ) -> ForStmtNode {
        ForStmtNode {
            line,
            variable,
            identifier,
            iterable,
            block,
            else_block: None,
            is_enum_iteration: false,
            enum_type_name: None,
        }
    }

    pub fn with_else(
        line: usize,
        variable: Option<VariableNode>,
        identifier: Option<IdentifierNode>,
        iterable: ExprType,
        block: BlockStmtNode,
        else_block: BlockStmtNode,
    ) -> ForStmtNode {
        ForStmtNode {
            line,
            variable,
            identifier,
            iterable,
            block,
            else_block: Some(else_block),
            is_enum_iteration: false,
            enum_type_name: None,
        }
    }

    pub fn new_enum_iteration(
        line: usize,
        variable: Option<VariableNode>,
        identifier: Option<IdentifierNode>,
        iterable: ExprType,
        block: BlockStmtNode,
        enum_type_name: String,
    ) -> ForStmtNode {
        ForStmtNode {
            line,
            variable,
            identifier,
            iterable,
            block,
            else_block: None,
            is_enum_iteration: true,
            enum_type_name: Some(enum_type_name),
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
    pub line: usize,
    pub condition: ExprType,
    pub block: BlockStmtNode,
    pub else_block: Option<BlockStmtNode>, // v0.51: else clause for loops
}

impl WhileStmtNode {
    pub fn new(line: usize, condition: ExprType, block: BlockStmtNode) -> WhileStmtNode {
        WhileStmtNode {
            line,
            condition,
            block,
            else_block: None,
        }
    }

    pub fn with_else(
        line: usize,
        condition: ExprType,
        block: BlockStmtNode,
        else_block: BlockStmtNode,
    ) -> WhileStmtNode {
        WhileStmtNode {
            line,
            condition,
            block,
            else_block: Some(else_block),
        }
    }
}

impl NodeElement for WhileStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_while_stmt_node(self);
    }
}

//-----------------------------------------------------//

pub struct LoopStmtNode {
    pub line: usize,
    pub loop_types: LoopStmtTypes,
}

impl LoopStmtNode {
    pub fn new(line: usize, loop_types: LoopStmtTypes) -> LoopStmtNode {
        LoopStmtNode { line, loop_types }
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
    StateStackPush {},
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
    pub line: usize,
    pub target_state_context_t: TargetStateContextType,
    pub label_opt: Option<String>,
    pub forward_event: bool,
}

impl TransitionExprNode {
    pub fn new(
        line: usize,
        target_state_context_t: TargetStateContextType,
        label_opt: Option<String>,
        forward_event: bool,
    ) -> TransitionExprNode {
        TransitionExprNode {
            line,
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
    pub line: usize,
    pub transition_expr_node: TransitionExprNode,
    pub exit_args_opt: Option<ExprListNode>,
}

// TODO - why is new() commented out?
impl TransitionStatementNode {
    pub fn new(
        line: usize,
        transition_expr_node: TransitionExprNode,
        exit_args_opt: Option<ExprListNode>,
    ) -> TransitionStatementNode {
        TransitionStatementNode {
            line,
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
#[derive(Clone)]
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

#[derive(PartialEq, Clone)]
pub enum CallOrigin {
    External,
    Internal,
}

pub struct InterfaceMethodCallExprNode {
    pub line: usize,
    pub identifier: IdentifierNode,
    pub call_expr_list: CallExprListNode,
    pub call_origin: CallOrigin,
    pub interface_symbol_rcref_opt: Option<Rc<RefCell<InterfaceMethodSymbol>>>,
}

impl InterfaceMethodCallExprNode {
    // Harvest the id and arguments from the CallExpressionNode.
    // It will be discarded.

    pub fn new(
        line: usize,
        call_expr_node: CallExprNode,
        interface_method_call_t: CallOrigin,
    ) -> InterfaceMethodCallExprNode {
        InterfaceMethodCallExprNode {
            line,
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
    pub line: usize,
    pub identifier: IdentifierNode,
    pub call_expr_list: CallExprListNode,
    pub action_symbol_rcref_opt: Option<Rc<RefCell<ActionScopeSymbol>>>,
}

impl ActionCallExprNode {
    // Harvest the id and arguments from the CallExpressionNode.
    // It will be discarded.

    pub fn new(line: usize, call_expr_node: CallExprNode) -> ActionCallExprNode {
        ActionCallExprNode {
            line,
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
    pub line: usize,
    pub identifier: IdentifierNode,
    pub call_expr_list: CallExprListNode,
    pub operation_symbol_rcref_opt: Option<Rc<RefCell<OperationScopeSymbol>>>,
}

impl OperationCallExprNode {
    // Harvest the id and arguments from the CallExpressionNode.
    // It will be discarded.

    pub fn new(line: usize, call_expr_node: CallExprNode) -> OperationCallExprNode {
        OperationCallExprNode {
            line,
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
    pub line: usize,
    pub name: String,
}

impl OperationRefExprNode {
    pub fn new(line: usize, name: String) -> OperationRefExprNode {
        OperationRefExprNode { line, name }
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
    pub line: usize,
    pub statements: Vec<DeclOrStmtType>,
}

impl LoopInfiniteStmtNode {
    pub fn new(line: usize, statements: Vec<DeclOrStmtType>) -> LoopInfiniteStmtNode {
        LoopInfiniteStmtNode { line, statements }
    }
}

impl NodeElement for LoopInfiniteStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_loop_infinite_stmt_node(self);
    }
}

//-----------------------------------------------------//

pub struct LoopInStmtNode {
    pub line: usize,
    pub loop_first_stmt: LoopFirstStmt,
    pub iterable_expr: Box<ExprType>,
    pub statements: Vec<DeclOrStmtType>,
}

impl LoopInStmtNode {
    pub fn new(
        line: usize,
        loop_first_stmt: LoopFirstStmt,
        iterable_expr: Box<ExprType>,
        statements: Vec<DeclOrStmtType>,
    ) -> LoopInStmtNode {
        LoopInStmtNode {
            line,
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
    pub line: usize,
    pub loop_init_expr_rcref_opt: Option<Rc<RefCell<LoopFirstStmt>>>,
    pub test_expr_rcref_opt: Option<Rc<RefCell<ExprType>>>,
    pub post_expr_rcref_opt: Option<Rc<RefCell<ExprType>>>,
    pub statements: Vec<DeclOrStmtType>,
}

impl LoopForStmtNode {
    pub fn new(
        line: usize,
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
            line,
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
    pub line: usize, // v0.78.10: source map support for block start
    pub statements: Vec<DeclOrStmtType>,
}

impl BlockStmtNode {
    pub fn new(line: usize, statements: Vec<DeclOrStmtType>) -> BlockStmtNode {
        BlockStmtNode { line, statements }
    }
}

impl NodeElement for BlockStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_block_stmt_node(self);
    }
}

//-----------------------------------------------------//

#[derive(Clone)]
pub struct ContinueStmtNode {
    pub line: usize,
}

impl ContinueStmtNode {
    pub fn new(line: usize) -> ContinueStmtNode {
        ContinueStmtNode { line }
    }
}

impl NodeElement for ContinueStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_continue_stmt_node(self);
    }
}

//-----------------------------------------------------//

#[derive(Clone)]
pub struct BreakStmtNode {
    pub line: usize,
}

impl BreakStmtNode {
    pub fn new(line: usize) -> BreakStmtNode {
        BreakStmtNode { line }
    }
}

impl NodeElement for BreakStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_break_stmt_node(self);
    }
}

//-----------------------------------------------------//

// v0.50: Del statement support
pub struct DelStmtNode {
    pub line: usize,
    pub target: ExprType, // The expression to delete (e.g., list[i], dict[key], var)
}

impl DelStmtNode {
    pub fn new(line: usize, target: ExprType) -> DelStmtNode {
        DelStmtNode { line, target }
    }
}

impl NodeElement for DelStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_del_stmt_node(self);
    }
}

// v0.46: Assert statement support
pub struct AssertStmtNode {
    pub line: usize,
    pub expr: ExprType,
}

impl AssertStmtNode {
    pub fn new(line: usize, expr: ExprType) -> AssertStmtNode {
        AssertStmtNode { line, expr }
    }
}

impl NodeElement for AssertStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_assert_stmt_node(self);
    }
}

//-----------------------------------------------------//

pub struct TryStmtNode {
    pub line: usize,
    pub try_block: BlockStmtNode,
    pub except_clauses: Vec<ExceptClauseNode>,
    pub else_block: Option<BlockStmtNode>,
    pub finally_block: Option<BlockStmtNode>,
}

impl TryStmtNode {
    pub fn new(
        line: usize,
        try_block: BlockStmtNode,
        except_clauses: Vec<ExceptClauseNode>,
        else_block: Option<BlockStmtNode>,
        finally_block: Option<BlockStmtNode>,
    ) -> TryStmtNode {
        TryStmtNode {
            line,
            try_block,
            except_clauses,
            else_block,
            finally_block,
        }
    }
}

impl NodeElement for TryStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_try_stmt_node(self);
    }
}

//-----------------------------------------------------//

pub struct ExceptClauseNode {
    pub line: usize,                          // v0.78.7: source map support
    pub exception_types: Option<Vec<String>>, // None means catch all
    pub var_name: Option<String>,             // Variable to bind exception to
    pub block: BlockStmtNode,
}

impl ExceptClauseNode {
    pub fn new(
        line: usize,
        exception_types: Option<Vec<String>>,
        var_name: Option<String>,
        block: BlockStmtNode,
    ) -> ExceptClauseNode {
        ExceptClauseNode {
            line,
            exception_types,
            var_name,
            block,
        }
    }
}

impl NodeElement for ExceptClauseNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_except_clause_node(self);
    }
}

//-----------------------------------------------------//

pub struct RaiseStmtNode {
    pub line: usize,
    pub exception_expr: Option<ExprType>, // What to raise
    pub from_expr: Option<ExprType>,      // Optional 'from' expression for chaining
}

impl RaiseStmtNode {
    pub fn new(
        line: usize,
        exception_expr: Option<ExprType>,
        from_expr: Option<ExprType>,
    ) -> RaiseStmtNode {
        RaiseStmtNode {
            line,
            exception_expr,
            from_expr,
        }
    }
}

impl NodeElement for RaiseStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_raise_stmt_node(self);
    }
}

//-----------------------------------------------------//

pub struct WithStmtNode {
    pub line: usize,
    pub is_async: bool,
    pub context_expr: ExprType,
    pub target_var: Option<String>,
    pub with_block: BlockStmtNode,
}

impl WithStmtNode {
    pub fn new(
        line: usize,
        is_async: bool,
        context_expr: ExprType,
        target_var: Option<String>,
        with_block: BlockStmtNode,
    ) -> WithStmtNode {
        WithStmtNode {
            line,
            is_async,
            context_expr,
            target_var,
            with_block,
        }
    }
}

impl NodeElement for WithStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_with_stmt_node(self);
    }
}

//-----------------------------------------------------//
// Match statement node (v0.44)
pub struct MatchStmtNode {
    pub line: usize,
    pub match_expr: ExprType,
    pub cases: Vec<CaseNode>,
}

impl MatchStmtNode {
    pub fn new(line: usize, match_expr: ExprType, cases: Vec<CaseNode>) -> MatchStmtNode {
        MatchStmtNode {
            line,
            match_expr,
            cases,
        }
    }
}

impl NodeElement for MatchStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_match_stmt_node(self);
    }
}

// Case node for match statement (v0.44)
pub struct CaseNode {
    pub line: usize, // v0.78.7: source map support
    pub pattern: PatternNode,
    pub guard: Option<ExprType>, // Optional guard clause
    pub statements: Vec<DeclOrStmtType>,
}

impl CaseNode {
    pub fn new(
        line: usize,
        pattern: PatternNode,
        guard: Option<ExprType>,
        statements: Vec<DeclOrStmtType>,
    ) -> CaseNode {
        CaseNode {
            line,
            pattern,
            guard,
            statements,
        }
    }
}

impl NodeElement for CaseNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_case_node(self);
    }
}

// Pattern types for match-case (v0.44)
pub enum PatternNode {
    Literal(LiteralExprNode),            // Literal pattern: 42, "hello", True
    Capture(String),                     // Capture pattern: x (binds to variable)
    Wildcard,                            // Wildcard pattern: _
    Sequence(Vec<PatternNode>),          // List/tuple pattern: [a, b, c]
    Mapping(Vec<(String, PatternNode)>), // Dict pattern: {"key": value}
    Class(String, Vec<PatternNode>),     // Class pattern: Point(x, y)
    Or(Vec<PatternNode>),                // Or pattern: pattern1 | pattern2
    As(Box<PatternNode>, String),        // As pattern: pattern as name
    Star(String),                        // Star pattern: *rest (captures remaining elements)
}

impl NodeElement for PatternNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_pattern_node(self);
    }
}

//-----------------------------------------------------//

// SuperStringStmtNode removed - backticks no longer supported

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
    pub line: usize,
    pub call_chain: VecDeque<CallChainNodeType>,
    pub is_new_expr: bool,
    pub inc_dec: IncDecExpr,
}

impl CallChainExprNode {
    pub fn new(line: usize, call_chain: VecDeque<CallChainNodeType>) -> CallChainExprNode {
        CallChainExprNode {
            line,
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
                CallChainNodeType::SelfT { .. } => {
                    output.push_str("self");
                }
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
                CallChainNodeType::SliceNodeT { .. } => {
                    // Slice nodes will be handled similarly to list elements
                }
                CallChainNodeType::UndeclaredSliceT { .. } => {
                    // Undeclared slice nodes similarly
                }
                CallChainNodeType::CallChainLiteralExprT {
                    call_chain_literal_expr_node,
                } => match &call_chain_literal_expr_node.token_t {
                    TokenType::String => {
                        output.push_str(&format!("\"{}\"", call_chain_literal_expr_node.value))
                    }
                    TokenType::FString => output.push_str(&call_chain_literal_expr_node.value),
                    _ => output.push_str(&call_chain_literal_expr_node.value),
                },
            }
            separator = ".";
        }
        write!(f, "{}", output)
    }
}
//-----------------------------------------------------//
#[derive(PartialEq, Clone)]
pub enum OperatorType {
    Plus,
    Minus,
    Multiply,
    Divide,
    FloorDivide, // Floor division operator //
    Power,       // For exponent operator **
    Greater,
    GreaterEqual,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Not,
    LogicalAnd,
    LogicalOr,
    Negated,
    Percent,
    BitwiseOr,  // For dict union operator |
    BitwiseAnd, // Bitwise AND &
    BitwiseXor, // Bitwise XOR ^
    BitwiseNot, // Bitwise NOT ~
    LeftShift,  // Left shift <<
    RightShift, // Right shift >>
    MatMul,     // Matrix multiplication @ (v0.40)
    In,         // For membership test operator 'in'
    NotIn,      // For negated membership test 'not in'
    Is,         // Identity operator 'is'
    IsNot,      // Identity operator 'is not'
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
            TokenType::Plus => OperatorType::Plus,
            TokenType::Dash => OperatorType::Minus,
            TokenType::Star => OperatorType::Multiply,
            TokenType::StarStar => OperatorType::Power, // Exponent operator **
            TokenType::ForwardSlash => OperatorType::Divide,
            TokenType::FloorDivide => OperatorType::FloorDivide, // Floor division //
            TokenType::GT => OperatorType::Greater,
            TokenType::GreaterEqual => OperatorType::GreaterEqual,
            TokenType::LT => OperatorType::Less,
            TokenType::LessEqual => OperatorType::LessEqual,
            TokenType::Not => OperatorType::Not, // Python 'not' keyword only
            TokenType::EqualEqual => OperatorType::EqualEqual,
            TokenType::BangEqual => OperatorType::NotEqual,
            TokenType::And => OperatorType::LogicalAnd, // Python 'and' keyword only
            TokenType::Or => OperatorType::LogicalOr,   // Python 'or' keyword only
            TokenType::Percent => OperatorType::Percent,
            TokenType::Pipe => OperatorType::BitwiseOr, // Bitwise OR |
            TokenType::Ampersand => OperatorType::BitwiseAnd, // Bitwise AND &
            TokenType::Caret => OperatorType::BitwiseXor, // Bitwise XOR ^
            TokenType::Tilde => OperatorType::BitwiseNot, // Bitwise NOT ~
            TokenType::LeftShift => OperatorType::LeftShift, // Left shift <<
            TokenType::RightShift => OperatorType::RightShift, // Right shift >>
            TokenType::At => OperatorType::MatMul,      // Matrix multiplication @
            TokenType::In => OperatorType::In,          // Membership test operator
            TokenType::Is => OperatorType::Is,          // Identity operator 'is'
            _ => OperatorType::Unknown,
        }
    }
}

//-----------------------------------------------------//

pub struct UnaryExprNode {
    pub line: usize,
    pub operator: OperatorType,
    pub right_rcref: Rc<RefCell<ExprType>>,
}

impl UnaryExprNode {
    pub fn new(line: usize, operator: OperatorType, right: ExprType) -> UnaryExprNode {
        UnaryExprNode {
            line,
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
    pub line: usize,
    pub left_rcref: Rc<RefCell<ExprType>>,
    pub operator: OperatorType,
    pub right_rcref: Rc<RefCell<ExprType>>,
}

impl BinaryExprNode {
    pub fn new(
        line: usize,
        left: ExprType,
        operator: OperatorType,
        right: ExprType,
    ) -> BinaryExprNode {
        BinaryExprNode {
            line,
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
    pub line: usize,
    pub identifier: IdentifierNode,
    pub call_expr_list: CallExprListNode,
    pub call_chain: Option<Vec<Box<dyn CallableExpr>>>,
    pub context: CallContextType, // NEW: explicit self/system context
    pub resolved_type: Option<ResolvedCallType>, // v0.62: Semantic resolution (gradual migration)
}

impl CallExprNode {
    pub fn new(
        line: usize,
        identifier: IdentifierNode,
        call_expr_list: CallExprListNode,
        call_chain: Option<Vec<Box<dyn CallableExpr>>>,
    ) -> CallExprNode {
        CallExprNode {
            line,
            identifier,
            call_expr_list,
            call_chain,
            context: CallContextType::ExternalCall, // Default to external
            resolved_type: None,                    // Will be set by semantic analysis
        }
    }

    pub fn new_with_context(
        line: usize,
        identifier: IdentifierNode,
        call_expr_list: CallExprListNode,
        call_chain: Option<Vec<Box<dyn CallableExpr>>>,
        context: CallContextType,
    ) -> CallExprNode {
        CallExprNode {
            line,
            identifier,
            call_expr_list,
            call_chain,
            context,
            resolved_type: None, // Will be set by semantic analysis
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
    pub line: usize,
    pub exprs_t: Vec<ExprType>,
}

impl CallExprListNode {
    pub fn new(exprs_t: Vec<ExprType>) -> CallExprListNode {
        CallExprListNode { line: 0, exprs_t }
    }

    pub fn new_with_line(line: usize, exprs_t: Vec<ExprType>) -> CallExprListNode {
        CallExprListNode { line, exprs_t }
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
    pub line: usize,
    pub exprs_t: Vec<ExprType>,
    //    pub inc_dec: IncDecExpr,
}

impl ExprListNode {
    pub fn new(exprs_t: Vec<ExprType>) -> ExprListNode {
        ExprListNode {
            line: 0,
            exprs_t,
            //            inc_dec: IncDecExpr::None,
        }
    }

    pub fn new_with_line(line: usize, exprs_t: Vec<ExprType>) -> ExprListNode {
        ExprListNode {
            line,
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
#[derive(Clone, PartialEq, Debug)]
pub enum IdentifierDeclScope {
    ModuleScope,  // v0.31: Module-level variables
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
    ClassStaticScope,   // v0.45: Class static variables
    ClassInstanceScope, // v0.45: Class instance variables
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
    pub line: usize,
    pub token_t: TokenType,
    pub value: String,
    pub is_reference: bool,
    pub inc_dec: IncDecExpr,
}

impl LiteralExprNode {
    pub fn new(line: usize, token_t: TokenType, value: String) -> LiteralExprNode {
        LiteralExprNode {
            line,
            token_t,
            value,
            is_reference: false,
            inc_dec: IncDecExpr::None,
        }
    }
}

// Node for literal expressions in call chains (e.g., "string".upper())
pub struct CallChainLiteralExprNode {
    pub line: usize,
    pub token_t: TokenType,
    pub value: String,
}

impl CallChainLiteralExprNode {
    pub fn new(line: usize, token_t: TokenType, value: String) -> CallChainLiteralExprNode {
        CallChainLiteralExprNode {
            line,
            token_t,
            value,
        }
    }
}

impl NodeElement for CallChainLiteralExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        // For now, treat it like a regular literal
        ast_visitor.visit_literal_expression_node(&LiteralExprNode {
            line: 0,
            token_t: self.token_t.clone(),
            value: self.value.clone(),
            is_reference: false,
            inc_dec: IncDecExpr::None,
        });
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_literal_expression_node_to_string(
            &LiteralExprNode {
                line: 0,
                token_t: self.token_t.clone(),
                value: self.value.clone(),
                is_reference: false,
                inc_dec: IncDecExpr::None,
            },
            output,
        );
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
    pub line: usize, // v0.78.11: source map support
    pub operation_t: StateStackOperationType,
}

impl StateStackOperationNode {
    pub fn new(line: usize, operation_t: StateStackOperationType) -> StateStackOperationNode {
        StateStackOperationNode { line, operation_t }
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
    pub line: usize,
    pub identifier: IdentifierNode,
    pub start_state_state_args_opt: Option<ExprListNode>,
    pub start_state_enter_args_opt: Option<ExprListNode>,
    pub domain_args_opt: Option<ExprListNode>,
}

impl SystemInstanceExprNode {
    pub fn new(
        line: usize,
        identifier: IdentifierNode,
        start_state_state_args_opt: Option<ExprListNode>,
        start_state_enter_args_opt: Option<ExprListNode>,
        domain_args_opt: Option<ExprListNode>,
    ) -> SystemInstanceExprNode {
        SystemInstanceExprNode {
            line,
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
    pub line: usize,
}

impl SelfExprNode {
    pub fn new(line: usize) -> SelfExprNode {
        SelfExprNode { line }
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
    pub line: usize,
    pub identifier: IdentifierNode,
    pub call_chain_opt: Box<Option<ExprType>>,
}

impl SystemTypeExprNode {
    pub fn new(
        line: usize,
        identifier: IdentifierNode,
        call_chain_opt: Box<Option<ExprType>>,
    ) -> SystemTypeExprNode {
        SystemTypeExprNode {
            line,
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
    pub line: usize,
    pub exprs_t: Vec<ExprType>,
    //    pub inc_dec: IncDecExpr,
}

impl ListNode {
    pub fn new(line: usize, exprs_t: Vec<ExprType>) -> ListNode {
        ListNode { line, exprs_t }
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

// Dictionary literal node for {key: value, ...} syntax
pub struct DictLiteralNode {
    pub line: usize,
    pub pairs: Vec<(ExprType, ExprType)>, // (key, value) pairs
}

impl DictLiteralNode {
    pub fn new(line: usize, pairs: Vec<(ExprType, ExprType)>) -> DictLiteralNode {
        DictLiteralNode { line, pairs }
    }
}

impl NodeElement for DictLiteralNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_dict_literal_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_dict_literal_node_to_string(self, output);
    }
}

//-----------------------------------------------------//

// Set literal node for {1, 2, 3} syntax
pub struct SetLiteralNode {
    pub line: usize,
    pub elements: Vec<ExprType>,
}

impl SetLiteralNode {
    pub fn new(line: usize, elements: Vec<ExprType>) -> SetLiteralNode {
        SetLiteralNode { line, elements }
    }
}

impl NodeElement for SetLiteralNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_set_literal_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_set_literal_node_to_string(self, output);
    }
}

//-----------------------------------------------------//

// Tuple literal node for (1, 2, 3) syntax
pub struct TupleLiteralNode {
    pub line: usize,
    pub elements: Vec<ExprType>,
}

impl TupleLiteralNode {
    pub fn new(line: usize, elements: Vec<ExprType>) -> TupleLiteralNode {
        TupleLiteralNode { line, elements }
    }
}

impl NodeElement for TupleLiteralNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_tuple_literal_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_tuple_literal_node_to_string(self, output);
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

// SliceNode captures slice operations such as
// text[1:5], array[:10], list[5:], data[::2]
pub struct SliceNode {
    pub line: usize,
    pub identifier: IdentifierNode,
    pub scope: IdentifierDeclScope,
    pub start_expr: Option<Box<ExprType>>,
    pub end_expr: Option<Box<ExprType>>,
    pub step_expr: Option<Box<ExprType>>,
}

impl SliceNode {
    pub fn new(
        line: usize,
        identifier: IdentifierNode,
        scope: IdentifierDeclScope,
        start_expr: Option<Box<ExprType>>,
        end_expr: Option<Box<ExprType>>,
        step_expr: Option<Box<ExprType>>,
    ) -> SliceNode {
        SliceNode {
            line,
            identifier,
            scope,
            start_expr,
            end_expr,
            step_expr,
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

impl NodeElement for SliceNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_slice_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_slice_node_to_string(self, output);
    }
}

//-----------------------------------------------------//

// UnpackExprNode for *args unpacking (v0.34)
pub struct UnpackExprNode {
    pub line: usize,
    pub expr: Box<ExprType>,
}

impl UnpackExprNode {
    pub fn new(line: usize, expr: ExprType) -> UnpackExprNode {
        UnpackExprNode {
            line,
            expr: Box::new(expr),
        }
    }
}

impl NodeElement for UnpackExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_unpack_expr_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_unpack_expr_node_to_string(self, output);
    }
}

//-----------------------------------------------------//

// StarExprNode for *var unpacking (v0.54)
pub struct StarExprNode {
    pub line: usize,
    pub identifier: String,
}

impl StarExprNode {
    pub fn new(line: usize, identifier: String) -> StarExprNode {
        StarExprNode { line, identifier }
    }
}

impl NodeElement for StarExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_star_expr_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_star_expr_node_to_string(self, output);
    }
}

//-----------------------------------------------------//

// DictUnpackExprNode for **kwargs unpacking (v0.38)
pub struct DictUnpackExprNode {
    pub line: usize,
    pub expr: Box<ExprType>,
}

impl DictUnpackExprNode {
    pub fn new(line: usize, expr: ExprType) -> DictUnpackExprNode {
        DictUnpackExprNode {
            line,
            expr: Box::new(expr),
        }
    }
}

impl NodeElement for DictUnpackExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_dict_unpack_expr_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_dict_unpack_expr_node_to_string(self, output);
    }
}

//-----------------------------------------------------//

// AwaitExprNode for await expressions (v0.35)
pub struct AwaitExprNode {
    pub line: usize,
    pub expr: Box<ExprType>,
}

impl AwaitExprNode {
    pub fn new(line: usize, expr: ExprType) -> AwaitExprNode {
        AwaitExprNode {
            line,
            expr: Box::new(expr),
        }
    }
}

// YieldExprNode for yield expressions (v0.42)
pub struct YieldExprNode {
    pub line: usize,
    pub expr: Option<Box<ExprType>>, // yield can be used without value
}

impl YieldExprNode {
    pub fn new(line: usize, expr: Option<ExprType>) -> YieldExprNode {
        YieldExprNode {
            line,
            expr: expr.map(Box::new),
        }
    }
}

// YieldFromExprNode for yield from expressions (v0.42)
pub struct YieldFromExprNode {
    pub line: usize,
    pub expr: Box<ExprType>, // yield from requires an iterable
}

impl YieldFromExprNode {
    pub fn new(line: usize, expr: ExprType) -> YieldFromExprNode {
        YieldFromExprNode {
            line,
            expr: Box::new(expr),
        }
    }
}

// GeneratorExprNode for generator expressions (v0.42)
pub struct GeneratorExprNode {
    pub line: usize,
    pub expr: Box<ExprType>,
    pub target: String,
    pub iter: Box<ExprType>,
    pub condition: Option<Box<ExprType>>,
}

impl GeneratorExprNode {
    pub fn new(
        line: usize,
        expr: ExprType,
        target: String,
        iter: ExprType,
        condition: Option<ExprType>,
    ) -> GeneratorExprNode {
        GeneratorExprNode {
            line,
            expr: Box::new(expr),
            target,
            iter: Box::new(iter),
            condition: condition.map(Box::new),
        }
    }
}

impl NodeElement for AwaitExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_await_expr_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_await_expr_node_to_string(self, output);
    }
}

impl NodeElement for YieldExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_yield_expr_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_yield_expr_node_to_string(self, output);
    }
}

impl NodeElement for YieldFromExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_yield_from_expr_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_yield_from_expr_node_to_string(self, output);
    }
}

impl NodeElement for GeneratorExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_generator_expr_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_generator_expr_node_to_string(self, output);
    }
}

//-----------------------------------------------------//

// LambdaExprNode for lambda expressions (v0.38)
pub struct LambdaExprNode {
    pub line: usize,
    pub params: Vec<String>, // Parameter names
    pub body: Box<ExprType>, // Lambda body expression
}

impl LambdaExprNode {
    pub fn new(line: usize, params: Vec<String>, body: ExprType) -> LambdaExprNode {
        LambdaExprNode {
            line,
            params,
            body: Box::new(body),
        }
    }
}

impl NodeElement for LambdaExprNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_lambda_expr_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_lambda_expr_node_to_string(self, output);
    }
}

//-----------------------------------------------------//

// ListComprehensionNode for [expr for var in iterable if condition] (v0.34)
pub struct ListComprehensionNode {
    pub line: usize,
    pub expr: Box<ExprType>,              // The expression to evaluate
    pub target: String,                   // Loop variable name
    pub iter: Box<ExprType>,              // The iterable to loop over
    pub condition: Option<Box<ExprType>>, // Optional filter condition
}

impl ListComprehensionNode {
    pub fn new(
        line: usize,
        expr: ExprType,
        target: String,
        iter: ExprType,
        condition: Option<ExprType>,
    ) -> ListComprehensionNode {
        ListComprehensionNode {
            line,
            expr: Box::new(expr),
            target,
            iter: Box::new(iter),
            condition: condition.map(Box::new),
        }
    }
}

impl NodeElement for ListComprehensionNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_list_comprehension_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_list_comprehension_node_to_string(self, output);
    }
}

//-----------------------------------------------------//

// DictComprehensionNode for {key: value for var in iterable if condition} (v0.38)
pub struct DictComprehensionNode {
    pub line: usize,
    pub key_expr: Box<ExprType>,   // The key expression to evaluate
    pub value_expr: Box<ExprType>, // The value expression to evaluate
    pub target: String,            // Loop variable name
    pub iter: Box<ExprType>,       // The iterable to loop over
    pub condition: Option<Box<ExprType>>, // Optional filter condition
}

impl DictComprehensionNode {
    pub fn new(
        line: usize,
        key_expr: ExprType,
        value_expr: ExprType,
        target: String,
        iter: ExprType,
        condition: Option<ExprType>,
    ) -> DictComprehensionNode {
        DictComprehensionNode {
            line,
            key_expr: Box::new(key_expr),
            value_expr: Box::new(value_expr),
            target,
            iter: Box::new(iter),
            condition: condition.map(Box::new),
        }
    }
}

impl NodeElement for DictComprehensionNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_dict_comprehension_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_dict_comprehension_node_to_string(self, output);
    }
}

//-----------------------------------------------------//

// SetComprehensionNode for {expr for var in iterable if condition} (v0.41)
pub struct SetComprehensionNode {
    pub line: usize,
    pub expr: Box<ExprType>,              // The expression to evaluate
    pub target: String,                   // Loop variable name
    pub iter: Box<ExprType>,              // The iterable to loop over
    pub condition: Option<Box<ExprType>>, // Optional filter condition
}

impl SetComprehensionNode {
    pub fn new(
        line: usize,
        expr: ExprType,
        target: String,
        iter: ExprType,
        condition: Option<ExprType>,
    ) -> SetComprehensionNode {
        SetComprehensionNode {
            line,
            expr: Box::new(expr),
            target,
            iter: Box::new(iter),
            condition: condition.map(Box::new),
        }
    }
}

impl NodeElement for SetComprehensionNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_set_comprehension_node(self);
    }

    fn accept_to_string(&self, ast_visitor: &mut dyn AstVisitor, output: &mut String) {
        ast_visitor.visit_set_comprehension_node_to_string(self, output);
    }
}

//-----------------------------------------------------//

// ReturnAssignStmtNode captures "return = expr" statements (v0.20 syntax).

pub struct ReturnAssignStmtNode {
    pub line: usize,
    pub expr_t: ExprType,
}

impl ReturnAssignStmtNode {
    pub fn new(line: usize, expr_t: ExprType) -> ReturnAssignStmtNode {
        ReturnAssignStmtNode { line, expr_t }
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
    pub line: usize,
    pub expr_t_opt: Option<ExprType>,
}

impl ReturnStmtNode {
    pub fn new(line: usize, expr_t_opt: Option<ExprType>) -> ReturnStmtNode {
        ReturnStmtNode { line, expr_t_opt }
    }
}

impl NodeElement for ReturnStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_return_stmt_node(self);
    }
}

//-----------------------------------------------------//

#[derive(Clone)]
pub struct ParentDispatchStmtNode {
    pub target_state_ref_opt: Option<StateRefNode>,
    pub line: usize,
}

impl ParentDispatchStmtNode {
    pub fn new(target_state_ref_opt: Option<StateRefNode>, line: usize) -> ParentDispatchStmtNode {
        ParentDispatchStmtNode {
            target_state_ref_opt,
            line,
        }
    }
}

impl NodeElement for ParentDispatchStmtNode {
    fn accept(&self, ast_visitor: &mut dyn AstVisitor) {
        ast_visitor.visit_parent_dispatch_stmt_node(self);
    }
}

//-----------------------------------------------------//
// v0.37: Runtime infrastructure nodes for async chain analysis

pub struct RuntimeInfo {
    pub kernel: KernelNode,
    pub router: RouterNode,
    pub transitions: Vec<TransitionNode>,
    pub state_dispatchers: Vec<StateDispatcherNode>,
}

impl RuntimeInfo {
    pub fn new() -> RuntimeInfo {
        RuntimeInfo {
            kernel: KernelNode::new(false),
            router: RouterNode::new(false),
            transitions: Vec::new(),
            state_dispatchers: Vec::new(),
        }
    }
}

pub struct KernelNode {
    pub is_async: bool,
    pub system_ref: String,
}

impl KernelNode {
    pub fn new(is_async: bool) -> KernelNode {
        KernelNode {
            is_async,
            system_ref: String::new(),
        }
    }
}

pub struct RouterNode {
    pub is_async: bool,
    pub system_ref: String,
}

impl RouterNode {
    pub fn new(is_async: bool) -> RouterNode {
        RouterNode {
            is_async,
            system_ref: String::new(),
        }
    }
}

pub struct TransitionNode {
    pub from_state: String,
    pub to_state: String,
    pub is_async: bool,
    pub handler_name: String, // Which handler triggers this transition
}

impl TransitionNode {
    pub fn new(
        from_state: String,
        to_state: String,
        is_async: bool,
        handler_name: String,
    ) -> TransitionNode {
        TransitionNode {
            from_state,
            to_state,
            is_async,
            handler_name,
        }
    }
}

pub struct StateDispatcherNode {
    pub state_name: String,
    pub is_async: bool,
}

impl StateDispatcherNode {
    pub fn new(state_name: String, is_async: bool) -> StateDispatcherNode {
        StateDispatcherNode {
            state_name,
            is_async,
        }
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
