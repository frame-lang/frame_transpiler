//! Frame AST - Abstract Syntax Tree for Frame language constructs
//!
//! This module defines the AST representation for Frame v4, which will be used
//! in the hybrid compiler architecture to represent Frame constructs independently
//! of native code, before merging into a unified Hybrid AST.
//!
//! This is the SINGLE unified AST for Frame V4. The old `ast.rs` module has been
//! merged into this file to eliminate the dual-AST problem.

use std::collections::{HashMap, HashSet};
use crate::frame_c::v4::native_region_scanner::{RegionV3, RegionSpan};

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

// ============================================================================
// Section and Attribute Types (merged from old ast.rs)
// ============================================================================

/// Kinds of sections in a Frame system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemSectionKind {
    Operations,
    Interface,
    Machine,
    Actions,
    Domain,
}

/// Section span tracking for validation (tracks where each section is located)
#[derive(Debug, Clone, Default)]
pub struct SystemSectionSpans {
    pub operations: Option<Span>,
    pub interface: Option<Span>,
    pub machine: Option<Span>,
    pub actions: Option<Span>,
    pub domain: Option<Span>,
}

/// Persistence attribute parsed from `@@persist` annotation
#[derive(Debug, Clone)]
pub struct PersistAttr {
    /// Optional custom save method name. When None, language-specific
    /// defaults are used (e.g., save_to_json / saveToJson).
    pub save_name: Option<String>,
    /// Optional custom restore method name. When None, language-specific
    /// defaults are used (e.g., restore_from_json / restoreFromJson).
    pub restore_name: Option<String>,
    /// Serialization library for Rust (e.g., "serde")
    pub library: Option<String>,
    pub span: Span,
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
    // NEW fields for unified AST:
    /// Section span tracking for validation
    pub section_spans: SystemSectionSpans,
    /// Optional persistence metadata from `@@persist`
    pub persist_attr: Option<PersistAttr>,
    /// Section order as encountered in source (may contain duplicates for validation)
    pub section_order: Vec<SystemSectionKind>,
}

/// System parameter (for parameterized systems)
#[derive(Debug, Clone)]
pub struct SystemParam {
    pub name: String,
    pub param_type: Type,
    pub default: Option<String>,
    /// Whether this is a state parameter (uses $() syntax to pass to initial state)
    pub is_state_param: bool,
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

/// State variable declaration ($.varName: type = init)
#[derive(Debug, Clone)]
pub struct StateVarAst {
    pub name: String,
    pub var_type: Type,
    pub init: Option<Expression>,
    pub span: Span,
}

/// State definition
#[derive(Debug, Clone)]
pub struct StateAst {
    pub name: String,
    pub params: Vec<StateParam>,
    pub parent: Option<String>,  // For HSM parent state
    pub state_vars: Vec<StateVarAst>,  // State-local variables ($.varName)
    pub handlers: Vec<HandlerAst>,
    pub enter: Option<EnterHandler>,
    pub exit: Option<ExitHandler>,
    /// State-level default forward to parent (bare `=> $^` at state level)
    pub default_forward: bool,
    pub span: Span,
    /// Body span (inside braces only, for precise error reporting)
    pub body_span: Span,
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
    pub return_type: Option<Type>,
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
    pub params: Vec<EventParam>,
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

/// Handler body contains Frame statements only
/// Native code is handled by the splicer in codegen, not stored in AST
#[derive(Debug, Clone)]
pub struct HandlerBody {
    /// Frame statements only (transitions, forwards, etc.)
    pub statements: Vec<Statement>,
    /// Full span of handler body (used by scanner/splicer)
    pub span: Span,
}

/// Statement in a handler - Frame statements only
/// Native code is NOT in the AST - it's handled by splicer in codegen
#[derive(Debug, Clone)]
pub enum Statement {
    /// Frame transition statement (->)
    Transition(TransitionAst),
    /// Frame transition-forward (-> => $State)
    TransitionForward(TransitionForwardAst),
    /// Frame forward to parent (=>)
    Forward(ForwardAst),
    /// Frame stack push (push$)
    StackPush(StackPushAst),
    /// Frame stack pop (pop$)
    StackPop(StackPopAst),
    /// Frame return (return <expr>)
    Return(ReturnAst),
    /// Frame continue (deprecated)
    Continue(ContinueAst),
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
    /// Source indentation level (for proper code generation)
    pub indent: usize,
}

/// Transition-forward statement (-> => $State)
/// Transitions to state then dispatches current event to new state's handler
#[derive(Debug, Clone)]
pub struct TransitionForwardAst {
    pub target: String,
    pub span: Span,
    /// Source indentation level (for proper code generation)
    pub indent: usize,
}

/// Forward to parent (=> event)
#[derive(Debug, Clone)]
pub struct ForwardAst {
    pub event: String,
    pub args: Vec<Expression>,
    pub span: Span,
    /// Source indentation level (for proper code generation)
    pub indent: usize,
}

/// Stack push (push$)
#[derive(Debug, Clone)]
pub struct StackPushAst {
    pub span: Span,
    /// Source indentation level (for proper code generation)
    pub indent: usize,
}

/// Stack pop (pop$)
#[derive(Debug, Clone)]
pub struct StackPopAst {
    pub span: Span,
    /// Source indentation level (for proper code generation)
    pub indent: usize,
}

/// Return statement (return <expr>)
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
    /// Native expression - raw source passed through verbatim
    /// Used for language-specific expressions the parser doesn't understand
    NativeExpr(String),
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

/// Action body - native code only, content preserved by splicer
#[derive(Debug, Clone)]
pub struct ActionBody {
    /// Span referencing original source - splicer extracts content directly
    pub span: Span,
}

/// Operation definition (with return type)
#[derive(Debug, Clone)]
pub struct OperationAst {
    pub name: String,
    pub params: Vec<OperationParam>,
    pub return_type: Type,
    pub body: OperationBody,
    pub is_static: bool,
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

/// Operation body - native code only, content preserved by splicer
#[derive(Debug, Clone)]
pub struct OperationBody {
    /// Span referencing original source - splicer extracts content directly
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
    /// Create a new minimal SystemAst (useful for tests and builder patterns)
    pub fn new(name: String, span: Span) -> Self {
        Self {
            name,
            params: vec![],
            interface: vec![],
            machine: None,
            actions: vec![],
            operations: vec![],
            domain: vec![],
            span,
            section_spans: SystemSectionSpans::default(),
            persist_attr: None,
            section_order: vec![],
        }
    }

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

    /// Get section span for a given section kind
    pub fn get_section_span(&self, kind: SystemSectionKind) -> Option<&Span> {
        match kind {
            SystemSectionKind::Operations => self.section_spans.operations.as_ref(),
            SystemSectionKind::Interface => self.section_spans.interface.as_ref(),
            SystemSectionKind::Machine => self.section_spans.machine.as_ref(),
            SystemSectionKind::Actions => self.section_spans.actions.as_ref(),
            SystemSectionKind::Domain => self.section_spans.domain.as_ref(),
        }
    }

    /// Check if a section appears more than once (for duplicate detection)
    pub fn has_duplicate_sections(&self) -> Option<SystemSectionKind> {
        let mut seen = std::collections::HashSet::new();
        for kind in &self.section_order {
            if !seen.insert(*kind) {
                return Some(*kind);
            }
        }
        None
    }
}

impl StateAst {
    /// Create a new minimal StateAst
    pub fn new(name: String, span: Span) -> Self {
        Self {
            name,
            params: vec![],
            parent: None,
            state_vars: vec![],
            handlers: vec![],
            enter: None,
            exit: None,
            default_forward: false,
            span: span.clone(),
            body_span: span,
        }
    }

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

impl HandlerBody {
    /// Create a new empty handler body
    pub fn empty(span: Span) -> Self {
        Self {
            statements: vec![],
            span,
        }
    }
}

impl SystemSectionSpans {
    /// Set the span for a given section kind
    pub fn set(&mut self, kind: SystemSectionKind, span: Span) {
        match kind {
            SystemSectionKind::Operations => self.operations = Some(span),
            SystemSectionKind::Interface => self.interface = Some(span),
            SystemSectionKind::Machine => self.machine = Some(span),
            SystemSectionKind::Actions => self.actions = Some(span),
            SystemSectionKind::Domain => self.domain = Some(span),
        }
    }

    /// Get the span for a given section kind
    pub fn get(&self, kind: SystemSectionKind) -> Option<&Span> {
        match kind {
            SystemSectionKind::Operations => self.operations.as_ref(),
            SystemSectionKind::Interface => self.interface.as_ref(),
            SystemSectionKind::Machine => self.machine.as_ref(),
            SystemSectionKind::Actions => self.actions.as_ref(),
            SystemSectionKind::Domain => self.domain.as_ref(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_ast_creation() {
        let mut system = SystemAst::new("TrafficLight".to_string(), Span::new(0, 100));
        system.machine = Some(MachineAst {
            states: vec![
                StateAst::new("Red".to_string(), Span::new(0, 10)),
            ],
            span: Span::new(0, 20),
        });

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
            indent: 8,
        };

        assert_eq!(transition.target, "Green");
        assert!(transition.args.is_empty());
        assert_eq!(transition.indent, 8);
    }

    #[test]
    fn test_section_spans() {
        let mut spans = SystemSectionSpans::default();
        spans.set(SystemSectionKind::Machine, Span::new(10, 50));
        spans.set(SystemSectionKind::Actions, Span::new(50, 80));

        assert!(spans.get(SystemSectionKind::Machine).is_some());
        assert!(spans.get(SystemSectionKind::Actions).is_some());
        assert!(spans.get(SystemSectionKind::Interface).is_none());
    }

    #[test]
    fn test_duplicate_sections() {
        let mut system = SystemAst::new("Test".to_string(), Span::new(0, 100));
        system.section_order = vec![
            SystemSectionKind::Machine,
            SystemSectionKind::Actions,
            SystemSectionKind::Machine, // duplicate!
        ];

        assert_eq!(system.has_duplicate_sections(), Some(SystemSectionKind::Machine));
    }

    #[test]
    fn test_persist_attr() {
        let mut system = SystemAst::new("PersistentSystem".to_string(), Span::new(0, 100));
        system.persist_attr = Some(PersistAttr {
            save_name: Some("custom_save".to_string()),
            restore_name: None,
            span: Span::new(0, 20),
        });

        assert!(system.persist_attr.is_some());
        assert_eq!(system.persist_attr.as_ref().unwrap().save_name, Some("custom_save".to_string()));
    }
}