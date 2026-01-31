//! Frame AST - Abstract Syntax Tree for Frame language constructs
//! 
//! This module defines the AST representation for Frame v4, which will be used
//! in the hybrid compiler architecture to represent Frame constructs independently
//! of native code, before merging into a unified Hybrid AST.

use std::collections::HashMap;

/// Span represents a source location in the original Frame code
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

/// Type information for parameters and variables
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// Basic types
    Int,
    Float,
    String,
    Bool,
    /// Custom type (user-defined)
    Custom(String),
    /// Unknown/inferred type
    Unknown,
}

/// Root AST node - either a system or a module
#[derive(Debug, Clone)]
pub enum FrameAst {
    System(SystemAst),
    Module(ModuleAst),
}

/// Module containing multiple systems
#[derive(Debug, Clone)]
pub struct ModuleAst {
    pub name: String,
    pub systems: Vec<SystemAst>,
    pub imports: Vec<Import>,
    pub span: Span,
}

/// Import statement
#[derive(Debug, Clone)]
pub struct Import {
    pub module: String,
    pub symbols: Vec<String>,
    pub alias: Option<String>,
    pub span: Span,
}

/// Frame system definition
#[derive(Debug, Clone)]
pub struct SystemAst {
    pub name: String,
    pub params: Vec<SystemParam>,
    pub interface: Vec<InterfaceMethod>,
    pub machine: Option<MachineAst>,
    pub actions: Vec<ActionAst>,
    pub operations: Vec<OperationAst>,
    pub domain: Vec<DomainVar>,
    pub span: Span,
}

/// System parameter (for parameterized systems)
#[derive(Debug, Clone)]
pub struct SystemParam {
    pub name: String,
    pub param_type: Type,
    pub default: Option<String>,
    pub span: Span,
}

/// Interface method declaration
#[derive(Debug, Clone)]
pub struct InterfaceMethod {
    pub name: String,
    pub params: Vec<MethodParam>,
    pub return_type: Option<Type>,
    pub span: Span,
}

/// Method parameter
#[derive(Debug, Clone)]
pub struct MethodParam {
    pub name: String,
    pub param_type: Type,
    pub default: Option<String>,
    pub span: Span,
}

/// State machine definition
#[derive(Debug, Clone)]
pub struct MachineAst {
    pub states: Vec<StateAst>,
    pub span: Span,
}

/// State definition
#[derive(Debug, Clone)]
pub struct StateAst {
    pub name: String,
    pub params: Vec<StateParam>,
    pub parent: Option<String>,  // For HSM parent state
    pub handlers: Vec<HandlerAst>,
    pub enter: Option<EnterHandler>,
    pub exit: Option<ExitHandler>,
    pub span: Span,
}

/// State parameter
#[derive(Debug, Clone)]
pub struct StateParam {
    pub name: String,
    pub param_type: Type,
    pub span: Span,
}

/// Event handler in a state
#[derive(Debug, Clone)]
pub struct HandlerAst {
    pub event: String,
    pub params: Vec<EventParam>,
    pub body: HandlerBody,
    pub span: Span,
}

/// Enter handler ($>)
#[derive(Debug, Clone)]
pub struct EnterHandler {
    pub params: Vec<EventParam>,
    pub body: HandlerBody,
    pub span: Span,
}

/// Exit handler ($<)
#[derive(Debug, Clone)]
pub struct ExitHandler {
    pub body: HandlerBody,
    pub span: Span,
}

/// Event parameter
#[derive(Debug, Clone)]
pub struct EventParam {
    pub name: String,
    pub param_type: Type,
    pub span: Span,
}

/// Handler body contains mixed Frame statements and native code
#[derive(Debug, Clone)]
pub struct HandlerBody {
    pub statements: Vec<Statement>,
    pub span: Span,
}

/// Statement in a handler - either Frame or native
#[derive(Debug, Clone)]
pub enum Statement {
    /// Frame transition statement (->)
    Transition(TransitionAst),
    /// Frame forward to parent (=>)
    Forward(ForwardAst),
    /// Frame stack push ($$[+])
    StackPush(StackPushAst),
    /// Frame stack pop ($$[-])
    StackPop(StackPopAst),
    /// Frame return (^)
    Return(ReturnAst),
    /// Frame continue (^>)
    Continue(ContinueAst),
    /// Native code block (preserved as-is)
    Native(NativeBlock),
    /// Frame if statement
    If(IfAst),
    /// Frame loop statement
    Loop(LoopAst),
    /// Frame expression (assignments, calls, etc.)
    Expression(ExpressionAst),
}

/// Transition statement (-> $State)
#[derive(Debug, Clone)]
pub struct TransitionAst {
    pub target: String,
    pub args: Vec<Expression>,
    pub span: Span,
}

/// Forward to parent (=> event)
#[derive(Debug, Clone)]
pub struct ForwardAst {
    pub event: String,
    pub args: Vec<Expression>,
    pub span: Span,
}

/// Stack push ($$[+])
#[derive(Debug, Clone)]
pub struct StackPushAst {
    pub span: Span,
}

/// Stack pop ($$[-])
#[derive(Debug, Clone)]
pub struct StackPopAst {
    pub span: Span,
}

/// Return statement (^)
#[derive(Debug, Clone)]
pub struct ReturnAst {
    pub value: Option<Expression>,
    pub span: Span,
}

/// Continue statement (^>)
#[derive(Debug, Clone)]
pub struct ContinueAst {
    pub span: Span,
}

/// Native code block (preserved verbatim)
#[derive(Debug, Clone)]
pub struct NativeBlock {
    pub content: String,
    pub language: TargetLanguage,
    pub span: Span,
}

/// If statement
#[derive(Debug, Clone)]
pub struct IfAst {
    pub condition: Expression,
    pub then_branch: Box<Statement>,
    pub else_branch: Option<Box<Statement>>,
    pub span: Span,
}

/// Loop statement
#[derive(Debug, Clone)]
pub struct LoopAst {
    pub kind: LoopKind,
    pub body: Box<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum LoopKind {
    While(Expression),
    For(String, Expression),  // for var in expr
    Loop,  // infinite loop
}

/// Expression AST
#[derive(Debug, Clone)]
pub struct ExpressionAst {
    pub expr: Expression,
    pub span: Span,
}

/// Expression types
#[derive(Debug, Clone)]
pub enum Expression {
    /// Variable reference
    Var(String),
    /// Literal value
    Literal(Literal),
    /// Binary operation
    Binary {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
    /// Unary operation
    Unary {
        op: UnaryOp,
        expr: Box<Expression>,
    },
    /// Method/function call
    Call {
        func: String,
        args: Vec<Expression>,
    },
    /// Member access (obj.field)
    Member {
        object: Box<Expression>,
        field: String,
    },
    /// Index access (arr[idx])
    Index {
        object: Box<Expression>,
        index: Box<Expression>,
    },
    /// Assignment
    Assign {
        target: Box<Expression>,
        value: Box<Expression>,
    },
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

/// Binary operators
#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Mod,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or,
    BitAnd, BitOr, BitXor,
}

/// Unary operators
#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not,
    Neg,
    BitNot,
}

/// Action definition
#[derive(Debug, Clone)]
pub struct ActionAst {
    pub name: String,
    pub params: Vec<ActionParam>,
    pub body: ActionBody,
    pub span: Span,
}

/// Action parameter
#[derive(Debug, Clone)]
pub struct ActionParam {
    pub name: String,
    pub param_type: Type,
    pub default: Option<String>,
    pub span: Span,
}

/// Action body (native code)
#[derive(Debug, Clone)]
pub struct ActionBody {
    pub native: NativeBlock,
    pub span: Span,
}

/// Operation definition (with return type)
#[derive(Debug, Clone)]
pub struct OperationAst {
    pub name: String,
    pub params: Vec<OperationParam>,
    pub return_type: Type,
    pub body: OperationBody,
    pub span: Span,
}

/// Operation parameter
#[derive(Debug, Clone)]
pub struct OperationParam {
    pub name: String,
    pub param_type: Type,
    pub default: Option<String>,
    pub span: Span,
}

/// Operation body (native code with return)
#[derive(Debug, Clone)]
pub struct OperationBody {
    pub native: NativeBlock,
    pub span: Span,
}

/// Domain variable
#[derive(Debug, Clone)]
pub struct DomainVar {
    pub name: String,
    pub var_type: Type,
    pub initializer: Option<Expression>,
    pub is_frame: bool,  // true if Frame-managed, false if native
    pub span: Span,
}

/// Target language for native blocks
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TargetLanguage {
    Python3,
    TypeScript,
    Rust,
    CSharp,
    C,
    Cpp,
    Java,
}

// Helper methods for AST nodes
impl SystemAst {
    /// Get the start state of the machine (first state defined)
    pub fn start_state(&self) -> Option<&StateAst> {
        self.machine.as_ref()?.states.first()
    }
    
    /// Find a state by name
    pub fn find_state(&self, name: &str) -> Option<&StateAst> {
        self.machine.as_ref()?
            .states.iter()
            .find(|s| s.name == name)
    }
    
    /// Check if an interface method exists
    pub fn has_interface_method(&self, name: &str) -> bool {
        self.interface.iter().any(|m| m.name == name)
    }
    
    /// Check if an action exists
    pub fn has_action(&self, name: &str) -> bool {
        self.actions.iter().any(|a| a.name == name)
    }
    
    /// Check if an operation exists
    pub fn has_operation(&self, name: &str) -> bool {
        self.operations.iter().any(|o| o.name == name)
    }
}

impl StateAst {
    /// Get parameter count
    pub fn param_count(&self) -> usize {
        self.params.len()
    }
    
    /// Find handler by event name
    pub fn find_handler(&self, event: &str) -> Option<&HandlerAst> {
        self.handlers.iter().find(|h| h.event == event)
    }
    
    /// Check if state has a parent (HSM)
    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_system_ast_creation() {
        let system = SystemAst {
            name: "TrafficLight".to_string(),
            params: vec![],
            interface: vec![],
            machine: Some(MachineAst {
                states: vec![
                    StateAst {
                        name: "Red".to_string(),
                        params: vec![],
                        parent: None,
                        handlers: vec![],
                        enter: None,
                        exit: None,
                        span: Span::new(0, 10),
                    },
                ],
                span: Span::new(0, 20),
            }),
            actions: vec![],
            operations: vec![],
            domain: vec![],
            span: Span::new(0, 100),
        };
        
        assert_eq!(system.name, "TrafficLight");
        assert!(system.find_state("Red").is_some());
        assert!(system.find_state("Green").is_none());
    }
    
    #[test]
    fn test_transition_ast() {
        let transition = TransitionAst {
            target: "Green".to_string(),
            args: vec![],
            span: Span::new(10, 20),
        };
        
        assert_eq!(transition.target, "Green");
        assert!(transition.args.is_empty());
    }
}