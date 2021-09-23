//! This module provides infrastructure to support registering and invoking
//! callbacks that notify clients of events within a running state machine.

use crate::environment::{Environment, EMPTY};
use crate::state::State;
use crate::transition::{TransitionInfo, TransitionKind};

/// Callback manager.
pub struct CallbackManager<'a> {
    transition_callbacks: Vec<Box<dyn FnMut(&TransitionInfo) + 'a>>,
}

impl<'a> CallbackManager<'a> {
    /// Create a new callback manager.
    pub fn new() -> CallbackManager<'a> {
        CallbackManager {
            transition_callbacks: Vec::new(),
        }
    }

    /// Register a callback to be called on each transition.
    pub fn add_transition_callback(&mut self, callback: impl FnMut(&TransitionInfo) + 'a) {
        self.transition_callbacks.push(Box::new(callback));
    }

    /// Invoke all the transition callbacks for a change-state transition.
    pub fn change_state(&mut self, old_state: &dyn State, new_state: &dyn State) {
        let info = TransitionInfo {
            kind: TransitionKind::ChangeState,
            old_state,
            new_state,
            exit_arguments: EMPTY,
            enter_arguments: EMPTY,
        };
        self.call_transition_callbacks(&info);
    }

    /// Invoke all the transition callbacks for a standard transition.
    pub fn transition(&mut self, old_state: &dyn State, new_state: &dyn State) {
        self.transition_with_args(old_state, new_state, EMPTY, EMPTY);
    }

    /// Invoke all the transition callbacks for a transition with enter/exit
    /// arguments.
    pub fn transition_with_args(
        &mut self,
        old_state: &dyn State,
        new_state: &dyn State,
        exit_arguments: &dyn Environment,
        enter_arguments: &dyn Environment,
    ) {
        let info = TransitionInfo {
            kind: TransitionKind::Transition,
            old_state,
            new_state,
            exit_arguments,
            enter_arguments,
        };
        self.call_transition_callbacks(&info);
    }

    /// Invoke all the transition callbacks.
    fn call_transition_callbacks(&mut self, info: &TransitionInfo) {
        for c in &mut self.transition_callbacks {
            (**c)(info);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::RefCell;
    use std::rc::Rc;

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
        fn state_arguments(&self) -> &dyn Environment {
            EMPTY
        }
        fn state_variables(&self) -> &dyn Environment {
            EMPTY
        }
    }

    #[test]
    fn callbacks_are_called() {
        let tape: Vec<String> = Vec::new();
        let tape_rc = Rc::new(RefCell::new(tape));
        let mut cm = CallbackManager::new();
        cm.add_transition_callback(|i| {
            tape_rc
                .borrow_mut()
                .push(format!("old: {}", i.old_state.name()))
        });
        cm.add_transition_callback(|i| {
            tape_rc
                .borrow_mut()
                .push(format!("new: {}", i.new_state.name()))
        });

        cm.transition(&TestState::A, &TestState::B);
        assert_eq!(*tape_rc.borrow(), vec!["old: A", "new: B"]);
        tape_rc.borrow_mut().clear();

        cm.change_state(&TestState::B, &TestState::A);
        assert_eq!(*tape_rc.borrow(), vec!["old: B", "new: A"]);
    }
}
