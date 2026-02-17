//! Codegen AST - Language-agnostic intermediate representation
//!
//! This module defines the CodegenNode enum, which represents a language-agnostic
//! intermediate representation for code generation. Frame AST is transformed into
//! CodegenNode, which is then emitted by language-specific backends.

use crate::frame_c::v4::frame_ast::Span;

/// Parameter in a method or function
#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub type_annotation: Option<String>,
    pub default_value: Option<Box<CodegenNode>>,
}

impl Param {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            type_annotation: None,
            default_value: None,
        }
    }

    pub fn with_type(mut self, type_annotation: &str) -> Self {
        self.type_annotation = Some(type_annotation.to_string());
        self
    }

    pub fn with_default(mut self, default: CodegenNode) -> Self {
        self.default_value = Some(Box::new(default));
        self
    }
}

/// Field in a class or struct
#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub type_annotation: Option<String>,
    pub visibility: Visibility,
    pub is_static: bool,
    pub initializer: Option<Box<CodegenNode>>,
}

impl Field {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            type_annotation: None,
            visibility: Visibility::Private,
            is_static: false,
            initializer: None,
        }
    }

    pub fn with_type(mut self, type_annotation: &str) -> Self {
        self.type_annotation = Some(type_annotation.to_string());
        self
    }

    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }

    pub fn with_initializer(mut self, init: CodegenNode) -> Self {
        self.initializer = Some(Box::new(init));
        self
    }
}

/// Visibility modifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

/// Match arm for pattern matching
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Box<CodegenNode>,
    pub guard: Option<Box<CodegenNode>>,
    pub body: Vec<CodegenNode>,
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // Comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    // Logical
    And,
    Or,
    // Bitwise
    BitAnd,
    BitOr,
    BitXor,
}

impl BinaryOp {
    pub fn as_str(&self) -> &'static str {
        match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Mod => "%",
            BinaryOp::Eq => "==",
            BinaryOp::Ne => "!=",
            BinaryOp::Lt => "<",
            BinaryOp::Le => "<=",
            BinaryOp::Gt => ">",
            BinaryOp::Ge => ">=",
            BinaryOp::And => "&&",
            BinaryOp::Or => "||",
            BinaryOp::BitAnd => "&",
            BinaryOp::BitOr => "|",
            BinaryOp::BitXor => "^",
        }
    }
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
    BitNot,
}

impl UnaryOp {
    pub fn as_str(&self) -> &'static str {
        match self {
            UnaryOp::Neg => "-",
            UnaryOp::Not => "!",
            UnaryOp::BitNot => "~",
        }
    }
}

/// Language-agnostic intermediate representation for code generation
///
/// This enum represents all possible code constructs that Frame can generate.
/// Language-specific backends convert these nodes into target language code.
#[derive(Debug, Clone)]
pub enum CodegenNode {
    // ===== Structural =====

    /// A complete module with imports and items
    Module {
        imports: Vec<CodegenNode>,
        items: Vec<CodegenNode>,
    },

    /// Import statement
    Import {
        module: String,
        items: Vec<String>,
        alias: Option<String>,
    },

    /// Class or struct definition
    Class {
        name: String,
        fields: Vec<Field>,
        methods: Vec<CodegenNode>,
        base_classes: Vec<String>,
        is_abstract: bool,
        /// Derive attributes for Rust (e.g., ["Serialize", "Deserialize"])
        derives: Vec<String>,
    },

    /// Enum definition
    Enum {
        name: String,
        variants: Vec<EnumVariant>,
    },

    // ===== Methods =====

    /// Method definition
    Method {
        name: String,
        params: Vec<Param>,
        return_type: Option<String>,
        body: Vec<CodegenNode>,
        is_async: bool,
        is_static: bool,
        visibility: Visibility,
        decorators: Vec<String>,
    },

    /// Constructor
    Constructor {
        params: Vec<Param>,
        body: Vec<CodegenNode>,
        super_call: Option<Box<CodegenNode>>,
    },

    // ===== Statements =====

    /// Variable declaration
    VarDecl {
        name: String,
        type_annotation: Option<String>,
        init: Option<Box<CodegenNode>>,
        is_const: bool,
    },

    /// Assignment
    Assignment {
        target: Box<CodegenNode>,
        value: Box<CodegenNode>,
    },

    /// Return statement
    Return {
        value: Option<Box<CodegenNode>>,
    },

    /// If statement
    If {
        condition: Box<CodegenNode>,
        then_block: Vec<CodegenNode>,
        else_block: Option<Vec<CodegenNode>>,
    },

    /// Match/switch statement
    Match {
        scrutinee: Box<CodegenNode>,
        arms: Vec<MatchArm>,
    },

    /// While loop
    While {
        condition: Box<CodegenNode>,
        body: Vec<CodegenNode>,
    },

    /// For loop
    For {
        var: String,
        iterable: Box<CodegenNode>,
        body: Vec<CodegenNode>,
    },

    /// Break statement
    Break,

    /// Continue statement
    Continue,

    /// Expression statement
    ExprStmt(Box<CodegenNode>),

    /// Comment
    Comment {
        text: String,
        is_doc: bool,
    },

    /// Empty statement / no-op
    Empty,

    // ===== Expressions =====

    /// Identifier reference
    Ident(String),

    /// Literal value
    Literal(Literal),

    /// Binary operation
    BinaryOp {
        op: BinaryOp,
        left: Box<CodegenNode>,
        right: Box<CodegenNode>,
    },

    /// Unary operation
    UnaryOp {
        op: UnaryOp,
        operand: Box<CodegenNode>,
    },

    /// Function/method call
    Call {
        target: Box<CodegenNode>,
        args: Vec<CodegenNode>,
    },

    /// Method call on an object
    MethodCall {
        object: Box<CodegenNode>,
        method: String,
        args: Vec<CodegenNode>,
    },

    /// Field/member access
    FieldAccess {
        object: Box<CodegenNode>,
        field: String,
    },

    /// Index access (array/dict)
    IndexAccess {
        object: Box<CodegenNode>,
        index: Box<CodegenNode>,
    },

    /// Self/this reference
    SelfRef,

    /// Array/list literal
    Array(Vec<CodegenNode>),

    /// Dictionary/map literal
    Dict(Vec<(CodegenNode, CodegenNode)>),

    /// Ternary/conditional expression
    Ternary {
        condition: Box<CodegenNode>,
        then_expr: Box<CodegenNode>,
        else_expr: Box<CodegenNode>,
    },

    /// Lambda/closure expression
    Lambda {
        params: Vec<Param>,
        body: Box<CodegenNode>,
    },

    /// Type cast
    Cast {
        expr: Box<CodegenNode>,
        target_type: String,
    },

    /// New/construct instance
    New {
        class: String,
        args: Vec<CodegenNode>,
    },

    // ===== Frame-Specific =====

    /// State transition
    Transition {
        target_state: String,
        exit_args: Vec<CodegenNode>,
        enter_args: Vec<CodegenNode>,
        state_args: Vec<CodegenNode>,
        /// Source indentation level for proper code placement
        indent: usize,
    },

    /// Change state (no enter/exit)
    ChangeState {
        target_state: String,
        state_args: Vec<CodegenNode>,
        /// Source indentation level for proper code placement
        indent: usize,
    },

    /// Forward to parent state
    Forward {
        to_parent: bool,
        /// Source indentation level for proper code placement
        indent: usize,
    },

    /// Push current state onto stack
    StackPush {
        /// Source indentation level for proper code placement
        indent: usize,
    },

    /// Pop and transition to stacked state
    StackPop {
        /// Source indentation level for proper code placement
        indent: usize,
    },

    /// State context (for state parameters)
    StateContext {
        state_name: String,
    },

    /// Frame message/event send
    SendEvent {
        event: String,
        args: Vec<CodegenNode>,
    },

    // ===== Native Code Preservation =====

    /// Native code block (ocean model)
    /// This preserves user-written native code exactly as-is
    NativeBlock {
        code: String,
        span: Option<Span>,
    },

    /// Placeholder for splice point
    SplicePoint {
        id: String,
    },
}

/// Enum variant
#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub value: Option<Box<CodegenNode>>,
    pub fields: Vec<Field>,
}

/// Literal values
#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
}

impl CodegenNode {
    // Convenience constructors

    pub fn int(value: i64) -> Self {
        CodegenNode::Literal(Literal::Int(value))
    }

    pub fn float(value: f64) -> Self {
        CodegenNode::Literal(Literal::Float(value))
    }

    pub fn string(value: &str) -> Self {
        CodegenNode::Literal(Literal::String(value.to_string()))
    }

    pub fn bool(value: bool) -> Self {
        CodegenNode::Literal(Literal::Bool(value))
    }

    pub fn null() -> Self {
        CodegenNode::Literal(Literal::Null)
    }

    pub fn ident(name: &str) -> Self {
        CodegenNode::Ident(name.to_string())
    }

    pub fn self_ref() -> Self {
        CodegenNode::SelfRef
    }

    pub fn field(obj: CodegenNode, field: &str) -> Self {
        CodegenNode::FieldAccess {
            object: Box::new(obj),
            field: field.to_string(),
        }
    }

    pub fn call(target: CodegenNode, args: Vec<CodegenNode>) -> Self {
        CodegenNode::Call {
            target: Box::new(target),
            args,
        }
    }

    pub fn method_call(obj: CodegenNode, method: &str, args: Vec<CodegenNode>) -> Self {
        CodegenNode::MethodCall {
            object: Box::new(obj),
            method: method.to_string(),
            args,
        }
    }

    pub fn assign(target: CodegenNode, value: CodegenNode) -> Self {
        CodegenNode::Assignment {
            target: Box::new(target),
            value: Box::new(value),
        }
    }

    pub fn ret(value: Option<CodegenNode>) -> Self {
        CodegenNode::Return {
            value: value.map(Box::new),
        }
    }

    pub fn if_stmt(condition: CodegenNode, then_block: Vec<CodegenNode>, else_block: Option<Vec<CodegenNode>>) -> Self {
        CodegenNode::If {
            condition: Box::new(condition),
            then_block,
            else_block,
        }
    }

    pub fn native(code: &str) -> Self {
        CodegenNode::NativeBlock {
            code: code.to_string(),
            span: None,
        }
    }

    pub fn transition(target: &str) -> Self {
        CodegenNode::Transition {
            target_state: target.to_string(),
            exit_args: vec![],
            enter_args: vec![],
            state_args: vec![],
            indent: 0,
        }
    }

    pub fn comment(text: &str) -> Self {
        CodegenNode::Comment {
            text: text.to_string(),
            is_doc: false,
        }
    }

    pub fn doc_comment(text: &str) -> Self {
        CodegenNode::Comment {
            text: text.to_string(),
            is_doc: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_constructors() {
        match CodegenNode::int(42) {
            CodegenNode::Literal(Literal::Int(n)) => assert_eq!(n, 42),
            _ => panic!("Expected Int literal"),
        }

        match CodegenNode::string("hello") {
            CodegenNode::Literal(Literal::String(s)) => assert_eq!(s, "hello"),
            _ => panic!("Expected String literal"),
        }

        match CodegenNode::bool(true) {
            CodegenNode::Literal(Literal::Bool(b)) => assert!(b),
            _ => panic!("Expected Bool literal"),
        }
    }

    #[test]
    fn test_field_access() {
        let node = CodegenNode::field(CodegenNode::self_ref(), "state");
        match node {
            CodegenNode::FieldAccess { object, field } => {
                assert!(matches!(*object, CodegenNode::SelfRef));
                assert_eq!(field, "state");
            }
            _ => panic!("Expected FieldAccess"),
        }
    }

    #[test]
    fn test_method_call() {
        let node = CodegenNode::method_call(
            CodegenNode::self_ref(),
            "_transition",
            vec![CodegenNode::ident("new_state")],
        );
        match node {
            CodegenNode::MethodCall { object, method, args } => {
                assert!(matches!(*object, CodegenNode::SelfRef));
                assert_eq!(method, "_transition");
                assert_eq!(args.len(), 1);
            }
            _ => panic!("Expected MethodCall"),
        }
    }

    #[test]
    fn test_transition() {
        let node = CodegenNode::transition("$Running");
        match node {
            CodegenNode::Transition { target_state, .. } => {
                assert_eq!(target_state, "$Running");
            }
            _ => panic!("Expected Transition"),
        }
    }
}
