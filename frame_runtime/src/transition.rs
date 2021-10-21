//! This module defines a generic representation of a transition event within
//! a running state machine.

use crate::environment::Environment;
use crate::state::{ActiveState, State};
use std::cell::Ref;

/// Is this a standard transition or a change-state transition (which bypasses
/// enter/exit events)?
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum TransitionKind {
    ChangeState,
    Transition,
}

/// Static information about a potential transition.
pub struct TransitionInfo<'a> {
    pub kind: TransitionKind,
    pub message: &'static str,
    pub label: &'static str,
    pub target: &'a dyn State<'a>,
}

/// Information about a transition or change-state event.
pub struct TransitionEvent<'a> {
    /// What kind of transition occurred?
    pub kind: TransitionKind,

    /// The state before the transition.
    pub old_state: Ref<'a, dyn ActiveState<'a>>,

    /// The state after the transition.
    pub new_state: Ref<'a, dyn ActiveState<'a>>,

    /// Arguments to the exit handler of the old state.
    pub exit_arguments: &'a dyn Environment,

    /// Arguments to the enter handler of the new state.
    pub enter_arguments: &'a dyn Environment,
}
