/// GraphViz-specific intermediate representation.
///
/// Captures graph semantics directly from SystemAst + Arcanum,
/// bypassing CodegenNode (which models imperative code, not graphs).

/// Complete graph representation for one Frame system.
#[derive(Debug, Clone)]
pub struct SystemGraph {
    pub name: String,
    pub states: Vec<StateNode>,
    pub transitions: Vec<TransitionEdge>,
    pub entry_state: Option<String>,
    pub has_state_stack: bool,
}

/// A state in the graph.
#[derive(Debug, Clone)]
pub struct StateNode {
    pub name: String,
    pub parent: Option<String>,
    pub children: Vec<String>,
    pub has_enter: bool,
    pub has_exit: bool,
    pub handlers: Vec<HandlerInfo>,
    pub state_vars: Vec<StateVar>,
    pub state_params: Vec<StateParam>,
}

/// Minimal handler info for display in state node labels.
#[derive(Debug, Clone)]
pub struct HandlerInfo {
    pub event: String,
    pub params: Vec<(String, String)>, // (name, type)
}

/// State variable for display in node labels.
#[derive(Debug, Clone)]
pub struct StateVar {
    pub name: String,
    pub var_type: Option<String>,
}

/// State parameter for display in node labels.
#[derive(Debug, Clone)]
pub struct StateParam {
    pub name: String,
    pub param_type: Option<String>,
}

/// A transition edge in the graph.
#[derive(Debug, Clone)]
pub struct TransitionEdge {
    pub source: String,
    pub target: TransitionTarget,
    pub event: String,
    /// User-provided label (e.g., `-> "Path A" $Target`).
    /// When present, replaces event name on the edge label.
    pub label: Option<String>,
    pub kind: TransitionKind,
    /// Condition text from enclosing if-branch.
    pub guard: Option<String>,
}

/// What a transition points to.
#[derive(Debug, Clone)]
pub enum TransitionTarget {
    State(String),
    StackPop,
    ParentForward,
}

/// Visual distinction for edge types.
#[derive(Debug, Clone, PartialEq)]
pub enum TransitionKind {
    /// `->` full transition with enter/exit
    Transition,
    /// `->>` direct state change, no enter/exit
    ChangeState,
    /// `=>` forward event to parent
    Forward,
}
