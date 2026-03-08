use std::collections::HashMap;
use std::ffi::CString;

#[derive(Clone, Debug)]
pub enum StateValue {
    I32(i32),
    F64(f64),
    Bool(bool),
    CString(CString),
}

impl StateValue {
    pub fn as_i32(&self) -> Option<i32> {
        if let StateValue::I32(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        if let StateValue::F64(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let StateValue::Bool(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn as_c_str_ptr(&self) -> Option<*const libc::c_char> {
        if let StateValue::CString(value) = self {
            Some(value.as_c_str().as_ptr())
        } else {
            None
        }
    }
}

/// Simple event structure used by the LLVM backend.
#[derive(Clone, Debug)]
pub struct FrameEvent {
    message: String,
    parameters: Vec<StateValue>,
}

impl FrameEvent {
    pub fn new<S: Into<String>>(message: S) -> Self {
        FrameEvent {
            message: message.into(),
            parameters: Vec::new(),
        }
    }

    pub fn with_parameters(message: impl Into<String>, parameters: Vec<StateValue>) -> Self {
        FrameEvent {
            message: message.into(),
            parameters,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn parameters(&self) -> &[StateValue] {
        &self.parameters
    }

    pub fn push_param(&mut self, value: StateValue) {
        self.parameters.push(value);
    }

    pub fn param(&self, index: usize) -> Option<&StateValue> {
        self.parameters.get(index)
    }
}

/// Compartment data mirrors the runtime structs generated for Python/TypeScript.
#[derive(Clone, Debug)]
pub struct FrameCompartment {
    pub state: String,
    pub forward_event: Option<FrameEvent>,
    enter_event: Option<FrameEvent>,
    exit_event: Option<FrameEvent>,
    parent: Option<Box<FrameCompartment>>,
    state_args: HashMap<String, StateValue>,
    enter_args: HashMap<String, StateValue>,
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
            enter_args: HashMap::new(),
        }
    }

    pub fn set_parent_box(&mut self, parent: Box<FrameCompartment>) {
        self.parent = Some(parent);
    }

    pub fn take_parent(&mut self) -> Option<Box<FrameCompartment>> {
        self.parent.take()
    }

    pub fn parent_ptr(&self) -> Option<*mut FrameCompartment> {
        self.parent
            .as_ref()
            .map(|boxed| boxed.as_ref() as *const FrameCompartment as *mut FrameCompartment)
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

    pub fn set_forward_event(&mut self, event: Option<FrameEvent>) {
        self.forward_event = event;
    }

    pub fn set_state_arg(&mut self, key: String, value: StateValue) {
        self.state_args.insert(key, value);
    }

    pub fn state_arg(&self, key: &str) -> Option<&StateValue> {
        self.state_args.get(key)
    }

    pub fn set_enter_arg(&mut self, key: String, value: StateValue) {
        self.enter_args.insert(key, value);
    }

    pub fn enter_arg(&self, key: &str) -> Option<&StateValue> {
        self.enter_args.get(key)
    }

    pub fn clear_enter_args(&mut self) {
        self.enter_args.clear();
    }
}
