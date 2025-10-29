use crate::event_system::{FrameCompartment, FrameEvent};
use std::collections::VecDeque;
use std::mem;

/// Result emitted after the kernel processes an event.
pub enum FrameKernelResult {
    Continue,
    Halt,
}

/// Minimal kernel loop – will expand as more semantics come online.
pub struct FrameKernel {
    compartment: Box<FrameCompartment>,
    queue: VecDeque<FrameEvent>,
}

impl FrameKernel {
    pub fn new(compartment: *mut FrameCompartment) -> Option<Self> {
        if compartment.is_null() {
            return None;
        }
        let compartment = unsafe { Box::from_raw(compartment) };
        Some(FrameKernel {
            compartment,
            queue: VecDeque::new(),
        })
    }

    pub fn dispatch(&mut self, event: &FrameEvent) -> FrameKernelResult {
        let _ = event;
        FrameKernelResult::Continue
    }

    pub fn compartment(&self) -> *mut FrameCompartment {
        &*self.compartment as *const FrameCompartment as *mut FrameCompartment
    }

    pub fn push_compartment(&mut self, next: Box<FrameCompartment>) -> *mut FrameCompartment {
        let old = mem::replace(&mut self.compartment, next);
        self.compartment.set_parent_box(old);
        self.compartment()
    }

    pub fn compartment_mut(&mut self) -> &mut FrameCompartment {
        self.compartment.as_mut()
    }

    pub fn set_state<S: Into<String>>(&mut self, state: S) {
        self.compartment.state = state.into();
    }

    pub fn next_event(&mut self) -> Option<FrameEvent> {
        if let Some(event) = self.compartment.forward_event.take() {
            return Some(event);
        }
        self.queue.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_owns_compartment_and_retains_state() {
        let compartment_ptr = Box::into_raw(Box::new(FrameCompartment::new("Start")));
        let mut kernel = FrameKernel::new(compartment_ptr).expect("kernel should be created");

        let compartment_ref = unsafe { &*kernel.compartment() };
        assert_eq!(compartment_ref.state, "Start");

        let event = FrameEvent::new("noop");
        assert!(matches!(
            kernel.dispatch(&event),
            FrameKernelResult::Continue
        ));

        let updated_state = "Running";
        kernel.set_state(updated_state);
        let compartment_ref = unsafe { &*kernel.compartment() };
        assert_eq!(compartment_ref.state, updated_state);
        assert!(kernel.next_event().is_none());
    }
}
