use crate::event_system::{FrameCompartment, FrameEvent};
use std::collections::VecDeque;

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
        self.compartment.forward_event = Some(event.clone());
        self.queue.push_back(event.clone());
        FrameKernelResult::Continue
    }

    pub fn compartment(&self) -> *mut FrameCompartment {
        &*self.compartment as *const FrameCompartment as *mut FrameCompartment
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
        assert!(compartment_ref.forward_event.is_some());

        let queued = kernel.next_event().expect("event should be queued");
        assert_eq!(queued.message(), "noop");
        let queued_again = kernel
            .next_event()
            .expect("queue should retain event history");
        assert_eq!(queued_again.message(), "noop");
        assert!(kernel.next_event().is_none());
    }
}
