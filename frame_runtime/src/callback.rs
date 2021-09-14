//! This module provides infrastructure to support registering and invoking
//! callbacks that notify clients of events within a running state machine.

use crate::environment::{EMPTY, Environment};
use crate::state::State;

/// Was the transition a standard transition or a change-state transition,
/// which bypasses enter/exit events?
pub enum TransitionKind {
    ChangeState,
    Transition,
}

/// Information about a transition or change-state operation, passed to
/// callbacks that are registered to monitor
pub struct TransitionInfo<'a> {
    /// What kind of transition occurred?
    kind: TransitionKind,
    /// The state before the transition.
    old_state: &'a dyn State,
    /// The state after the transition.
    new_state: &'a dyn State,
    /// Arguments to the exit handler of the old state.
    exit_arguments: &'a dyn Environment,
}

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
        };
        self.call_transition_callbacks(&info);
    }

    /// Invoke all the transition callbacks for a standard transition.
    pub fn transition(&mut self, old_state: &dyn State, new_state: &dyn State) {
        self.transition_with_args(old_state, new_state, EMPTY); 
    }

    /// Invoke all the transition callbacks for a transition with enter/exit
    /// arguments.
    pub fn transition_with_args(
        &mut self,
        old_state: &dyn State,
        new_state: &dyn State,
        exit_arguments: &dyn Environment,
    ) {
        let info = TransitionInfo {
            kind: TransitionKind::Transition,
            old_state,
            new_state,
            exit_arguments,
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
