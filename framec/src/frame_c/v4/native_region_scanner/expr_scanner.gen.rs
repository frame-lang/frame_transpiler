
// ExprScanner â PDA (pushdown automaton) for scanning assignment RHS expressions.
//
// Scans from `pos` (after the `=`) to a terminator (`;` or `\n`) at depth 0,
// respecting nested `()[]{}` and string literals with escape handling.
//
// This replaces 3 duplicated inline expression scanners in unified.rs.

struct ExprScannerFsmFrameEvent {
    message: String,
    parameters: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl Clone for ExprScannerFsmFrameEvent {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            parameters: std::collections::HashMap::new(),
        }
    }
}

impl ExprScannerFsmFrameEvent {
    fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            parameters: std::collections::HashMap::new(),
        }
    }
    fn new_with_params(message: &str, params: &std::collections::HashMap<String, String>) -> Self {
        Self {
            message: message.to_string(),
            parameters: params.iter().map(|(k, v)| (k.clone(), Box::new(v.clone()) as Box<dyn std::any::Any>)).collect(),
        }
    }
}

struct ExprScannerFsmFrameContext {
    event: ExprScannerFsmFrameEvent,
    _return: Option<Box<dyn std::any::Any>>,
    _data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl ExprScannerFsmFrameContext {
    fn new(event: ExprScannerFsmFrameEvent, default_return: Option<Box<dyn std::any::Any>>) -> Self {
        Self {
            event,
            _return: default_return,
            _data: std::collections::HashMap::new(),
        }
    }
}

#[derive(Clone)]
struct ExprScannerFsmCompartment {
    state: String,
    state_vars: std::collections::HashMap<String, i32>,
    enter_args: std::collections::HashMap<String, String>,
    exit_args: std::collections::HashMap<String, String>,
    forward_event: Option<ExprScannerFsmFrameEvent>,
    parent_compartment: Option<Box<ExprScannerFsmCompartment>>,
}

impl ExprScannerFsmCompartment {
    fn new(state: &str) -> Self {
        Self {
            state: state.to_string(),
            state_vars: std::collections::HashMap::new(),
            enter_args: std::collections::HashMap::new(),
            exit_args: std::collections::HashMap::new(),
            forward_event: None,
            parent_compartment: None,
        }
    }
}

#[derive(Clone)]
enum ExprScannerFsmStateContext {
    Init,
    Scanning,
    Empty,
}

impl Default for ExprScannerFsmStateContext {
    fn default() -> Self {
        ExprScannerFsmStateContext::Init
    }
}

pub struct ExprScannerFsm {
    _state_stack: Vec<(String, ExprScannerFsmStateContext)>,
    __compartment: ExprScannerFsmCompartment,
    __next_compartment: Option<ExprScannerFsmCompartment>,
    _context_stack: Vec<ExprScannerFsmFrameContext>,
    pub bytes: Vec<u8>,
    pub pos: usize,
    pub end: usize,
    pub result_end: usize,
}

impl ExprScannerFsm {
    pub fn new() -> Self {
        let mut this = Self {
            _state_stack: vec![],
            _context_stack: vec![],
            bytes: Vec::new(),
            pos: 0,
            end: 0,
            result_end: 0,
            __compartment: ExprScannerFsmCompartment::new("Init"),
            __next_compartment: None,
        };
let __frame_event = ExprScannerFsmFrameEvent::new("$>");
this.__kernel(__frame_event);
        this
    }

    fn __kernel(&mut self, __e: ExprScannerFsmFrameEvent) {
// Route event to current state
self.__router(&__e);
// Process any pending transition
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    // Exit current state (with exit_args from current compartment)
    let exit_event = ExprScannerFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    // Switch to new compartment
    self.__compartment = next_compartment;
    // Enter new state (or forward event)
    if self.__compartment.forward_event.is_none() {
        let enter_event = ExprScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        // Forward event to new state
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            // Forwarding enter event - just send it
            self.__router(&forward_event);
        } else {
            // Forwarding other event - send $> first, then forward
            let enter_event = ExprScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
    }

    fn __router(&mut self, __e: &ExprScannerFsmFrameEvent) {
match self.__compartment.state.as_str() {
    "Init" => self._state_Init(__e),
    "Scanning" => self._state_Scanning(__e),
    _ => {}
}
    }

    fn __transition(&mut self, next_compartment: ExprScannerFsmCompartment) {
self.__next_compartment = Some(next_compartment);
    }

    fn _state_stack_push(&mut self) {
let state_context = match self.__compartment.state.as_str() {
    "Init" => ExprScannerFsmStateContext::Init,
    "Scanning" => ExprScannerFsmStateContext::Scanning,
    _ => ExprScannerFsmStateContext::Empty,
};
self._state_stack.push((self.__compartment.state.clone(), state_context));
    }

    fn _state_stack_pop(&mut self) {
let (saved_state, state_context) = self._state_stack.pop().unwrap();
let exit_event = ExprScannerFsmFrameEvent::new("$<");
self.__router(&exit_event);
self.__compartment.state = saved_state;
match state_context {
    ExprScannerFsmStateContext::Init => {}
    ExprScannerFsmStateContext::Scanning => {}
    ExprScannerFsmStateContext::Empty => {}
}
    }

    pub fn do_scan(&mut self) {
let mut __e = ExprScannerFsmFrameEvent::new("do_scan");
let __ctx = ExprScannerFsmFrameContext::new(__e.clone(), None);
self._context_stack.push(__ctx);
match self.__compartment.state.as_str() {
            "Init" => { self._s_Init_do_scan(&__e); }
            _ => {}
        }
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    let exit_event = ExprScannerFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    self.__compartment = next_compartment;
    if self.__compartment.forward_event.is_none() {
        let enter_event = ExprScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            self.__router(&forward_event);
        } else {
            let enter_event = ExprScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
self._context_stack.pop();
    }

    fn _state_Scanning(&mut self, __e: &ExprScannerFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_Scanning_enter(__e); }
    _ => {}
}
    }

    fn _state_Init(&mut self, __e: &ExprScannerFsmFrameEvent) {
match __e.message.as_str() {
    "do_scan" => { self._s_Init_do_scan(__e); }
    _ => {}
}
    }

    fn _s_Scanning_enter(&mut self, __e: &ExprScannerFsmFrameEvent) {
let mut i = self.pos;
let end = self.end;
let bytes = &self.bytes;
let mut depth: i32 = 0;
let mut in_string: Option<u8> = None;

while i < end {
    let b = bytes[i];

    // Handle string literals
    if let Some(q) = in_string {
        if b == b'\\' && i + 1 < end {
            i += 2;
            continue;
        }
        if b == q {
            in_string = None;
        }
        i += 1;
        continue;
    }

    // Enter string literal
    if b == b'"' || b == b'\'' {
        in_string = Some(b);
        i += 1;
        continue;
    }

    // Track nesting depth (PDA stack via counter)
    match b {
        b'(' | b'[' | b'{' => { depth += 1; }
        b')' | b']' | b'}' => { depth = (depth - 1).max(0); }
        b';' if depth == 0 => {
            i += 1; // Include the semicolon
            break;
        }
        b'\n' if depth == 0 => {
            break; // Don't include the newline
        }
        _ => {}
    }
    i += 1;
}

self.result_end = i;
    }

    fn _s_Init_do_scan(&mut self, __e: &ExprScannerFsmFrameEvent) {
let mut __compartment = ExprScannerFsmCompartment::new("Scanning");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }
}
