//! This package provides a way to generically inspect and monitor a running
//! state machine. In order to use this interface, you must compile your Frame
//! spec with the `runtime_support` feature enabled.

#[allow(dead_code)]
pub mod callback;
pub mod environment;
pub mod machine;
pub mod state;
pub mod transition;
