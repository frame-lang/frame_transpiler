
// ContextParser â FSM for parsing all @@ context constructs.
//
// Dispatches on the character after @@ to parse:
//   @@.param           â ContextParamShorthand (kind=1)
//   @@:return [= expr] â ContextReturn (kind=2)
//   @@:event           â ContextEvent (kind=3)
//   @@:data[key] [= e] â ContextData (kind=4) or ContextDataAssign (kind=5)
//   @@:params[key]     â ContextParams (kind=6)
//   @@SystemName()     â TaggedInstantiation (kind=7)
//   other              â no match (has_result=false)
//
// Demonstrates hierarchical composition: $ParseReturn and $ParseData
// create ExprScannerFsm sub-machines when they detect assignment `=`.

include!("expr_scanner.gen.rs");

struct ContextParserFsmFrameEvent {
    message: String,
    parameters: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl Clone for ContextParserFsmFrameEvent {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            parameters: std::collections::HashMap::new(),
        }
    }
}

impl ContextParserFsmFrameEvent {
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

struct ContextParserFsmFrameContext {
    event: ContextParserFsmFrameEvent,
    _return: Option<Box<dyn std::any::Any>>,
    _data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl ContextParserFsmFrameContext {
    fn new(event: ContextParserFsmFrameEvent, default_return: Option<Box<dyn std::any::Any>>) -> Self {
        Self {
            event,
            _return: default_return,
            _data: std::collections::HashMap::new(),
        }
    }
}

#[derive(Clone)]
struct ContextParserFsmCompartment {
    state: String,
    state_vars: std::collections::HashMap<String, i32>,
    enter_args: std::collections::HashMap<String, String>,
    exit_args: std::collections::HashMap<String, String>,
    forward_event: Option<ContextParserFsmFrameEvent>,
    parent_compartment: Option<Box<ContextParserFsmCompartment>>,
}

impl ContextParserFsmCompartment {
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
enum ContextParserFsmStateContext {
    Init,
    Dispatching,
    ParseParamShorthand,
    DispatchColon,
    ParseReturn,
    ParseData,
    ParseParams,
    ParseInstantiation,
    Done,
    Empty,
}

impl Default for ContextParserFsmStateContext {
    fn default() -> Self {
        ContextParserFsmStateContext::Init
    }
}

pub struct ContextParserFsm {
    _state_stack: Vec<(String, ContextParserFsmStateContext)>,
    __compartment: ContextParserFsmCompartment,
    __next_compartment: Option<ContextParserFsmCompartment>,
    _context_stack: Vec<ContextParserFsmFrameContext>,
    pub bytes: Vec<u8>,
    pub pos: usize,
    pub end: usize,
    pub result_end: usize,
    pub result_kind: usize,
    pub has_result: bool,
    pub paren_end: usize,
}

impl ContextParserFsm {
    pub fn new() -> Self {
        let mut this = Self {
            _state_stack: vec![],
            _context_stack: vec![],
            bytes: Vec::new(),
            pos: 0,
            end: 0,
            result_end: 0,
            result_kind: 0,
            has_result: false,
            paren_end: 0,
            __compartment: ContextParserFsmCompartment::new("Init"),
            __next_compartment: None,
        };
let __frame_event = ContextParserFsmFrameEvent::new("$>");
this.__kernel(__frame_event);
        this
    }

    fn __kernel(&mut self, __e: ContextParserFsmFrameEvent) {
// Route event to current state
self.__router(&__e);
// Process any pending transition
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    // Exit current state (with exit_args from current compartment)
    let exit_event = ContextParserFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    // Switch to new compartment
    self.__compartment = next_compartment;
    // Enter new state (or forward event)
    if self.__compartment.forward_event.is_none() {
        let enter_event = ContextParserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        // Forward event to new state
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            // Forwarding enter event - just send it
            self.__router(&forward_event);
        } else {
            // Forwarding other event - send $> first, then forward
            let enter_event = ContextParserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
    }

    fn __router(&mut self, __e: &ContextParserFsmFrameEvent) {
match self.__compartment.state.as_str() {
    "Init" => self._state_Init(__e),
    "Dispatching" => self._state_Dispatching(__e),
    "ParseParamShorthand" => self._state_ParseParamShorthand(__e),
    "DispatchColon" => self._state_DispatchColon(__e),
    "ParseReturn" => self._state_ParseReturn(__e),
    "ParseData" => self._state_ParseData(__e),
    "ParseParams" => self._state_ParseParams(__e),
    "ParseInstantiation" => self._state_ParseInstantiation(__e),
    "Done" => self._state_Done(__e),
    _ => {}
}
    }

    fn __transition(&mut self, next_compartment: ContextParserFsmCompartment) {
self.__next_compartment = Some(next_compartment);
    }

    fn _state_stack_push(&mut self) {
let state_context = match self.__compartment.state.as_str() {
    "Init" => ContextParserFsmStateContext::Init,
    "Dispatching" => ContextParserFsmStateContext::Dispatching,
    "ParseParamShorthand" => ContextParserFsmStateContext::ParseParamShorthand,
    "DispatchColon" => ContextParserFsmStateContext::DispatchColon,
    "ParseReturn" => ContextParserFsmStateContext::ParseReturn,
    "ParseData" => ContextParserFsmStateContext::ParseData,
    "ParseParams" => ContextParserFsmStateContext::ParseParams,
    "ParseInstantiation" => ContextParserFsmStateContext::ParseInstantiation,
    "Done" => ContextParserFsmStateContext::Done,
    _ => ContextParserFsmStateContext::Empty,
};
self._state_stack.push((self.__compartment.state.clone(), state_context));
    }

    fn _state_stack_pop(&mut self) {
let (saved_state, state_context) = self._state_stack.pop().unwrap();
let exit_event = ContextParserFsmFrameEvent::new("$<");
self.__router(&exit_event);
self.__compartment.state = saved_state;
match state_context {
    ContextParserFsmStateContext::Init => {}
    ContextParserFsmStateContext::Dispatching => {}
    ContextParserFsmStateContext::ParseParamShorthand => {}
    ContextParserFsmStateContext::DispatchColon => {}
    ContextParserFsmStateContext::ParseReturn => {}
    ContextParserFsmStateContext::ParseData => {}
    ContextParserFsmStateContext::ParseParams => {}
    ContextParserFsmStateContext::ParseInstantiation => {}
    ContextParserFsmStateContext::Done => {}
    ContextParserFsmStateContext::Empty => {}
}
    }

    pub fn do_parse(&mut self) {
let mut __e = ContextParserFsmFrameEvent::new("do_parse");
let __ctx = ContextParserFsmFrameContext::new(__e.clone(), None);
self._context_stack.push(__ctx);
match self.__compartment.state.as_str() {
            "Init" => { self._s_Init_do_parse(&__e); }
            _ => {}
        }
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    let exit_event = ContextParserFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    self.__compartment = next_compartment;
    if self.__compartment.forward_event.is_none() {
        let enter_event = ContextParserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            self.__router(&forward_event);
        } else {
            let enter_event = ContextParserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
self._context_stack.pop();
    }

    fn _state_Done(&mut self, __e: &ContextParserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_Done_enter(__e); }
    _ => {}
}
    }

    fn _state_ParseParamShorthand(&mut self, __e: &ContextParserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_ParseParamShorthand_enter(__e); }
    _ => {}
}
    }

    fn _state_ParseReturn(&mut self, __e: &ContextParserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_ParseReturn_enter(__e); }
    _ => {}
}
    }

    fn _state_DispatchColon(&mut self, __e: &ContextParserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_DispatchColon_enter(__e); }
    _ => {}
}
    }

    fn _state_ParseParams(&mut self, __e: &ContextParserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_ParseParams_enter(__e); }
    _ => {}
}
    }

    fn _state_ParseInstantiation(&mut self, __e: &ContextParserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_ParseInstantiation_enter(__e); }
    _ => {}
}
    }

    fn _state_Dispatching(&mut self, __e: &ContextParserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_Dispatching_enter(__e); }
    _ => {}
}
    }

    fn _state_Init(&mut self, __e: &ContextParserFsmFrameEvent) {
match __e.message.as_str() {
    "do_parse" => { self._s_Init_do_parse(__e); }
    _ => {}
}
    }

    fn _state_ParseData(&mut self, __e: &ContextParserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_ParseData_enter(__e); }
    _ => {}
}
    }

    fn _s_Done_enter(&mut self, __e: &ContextParserFsmFrameEvent) {
// Terminal state — results are in domain vars;
    }

    fn _s_ParseParamShorthand_enter(&mut self, __e: &ContextParserFsmFrameEvent) {
// @@.param — scan identifier after '.'
let mut i = self.pos;
let end = self.end;
let bytes = &self.bytes;

while i < end && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
    i += 1;
}

self.result_end = i;
self.result_kind = 1; // ContextParamShorthand
self.has_result = true;
let mut __compartment = ContextParserFsmCompartment::new("Done");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }

    fn _s_ParseReturn_enter(&mut self, __e: &ContextParserFsmFrameEvent) {
// @@:return — check for assignment, optionally create ExprScanner PDA
let mut i = self.pos;
let end = self.end;
let bytes = &self.bytes;

// Skip whitespace
while i < end && (bytes[i] == b' ' || bytes[i] == b'\t') {
    i += 1;
}

if i < end && bytes[i] == b'=' && (i + 1 >= end || bytes[i + 1] != b'=') {
    // @@:return = <expr> — create ExprScanner sub-machine
    i += 1; // Skip '='
    let mut expr = ExprScannerFsm::new();
    expr.bytes = bytes.to_vec();
    expr.pos = i;
    expr.end = end;
    expr.do_scan();
    i = expr.result_end;
    // expr is destroyed here (state manager pattern)
}

self.result_end = i;
self.result_kind = 2; // ContextReturn
self.has_result = true;
let mut __compartment = ContextParserFsmCompartment::new("Done");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }

    fn _s_DispatchColon_enter(&mut self, __e: &ContextParserFsmFrameEvent) {
// @@: — dispatch on the keyword after ':'
let i = self.pos;
let end = self.end;
let bytes = &self.bytes;

if i + 5 < end && &bytes[i..i + 6] == b"return" {
    self.pos = i + 6;
    let mut __compartment = ContextParserFsmCompartment::new("ParseReturn");
    __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
    self.__transition(__compartment);
    return;
} else if i + 4 < end && &bytes[i..i + 5] == b"event" {
    self.result_end = i + 5;
    self.result_kind = 3; // ContextEvent
    self.has_result = true;
    let mut __compartment = ContextParserFsmCompartment::new("Done");
    __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
    self.__transition(__compartment);
    return;
} else if i + 3 < end && &bytes[i..i + 4] == b"data" {
    self.pos = i + 4;
    let mut __compartment = ContextParserFsmCompartment::new("ParseData");
    __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
    self.__transition(__compartment);
    return;
} else if i + 5 < end && &bytes[i..i + 6] == b"params" {
    self.pos = i + 6;
    let mut __compartment = ContextParserFsmCompartment::new("ParseParams");
    __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
    self.__transition(__compartment);
    return;
} else {
    // Unknown @@: variant
    self.result_end = i;
    self.has_result = false;
    let mut __compartment = ContextParserFsmCompartment::new("Done");
    __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
    self.__transition(__compartment);
    return;
}
    }

    fn _s_ParseParams_enter(&mut self, __e: &ContextParserFsmFrameEvent) {
// @@:params[key]
let mut i = self.pos;
let end = self.end;
let bytes = &self.bytes;

if i < end && bytes[i] == b'[' {
    while i < end && bytes[i] != b']' {
        i += 1;
    }
    if i < end {
        i += 1; // Skip ']'
    }
}

self.result_end = i;
self.result_kind = 6; // ContextParams
self.has_result = true;
let mut __compartment = ContextParserFsmCompartment::new("Done");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }

    fn _s_ParseInstantiation_enter(&mut self, __e: &ContextParserFsmFrameEvent) {
// @@SystemName() — scan name, find balanced parens
let mut i = self.pos;
let end = self.end;
let bytes = &self.bytes;

// Scan identifier
while i < end && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
    i += 1;
}

// Must be followed by (
if i < end && bytes[i] == b'(' {
    // Use the pre-computed paren_end if available
    if self.paren_end > 0 {
        i = self.paren_end;
        self.result_end = i;
        self.result_kind = 7; // TaggedInstantiation
        self.has_result = true;
    } else {
        // No paren_end provided — caller must handle
        self.result_end = i;
        self.has_result = false;
    }
} else {
    // @@SomeName without () — treat as native
    self.result_end = i;
    self.has_result = false;
}

let mut __compartment = ContextParserFsmCompartment::new("Done");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }

    fn _s_Dispatching_enter(&mut self, __e: &ContextParserFsmFrameEvent) {
let i = self.pos;
let end = self.end;
let bytes = &self.bytes;

if i >= end {
    self.has_result = false;
    let mut __compartment = ContextParserFsmCompartment::new("Done");
    __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
    self.__transition(__compartment);
    return;
}

let b = bytes[i];

if b == b'.' {
    self.pos = i + 1;
    let mut __compartment = ContextParserFsmCompartment::new("ParseParamShorthand");
    __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
    self.__transition(__compartment);
    return;
} else if b == b':' {
    self.pos = i + 1;
    let mut __compartment = ContextParserFsmCompartment::new("DispatchColon");
    __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
    self.__transition(__compartment);
    return;
} else if b.is_ascii_uppercase() {
    // @@SystemName — pos stays at start of name
    let mut __compartment = ContextParserFsmCompartment::new("ParseInstantiation");
    __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
    self.__transition(__compartment);
    return;
} else {
    // Just @@ without . or : or uppercase
    self.result_end = i;
    self.has_result = false;
    let mut __compartment = ContextParserFsmCompartment::new("Done");
    __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
    self.__transition(__compartment);
    return;
}
    }

    fn _s_Init_do_parse(&mut self, __e: &ContextParserFsmFrameEvent) {
let mut __compartment = ContextParserFsmCompartment::new("Dispatching");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }

    fn _s_ParseData_enter(&mut self, __e: &ContextParserFsmFrameEvent) {
// @@:data[key] or @@:data[key] = expr
let mut i = self.pos;
let end = self.end;
let bytes = &self.bytes;

// Scan [key]
if i < end && bytes[i] == b'[' {
    while i < end && bytes[i] != b']' {
        i += 1;
    }
    if i < end {
        i += 1; // Skip ']'
    }
}

// Check for assignment
let mut j = i;
while j < end && (bytes[j] == b' ' || bytes[j] == b'\t') {
    j += 1;
}

if j < end && bytes[j] == b'=' && (j + 1 >= end || bytes[j + 1] != b'=') {
    // @@:data[key] = expr — create ExprScanner sub-machine
    j += 1; // Skip '='
    let mut expr = ExprScannerFsm::new();
    expr.bytes = bytes.to_vec();
    expr.pos = j;
    expr.end = end;
    expr.do_scan();
    self.result_end = expr.result_end;
    // expr is destroyed here (state manager pattern)
    self.result_kind = 5; // ContextDataAssign
} else {
    self.result_end = i;
    self.result_kind = 4; // ContextData
}

self.has_result = true;
let mut __compartment = ContextParserFsmCompartment::new("Done");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }
}
