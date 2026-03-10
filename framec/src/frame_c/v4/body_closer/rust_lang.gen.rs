
// Dogfooded body closer â Rust language brace matcher.
// Extends C with nested block comments (/* /* */ */) and raw strings (r#"..."#).
//
// State machine flow:
//   $Init.scan() â $Scanning.$>() â $InString/$InCharLiteral/$InLineComment/$InBlockComment/$InRawString

struct RustBodyCloserFsmFrameEvent {
    message: String,
    parameters: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl Clone for RustBodyCloserFsmFrameEvent {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            parameters: std::collections::HashMap::new(),
        }
    }
}

impl RustBodyCloserFsmFrameEvent {
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

struct RustBodyCloserFsmFrameContext {
    event: RustBodyCloserFsmFrameEvent,
    _return: Option<Box<dyn std::any::Any>>,
    _data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl RustBodyCloserFsmFrameContext {
    fn new(event: RustBodyCloserFsmFrameEvent, default_return: Option<Box<dyn std::any::Any>>) -> Self {
        Self {
            event,
            _return: default_return,
            _data: std::collections::HashMap::new(),
        }
    }
}

#[derive(Clone)]
struct RustBodyCloserFsmCompartment {
    state: String,
    state_vars: std::collections::HashMap<String, i32>,
    enter_args: std::collections::HashMap<String, String>,
    exit_args: std::collections::HashMap<String, String>,
    forward_event: Option<RustBodyCloserFsmFrameEvent>,
    parent_compartment: Option<Box<RustBodyCloserFsmCompartment>>,
}

impl RustBodyCloserFsmCompartment {
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
enum RustBodyCloserFsmStateContext {
    Init,
    Scanning,
    InString,
    InCharLiteral,
    InLineComment,
    InBlockComment,
    InRawString,
    Empty,
}

impl Default for RustBodyCloserFsmStateContext {
    fn default() -> Self {
        RustBodyCloserFsmStateContext::Init
    }
}

pub struct RustBodyCloserFsm {
    _state_stack: Vec<(String, RustBodyCloserFsmStateContext)>,
    __compartment: RustBodyCloserFsmCompartment,
    __next_compartment: Option<RustBodyCloserFsmCompartment>,
    _context_stack: Vec<RustBodyCloserFsmFrameContext>,
    pub bytes: Vec<u8>,
    pub pos: usize,
    pub depth: i32,
    pub result_pos: usize,
    pub error_kind: usize,
    pub error_msg: String,
    pub block_comment_nest: i32,
    pub raw_hashes: usize,
}

impl RustBodyCloserFsm {
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
            block_comment_nest: 0,
            raw_hashes: 0,
            __compartment: RustBodyCloserFsmCompartment::new("Init"),
            __next_compartment: None,
        };
let __frame_event = RustBodyCloserFsmFrameEvent::new("$>");
this.__kernel(__frame_event);
        this
    }

    fn __kernel(&mut self, __e: RustBodyCloserFsmFrameEvent) {
// Route event to current state
self.__router(&__e);
// Process any pending transition
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    // Exit current state (with exit_args from current compartment)
    let exit_event = RustBodyCloserFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    // Switch to new compartment
    self.__compartment = next_compartment;
    // Enter new state (or forward event)
    if self.__compartment.forward_event.is_none() {
        let enter_event = RustBodyCloserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        // Forward event to new state
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            // Forwarding enter event - just send it
            self.__router(&forward_event);
        } else {
            // Forwarding other event - send $> first, then forward
            let enter_event = RustBodyCloserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
    }

    fn __router(&mut self, __e: &RustBodyCloserFsmFrameEvent) {
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

    fn __transition(&mut self, next_compartment: RustBodyCloserFsmCompartment) {
self.__next_compartment = Some(next_compartment);
    }

    fn _state_stack_push(&mut self) {
let state_context = match self.__compartment.state.as_str() {
    "Init" => RustBodyCloserFsmStateContext::Init,
    "Scanning" => RustBodyCloserFsmStateContext::Scanning,
    "InString" => RustBodyCloserFsmStateContext::InString,
    "InCharLiteral" => RustBodyCloserFsmStateContext::InCharLiteral,
    "InLineComment" => RustBodyCloserFsmStateContext::InLineComment,
    "InBlockComment" => RustBodyCloserFsmStateContext::InBlockComment,
    "InRawString" => RustBodyCloserFsmStateContext::InRawString,
    _ => RustBodyCloserFsmStateContext::Empty,
};
self._state_stack.push((self.__compartment.state.clone(), state_context));
    }

    fn _state_stack_pop(&mut self) {
let (saved_state, state_context) = self._state_stack.pop().unwrap();
let exit_event = RustBodyCloserFsmFrameEvent::new("$<");
self.__router(&exit_event);
self.__compartment.state = saved_state;
match state_context {
    RustBodyCloserFsmStateContext::Init => {}
    RustBodyCloserFsmStateContext::Scanning => {}
    RustBodyCloserFsmStateContext::InString => {}
    RustBodyCloserFsmStateContext::InCharLiteral => {}
    RustBodyCloserFsmStateContext::InLineComment => {}
    RustBodyCloserFsmStateContext::InBlockComment => {}
    RustBodyCloserFsmStateContext::InRawString => {}
    RustBodyCloserFsmStateContext::Empty => {}
}
    }

    pub fn scan(&mut self) {
let mut __e = RustBodyCloserFsmFrameEvent::new("scan");
let __ctx = RustBodyCloserFsmFrameContext::new(__e.clone(), None);
self._context_stack.push(__ctx);
match self.__compartment.state.as_str() {
            "Init" => { self._s_Init_scan(&__e); }
            _ => {}
        }
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    let exit_event = RustBodyCloserFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    self.__compartment = next_compartment;
    if self.__compartment.forward_event.is_none() {
        let enter_event = RustBodyCloserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            self.__router(&forward_event);
        } else {
            let enter_event = RustBodyCloserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
self._context_stack.pop();
    }

    fn _state_InString(&mut self, __e: &RustBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InString_enter(__e); }
    _ => {}
}
    }

    fn _state_InCharLiteral(&mut self, __e: &RustBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InCharLiteral_enter(__e); }
    _ => {}
}
    }

    fn _state_Init(&mut self, __e: &RustBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "scan" => { self._s_Init_scan(__e); }
    _ => {}
}
    }

    fn _state_Scanning(&mut self, __e: &RustBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_Scanning_enter(__e); }
    _ => {}
}
    }

    fn _state_InBlockComment(&mut self, __e: &RustBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InBlockComment_enter(__e); }
    _ => {}
}
    }

    fn _state_InRawString(&mut self, __e: &RustBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InRawString_enter(__e); }
    _ => {}
}
    }

    fn _state_InLineComment(&mut self, __e: &RustBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InLineComment_enter(__e); }
    _ => {}
}
    }

    fn _s_InString_enter(&mut self, __e: &RustBodyCloserFsmFrameEvent) {
let n = self.bytes.len();
while self.pos < n {
    if self.bytes[self.pos] == b'\\' {
        self.pos += 2;
        continue;
    }
    if self.bytes[self.pos] == b'"' {
        self.pos += 1;
        let mut __compartment = RustBodyCloserFsmCompartment::new("Scanning");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment);
        return;
    }
    self.pos += 1;
}
self.error_kind = 1;
self.error_msg = "unterminated string".to_string();
    }

    fn _s_InCharLiteral_enter(&mut self, __e: &RustBodyCloserFsmFrameEvent) {
let n = self.bytes.len();
while self.pos < n {
    if self.bytes[self.pos] == b'\\' {
        self.pos += 2;
        continue;
    }
    if self.bytes[self.pos] == b'\'' {
        self.pos += 1;
        let mut __compartment = RustBodyCloserFsmCompartment::new("Scanning");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment);
        return;
    }
    self.pos += 1;
}
self.error_kind = 1;
self.error_msg = "unterminated char".to_string();
    }

    fn _s_Init_scan(&mut self, __e: &RustBodyCloserFsmFrameEvent) {
let mut __compartment = RustBodyCloserFsmCompartment::new("Scanning");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }

    fn _s_Scanning_enter(&mut self, __e: &RustBodyCloserFsmFrameEvent) {
let n = self.bytes.len();
while self.pos < n {
    let b = self.bytes[self.pos];
    if b == b'\n' {
        self.pos += 1;
    } else if b == b'/' && self.pos + 1 < n && self.bytes[self.pos + 1] == b'/' {
        self.pos += 2;
        let mut __compartment = RustBodyCloserFsmCompartment::new("InLineComment");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment);
        return;
    } else if b == b'/' && self.pos + 1 < n && self.bytes[self.pos + 1] == b'*' {
        self.block_comment_nest = 1;
        self.pos += 2;
        let mut __compartment = RustBodyCloserFsmCompartment::new("InBlockComment");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment);
        return;
    } else if b == b'\'' {
        self.pos += 1;
        let mut __compartment = RustBodyCloserFsmCompartment::new("InCharLiteral");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment);
        return;
    } else if b == b'"' {
        self.pos += 1;
        let mut __compartment = RustBodyCloserFsmCompartment::new("InString");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment);
        return;
    } else if b == b'r' {
        // Rust raw string r#"..."# or just r"..."
        let mut j = self.pos + 1;
        let mut hashes: usize = 0;
        while j < n && self.bytes[j] == b'#' {
            hashes += 1;
            j += 1;
        }
        if j < n && self.bytes[j] == b'"' {
            self.raw_hashes = hashes;
            self.pos = j + 1;
            let mut __compartment = RustBodyCloserFsmCompartment::new("InRawString");
            __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
            self.__transition(__compartment);
            return;
        } else {
            self.pos += 1;
            continue;
        }
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

    fn _s_InBlockComment_enter(&mut self, __e: &RustBodyCloserFsmFrameEvent) {
// Rust supports nested block comments: /* /* */ */
let n = self.bytes.len();
while self.pos + 1 < n {
    if self.bytes[self.pos] == b'/' && self.bytes[self.pos + 1] == b'*' {
        self.block_comment_nest += 1;
        self.pos += 2;
        continue;
    }
    if self.bytes[self.pos] == b'*' && self.bytes[self.pos + 1] == b'/' {
        self.block_comment_nest -= 1;
        self.pos += 2;
        if self.block_comment_nest == 0 {
            let mut __compartment = RustBodyCloserFsmCompartment::new("Scanning");
            __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
            self.__transition(__compartment);
            return;
        }
        continue;
    }
    self.pos += 1;
}
self.error_kind = 2;
self.error_msg = "unterminated comment".to_string();
    }

    fn _s_InRawString_enter(&mut self, __e: &RustBodyCloserFsmFrameEvent) {
// Find closing "###
let n = self.bytes.len();
loop {
    if self.pos >= n {
        self.error_kind = 4;
        self.error_msg = "unterminated raw".to_string();
        return
    }
    if self.bytes[self.pos] == b'"' {
        let mut k = self.pos + 1;
        let mut m: usize = 0;
        while m < self.raw_hashes && k < n && self.bytes[k] == b'#' {
            m += 1;
            k += 1;
        }
        if m == self.raw_hashes {
            self.pos = k;
            let mut __compartment = RustBodyCloserFsmCompartment::new("Scanning");
            __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
            self.__transition(__compartment);
            return;
        }
    }
    self.pos += 1;
}
    }

    fn _s_InLineComment_enter(&mut self, __e: &RustBodyCloserFsmFrameEvent) {
let n = self.bytes.len();
while self.pos < n && self.bytes[self.pos] != b'\n' {
    self.pos += 1;
}
let mut __compartment = RustBodyCloserFsmCompartment::new("Scanning");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }
}
