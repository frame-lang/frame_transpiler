//! This module provides a generic interface to a running state machine.

use crate::callback::CallbackManager;
use crate::environment::Environment;
use crate::state::State;
use std::cell::Ref;

/// An interface to a running state machine that supports inspecting its
/// current state and variables, and registering callbacks to be notified of
/// various events.
pub trait StateMachine<'a> {
    /// The current state of this machine.
    fn current_state(&self) -> Ref<dyn State>;

    /// The domain variables associated with this machine.
    fn domain_variables(&self) -> &dyn Environment;

    /// The callback manager for this state machine.
    fn callback_manager(&mut self) -> &mut CallbackManager<'a>;
}
