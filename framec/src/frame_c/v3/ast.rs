use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span { pub start: usize, pub end: usize }

#[derive(Debug, Clone)]
pub struct SystemDecl {
    pub name: String,
    pub machines: HashMap<String, MachineDecl>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct MachineDecl {
    pub name: String,
    pub states: HashMap<String, StateDecl>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct StateDecl {
    pub name: String,
    pub parent: Option<String>,
    pub params: Vec<String>,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlerKind { Handler, Action, Operation }

#[derive(Debug, Clone)]
pub struct HandlerDecl {
    pub name: String,
    pub kind: HandlerKind,
    pub span: Span,
}

// High-level outer AST for V3 modules and systems. This will become the
// single source of truth for system/block structure instead of ad-hoc
// byte scans spread across the validator.

#[derive(Debug, Clone)]
pub struct SystemParamsAst {
    pub start_params: Vec<String>,
    pub enter_params: Vec<String>,
    pub domain_params: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemSectionKind {
    Operations,
    Interface,
    Machine,
    Actions,
    Domain,
}

#[derive(Debug, Clone, Default)]
pub struct SystemSectionsAst {
    pub operations: Option<Span>,
    pub interface: Option<Span>,
    pub machine: Option<Span>,
    pub actions: Option<Span>,
    pub domain: Option<Span>,
}

#[derive(Debug, Clone)]
pub struct SystemAst {
    pub name: String,
    pub params: SystemParamsAst,
    pub span: Span,
    pub sections: SystemSectionsAst,
    /// Section order as encountered in the source (may contain duplicates).
    pub section_order: Vec<SystemSectionKind>,
}

#[derive(Debug, Clone, Default)]
pub struct ModuleAst {
    pub systems: Vec<SystemAst>,
}
