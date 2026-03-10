
// StateVarParser â FSM for parsing $.varName (read) and $.varName = expr (assignment).
//
// Demonstrates hierarchical composition: $ScanExpr creates an ExprScannerFsm
// sub-machine when it detects an assignment.

include!("expr_scanner.gen.rs");

struct StateVarParserFsmFrameEvent {
    message: String,
    parameters: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl Clone for StateVarParserFsmFrameEvent {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            parameters: std::collections::HashMap::new(),
        }
    }
}

impl StateVarParserFsmFrameEvent {
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

struct StateVarParserFsmFrameContext {
    event: StateVarParserFsmFrameEvent,
    _return: Option<Box<dyn std::any::Any>>,
    _data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl StateVarParserFsmFrameContext {
    fn new(event: StateVarParserFsmFrameEvent, default_return: Option<Box<dyn std::any::Any>>) -> Self {
        Self {
            event,
            _return: default_return,
            _data: std::collections::HashMap::new(),
        }
    }
}

#[derive(Clone)]
struct StateVarParserFsmCompartment {
    state: String,
    state_vars: std::collections::HashMap<String, i32>,
    enter_args: std::collections::HashMap<String, String>,
    exit_args: std::collections::HashMap<String, String>,
    forward_event: Option<StateVarParserFsmFrameEvent>,
    parent_compartment: Option<Box<StateVarParserFsmCompartment>>,
}

impl StateVarParserFsmCompartment {
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
enum StateVarParserFsmStateContext {
    Init,
    ScanIdent,
    CheckAssign,
    ScanExpr,
    Done,
    Empty,
}

impl Default for StateVarParserFsmStateContext {
    fn default() -> Self {
        StateVarParserFsmStateContext::Init
    }
}

pub struct StateVarParserFsm {
    _state_stack: Vec<(String, StateVarParserFsmStateContext)>,
    __compartment: StateVarParserFsmCompartment,
    __next_compartment: Option<StateVarParserFsmCompartment>,
    _context_stack: Vec<StateVarParserFsmFrameContext>,
    pub bytes: Vec<u8>,
    pub pos: usize,
    pub end: usize,
    pub ident_end: usize,
    pub result_end: usize,
    pub is_assignment: bool,
}

impl StateVarParserFsm {
    pub fn new() -> Self {
        let mut this = Self {
            _state_stack: vec![],
            _context_stack: vec![],
            bytes: Vec::new(),
            pos: 0,
            end: 0,
            ident_end: 0,
            result_end: 0,
            is_assignment: false,
            __compartment: StateVarParserFsmCompartment::new("Init"),
            __next_compartment: None,
        };
let __frame_event = StateVarParserFsmFrameEvent::new("$>");
this.__kernel(__frame_event);
        this
    }

    fn __kernel(&mut self, __e: StateVarParserFsmFrameEvent) {
// Route event to current state
self.__router(&__e);
// Process any pending transition
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    // Exit current state (with exit_args from current compartment)
    let exit_event = StateVarParserFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    // Switch to new compartment
    self.__compartment = next_compartment;
    // Enter new state (or forward event)
    if self.__compartment.forward_event.is_none() {
        let enter_event = StateVarParserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        // Forward event to new state
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            // Forwarding enter event - just send it
            self.__router(&forward_event);
        } else {
            // Forwarding other event - send $> first, then forward
            let enter_event = StateVarParserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
    }

    fn __router(&mut self, __e: &StateVarParserFsmFrameEvent) {
match self.__compartment.state.as_str() {
    "Init" => self._state_Init(__e),
    "ScanIdent" => self._state_ScanIdent(__e),
    "CheckAssign" => self._state_CheckAssign(__e),
    "ScanExpr" => self._state_ScanExpr(__e),
    "Done" => self._state_Done(__e),
    _ => {}
}
    }

    fn __transition(&mut self, next_compartment: StateVarParserFsmCompartment) {
self.__next_compartment = Some(next_compartment);
    }

    fn _state_stack_push(&mut self) {
let state_context = match self.__compartment.state.as_str() {
    "Init" => StateVarParserFsmStateContext::Init,
    "ScanIdent" => StateVarParserFsmStateContext::ScanIdent,
    "CheckAssign" => StateVarParserFsmStateContext::CheckAssign,
    "ScanExpr" => StateVarParserFsmStateContext::ScanExpr,
    "Done" => StateVarParserFsmStateContext::Done,
    _ => StateVarParserFsmStateContext::Empty,
};
self._state_stack.push((self.__compartment.state.clone(), state_context));
    }

    fn _state_stack_pop(&mut self) {
let (saved_state, state_context) = self._state_stack.pop().unwrap();
let exit_event = StateVarParserFsmFrameEvent::new("$<");
self.__router(&exit_event);
self.__compartment.state = saved_state;
match state_context {
    StateVarParserFsmStateContext::Init => {}
    StateVarParserFsmStateContext::ScanIdent => {}
    StateVarParserFsmStateContext::CheckAssign => {}
    StateVarParserFsmStateContext::ScanExpr => {}
    StateVarParserFsmStateContext::Done => {}
    StateVarParserFsmStateContext::Empty => {}
}
    }

    pub fn do_parse(&mut self) {
let mut __e = StateVarParserFsmFrameEvent::new("do_parse");
let __ctx = StateVarParserFsmFrameContext::new(__e.clone(), None);
self._context_stack.push(__ctx);
match self.__compartment.state.as_str() {
            "Init" => { self._s_Init_do_parse(&__e); }
            _ => {}
        }
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    let exit_event = StateVarParserFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    self.__compartment = next_compartment;
    if self.__compartment.forward_event.is_none() {
        let enter_event = StateVarParserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            self.__router(&forward_event);
        } else {
            let enter_event = StateVarParserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
self._context_stack.pop();
    }

    fn _state_Done(&mut self, __e: &StateVarParserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_Done_enter(__e); }
    _ => {}
}
    }

    fn _state_Init(&mut self, __e: &StateVarParserFsmFrameEvent) {
match __e.message.as_str() {
    "do_parse" => { self._s_Init_do_parse(__e); }
    _ => {}
}
    }

    fn _state_ScanIdent(&mut self, __e: &StateVarParserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_ScanIdent_enter(__e); }
    _ => {}
}
    }

    fn _state_CheckAssign(&mut self, __e: &StateVarParserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_CheckAssign_enter(__e); }
    _ => {}
}
    }

    fn _state_ScanExpr(&mut self, __e: &StateVarParserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_ScanExpr_enter(__e); }
    _ => {}
}
    }

    fn _s_Done_enter(&mut self, __e: &StateVarParserFsmFrameEvent) {
// Terminal state — results in domain vars;
    }

    fn _s_Init_do_parse(&mut self, __e: &StateVarParserFsmFrameEvent) {
let mut __compartment = StateVarParserFsmCompartment::new("ScanIdent");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }

    fn _s_ScanIdent_enter(&mut self, __e: &StateVarParserFsmFrameEvent) {
// Skip "$." prefix and scan identifier
let mut i = self.pos + 2; // Skip "$."
let end = self.end;
let bytes = &self.bytes;

while i < end && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
    i += 1;
}

self.ident_end = i;
let mut __compartment = StateVarParserFsmCompartment::new("CheckAssign");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }

    fn _s_CheckAssign_enter(&mut self, __e: &StateVarParserFsmFrameEvent) {
// Lookahead: skip whitespace, check for = (but not ==)
let mut j = self.ident_end;
let end = self.end;
let bytes = &self.bytes;

while j < end && (bytes[j] == b' ' || bytes[j] == b'\t') {
    j += 1;
}

if j < end && bytes[j] == b'=' && (j + 1 >= end || bytes[j + 1] != b'=') {
    // Assignment detected
    j += 1; // Skip '='
    self.pos = j;
    self.is_assignment = true;
    let mut __compartment = StateVarParserFsmCompartment::new("ScanExpr");
    __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
    self.__transition(__compartment);
    return;
} else {
    // Read-only access
    self.result_end = self.ident_end;
    self.is_assignment = false;
    let mut __compartment = StateVarParserFsmCompartment::new("Done");
    __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
    self.__transition(__compartment);
    return;
}
    }

    fn _s_ScanExpr_enter(&mut self, __e: &StateVarParserFsmFrameEvent) {
// Create ExprScanner sub-machine (state manager pattern)
let bytes = &self.bytes;
let mut expr = ExprScannerFsm::new();
expr.bytes = bytes.to_vec();
expr.pos = self.pos;
expr.end = self.end;
expr.do_scan();
self.result_end = expr.result_end;
// expr is destroyed here
let mut __compartment = StateVarParserFsmCompartment::new("Done");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }
}
