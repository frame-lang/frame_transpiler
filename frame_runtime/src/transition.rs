//! This module defines a type that captures state transitions.

use crate::env::Environment;
use crate::event::Event;
use crate::info::TransitionInfo;
use crate::machine::{Machine, State};
use std::fmt;
use std::ops::Deref;

/// Captures the occurrence of a transition between two states.
pub struct Transition<M: Machine + ?Sized>
where
    <M::EnvironmentPtr as Deref>::Target: Environment,
    <M::EventPtr as Deref>::Target: Event<M>,
    <M::StatePtr as Deref>::Target: State<M>,
{
    /// Information about the transition statement that triggered this transition.
    pub info: &'static TransitionInfo,

    /// The source state instance immediately before the transition.
    pub old_state: M::StatePtr,

    /// The target state instance immediately after the transition.
    pub new_state: M::StatePtr,

    /// The exit event sent to the source state. Will be `None` for a change-state transition.
    pub exit_event: Option<M::EventPtr>,

    /// The enter event sent to the target state. Will be `None` for a change-state transition.
    pub enter_event: Option<M::EventPtr>,
}

impl<M: Machine> Transition<M>
where
    <M::EnvironmentPtr as Deref>::Target: Environment,
    <M::EventPtr as Deref>::Target: Event<M>,
    <M::StatePtr as Deref>::Target: State<M>,
{
    /// Create a transition instance for a standard transition with exit/enter events.
    pub fn new(
        info: &'static TransitionInfo,
        old_state: M::StatePtr,
        new_state: M::StatePtr,
        exit_event: M::EventPtr,
        enter_event: M::EventPtr,
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
        old_state: M::StatePtr,
        new_state: M::StatePtr,
    ) -> Self {
        Transition {
            info,
            old_state,
            new_state,
            exit_event: None,
            enter_event: None,
        }
    }

    /// Get the arguments from the exit event, or an empty environment if there is no exit
    /// event.
    pub fn exit_arguments(&self) -> M::EnvironmentPtr {
        match &self.exit_event {
            Some(event) => event.arguments(),
            None => M::empty_environment(),
        }
    }

    /// Get the arguments from the enter event or an empty environment if there is no enter
    /// event.
    pub fn enter_arguments(&self) -> M::EnvironmentPtr {
        match &self.enter_event {
            Some(event) => event.arguments(),
            None => M::empty_environment(),
        }
    }
}

impl<M: Machine> Clone for Transition<M>
where
    <M::EnvironmentPtr as Deref>::Target: Environment,
    <M::EventPtr as Deref>::Target: Event<M>,
    <M::StatePtr as Deref>::Target: State<M>,
{
    fn clone(&self) -> Self {
        Transition {
            info: self.info,
            old_state: self.old_state.clone(),
            new_state: self.new_state.clone(),
            exit_event: self.exit_event.clone(),
            enter_event: self.enter_event.clone(),
        }
    }
}

impl<M: Machine> fmt::Display for Transition<M>
where
    <M::EnvironmentPtr as Deref>::Target: Environment,
    <M::EventPtr as Deref>::Target: Event<M>,
    <M::StatePtr as Deref>::Target: State<M>,
{
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
