//! This module provides a generic interface to states in a running state
//! machine.

use crate::environment::Environment;

/// A snapshot of a state within a running state machine.
pub trait State {
    /// The name of this state.
    fn name(&self) -> &'static str;
    /// Arguments to the enter handler when this state was entered.
    fn state_arguments(&self) -> Box<dyn Environment>;
    /// Arguments to the enter handler when this state was entered.
    fn state_variables(&self) -> Box<dyn Environment>;
    /// Arguments to the enter handler when this state was entered.
    fn enter_arguments(&self) -> Box<dyn Environment>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::EmptyEnvironment;

    enum TestState {
        A,
        B,
    }

    impl State for TestState {
        fn name(&self) -> &'static str {
            match self {
                TestState::A => "A",
                TestState::B => "B",
            }
        }
        fn state_arguments(&self) -> Box<dyn Environment> {
            Box::new(EmptyEnvironment {})
        }
        fn state_variables(&self) -> Box<dyn Environment> {
            Box::new(EmptyEnvironment {})
        }
        fn enter_arguments(&self) -> Box<dyn Environment> {
            Box::new(EmptyEnvironment {})
        }
    }

    #[test]
    fn state_name() {
        assert_eq!(TestState::A.name(), "A");
        assert_eq!(TestState::B.name(), "B");
    }
}
