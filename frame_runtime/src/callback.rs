//! This module defines wrappers for callbacks that can be registered with a state machine's
//! [EventMonitor](crate::event::EventMonitor). Use the [Callback] wrapper if the state machine was
//! generated with the Framec feature `thread_safe` set to `false` . Use the [CallbackSend] wrapper
//! if the state machine was generated with `thread_safe=true`.

use std::sync::{Arc, Mutex};

/// Trait for wrappers around callback functions that have a name and accept a reference to `Arg`
/// as an argument.
pub trait IsCallback<Arg> {
    /// A name/ID associated with this callback to enable removing it later.
    fn name(&self) -> &str;

    /// Apply the wrapped function.
    fn apply(&mut self, arg: &Arg);
}

/// A named callback function that accepts a reference to `Arg` as an argument. Use this struct to
/// wrap callbacks if the state machine was generated with `thread_safe=false`.
pub struct Callback<Arg> {
    name: String,
    closure: Box<dyn FnMut(&Arg) + 'static>,
}

impl<Arg> Callback<Arg> {
    /// Create a new callback from the given closure.
    pub fn new(name: &str, f: impl FnMut(&Arg) + 'static) -> Self {
        Callback {
            closure: Box::new(f),
            name: name.to_string(),
        }
    }
}

impl<Arg> IsCallback<Arg> for Callback<Arg> {
    fn name(&self) -> &str {
        &self.name
    }
    fn apply(&mut self, arg: &Arg) {
        (*self.closure)(arg)
    }
}

/// A named callback function that accepts a reference to `Arg` as an argument and implements the
/// [Send] trait. Use this struct to wrap callbacks if the state machine was generated with
/// `thread_safe=true`.
pub struct CallbackSend<Arg> {
    name: String,
    closure: Arc<Mutex<dyn FnMut(&Arg) + Send + 'static>>,
}

impl<Arg> CallbackSend<Arg> {
    /// Create a new callback from the given closure.
    pub fn new(name: &str, f: impl FnMut(&Arg) + Send + 'static) -> Self {
        CallbackSend {
            closure: Arc::new(Mutex::new(f)),
            name: name.to_string(),
        }
    }
}

impl<Arg> IsCallback<Arg> for CallbackSend<Arg> {
    fn name(&self) -> &str {
        &self.name
    }
    fn apply(&mut self, arg: &Arg) {
        (*self.closure.lock().unwrap())(arg)
    }
}
