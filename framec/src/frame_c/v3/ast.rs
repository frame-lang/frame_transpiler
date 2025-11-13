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
