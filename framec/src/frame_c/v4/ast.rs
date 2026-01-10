// Frame v4 AST - Simplified without MIR/MixedBody
//
// The v4 AST stores native code blocks as opaque strings
// No complex parsing of native code, just Frame structure

use std::collections::HashMap;

/// Root AST node for a Frame system
#[derive(Debug, Clone)]
pub struct SystemAst {
    pub target: String,
    pub native_imports: Vec<String>,
    pub annotations: Vec<Annotation>,
    pub name: String,
    pub params: SystemParams,
    pub operations: Option<OperationsBlock>,
    pub interface: Option<InterfaceBlock>,
    pub machine: Option<MachineBlock>,
    pub actions: Option<ActionsBlock>,
    pub domain: Option<DomainBlock>,
    pub source_location: SourceLocation,
}

/// System parameters
#[derive(Debug, Clone, Default)]
pub struct SystemParams {
    pub start_state_params: Vec<Parameter>,  // $(x, y)
    pub enter_params: Vec<Parameter>,         // $>(init)
    pub domain_params: Vec<Parameter>,        // plain params
}

/// A parameter with name and optional type
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub type_hint: Option<String>,  // Native type hint if provided
}

/// Operations block - public methods for direct system access
#[derive(Debug, Clone)]
pub struct OperationsBlock {
    pub methods: Vec<Method>,
    pub source_location: SourceLocation,
}

/// Interface block - public API through state machine
#[derive(Debug, Clone)]
pub struct InterfaceBlock {
    pub methods: Vec<InterfaceMethod>,
    pub source_location: SourceLocation,
}

/// Machine block - state definitions
#[derive(Debug, Clone)]
pub struct MachineBlock {
    pub states: Vec<State>,
    pub source_location: SourceLocation,
}

/// Actions block - private implementation
#[derive(Debug, Clone)]
pub struct ActionsBlock {
    pub methods: Vec<Method>,
    pub source_location: SourceLocation,
}

/// Domain block - private state variables
#[derive(Debug, Clone)]
pub struct DomainBlock {
    pub variables: Vec<DomainVariable>,
    pub source_location: SourceLocation,
}

/// A state in the state machine
#[derive(Debug, Clone)]
pub struct State {
    pub name: String,
    pub params: Vec<Parameter>,
    pub parent: Option<String>,  // For hierarchical states
    pub handlers: Vec<Handler>,
    pub source_location: SourceLocation,
}

/// Event handler in a state
#[derive(Debug, Clone)]
pub struct Handler {
    pub handler_type: HandlerType,
    pub name: Option<String>,  // Event name (None for enter/exit)
    pub params: Vec<Parameter>,
    pub return_type: Option<String>,
    pub mir_block: super::mir::MirBlock,  // MIR representation of native code with Frame constructs
    pub source_location: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HandlerType {
    Event,
    Enter,
    Exit,
}

/// Frame-specific statements found in native code
#[derive(Debug, Clone)]
pub enum FrameStatement {
    Transition {
        target: String,
        args: Vec<String>,
        source_location: SourceLocation,
    },
    ChangeState {
        target: String,
        args: Vec<String>,
        source_location: SourceLocation,
    },
    Forward {
        source_location: SourceLocation,
    },
    StackPush {
        source_location: SourceLocation,
    },
    StackPop {
        source_location: SourceLocation,
    },
    SystemReturn {
        expression: Option<String>,
        source_location: SourceLocation,
    },
}

/// Method in operations or actions
#[derive(Debug, Clone)]
pub struct Method {
    pub name: String,
    pub is_static: bool,
    pub params: Vec<Parameter>,
    pub return_type: Option<String>,
    pub mir_block: super::mir::MirBlock,  // MIR representation of native code
    pub source_location: SourceLocation,
}

/// Interface method declaration
#[derive(Debug, Clone)]
pub struct InterfaceMethod {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Option<String>,
    pub source_location: SourceLocation,
}

/// Domain variable
#[derive(Debug, Clone)]
pub struct DomainVariable {
    pub name: String,
    pub type_hint: Option<String>,
    pub initializer: Option<String>,  // Native expression
    pub source_location: SourceLocation,
}

/// Annotation (Frame or native)
#[derive(Debug, Clone)]
pub enum Annotation {
    Frame {
        name: String,
        args: HashMap<String, String>,
    },
    Native {
        content: String,  // Opaque native annotation
    },
}

/// Source location for error reporting
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub offset: usize,
    pub length: usize,
}

impl SourceLocation {
    pub fn new(file: String, line: usize, column: usize, offset: usize, length: usize) -> Self {
        Self {
            file,
            line,
            column,
            offset,
            length,
        }
    }

    pub fn unknown() -> Self {
        Self {
            file: String::from("<unknown>"),
            line: 0,
            column: 0,
            offset: 0,
            length: 0,
        }
    }
}

// Helper functions for AST traversal and manipulation
impl SystemAst {
    /// Get the initial state (first state in machine block)
    pub fn initial_state(&self) -> Option<&State> {
        self.machine.as_ref()?.states.first()
    }

    /// Find a state by name
    pub fn find_state(&self, name: &str) -> Option<&State> {
        self.machine
            .as_ref()?
            .states
            .iter()
            .find(|s| s.name == name)
    }

    /// Check if a method is in the interface
    pub fn has_interface_method(&self, name: &str) -> bool {
        self.interface
            .as_ref()
            .map(|i| i.methods.iter().any(|m| m.name == name))
            .unwrap_or(false)
    }

    /// Get domain variable by name
    pub fn find_domain_var(&self, name: &str) -> Option<&DomainVariable> {
        self.domain
            .as_ref()?
            .variables
            .iter()
            .find(|v| v.name == name)
    }
}

impl State {
    /// Find handler by event name
    pub fn find_handler(&self, event: &str) -> Option<&Handler> {
        self.handlers
            .iter()
            .find(|h| h.handler_type == HandlerType::Event && h.name.as_deref() == Some(event))
    }

    /// Get enter handler
    pub fn enter_handler(&self) -> Option<&Handler> {
        self.handlers
            .iter()
            .find(|h| h.handler_type == HandlerType::Enter)
    }

    /// Get exit handler
    pub fn exit_handler(&self) -> Option<&Handler> {
        self.handlers
            .iter()
            .find(|h| h.handler_type == HandlerType::Exit)
    }
}