//! This module defines traits that provide access to a running state machine and snapshots of
//! active states within a running state machine.

use crate::env::Environment;
use crate::info::{MachineInfo, StateInfo};

/// An interface to a running state machine that supports inspecting its current state and
/// variables, and registering callbacks to be notified of various events.
pub trait Machine<StatePtr, EventMonitor> {
    /// Static information about the state machine declaration that gave rise to this machine
    /// instance.
    fn info(&self) -> &'static MachineInfo;

    /// The currently active state of this machine.
    fn state(&self) -> StatePtr;

    /// Environment containing the current values of the domain variables associated with this
    /// machine. The variable names and types can be obtained from `self.info().variables`.
    fn variables(&self) -> &dyn Environment;

    /// Get an immutable reference to this machine's event monitor, suitable for querying the
    /// transition/event history.
    fn event_monitor(&self) -> &EventMonitor;

    /// Get a mutable reference to this machine's event monitor, suitable for registering callbacks
    /// to be notified of Frame events.
    fn event_monitor_mut(&mut self) -> &mut EventMonitor;
}

/// A snapshot of an active state within a running state machine. State arguments and variables are
/// not saved between visits, so these names are bound to values only when the state is "active". A
/// state is active when it is the current state or when it is immediately involved in a
/// transition.
pub trait State<EnvironmentPtr> {
    /// Static information about the state declaration that gave rise to this state instance.
    fn info(&self) -> &'static StateInfo;

    /// Environment containing the values of the state arguments passed to this state on
    /// transition. The names and types of the parameters these arguments are bound to can be found
    /// at `self.info().parameters`.
    fn arguments(&self) -> EnvironmentPtr;

    /// Environment containing the current values of the variables associated with this state. The
    /// names and types of the variables can be found at `self.info().variables`.
    fn variables(&self) -> EnvironmentPtr;
}

/// Definitions specific to the synchronized/thread-safe interface.
pub mod sync {
    pub use super::*;
    use crate::env::sync::EnvironmentPtr;
    use std::sync::Arc;

    /// A reference-counted pointer to an active state.
    pub type StatePtr = Arc<dyn super::State<EnvironmentPtr> + Send + Sync>;
}

/// Definitions specific to the unsynchronized interface.
pub mod unsync {
    pub use super::*;
    use crate::env::unsync::EnvironmentPtr;
    use std::rc::Rc;

    /// A reference-counted pointer to an active state.
    pub type StatePtr = Rc<dyn super::State<EnvironmentPtr>>;
}
