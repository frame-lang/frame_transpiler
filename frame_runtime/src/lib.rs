//! This crate provides an interface for generically inspecting and monitoring state machines
//! generated by Frame's Rust backend.
//!
//! # How to use it
//!
//! In order to use this interface, you must compile your Frame spec with the `runtime_support`
//! feature enabled. You can do this by adding the following line to the top of your Frame spec:
//!
//! ```text
//! #[codegen.rust.features.runtime_support:bool="true"]
//! ```
//!
//! Once your state machine has been compiled with runtime support enabled, you can use the
//! interface by including *one* of the following two lines in your code:
//!
//!  * `use frame_runtime::sync`
//!  * `use frame_runtime::unsync`
//!
//!
//! ## Sync vs. unsync interface
//!
//! The runtime interface comes in two flavors, which correspond to whether the `thread_safe`
//! feature was enabled when you compiled your spec in Framec.
//!
//!  * If the spec was compiled with `thread_safe=true`, you should use
//!    [frame_runtime::sync](crate::sync).
//!
//!  * If the spec was compiled with `thread_safe=false`, you should use
//!    [frame_runtime::unsync](crate::unsync).
//!
//! Most of the differences between these interfaces are internal, so switching between them if you
//! change the setting of the `thread_safe` feature in Framec should be relatively painless.
//! Unfortunately, however, it is difficult to write code that is generic with respect to the choice
//! of interface.
//!
//!
//! # Interface overview
//!
//! This crate defines two traits, [live::Machine] and [live::State] that enable reflecting on a
//! running state machine. When the `runtime_support` feature is enabled, the code generated by
//! Frame will automatically implement these traits. These traits enable determining the current
//! state of a machine, provide access to the runtime values of variables and arguments, and enable
//! registering callbacks to be notified of transitions and events.
//!
//! This crate also defines several `*Info` structs that provide access to *static* information
//! about a state machine. Static information is shared among all running instances of a state
//! machine, and includes things like the names and types of declared states, variables, events,
//! and actions, as well as structural information such as possible transitions and the hierarchy
//! relationships among states. Values of these structs will be automatically generated and
//! associated with the state machine. The root `MachineInfo` struct can be obtained from the
//! `machine_info()` function associated with the generated struct for the state machine, and the
//! corresponding `*Info` struct can be obtained from each element of a running state machine via
//! an `info()` method.
//!
//! Throughout this crate (and within Frame more generally), it is assumed that in any collection
//! of elements, all elements will have unique names. For example, all states within a machine will
//! have unique names, and all variables within a particular state will have unique names. This
//! constraint does not hold among all names in the machine, however. For example, a variable in
//! state `A` may have the same name as a variable in state `B`, or have the same name as a
//! parameter in state `A`.

pub mod callback;
pub mod env;
pub mod event;
pub mod history;
pub mod info;
pub mod machine;
pub mod smcat;
pub mod transition;

pub use crate::callback::*;
pub use crate::env::*;
pub use crate::event::*;
pub use crate::history::*;
pub use crate::info::*;
pub use crate::machine::*;
pub use crate::smcat::*;
pub use crate::transition::*;
