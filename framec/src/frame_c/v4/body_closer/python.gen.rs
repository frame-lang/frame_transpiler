
// Dogfooded body closer â Python language brace matcher.
// Python-specific: triple quotes (''' and """), # line comments, string prefixes (f/r/b/F/R/B).
//
// State machine flow:
//   $Init.scan() â $Scanning.$>() â $InString/$InTripleString/$InLineComment

struct PythonBodyCloserFsmFrameEvent {
    message: String,
    parameters: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl Clone for PythonBodyCloserFsmFrameEvent {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            parameters: std::collections::HashMap::new(),
        }
    }
}

impl PythonBodyCloserFsmFrameEvent {
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

struct PythonBodyCloserFsmFrameContext {
    event: PythonBodyCloserFsmFrameEvent,
    _return: Option<Box<dyn std::any::Any>>,
    _data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl PythonBodyCloserFsmFrameContext {
    fn new(event: PythonBodyCloserFsmFrameEvent, default_return: Option<Box<dyn std::any::Any>>) -> Self {
        Self {
            event,
            _return: default_return,
            _data: std::collections::HashMap::new(),
        }
    }
}

#[derive(Clone)]
struct PythonBodyCloserFsmCompartment {
    state: String,
    state_vars: std::collections::HashMap<String, i32>,
    enter_args: std::collections::HashMap<String, String>,
    exit_args: std::collections::HashMap<String, String>,
    forward_event: Option<PythonBodyCloserFsmFrameEvent>,
    parent_compartment: Option<Box<PythonBodyCloserFsmCompartment>>,
}

impl PythonBodyCloserFsmCompartment {
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
enum PythonBodyCloserFsmStateContext {
    Init,
    Scanning,
    InString,
    InTripleString,
    InLineComment,
    Empty,
}

impl Default for PythonBodyCloserFsmStateContext {
    fn default() -> Self {
        PythonBodyCloserFsmStateContext::Init
    }
}

pub struct PythonBodyCloserFsm {
    _state_stack: Vec<(String, PythonBodyCloserFsmStateContext)>,
    __compartment: PythonBodyCloserFsmCompartment,
    __next_compartment: Option<PythonBodyCloserFsmCompartment>,
    _context_stack: Vec<PythonBodyCloserFsmFrameContext>,
    pub bytes: Vec<u8>,
    pub pos: usize,
    pub depth: i32,
    pub result_pos: usize,
    pub error_kind: usize,
    pub error_msg: String,
    pub quote_char: u8,
}

impl PythonBodyCloserFsm {
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
            __compartment: PythonBodyCloserFsmCompartment::new("Init"),
            __next_compartment: None,
        };
let __frame_event = PythonBodyCloserFsmFrameEvent::new("$>");
this.__kernel(__frame_event);
        this
    }

    fn __kernel(&mut self, __e: PythonBodyCloserFsmFrameEvent) {
// Route event to current state
self.__router(&__e);
// Process any pending transition
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    // Exit current state (with exit_args from current compartment)
    let exit_event = PythonBodyCloserFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    // Switch to new compartment
    self.__compartment = next_compartment;
    // Enter new state (or forward event)
    if self.__compartment.forward_event.is_none() {
        let enter_event = PythonBodyCloserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        // Forward event to new state
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            // Forwarding enter event - just send it
            self.__router(&forward_event);
        } else {
            // Forwarding other event - send $> first, then forward
            let enter_event = PythonBodyCloserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
    }

    fn __router(&mut self, __e: &PythonBodyCloserFsmFrameEvent) {
match self.__compartment.state.as_str() {
    "Init" => self._state_Init(__e),
    "Scanning" => self._state_Scanning(__e),
    "InString" => self._state_InString(__e),
    "InTripleString" => self._state_InTripleString(__e),
    "InLineComment" => self._state_InLineComment(__e),
    _ => {}
}
    }

    fn __transition(&mut self, next_compartment: PythonBodyCloserFsmCompartment) {
self.__next_compartment = Some(next_compartment);
    }

    fn _state_stack_push(&mut self) {
let state_context = match self.__compartment.state.as_str() {
    "Init" => PythonBodyCloserFsmStateContext::Init,
    "Scanning" => PythonBodyCloserFsmStateContext::Scanning,
    "InString" => PythonBodyCloserFsmStateContext::InString,
    "InTripleString" => PythonBodyCloserFsmStateContext::InTripleString,
    "InLineComment" => PythonBodyCloserFsmStateContext::InLineComment,
    _ => PythonBodyCloserFsmStateContext::Empty,
};
self._state_stack.push((self.__compartment.state.clone(), state_context));
    }

    fn _state_stack_pop(&mut self) {
let (saved_state, state_context) = self._state_stack.pop().unwrap();
let exit_event = PythonBodyCloserFsmFrameEvent::new("$<");
self.__router(&exit_event);
self.__compartment.state = saved_state;
match state_context {
    PythonBodyCloserFsmStateContext::Init => {}
    PythonBodyCloserFsmStateContext::Scanning => {}
    PythonBodyCloserFsmStateContext::InString => {}
    PythonBodyCloserFsmStateContext::InTripleString => {}
    PythonBodyCloserFsmStateContext::InLineComment => {}
    PythonBodyCloserFsmStateContext::Empty => {}
}
    }

    pub fn scan(&mut self) {
let mut __e = PythonBodyCloserFsmFrameEvent::new("scan");
let __ctx = PythonBodyCloserFsmFrameContext::new(__e.clone(), None);
self._context_stack.push(__ctx);
match self.__compartment.state.as_str() {
            "Init" => { self._s_Init_scan(&__e); }
            _ => {}
        }
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    let exit_event = PythonBodyCloserFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    self.__compartment = next_compartment;
    if self.__compartment.forward_event.is_none() {
        let enter_event = PythonBodyCloserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            self.__router(&forward_event);
        } else {
            let enter_event = PythonBodyCloserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
self._context_stack.pop();
    }

    fn _state_Init(&mut self, __e: &PythonBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "scan" => { self._s_Init_scan(__e); }
    _ => {}
}
    }

    fn _state_InTripleString(&mut self, __e: &PythonBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InTripleString_enter(__e); }
    _ => {}
}
    }

    fn _state_InLineComment(&mut self, __e: &PythonBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InLineComment_enter(__e); }
    _ => {}
}
    }

    fn _state_InString(&mut self, __e: &PythonBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InString_enter(__e); }
    _ => {}
}
    }

    fn _state_Scanning(&mut self, __e: &PythonBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_Scanning_enter(__e); }
    _ => {}
}
    }

    fn _s_Init_scan(&mut self, __e: &PythonBodyCloserFsmFrameEvent) {
let mut __compartment = PythonBodyCloserFsmCompartment::new("Scanning");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment); return;
    }

    fn _s_InTripleString_enter(&mut self, __e: &PythonBodyCloserFsmFrameEvent) {
let n = self.bytes.len();
let q = self.quote_char;
while self.pos < n {
    if self.bytes[self.pos] == q && self.pos + 2 < n && self.bytes[self.pos + 1] == q && self.bytes[self.pos + 2] == q {
        self.pos += 3;
        let mut __compartment = PythonBodyCloserFsmCompartment::new("Scanning");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment); return;
    }
    self.pos += 1;
}
self.error_kind = 1;
self.error_msg = "unterminated string".to_string();
    }

    fn _s_InLineComment_enter(&mut self, __e: &PythonBodyCloserFsmFrameEvent) {
let n = self.bytes.len();
while self.pos < n && self.bytes[self.pos] != b'\n' {
    self.pos += 1;
}
let mut __compartment = PythonBodyCloserFsmCompartment::new("Scanning");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment); return;
    }

    fn _s_InString_enter(&mut self, __e: &PythonBodyCloserFsmFrameEvent) {
let n = self.bytes.len();
while self.pos < n {
    if self.bytes[self.pos] == b'\\' {
        self.pos += 2;
        continue;
    }
    if self.bytes[self.pos] == self.quote_char {
        self.pos += 1;
        let mut __compartment = PythonBodyCloserFsmCompartment::new("Scanning");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment); return;
    }
    self.pos += 1;
}
self.error_kind = 1;
self.error_msg = "unterminated string".to_string();
    }

    fn _s_Scanning_enter(&mut self, __e: &PythonBodyCloserFsmFrameEvent) {
let n = self.bytes.len();
while self.pos < n {
    let b = self.bytes[self.pos];
    if b == b'\n' {
        self.pos += 1;
    } else if b == b'#' {
        self.pos += 1;
        let mut __compartment = PythonBodyCloserFsmCompartment::new("InLineComment");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment); return;
    } else if b == b'\'' || b == b'"' {
        let q = b;
        if self.pos + 2 < n && self.bytes[self.pos + 1] == q && self.bytes[self.pos + 2] == q {
            self.quote_char = q;
            self.pos += 3;
            let mut __compartment = PythonBodyCloserFsmCompartment::new("InTripleString");
            __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
            self.__transition(__compartment); return;
        } else {
            self.quote_char = q;
            self.pos += 1;
            let mut __compartment = PythonBodyCloserFsmCompartment::new("InString");
            __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
            self.__transition(__compartment); return;
        }
    } else if b == b'f' || b == b'F' || b == b'r' || b == b'R' || b == b'b' || b == b'B' {
        // String prefixes like f"..", r'..', etc.
        if self.pos + 1 < n && (self.bytes[self.pos + 1] == b'\'' || self.bytes[self.pos + 1] == b'"') {
            let q = self.bytes[self.pos + 1];
            if self.pos + 3 < n && self.bytes[self.pos + 2] == q && self.bytes[self.pos + 3] == q {
                self.quote_char = q;
                self.pos += 4;
                let mut __compartment = PythonBodyCloserFsmCompartment::new("InTripleString");
                __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
                self.__transition(__compartment); return;
            } else {
                self.quote_char = q;
                self.pos += 2;
                let mut __compartment = PythonBodyCloserFsmCompartment::new("InString");
                __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
                self.__transition(__compartment); return;
            }
        } else {
            self.pos += 1;
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
}

