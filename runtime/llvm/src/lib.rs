//! Frame LLVM runtime crate.
//!
//! This crate provides the minimal runtime scaffolding required for the
//! experimental LLVM backend. The goal for Week 8 is to mirror the same
//! responsibilities handled by the existing TypeScript/Python runtimes:
//!   * Event construction and dispatch bookkeeping.
//!   * Compartment storage (state, enter/exit args, forwarded events).
//!   * A thin kernel loop that repeatedly drains transitions.
//!
//! The initial implementation keeps the APIs intentionally small so we can
//! evolve them alongside the LLVM visitor without committing to a long-term
//! ABI. Everything here should be treated as internal until the backend
//! stabilises.

mod event_system;
mod ffi;
mod frame_kernel;
mod memory;

pub use event_system::{FrameCompartment, FrameEvent};
pub use ffi::*;
pub use frame_kernel::{FrameKernel, FrameKernelResult};
pub use memory::{FrameBox, FrameRc};
