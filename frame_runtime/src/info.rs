//! This module defines structs that provide access to *static* information about a state machine.
//! Static information is shared among all running instances of a state machine, and includes
//! things like the names and types of declared states, variables, events, and actions, as well as
//! structural information such as possible transitions and hierarchy relationships among states.

/// Information about a simple name declaration. Names in Frame include domain, state, and local
/// variables, as well as method parameters.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct NameInfo {
    /// The name of the variable or parameter.
    pub name: &'static str,

    /// The type associated with the declared name.
    pub vtype: &'static str,
}

/// Information about a method signature declaration. Methods signatures in Frame include events
/// that may be handled by states, and actions which may be invoked within handlers.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct MethodInfo {
    /// The name of the method.
    pub name: &'static str,

    /// The method parameters.
    pub parameters: Vec<NameInfo>,

    /// The return type.
    pub return_type: Option<&'static str>,
}

/// Static information about a state machine.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct MachineInfo {
    /// The system name for this state machine.
    pub name: &'static str,

    /// The variables declared in this machine's `domain` block.
    pub variables: Vec<NameInfo>,

    /// The states declared in this machine's `machine` block.
    pub states: Vec<StateInfo>,

    /// The signatures of events declared in this machine's `interface` block.
    pub events: Vec<MethodInfo>,

    /// The signatures of actions declared in this machine's `actions` block.
    pub actions: Vec<MethodInfo>,

    /// All of the possible transitions between states in this machine.
    pub transitions: Vec<TransitionInfo>,
}

impl MachineInfo {
    /// Get a state within this machine by name.
    pub fn get_state(&self, name: &str) -> Option<&StateInfo> {
        self.states.iter().find(|s| name == s.name)
    }

    /// Get the signature corresponding to the named event.
    pub fn get_event(&self, name: &str) -> Option<&MethodInfo> {
        self.events.iter().find(|m| name == m.name)
    }

    /// Get the signature corresponding to the named action.
    pub fn get_action(&self, name: &str) -> Option<&MethodInfo> {
        self.actions.iter().find(|m| name == m.name)
    }
}

/// Static information about a single state.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct StateInfo {
    /// The machine this state is contained in.
    pub machine: &'static MachineInfo,

    /// The name of this state.
    pub name: &'static str,

    /// The parent of this state, if any.
    pub parent: Option<&'static StateInfo>,

    /// The state parameters declared for this state, which will be bound to arguments on
    /// transition.
    pub parameters: Vec<NameInfo>,

    /// The state variables declared for this state, which can be read and assigned from event
    /// handlers.
    pub variables: Vec<NameInfo>,

    /// The events that this state handles.
    pub handlers: Vec<&'static MethodInfo>,
}

impl StateInfo {
    /// The children of this state, if any.
    pub fn children(&self) -> Vec<&StateInfo> {
        self.machine
            .states
            .iter()
            .filter(|s| match s.parent {
                Some(p) => self.name == p.name,
                None => false,
            })
            .collect()
    }

    /// All transitions in the machine with this state as the `target`.
    pub fn incoming_transitions(&self) -> Vec<&TransitionInfo> {
        self.machine
            .transitions
            .iter()
            .filter(|t| self.name == t.target.name)
            .collect()
    }

    /// All transitions in the machine with this state as the `source`.
    pub fn outgoing_transitions(&self) -> Vec<&TransitionInfo> {
        self.machine
            .transitions
            .iter()
            .filter(|t| self.name == t.source.name)
            .collect()
    }
}

/// Is this a standard transition or a change-state transition (which bypasses enter/exit events)?
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum TransitionKind {
    ChangeState,
    Transition,
}

/// Static information about a (potential) transition. Each `TransitionInfo` corresponds to a
/// transition statement in the Frame specification. When a transition is executed at runtime, an
/// `event::TransitionEvent` is produced, which links to the `TransitionInfo` for the statement
/// that triggered it.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct TransitionInfo {
    /// Whether this is a standard or change-state transition.
    pub kind: TransitionKind,

    /// The event that this transition may be triggered by.
    pub event: &'static MethodInfo,

    /// The label associated with this transition.
    pub label: &'static str,

    /// The source state of this transition.
    pub source: &'static StateInfo,

    /// The target state of this transition.
    pub target: &'static StateInfo,
}

impl TransitionInfo {
    /// Is this a change-state transition?
    pub fn is_change_state(&self) -> bool {
        self.kind == TransitionKind::ChangeState
    }

    /// Is this a standard transition (as opposed to a change-state)?
    pub fn is_transition(&self) -> bool {
        self.kind == TransitionKind::Transition
    }
}
