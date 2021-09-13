//! This package provides a way to generically inspect and monitor a running
//! state machine. In order to use this interface, you must compile your Frame
//! spec with the `runtime_support` feature enabled.
//!
//! Note that many of the tests associated with this package are trivial, but
//! their value is mostly in demonstrating what generated code that realizes
//! these interfaces should look like.

#[allow(dead_code)]
mod callback;
mod environment;
mod machine;
mod state;
