use crate::info::TransitionInfo;

/// Captures the occurrence of a transition between two states.
#[derive(Clone)]
pub struct Transition<StatePtr, EventPtr> {
    /// Information about the transition statement that triggered this transition.
    pub info: &'static TransitionInfo,

    /// The source state instance immediately before the transition.
    pub old_state: StatePtr,

    /// The target state instance immediately after the transition.
    pub new_state: StatePtr,

    /// The exit event sent to the source state. Will be `None` for a change-state transition.
    pub exit_event: Option<EventPtr>,

    /// The enter event sent to the target state. Will be `None` for a change-state transition.
    pub enter_event: Option<EventPtr>,
}

impl<StatePtr, EventPtr> Transition<StatePtr, EventPtr> {
    /// Create a transition instance for a standard transition with exit/enter events.
    pub fn new(
        info: &'static TransitionInfo,
        old_state: StatePtr,
        new_state: StatePtr,
        exit_event: EventPtr,
        enter_event: EventPtr,
    ) -> Self {
        Transition {
            info,
            old_state,
            new_state,
            exit_event: Some(exit_event),
            enter_event: Some(enter_event),
        }
    }

    /// Create a transition instance for a change-state transition without exit/enter events.
    pub fn new_change_state(
        info: &'static TransitionInfo,
        old_state: StatePtr,
        new_state: StatePtr,
    ) -> Self {
        Transition {
            info,
            old_state,
            new_state,
            exit_event: None,
            enter_event: None,
        }
    }
}

pub mod sync {
    use crate::env::sync::EnvironmentPtr;
    use crate::env::Empty;
    use crate::event::sync::EventPtr;
    use crate::live::sync::StatePtr;
    use std::fmt;

    /// Captures the occurrence of a transition between two states.
    pub type Transition = super::Transition<StatePtr, EventPtr>;

    impl Transition {
        /// Get the arguments from the exit event or an empty environment if there is no exit event.
        pub fn exit_arguments(&self) -> EnvironmentPtr {
            match &self.exit_event {
                Some(event) => event.arguments(),
                None => Empty::arc(),
            }
        }

        /// Get the arguments from the enter event or an empty environment if there is no enter event.
        pub fn enter_arguments(&self) -> EnvironmentPtr {
            match &self.enter_event {
                Some(event) => event.arguments(),
                None => Empty::arc(),
            }
        }
    }

    impl fmt::Display for Transition {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{}{}{}",
                self.old_state.info().name,
                self.info.kind,
                self.new_state.info().name
            )
        }
    }
}

pub mod unsync {
    use crate::env::unsync::EnvironmentPtr;
    use crate::env::Empty;
    use crate::event::unsync::EventPtr;
    use crate::live::unsync::StatePtr;
    use std::fmt;

    /// Captures the occurrence of a transition between two states.
    pub type Transition = super::Transition<StatePtr, EventPtr>;

    impl Transition {
        /// Get the arguments from the exit event or an empty environment if there is no exit event.
        pub fn exit_arguments(&self) -> EnvironmentPtr {
            match &self.exit_event {
                Some(event) => event.arguments(),
                None => Empty::rc(),
            }
        }

        /// Get the arguments from the enter event or an empty environment if there is no enter event.
        pub fn enter_arguments(&self) -> EnvironmentPtr {
            match &self.enter_event {
                Some(event) => event.arguments(),
                None => Empty::rc(),
            }
        }
    }

    impl fmt::Display for Transition {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{}{}{}",
                self.old_state.info().name,
                self.info.kind,
                self.new_state.info().name
            )
        }
    }
}
