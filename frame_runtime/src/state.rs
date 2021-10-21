//! This module provides a generic interface to states in a running state
//! machine.

use crate::environment::Environment;
use crate::transition::TransitionInfo;

/// Static information about a state.
pub trait State<'a> {
    /// The name of the state.
    fn name(&self) -> &'static str;

    /// Static information about this state.
    fn info(&self) -> StateInfo<'a>;
}

/// Static, structural information about a state.
pub struct StateInfo<'a> {
    pub parent: Option<&'a dyn State<'a>>,
    pub children: Vec<&'a dyn State<'a>>,
    pub transitions: Vec<TransitionInfo<'a>>,
}

/// An empty `StateInfo` for a state with no parents, children, or transitions.
impl<'a> Default for StateInfo<'a> {
    fn default() -> Self {
        StateInfo {
            parent: None,
            children: Vec::new(),
            transitions: Vec::new(),
        }
    }
}

/// A snapshot of an active state within a running state machine. A state is
/// "active" when it is the current state, or when it is involved in a
/// transition.
pub trait ActiveState<'a>: State<'a> {
    /// Arguments passed to the state on transition.
    fn state_arguments(&self) -> &dyn Environment;

    /// Variables associated with this state.
    fn state_variables(&self) -> &dyn Environment;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::EMPTY;

    enum TestState {
        A,
        B,
    }

    impl<'a> State<'a> for TestState {
        fn name(&self) -> &'static str {
            match self {
                TestState::A => "A",
                TestState::B => "B",
            }
        }
        fn info(&self) -> StateInfo<'a> {
            StateInfo::default()
        }
    }

    impl<'a> ActiveState<'a> for TestState {
        fn state_arguments(&self) -> &dyn Environment {
            EMPTY
        }
        fn state_variables(&self) -> &dyn Environment {
            EMPTY
        }
    }

    #[test]
    fn state_name() {
        assert_eq!(TestState::A.name(), "A");
        assert_eq!(TestState::B.name(), "B");
    }
}
