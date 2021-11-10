//! This module defines structs that provide access to *static* information about a state machine.
//! Static information is shared among all running instances of a state machine, and includes
//! things like the names and types of declared states, variables, events, and actions, as well as
//! structural information such as possible transitions and hierarchy relationships among states.

use once_cell::sync::OnceCell;

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
    pub parameters: &'static [NameInfo],

    /// The return type.
    pub return_type: Option<&'static str>,
}

/// Static information about a state machine.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MachineInfo {
    /// The system name for this state machine.
    pub name: &'static str,

    /// The variables declared in this machine's `domain` block.
    pub variables: &'static [NameInfo],

    /// The states declared in this machine's `machine` block.
    pub states: &'static [&'static StateInfo],

    /// The signatures of events declared in this machine's `interface` block.
    pub interface: &'static [&'static MethodInfo],

    /// The signatures of actions declared in this machine's `actions` block.
    pub actions: &'static [&'static MethodInfo],

    /// The signatures of all events that can occur in this machine. This includes events declared
    /// in this machine's `interface` block, as well as the enter/exit events for each state.
    pub events: &'static [&'static MethodInfo],

    /// All of the possible transitions between states in this machine.
    pub transitions: &'static [&'static TransitionInfo],
}

impl MachineInfo {
    /// The initial state of the machine, which is is the first state listed in the `machine` block.
    /// Returns `None` if the machine has no states.
    pub fn initial_state(&self) -> Option<&'static StateInfo> {
        if self.states.is_empty() {
            None
        } else {
            Some(self.states[0])
        }
    }

    /// The top-level states are those which are not children of another state.
    pub fn top_level_states(&self) -> Vec<&'static StateInfo> {
        self.states
            .iter()
            .cloned()
            .filter(|s| s.parent.is_none())
            .collect()
    }

    /// Get a domain variable declaration by name.
    pub fn get_variable(&self, name: &str) -> Option<&'static NameInfo> {
        self.variables.iter().find(|n| name == n.name)
    }

    /// Get a state within this machine by name.
    pub fn get_state(&self, name: &str) -> Option<&'static StateInfo> {
        self.states.iter().cloned().find(|s| name == s.name)
    }

    /// Get the signature corresponding to the named event. You can use this method to get
    /// the signatures of both interface events and enter/exit events.
    pub fn get_event(&self, name: &str) -> Option<&'static MethodInfo> {
        self.events.iter().cloned().find(|m| name == m.name)
    }

    /// Get the signature corresponding to the named action.
    pub fn get_action(&self, name: &str) -> Option<&'static MethodInfo> {
        self.actions.iter().cloned().find(|m| name == m.name)
    }
}

/// Static information about a single state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateInfo {
    /// A reference to the machine this state is contained in. This field should not be accessed
    /// directly; instead use the `machine()` method. This reference is wrapped in a `OnceCell` to
    /// avoid a cycle in the generated struct values. Framec should generate the code to initialize
    /// everything correctly the first time the machine info is accessed.
    pub machine_cell: &'static OnceCell<&'static MachineInfo>,

    /// The unique name of this state.
    pub name: &'static str,

    /// The parent of this state, if any.
    pub parent: Option<&'static StateInfo>,

    /// The state parameters declared for this state, which will be bound to arguments on
    /// transition.
    pub parameters: &'static [NameInfo],

    /// The state variables declared for this state, which can be read and assigned from event
    /// handlers.
    pub variables: &'static [NameInfo],

    /// The events that this state handles.
    pub handlers: &'static [&'static MethodInfo],

    /// Is this the special state-stack pop transition target state? This method will return
    /// `false` for any active state. However, a dummy `StateInfo` with this value set to `true`
    /// will be used in place of a specific state in stack-pop transitions, that is, transitions
    /// of the form `-> $$[-]` or `->> $$[-]`.
    pub is_stack_pop: bool,
}

impl StateInfo {
    /// The machine this state is contained in.
    pub fn machine(&self) -> &'static MachineInfo {
        self.machine_cell
            .get()
            .expect("Machine info cell not initialized")
    }

    /// The sequence of ancestors for this state. The first element in the returned vector will be
    /// the immediate parent of this state, the next will be the parent's parent, and so on.
    pub fn ancestors(&self) -> Vec<&'static StateInfo> {
        let mut result = Vec::new();
        let mut parent_opt = self.parent;
        while let Some(parent) = parent_opt {
            result.push(parent);
            parent_opt = parent.parent;
        }
        result
    }

    /// The children of this state, if any.
    pub fn children(&self) -> Vec<&'static StateInfo> {
        self.machine()
            .states
            .iter()
            .cloned()
            .filter(|s| match s.parent {
                Some(p) => self.name == p.name,
                None => false,
            })
            .collect()
    }

    /// Get a state parameter declaration by name.
    pub fn get_parameter(&self, name: &str) -> Option<&'static NameInfo> {
        self.parameters.iter().find(|n| name == n.name)
    }

    /// Get a state variable declaration by name.
    pub fn get_variable(&self, name: &str) -> Option<&'static NameInfo> {
        self.variables.iter().find(|n| name == n.name)
    }

    /// Get the signature of an event handler associated with this state by name.
    pub fn get_handler(&self, name: &str) -> Option<&'static MethodInfo> {
        self.handlers.iter().cloned().find(|m| name == m.name)
    }

    /// All transitions in the machine with this state as the `target`.
    pub fn incoming_transitions(&self) -> Vec<&'static TransitionInfo> {
        self.machine()
            .transitions
            .iter()
            .cloned()
            .filter(|t| self.name == t.target.name)
            .collect()
    }

    /// All transitions in the machine with this state as the `source`.
    pub fn outgoing_transitions(&self) -> Vec<&'static TransitionInfo> {
        self.machine()
            .transitions
            .iter()
            .cloned()
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransitionInfo {
    /// A unique ID for this transition within the current state machine. This ID corresponds to
    /// the index of the transition in the array of all transitions for the machine.
    pub id: usize,

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
