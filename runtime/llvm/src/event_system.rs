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
    pub enter_args: Option<FrameEvent>,
    pub exit_args: Option<FrameEvent>,
}

impl FrameCompartment {
    pub fn new<S: Into<String>>(state: S) -> Self {
        FrameCompartment {
            state: state.into(),
            forward_event: None,
            enter_args: None,
            exit_args: None,
        }
    }
}
