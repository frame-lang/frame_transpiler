//! This module defines a generic interface to a running state machine.

use std::any::Any;

/// An environment associates names (i.e. variables/parameters) with values.
trait Environment {
    /// Is this environment empty?
    fn is_empty(&self) -> bool;
    /// Get the value associated with a name.
    fn lookup(&self, name: &str) -> Option<&dyn Any>;
}

/// The trivial empty environment.
struct EmptyEnvironment {}

impl Environment for EmptyEnvironment {
    fn is_empty(&self) -> bool {
        true
    }
    fn lookup(&self, _name: &str) -> Option<&dyn Any> {
        None
    }
}

/// A state within a state machine.
trait State {
    /// The name of this state.
    fn name(&self) -> &'static str;
}

/// Generic interface to a running state machine.
trait StateMachine {
    /// The current state of this machine.
    fn current_state(&self) -> &dyn State;

    /// The domain variables associated with this machine.
    fn domain_variables(&self) -> &dyn Environment;

    /// The state variables associated with the current state. Note that this
    /// method is in the [StateMachine] trait rather than the [State] trait
    /// since state variables are not persistent and so are only defined for
    /// the currently active state.
    fn state_variables(&self) -> &dyn Environment;

    /// The callback manager for this state machine.
    fn callback_manager(&self) -> &CallbackManager;
}

/// Was the transition a standard transition or a change-state transition,
/// which bypasses enter/exit events?
enum TransitionKind {
    ChangeState,
    Transition,
}

/// Information about a transition or change-state operation, passed to
/// callbacks that are registered to monitor
struct TransitionInfo {
    /// What kind of transition occurred?
    kind: TransitionKind,
    /// The state before the transition.
    old_state: Box<dyn State>,
    /// The state after the transition.
    new_state: Box<dyn State>,
    /// Arguments to the enter handler of the new state.
    enter_args: Box<dyn Environment>,
    /// Arguments to the exit handler of the old state.
    exit_args: Box<dyn Environment>,
}

/// Callback manager.
struct CallbackManager {
    transition_callbacks: Vec<Box<dyn FnMut(&TransitionInfo)>>,
}

impl CallbackManager {
    /// Register a callback to be called on each transition.
    pub fn add_transition_callback(&mut self, callback: impl FnMut(&TransitionInfo) + 'static) {
        self.transition_callbacks.push(Box::new(callback));
    }

    /// Invoke all the transition callbacks.
    fn call_transition_callbacks(&mut self, info: &TransitionInfo) {
        for c in &mut self.transition_callbacks {
            (**c)(info);
        }
    }

    /// Invoke all the transition callbacks for a standard transition.
    pub fn transition(&mut self, old_state: Box<dyn State>, new_state: Box<dyn State>) {
        self.transition_with_args(
            old_state,
            new_state,
            Box::new(EmptyEnvironment {}),
            Box::new(EmptyEnvironment {}),
        );
    }

    /// Invoke all the transition callbacks for a transition with enter/exit
    /// arguments.
    pub fn transition_with_args(
        &mut self,
        old_state: Box<dyn State>,
        new_state: Box<dyn State>,
        enter_args: Box<dyn Environment>,
        exit_args: Box<dyn Environment>,
    ) {
        let info = TransitionInfo {
            kind: TransitionKind::Transition,
            old_state,
            new_state,
            enter_args,
            exit_args,
        };
        self.call_transition_callbacks(&info);
    }

    /// Invoke all the transition callbacks for a change-state transition.
    pub fn change_state(&mut self, old_state: Box<dyn State>, new_state: Box<dyn State>) {
        let info = TransitionInfo {
            kind: TransitionKind::ChangeState,
            old_state,
            new_state,
            enter_args: Box::new(EmptyEnvironment {}),
            exit_args: Box::new(EmptyEnvironment {}),
        };
        self.call_transition_callbacks(&info);
    }
}
