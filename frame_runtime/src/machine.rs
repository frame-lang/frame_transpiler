//! This module defines traits that provide access to a running state machine and snapshots of
//! active states within a running state machine.

use crate::callback::IsCallback;
use crate::env::Environment;
use crate::event::{Event, EventMonitor};
use crate::info::{MachineInfo, StateInfo};
use crate::transition::Transition;
use std::ops::Deref;

/// An interface to a running state machine that supports inspecting its current state and
/// variables, and registering callbacks to be notified of various events.
pub trait Machine
where
    <Self::EnvironmentPtr as Deref>::Target: Environment,
    <Self::EventPtr as Deref>::Target: Event<Self>,
    <Self::StatePtr as Deref>::Target: State<Self>,
{
    /// Type of pointers to environments within this machine.
    type EnvironmentPtr: Deref + Clone;

    /// Type of pointers to events within this machine.
    type EventPtr: Deref + Clone;

    /// Type of pointers to states within this machine.
    type StatePtr: Deref + Clone;

    /// Type of event callbacks within this machine.
    type EventFn: IsCallback<Self::EventPtr>;

    /// Type of transition callbacks within this machine.
    type TransitionFn: IsCallback<Transition<Self>>;

    /// Static information about the state machine declaration that gave rise to this machine
    /// instance. This method is just a synonym for the function `Self::machine_info()` but is
    /// provided for consistency with other elements of the runtime interface.
    fn info(&self) -> &'static MachineInfo {
        Self::machine_info()
    }

    /// The currently active state of this machine.
    fn state(&self) -> Self::StatePtr;

    /// Environment containing the current values of the domain variables associated with this
    /// machine. The variable names and types can be obtained from `self.info().variables`.
    fn variables(&self) -> &dyn Environment;

    /// Get an immutable reference to this machine's event monitor, suitable for querying the
    /// transition/event history.
    fn event_monitor(&self) -> &EventMonitor<Self>;

    /// Get a mutable reference to this machine's event monitor, suitable for registering callbacks
    /// to be notified of Frame events.
    fn event_monitor_mut(&mut self) -> &mut EventMonitor<Self>;

    /// Static information about the state machine declaration.
    fn machine_info() -> &'static MachineInfo;

    /// Get a pointer to an empty environment that is compatible with this machine. This is
    /// intended for use by generated code and library functions.
    fn empty_environment() -> Self::EnvironmentPtr;
}

/// A snapshot of an active state within a running state machine. State arguments and variables are
/// not saved between visits, so these names are bound to values only when the state is "active". A
/// state is active when it is the current state or when it is immediately involved in a
/// transition.
pub trait State<M: Machine + ?Sized>
where
    <M::EnvironmentPtr as Deref>::Target: Environment,
    <M::EventPtr as Deref>::Target: Event<M>,
    <M::StatePtr as Deref>::Target: State<M>,
{
    /// Static information about the state declaration that gave rise to this state instance.
    fn info(&self) -> &'static StateInfo;

    /// Environment containing the values of the state arguments passed to this state on
    /// transition. The names and types of the parameters these arguments are bound to can be found
    /// at `self.info().parameters`.
    fn arguments(&self) -> M::EnvironmentPtr;

    /// Environment containing the current values of the variables associated with this state. The
    /// names and types of the variables can be found at `self.info().variables`.
    fn variables(&self) -> M::EnvironmentPtr;
}
