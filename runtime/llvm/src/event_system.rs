use std::collections::HashMap;
use std::ptr::NonNull;

/// Simple event structure used by the LLVM backend.
#[derive(Clone, Debug)]
pub struct FrameEvent {
    message: String,
    parameters: Vec<String>,
}

impl FrameEvent {
    pub fn new<S: Into<String>>(message: S) -> Self {
        FrameEvent {
            message: message.into(),
            parameters: Vec::new(),
        }
    }

    pub fn with_parameters<S: Into<String>>(message: S, parameters: Vec<S>) -> Self {
        FrameEvent {
            message: message.into(),
            parameters: parameters.into_iter().map(Into::into).collect(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn parameters(&self) -> &[String] {
        &self.parameters
    }
}

/// Compartment data mirrors the runtime structs generated for Python/TypeScript.
#[derive(Debug)]
pub struct FrameCompartment {
    pub state: String,
    pub forward_event: Option<FrameEvent>,
    enter_event: Option<FrameEvent>,
    exit_event: Option<FrameEvent>,
    parent: Option<NonNull<FrameCompartment>>,
    state_args: HashMap<String, FrameEvent>,
}

impl FrameCompartment {
    pub fn new<S: Into<String>>(state: S) -> Self {
        FrameCompartment {
            state: state.into(),
            forward_event: None,
            enter_event: None,
            exit_event: None,
            parent: None,
            state_args: HashMap::new(),
        }
    }

    pub fn set_parent(&mut self, parent: *mut FrameCompartment) {
        self.parent = NonNull::new(parent);
    }

    pub fn parent(&self) -> Option<*mut FrameCompartment> {
        self.parent.map(|ptr| ptr.as_ptr())
    }

    pub fn set_enter_event(&mut self, event: Option<FrameEvent>) {
        self.enter_event = event;
    }

    pub fn take_enter_event(&mut self) -> Option<FrameEvent> {
        self.enter_event.take()
    }

    pub fn set_exit_event(&mut self, event: Option<FrameEvent>) {
        self.exit_event = event;
    }

    pub fn take_exit_event(&mut self) -> Option<FrameEvent> {
        self.exit_event.take()
    }

    pub fn forward_event_mut(&mut self) -> &mut Option<FrameEvent> {
        &mut self.forward_event
    }

    pub fn state_args(&self) -> &HashMap<String, FrameEvent> {
        &self.state_args
    }

    pub fn state_args_mut(&mut self) -> &mut HashMap<String, FrameEvent> {
        &mut self.state_args
    }
}
