//! This module provides infrastructure to support registering and invoking
//! callbacks that notify clients of events within a running state machine.

use crate::environment::{Environment, EMPTY};
use crate::state::State;
use crate::transition::{TransitionInfo, TransitionKind};

/// Callback manager.
pub struct CallbackManager {
    transition_callbacks: Vec<Box<dyn FnMut(&TransitionInfo)>>,
}

impl CallbackManager {
    /// Register a callback to be called on each transition.
    pub fn add_transition_callback(&mut self, callback: impl FnMut(&TransitionInfo) + 'static) {
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
