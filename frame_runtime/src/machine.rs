//! This module provides a generic interface to a running state machine.

use crate::callback::CallbackManager;
use crate::environment::Environment;
use crate::state::State;

/// An interface to a running state machine that supports inspecting its
/// current state and variables, and registering callbacks to be notified of
/// various events.
pub trait StateMachine {
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
