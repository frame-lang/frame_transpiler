
// Dogfooded body closer â TypeScript language brace matcher.
// TypeScript-specific: template literals with nested ${}, Frame V4 backtick statements.
//
// State machine flow:
//   $Init.scan() â $Scanning.$>() â $InString/$InTemplate/$InLineComment/$InBlockComment

struct TsBodyCloserFsmFrameEvent {
    message: String,
    parameters: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl Clone for TsBodyCloserFsmFrameEvent {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            parameters: std::collections::HashMap::new(),
        }
    }
}

impl TsBodyCloserFsmFrameEvent {
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

struct TsBodyCloserFsmFrameContext {
    event: TsBodyCloserFsmFrameEvent,
    _return: Option<Box<dyn std::any::Any>>,
    _data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl TsBodyCloserFsmFrameContext {
    fn new(event: TsBodyCloserFsmFrameEvent, default_return: Option<Box<dyn std::any::Any>>) -> Self {
        Self {
            event,
            _return: default_return,
            _data: std::collections::HashMap::new(),
        }
    }
}

#[derive(Clone)]
struct TsBodyCloserFsmCompartment {
    state: String,
    state_vars: std::collections::HashMap<String, i32>,
    enter_args: std::collections::HashMap<String, String>,
    exit_args: std::collections::HashMap<String, String>,
    forward_event: Option<TsBodyCloserFsmFrameEvent>,
    parent_compartment: Option<Box<TsBodyCloserFsmCompartment>>,
}

impl TsBodyCloserFsmCompartment {
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
enum TsBodyCloserFsmStateContext {
    Init,
    Scanning,
    InString,
    InTemplate,
    InLineComment,
    InBlockComment,
    Empty,
}

impl Default for TsBodyCloserFsmStateContext {
    fn default() -> Self {
        TsBodyCloserFsmStateContext::Init
    }
}

pub struct TsBodyCloserFsm {
    _state_stack: Vec<(String, TsBodyCloserFsmStateContext)>,
    __compartment: TsBodyCloserFsmCompartment,
    __next_compartment: Option<TsBodyCloserFsmCompartment>,
    _context_stack: Vec<TsBodyCloserFsmFrameContext>,
    pub bytes: Vec<u8>,
    pub pos: usize,
    pub depth: i32,
    pub result_pos: usize,
    pub error_kind: usize,
    pub error_msg: String,
    pub quote_char: u8,
}

impl TsBodyCloserFsm {
    pub fn new() -> Self {
        let mut this = Self {
            _state_stack: vec![],
            _context_stack: vec![],
            bytes: Vec::new(),
            pos: 0,
            depth: 1,
            result_pos: 0,
            error_kind: 0,
            error_msg: String::new(),
            quote_char: 0,
            __compartment: TsBodyCloserFsmCompartment::new("Init"),
            __next_compartment: None,
        };
let __frame_event = TsBodyCloserFsmFrameEvent::new("$>");
this.__kernel(__frame_event);
        this
    }

    fn __kernel(&mut self, __e: TsBodyCloserFsmFrameEvent) {
// Route event to current state
self.__router(&__e);
// Process any pending transition
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    // Exit current state (with exit_args from current compartment)
    let exit_event = TsBodyCloserFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    // Switch to new compartment
    self.__compartment = next_compartment;
    // Enter new state (or forward event)
    if self.__compartment.forward_event.is_none() {
        let enter_event = TsBodyCloserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        // Forward event to new state
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            // Forwarding enter event - just send it
            self.__router(&forward_event);
        } else {
            // Forwarding other event - send $> first, then forward
            let enter_event = TsBodyCloserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
    }

    fn __router(&mut self, __e: &TsBodyCloserFsmFrameEvent) {
match self.__compartment.state.as_str() {
    "Init" => self._state_Init(__e),
    "Scanning" => self._state_Scanning(__e),
    "InString" => self._state_InString(__e),
    "InTemplate" => self._state_InTemplate(__e),
    "InLineComment" => self._state_InLineComment(__e),
    "InBlockComment" => self._state_InBlockComment(__e),
    _ => {}
}
    }

    fn __transition(&mut self, next_compartment: TsBodyCloserFsmCompartment) {
self.__next_compartment = Some(next_compartment);
    }

    fn _state_stack_push(&mut self) {
let state_context = match self.__compartment.state.as_str() {
    "Init" => TsBodyCloserFsmStateContext::Init,
    "Scanning" => TsBodyCloserFsmStateContext::Scanning,
    "InString" => TsBodyCloserFsmStateContext::InString,
    "InTemplate" => TsBodyCloserFsmStateContext::InTemplate,
    "InLineComment" => TsBodyCloserFsmStateContext::InLineComment,
    "InBlockComment" => TsBodyCloserFsmStateContext::InBlockComment,
    _ => TsBodyCloserFsmStateContext::Empty,
};
self._state_stack.push((self.__compartment.state.clone(), state_context));
    }

    fn _state_stack_pop(&mut self) {
let (saved_state, state_context) = self._state_stack.pop().unwrap();
let exit_event = TsBodyCloserFsmFrameEvent::new("$<");
self.__router(&exit_event);
self.__compartment.state = saved_state;
match state_context {
    TsBodyCloserFsmStateContext::Init => {}
    TsBodyCloserFsmStateContext::Scanning => {}
    TsBodyCloserFsmStateContext::InString => {}
    TsBodyCloserFsmStateContext::InTemplate => {}
    TsBodyCloserFsmStateContext::InLineComment => {}
    TsBodyCloserFsmStateContext::InBlockComment => {}
    TsBodyCloserFsmStateContext::Empty => {}
}
    }

    pub fn scan(&mut self) {
let mut __e = TsBodyCloserFsmFrameEvent::new("scan");
let __ctx = TsBodyCloserFsmFrameContext::new(__e.clone(), None);
self._context_stack.push(__ctx);
match self.__compartment.state.as_str() {
            "Init" => { self._s_Init_scan(&__e); }
            _ => {}
        }
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    let exit_event = TsBodyCloserFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    self.__compartment = next_compartment;
    if self.__compartment.forward_event.is_none() {
        let enter_event = TsBodyCloserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            self.__router(&forward_event);
        } else {
            let enter_event = TsBodyCloserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
self._context_stack.pop();
    }

    fn _state_Init(&mut self, __e: &TsBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "scan" => { self._s_Init_scan(__e); }
    _ => {}
}
    }

    fn _state_InTemplate(&mut self, __e: &TsBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InTemplate_enter(__e); }
    _ => {}
}
    }

    fn _state_InLineComment(&mut self, __e: &TsBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InLineComment_enter(__e); }
    _ => {}
}
    }

    fn _state_Scanning(&mut self, __e: &TsBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_Scanning_enter(__e); }
    _ => {}
}
    }

    fn _state_InBlockComment(&mut self, __e: &TsBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InBlockComment_enter(__e); }
    _ => {}
}
    }

    fn _state_InString(&mut self, __e: &TsBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InString_enter(__e); }
    _ => {}
}
    }

    fn _s_Init_scan(&mut self, __e: &TsBodyCloserFsmFrameEvent) {
let mut __compartment = TsBodyCloserFsmCompartment::new("Scanning");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }

    fn _s_InTemplate_enter(&mut self, __e: &TsBodyCloserFsmFrameEvent) {
// Template literal with nested ${} expressions
let n = self.bytes.len();
let mut brace: i32 = 0;
while self.pos < n {
    if self.bytes[self.pos] == b'`' && brace == 0 {
        self.pos += 1;
        let mut __compartment = TsBodyCloserFsmCompartment::new("Scanning");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment);
        return;
    }
    if self.bytes[self.pos] == b'\\' {
        self.pos += 2;
        continue;
    }
    if self.bytes[self.pos] == b'$' && self.pos + 1 < n && self.bytes[self.pos + 1] == b'{' {
        brace += 1;
        self.pos += 2;
        continue;
    }
    if self.bytes[self.pos] == b'}' && brace > 0 {
        brace -= 1;
        self.pos += 1;
        continue;
    }
    self.pos += 1;
}
self.error_kind = 1;
self.error_msg = "unterminated template".to_string();
    }

    fn _s_InLineComment_enter(&mut self, __e: &TsBodyCloserFsmFrameEvent) {
let n = self.bytes.len();
while self.pos < n && self.bytes[self.pos] != b'\n' {
    self.pos += 1;
}
let mut __compartment = TsBodyCloserFsmCompartment::new("Scanning");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }

    fn _s_Scanning_enter(&mut self, __e: &TsBodyCloserFsmFrameEvent) {
let n = self.bytes.len();
while self.pos < n {
    let b = self.bytes[self.pos];
    if b == b'\n' {
        self.pos += 1;
    } else if b == b'/' && self.pos + 1 < n && self.bytes[self.pos + 1] == b'/' {
        self.pos += 2;
        let mut __compartment = TsBodyCloserFsmCompartment::new("InLineComment");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment);
        return;
    } else if b == b'/' && self.pos + 1 < n && self.bytes[self.pos + 1] == b'*' {
        self.pos += 2;
        let mut __compartment = TsBodyCloserFsmCompartment::new("InBlockComment");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment);
        return;
    } else if b == b'\'' || b == b'"' {
        self.quote_char = b;
        self.pos += 1;
        let mut __compartment = TsBodyCloserFsmCompartment::new("InString");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment);
        return;
    } else if b == b'`' {
        // Check for Frame V4 statements: `push$ or `-> pop$
        if self.pos + 5 < n && &self.bytes[self.pos + 1..self.pos + 6] == b"push$" {
            self.pos += 1;
            while self.pos < n && self.bytes[self.pos] != b'\n' { self.pos += 1; }
            continue;
        }
        if self.pos + 7 < n && &self.bytes[self.pos + 1..self.pos + 8] == b"-> pop$" {
            self.pos += 1;
            while self.pos < n && self.bytes[self.pos] != b'\n' { self.pos += 1; }
            continue;
        }
        self.pos += 1;
        let mut __compartment = TsBodyCloserFsmCompartment::new("InTemplate");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment);
        return;
    } else if b == b'{' {
        self.depth += 1;
        self.pos += 1;
    } else if b == b'}' {
        self.depth -= 1;
        self.pos += 1;
        if self.depth == 0 {
            self.result_pos = self.pos - 1;
            self.error_kind = 0;
            return
        }
    } else {
        self.pos += 1;
    }
}
self.error_kind = 3;
self.error_msg = "body not closed".to_string();
    }

    fn _s_InBlockComment_enter(&mut self, __e: &TsBodyCloserFsmFrameEvent) {
let n = self.bytes.len();
while self.pos + 1 < n {
    if self.bytes[self.pos] == b'*' && self.bytes[self.pos + 1] == b'/' {
        self.pos += 2;
        let mut __compartment = TsBodyCloserFsmCompartment::new("Scanning");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment);
        return;
    }
    self.pos += 1;
}
self.error_kind = 2;
self.error_msg = "unterminated comment".to_string();
    }

    fn _s_InString_enter(&mut self, __e: &TsBodyCloserFsmFrameEvent) {
let n = self.bytes.len();
while self.pos < n {
    if self.bytes[self.pos] == b'\\' {
        self.pos += 2;
        continue;
    }
    if self.bytes[self.pos] == self.quote_char {
        self.pos += 1;
        let mut __compartment = TsBodyCloserFsmCompartment::new("Scanning");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment);
        return;
    }
    self.pos += 1;
}
self.error_kind = 1;
self.error_msg = "unterminated string".to_string();
    }
}
