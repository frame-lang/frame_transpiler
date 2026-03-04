
// Dogfooded body closer â C++ language brace matcher.
// Extends C with C++ raw string literals: R"delim(...)delim"
//
// State machine flow:
//   $Init.scan() â $Scanning.$>() â $InString/$InCharLiteral/$InLineComment/$InBlockComment/$InRawString

struct CppBodyCloserFsmFrameEvent {
    message: String,
    parameters: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl Clone for CppBodyCloserFsmFrameEvent {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            parameters: std::collections::HashMap::new(),
        }
    }
}

impl CppBodyCloserFsmFrameEvent {
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

struct CppBodyCloserFsmFrameContext {
    event: CppBodyCloserFsmFrameEvent,
    _return: Option<Box<dyn std::any::Any>>,
    _data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl CppBodyCloserFsmFrameContext {
    fn new(event: CppBodyCloserFsmFrameEvent, default_return: Option<Box<dyn std::any::Any>>) -> Self {
        Self {
            event,
            _return: default_return,
            _data: std::collections::HashMap::new(),
        }
    }
}

#[derive(Clone)]
struct CppBodyCloserFsmCompartment {
    state: String,
    state_vars: std::collections::HashMap<String, i32>,
    enter_args: std::collections::HashMap<String, String>,
    exit_args: std::collections::HashMap<String, String>,
    forward_event: Option<CppBodyCloserFsmFrameEvent>,
    parent_compartment: Option<Box<CppBodyCloserFsmCompartment>>,
}

impl CppBodyCloserFsmCompartment {
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
enum CppBodyCloserFsmStateContext {
    Init,
    Scanning,
    InString,
    InCharLiteral,
    InLineComment,
    InBlockComment,
    InRawString,
    Empty,
}

impl Default for CppBodyCloserFsmStateContext {
    fn default() -> Self {
        CppBodyCloserFsmStateContext::Init
    }
}

pub struct CppBodyCloserFsm {
    _state_stack: Vec<(String, CppBodyCloserFsmStateContext)>,
    __compartment: CppBodyCloserFsmCompartment,
    __next_compartment: Option<CppBodyCloserFsmCompartment>,
    _context_stack: Vec<CppBodyCloserFsmFrameContext>,
    pub bytes: Vec<u8>,
    pub pos: usize,
    pub depth: i32,
    pub result_pos: usize,
    pub error_kind: usize,
    pub error_msg: String,
    pub raw_delim: Vec<u8>,
}

impl CppBodyCloserFsm {
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
            raw_delim: Vec::new(),
            __compartment: CppBodyCloserFsmCompartment::new("Init"),
            __next_compartment: None,
        };
let __frame_event = CppBodyCloserFsmFrameEvent::new("$>");
this.__kernel(__frame_event);
        this
    }

    fn __kernel(&mut self, __e: CppBodyCloserFsmFrameEvent) {
// Route event to current state
self.__router(&__e);
// Process any pending transition
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    // Exit current state (with exit_args from current compartment)
    let exit_event = CppBodyCloserFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    // Switch to new compartment
    self.__compartment = next_compartment;
    // Enter new state (or forward event)
    if self.__compartment.forward_event.is_none() {
        let enter_event = CppBodyCloserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        // Forward event to new state
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            // Forwarding enter event - just send it
            self.__router(&forward_event);
        } else {
            // Forwarding other event - send $> first, then forward
            let enter_event = CppBodyCloserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
    }

    fn __router(&mut self, __e: &CppBodyCloserFsmFrameEvent) {
match self.__compartment.state.as_str() {
    "Init" => self._state_Init(__e),
    "Scanning" => self._state_Scanning(__e),
    "InString" => self._state_InString(__e),
    "InCharLiteral" => self._state_InCharLiteral(__e),
    "InLineComment" => self._state_InLineComment(__e),
    "InBlockComment" => self._state_InBlockComment(__e),
    "InRawString" => self._state_InRawString(__e),
    _ => {}
}
    }

    fn __transition(&mut self, next_compartment: CppBodyCloserFsmCompartment) {
self.__next_compartment = Some(next_compartment);
    }

    fn _state_stack_push(&mut self) {
let state_context = match self.__compartment.state.as_str() {
    "Init" => CppBodyCloserFsmStateContext::Init,
    "Scanning" => CppBodyCloserFsmStateContext::Scanning,
    "InString" => CppBodyCloserFsmStateContext::InString,
    "InCharLiteral" => CppBodyCloserFsmStateContext::InCharLiteral,
    "InLineComment" => CppBodyCloserFsmStateContext::InLineComment,
    "InBlockComment" => CppBodyCloserFsmStateContext::InBlockComment,
    "InRawString" => CppBodyCloserFsmStateContext::InRawString,
    _ => CppBodyCloserFsmStateContext::Empty,
};
self._state_stack.push((self.__compartment.state.clone(), state_context));
    }

    fn _state_stack_pop(&mut self) {
let (saved_state, state_context) = self._state_stack.pop().unwrap();
let exit_event = CppBodyCloserFsmFrameEvent::new("$<");
self.__router(&exit_event);
self.__compartment.state = saved_state;
match state_context {
    CppBodyCloserFsmStateContext::Init => {}
    CppBodyCloserFsmStateContext::Scanning => {}
    CppBodyCloserFsmStateContext::InString => {}
    CppBodyCloserFsmStateContext::InCharLiteral => {}
    CppBodyCloserFsmStateContext::InLineComment => {}
    CppBodyCloserFsmStateContext::InBlockComment => {}
    CppBodyCloserFsmStateContext::InRawString => {}
    CppBodyCloserFsmStateContext::Empty => {}
}
    }

    pub fn scan(&mut self) {
let mut __e = CppBodyCloserFsmFrameEvent::new("scan");
let __ctx = CppBodyCloserFsmFrameContext::new(__e.clone(), None);
self._context_stack.push(__ctx);
match self.__compartment.state.as_str() {
            "Init" => { self._s_Init_scan(&__e); }
            _ => {}
        }
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    let exit_event = CppBodyCloserFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    self.__compartment = next_compartment;
    if self.__compartment.forward_event.is_none() {
        let enter_event = CppBodyCloserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            self.__router(&forward_event);
        } else {
            let enter_event = CppBodyCloserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
self._context_stack.pop();
    }

    fn _state_InCharLiteral(&mut self, __e: &CppBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InCharLiteral_enter(__e); }
    _ => {}
}
    }

    fn _state_InLineComment(&mut self, __e: &CppBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InLineComment_enter(__e); }
    _ => {}
}
    }

    fn _state_InBlockComment(&mut self, __e: &CppBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InBlockComment_enter(__e); }
    _ => {}
}
    }

    fn _state_InRawString(&mut self, __e: &CppBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InRawString_enter(__e); }
    _ => {}
}
    }

    fn _state_Init(&mut self, __e: &CppBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "scan" => { self._s_Init_scan(__e); }
    _ => {}
}
    }

    fn _state_Scanning(&mut self, __e: &CppBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_Scanning_enter(__e); }
    _ => {}
}
    }

    fn _state_InString(&mut self, __e: &CppBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InString_enter(__e); }
    _ => {}
}
    }

    fn _s_InCharLiteral_enter(&mut self, __e: &CppBodyCloserFsmFrameEvent) {
let n = self.bytes.len();
while self.pos < n {
    if self.bytes[self.pos] == b'\\' {
        self.pos += 2;
        continue;
    }
    if self.bytes[self.pos] == b'\'' {
        self.pos += 1;
        let mut __compartment = CppBodyCloserFsmCompartment::new("Scanning");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment); return;
    }
    self.pos += 1;
}
self.error_kind = 1;
self.error_msg = "unterminated char".to_string();
    }

    fn _s_InLineComment_enter(&mut self, __e: &CppBodyCloserFsmFrameEvent) {
let n = self.bytes.len();
while self.pos < n && self.bytes[self.pos] != b'\n' {
    self.pos += 1;
}
let mut __compartment = CppBodyCloserFsmCompartment::new("Scanning");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment); return;
    }

    fn _s_InBlockComment_enter(&mut self, __e: &CppBodyCloserFsmFrameEvent) {
let n = self.bytes.len();
while self.pos + 1 < n {
    if self.bytes[self.pos] == b'*' && self.bytes[self.pos + 1] == b'/' {
        self.pos += 2;
        let mut __compartment = CppBodyCloserFsmCompartment::new("Scanning");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment); return;
    }
    self.pos += 1;
}
self.error_kind = 2;
self.error_msg = "unterminated comment".to_string();
    }

    fn _s_InRawString_enter(&mut self, __e: &CppBodyCloserFsmFrameEvent) {
// Find closing )delim"
let n = self.bytes.len();
loop {
    if self.pos >= n {
        self.error_kind = 4;
        self.error_msg = "unterminated raw".to_string();
        return
    }
    if self.bytes[self.pos] == b')' {
        let mut k = self.pos + 1;
        let mut m: usize = 0;
        while m < self.raw_delim.len() && k < n && self.bytes[k] == self.raw_delim[m] {
            k += 1;
            m += 1;
        }
        if m == self.raw_delim.len() && k < n && self.bytes[k] == b'"' {
            self.pos = k + 1;
            let mut __compartment = CppBodyCloserFsmCompartment::new("Scanning");
            __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
            self.__transition(__compartment); return;
        }
    }
    self.pos += 1;
}
    }

    fn _s_Init_scan(&mut self, __e: &CppBodyCloserFsmFrameEvent) {
let mut __compartment = CppBodyCloserFsmCompartment::new("Scanning");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment); return;
    }

    fn _s_Scanning_enter(&mut self, __e: &CppBodyCloserFsmFrameEvent) {
let n = self.bytes.len();
while self.pos < n {
    let b = self.bytes[self.pos];
    if b == b'\n' {
        self.pos += 1;
    } else if b == b'/' && self.pos + 1 < n && self.bytes[self.pos + 1] == b'/' {
        self.pos += 2;
        let mut __compartment = CppBodyCloserFsmCompartment::new("InLineComment");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment); return;
    } else if b == b'/' && self.pos + 1 < n && self.bytes[self.pos + 1] == b'*' {
        self.pos += 2;
        let mut __compartment = CppBodyCloserFsmCompartment::new("InBlockComment");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment); return;
    } else if b == b'\'' {
        self.pos += 1;
        let mut __compartment = CppBodyCloserFsmCompartment::new("InCharLiteral");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment); return;
    } else if b == b'"' {
        self.pos += 1;
        let mut __compartment = CppBodyCloserFsmCompartment::new("InString");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment); return;
    } else if b == b'R' && self.pos + 1 < n && self.bytes[self.pos + 1] == b'"' {
        // C++ raw string R"delim(...)delim"
        let mut j = self.pos + 2;
        let mut delim: Vec<u8> = Vec::new();
        while j < n && self.bytes[j] != b'(' {
            delim.push(self.bytes[j]);
            j += 1;
            if delim.len() > 32 { break; }
        }
        if j >= n || self.bytes[j] != b'(' {
            self.pos += 1;
            continue;
        }
        j += 1;
        self.raw_delim = delim;
        self.pos = j;
        let mut __compartment = CppBodyCloserFsmCompartment::new("InRawString");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment); return;
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

    fn _s_InString_enter(&mut self, __e: &CppBodyCloserFsmFrameEvent) {
let n = self.bytes.len();
while self.pos < n {
    if self.bytes[self.pos] == b'\\' {
        self.pos += 2;
        continue;
    }
    if self.bytes[self.pos] == b'"' {
        self.pos += 1;
        let mut __compartment = CppBodyCloserFsmCompartment::new("Scanning");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment); return;
    }
    self.pos += 1;
}
self.error_kind = 1;
self.error_msg = "unterminated string".to_string();
    }
}

