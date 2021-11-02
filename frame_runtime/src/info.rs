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
pub trait MachineInfo {
    /// The system name for this state machine.
    fn name(&self) -> &'static str;

    /// The variables declared in this machine's `domain` block.
    fn variables(&self) -> Vec<NameInfo>;

    /// The states declared in this machine's `machine` block.
    fn states(&self) -> Vec<&dyn StateInfo>;

    /// The signatures of events declared in this machine's `interface` block.
    fn interface(&self) -> Vec<MethodInfo>;

    /// The signatures of actions declared in this machine's `actions` block.
    fn actions(&self) -> Vec<MethodInfo>;

    /// The signatures of all events that can occur in this machine. This includes events declared
    /// in this machine's `interface` block, as well as the enter/exit events for each state.
    fn events(&self) -> Vec<MethodInfo>;

    /// All of the possible transitions between states in this machine.
    fn transitions(&self) -> Vec<TransitionInfo>;

    /// The initial state of the machine, which is is the first state listed in the `machine` block.
    /// Returns `None` if the machine has no states.
    fn initial_state(&self) -> Option<&dyn StateInfo> {
        if self.states().is_empty() {
            None
        } else {
            Some(self.states()[0])
        }
    }

    /// The top-level states are those which are not children of another state.
    fn top_level_states(&self) -> Vec<&dyn StateInfo> {
        self.states()
            .into_iter()
            .filter(|s| s.parent().is_none())
            .collect()
    }

    /// Get a domain variable declaration by name.
    fn get_variable(&self, name: &str) -> Option<NameInfo> {
        self.variables().into_iter().find(|n| name == n.name)
    }

    /// Get a state within this machine by name.
    fn get_state(&self, name: &str) -> Option<&dyn StateInfo> {
        self.states().into_iter().find(|s| name == s.name())
    }

    /// Get the signature corresponding to the named event. You can use this method to get
    /// the signatures of both interface events and enter/exit events.
    fn get_event(&self, name: &str) -> Option<MethodInfo> {
        self.events().into_iter().find(|m| name == m.name)
    }

    /// Get the signature corresponding to the named action.
    fn get_action(&self, name: &str) -> Option<MethodInfo> {
        self.actions().into_iter().find(|m| name == m.name)
    }
}

/// Static information about a single state.
pub trait StateInfo {
    /// The machine this state is contained in.
    fn machine(&self) -> &dyn MachineInfo;

    /// The name of this state.
    fn name(&self) -> &'static str;

    /// The parent of this state, if any.
    fn parent(&self) -> Option<&dyn StateInfo>;

    /// The state parameters declared for this state, which will be bound to arguments on
    /// transition.
    fn parameters(&self) -> Vec<NameInfo>;

    /// The state variables declared for this state, which can be read and assigned from event
    /// handlers.
    fn variables(&self) -> Vec<NameInfo>;

    /// The events that this state handles.
    fn handlers(&self) -> Vec<MethodInfo>;

    /// Is this the special state-stack pop transition target state? This method will return
    /// `false` for any active state. However, a dummy `StateInfo` with this value set to `true`
    /// will be used in place of a specific state in stack-pop transitions, that is, transitions
    /// of the form `-> $$[-]` or `->> $$[-]`.
    fn is_stack_pop(&self) -> bool {
        false
    }

    /// The sequence of ancestors for this state. The first element in the returned vector will be
    /// the immediate parent of this state, the next will be the parent's parent, and so on.
    fn ancestors(&self) -> Vec<&dyn StateInfo> {
        let mut result = Vec::new();
        let mut parent_opt = self.parent();
        while let Some(parent) = parent_opt {
            result.push(parent);
            parent_opt = parent.parent();
        }
        result
    }

    /// The children of this state, if any.
    fn children(&self) -> Vec<&dyn StateInfo> {
        self.machine()
            .states()
            .into_iter()
            .filter(|s| match s.parent() {
                Some(p) => self.name() == p.name(),
                None => false,
            })
            .collect()
    }

    /// Get a state parameter declaration by name.
    fn get_parameter(&self, name: &str) -> Option<NameInfo> {
        self.parameters().into_iter().find(|n| name == n.name)
    }

    /// Get a state variable declaration by name.
    fn get_variable(&self, name: &str) -> Option<NameInfo> {
        self.variables().into_iter().find(|n| name == n.name)
    }

    /// Get the signature of an event handler associated with this state by name.
    fn get_handler(&self, name: &str) -> Option<MethodInfo> {
        self.handlers().into_iter().find(|m| name == m.name)
    }

    /// All transitions in the machine with this state as the `target`.
    fn incoming_transitions(&self) -> Vec<TransitionInfo> {
        self.machine()
            .transitions()
            .into_iter()
            .filter(|t| self.name() == t.target.name())
            .collect()
    }

    /// All transitions in the machine with this state as the `source`.
    fn outgoing_transitions(&self) -> Vec<TransitionInfo> {
        self.machine()
            .transitions()
            .into_iter()
            .filter(|t| self.name() == t.source.name())
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
pub struct TransitionInfo {
    /// Whether this is a standard or change-state transition.
    pub kind: TransitionKind,

    /// The event that this transition may be triggered by.
    pub event: MethodInfo,

    /// The label associated with this transition.
    pub label: &'static str,

    /// The source state of this transition.
    pub source: &'static dyn StateInfo,

    /// The target state of this transition.
    pub target: &'static dyn StateInfo,
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
