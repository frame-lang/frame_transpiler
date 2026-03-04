
// Dogfooded body closer â C# language brace matcher.
// C#-specific: verbatim strings (@"..."), interpolated ($"..."), raw ($"""..."""),
// combined (@$"..." / $@"..."), preprocessor directives (#region etc.), char literals.
//
// State machine flow:
//   $Init.scan() â $Scanning.$>() â $InString/$InCharLiteral/$InVerbatimString/$InRawString
//                                    /$InLineComment/$InBlockComment/$InPreprocessor

struct CsBodyCloserFsmFrameEvent {
    message: String,
    parameters: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl Clone for CsBodyCloserFsmFrameEvent {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            parameters: std::collections::HashMap::new(),
        }
    }
}

impl CsBodyCloserFsmFrameEvent {
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

struct CsBodyCloserFsmFrameContext {
    event: CsBodyCloserFsmFrameEvent,
    _return: Option<Box<dyn std::any::Any>>,
    _data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl CsBodyCloserFsmFrameContext {
    fn new(event: CsBodyCloserFsmFrameEvent, default_return: Option<Box<dyn std::any::Any>>) -> Self {
        Self {
            event,
            _return: default_return,
            _data: std::collections::HashMap::new(),
        }
    }
}

#[derive(Clone)]
struct CsBodyCloserFsmCompartment {
    state: String,
    state_vars: std::collections::HashMap<String, i32>,
    enter_args: std::collections::HashMap<String, String>,
    exit_args: std::collections::HashMap<String, String>,
    forward_event: Option<CsBodyCloserFsmFrameEvent>,
    parent_compartment: Option<Box<CsBodyCloserFsmCompartment>>,
}

impl CsBodyCloserFsmCompartment {
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
enum CsBodyCloserFsmStateContext {
    Init,
    Scanning,
    InString,
    InCharLiteral,
    InVerbatimString,
    InRawString,
    InLineComment,
    InBlockComment,
    InPreprocessor,
    Empty,
}

impl Default for CsBodyCloserFsmStateContext {
    fn default() -> Self {
        CsBodyCloserFsmStateContext::Init
    }
}

pub struct CsBodyCloserFsm {
    _state_stack: Vec<(String, CsBodyCloserFsmStateContext)>,
    __compartment: CsBodyCloserFsmCompartment,
    __next_compartment: Option<CsBodyCloserFsmCompartment>,
    _context_stack: Vec<CsBodyCloserFsmFrameContext>,
    pub bytes: Vec<u8>,
    pub pos: usize,
    pub depth: i32,
    pub result_pos: usize,
    pub error_kind: usize,
    pub error_msg: String,
    pub raw_quotes: usize,
}

impl CsBodyCloserFsm {
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
            raw_quotes: 0,
            __compartment: CsBodyCloserFsmCompartment::new("Init"),
            __next_compartment: None,
        };
let __frame_event = CsBodyCloserFsmFrameEvent::new("$>");
this.__kernel(__frame_event);
        this
    }

    fn __kernel(&mut self, __e: CsBodyCloserFsmFrameEvent) {
// Route event to current state
self.__router(&__e);
// Process any pending transition
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    // Exit current state (with exit_args from current compartment)
    let exit_event = CsBodyCloserFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    // Switch to new compartment
    self.__compartment = next_compartment;
    // Enter new state (or forward event)
    if self.__compartment.forward_event.is_none() {
        let enter_event = CsBodyCloserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        // Forward event to new state
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            // Forwarding enter event - just send it
            self.__router(&forward_event);
        } else {
            // Forwarding other event - send $> first, then forward
            let enter_event = CsBodyCloserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
    }

    fn __router(&mut self, __e: &CsBodyCloserFsmFrameEvent) {
match self.__compartment.state.as_str() {
    "Init" => self._state_Init(__e),
    "Scanning" => self._state_Scanning(__e),
    "InString" => self._state_InString(__e),
    "InCharLiteral" => self._state_InCharLiteral(__e),
    "InVerbatimString" => self._state_InVerbatimString(__e),
    "InRawString" => self._state_InRawString(__e),
    "InLineComment" => self._state_InLineComment(__e),
    "InBlockComment" => self._state_InBlockComment(__e),
    "InPreprocessor" => self._state_InPreprocessor(__e),
    _ => {}
}
    }

    fn __transition(&mut self, next_compartment: CsBodyCloserFsmCompartment) {
self.__next_compartment = Some(next_compartment);
    }

    fn _state_stack_push(&mut self) {
let state_context = match self.__compartment.state.as_str() {
    "Init" => CsBodyCloserFsmStateContext::Init,
    "Scanning" => CsBodyCloserFsmStateContext::Scanning,
    "InString" => CsBodyCloserFsmStateContext::InString,
    "InCharLiteral" => CsBodyCloserFsmStateContext::InCharLiteral,
    "InVerbatimString" => CsBodyCloserFsmStateContext::InVerbatimString,
    "InRawString" => CsBodyCloserFsmStateContext::InRawString,
    "InLineComment" => CsBodyCloserFsmStateContext::InLineComment,
    "InBlockComment" => CsBodyCloserFsmStateContext::InBlockComment,
    "InPreprocessor" => CsBodyCloserFsmStateContext::InPreprocessor,
    _ => CsBodyCloserFsmStateContext::Empty,
};
self._state_stack.push((self.__compartment.state.clone(), state_context));
    }

    fn _state_stack_pop(&mut self) {
let (saved_state, state_context) = self._state_stack.pop().unwrap();
let exit_event = CsBodyCloserFsmFrameEvent::new("$<");
self.__router(&exit_event);
self.__compartment.state = saved_state;
match state_context {
    CsBodyCloserFsmStateContext::Init => {}
    CsBodyCloserFsmStateContext::Scanning => {}
    CsBodyCloserFsmStateContext::InString => {}
    CsBodyCloserFsmStateContext::InCharLiteral => {}
    CsBodyCloserFsmStateContext::InVerbatimString => {}
    CsBodyCloserFsmStateContext::InRawString => {}
    CsBodyCloserFsmStateContext::InLineComment => {}
    CsBodyCloserFsmStateContext::InBlockComment => {}
    CsBodyCloserFsmStateContext::InPreprocessor => {}
    CsBodyCloserFsmStateContext::Empty => {}
}
    }

    pub fn scan(&mut self) {
let mut __e = CsBodyCloserFsmFrameEvent::new("scan");
let __ctx = CsBodyCloserFsmFrameContext::new(__e.clone(), None);
self._context_stack.push(__ctx);
match self.__compartment.state.as_str() {
            "Init" => { self._s_Init_scan(&__e); }
            _ => {}
        }
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    let exit_event = CsBodyCloserFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    self.__compartment = next_compartment;
    if self.__compartment.forward_event.is_none() {
        let enter_event = CsBodyCloserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            self.__router(&forward_event);
        } else {
            let enter_event = CsBodyCloserFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
self._context_stack.pop();
    }

    fn _state_Init(&mut self, __e: &CsBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "scan" => { self._s_Init_scan(__e); }
    _ => {}
}
    }

    fn _state_InString(&mut self, __e: &CsBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InString_enter(__e); }
    _ => {}
}
    }

    fn _state_InRawString(&mut self, __e: &CsBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InRawString_enter(__e); }
    _ => {}
}
    }

    fn _state_Scanning(&mut self, __e: &CsBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_Scanning_enter(__e); }
    _ => {}
}
    }

    fn _state_InBlockComment(&mut self, __e: &CsBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InBlockComment_enter(__e); }
    _ => {}
}
    }

    fn _state_InCharLiteral(&mut self, __e: &CsBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InCharLiteral_enter(__e); }
    _ => {}
}
    }

    fn _state_InPreprocessor(&mut self, __e: &CsBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InPreprocessor_enter(__e); }
    _ => {}
}
    }

    fn _state_InVerbatimString(&mut self, __e: &CsBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InVerbatimString_enter(__e); }
    _ => {}
}
    }

    fn _state_InLineComment(&mut self, __e: &CsBodyCloserFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_InLineComment_enter(__e); }
    _ => {}
}
    }

    fn _s_Init_scan(&mut self, __e: &CsBodyCloserFsmFrameEvent) {
let mut __compartment = CsBodyCloserFsmCompartment::new("Scanning");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment); return;
    }

    fn _s_InString_enter(&mut self, __e: &CsBodyCloserFsmFrameEvent) {
let n = self.bytes.len();
while self.pos < n {
    if self.bytes[self.pos] == b'\\' {
        self.pos += 2;
        continue;
    }
    if self.bytes[self.pos] == b'"' {
        self.pos += 1;
        let mut __compartment = CsBodyCloserFsmCompartment::new("Scanning");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment); return;
    }
    self.pos += 1;
}
self.error_kind = 1;
self.error_msg = "unterminated string".to_string();
    }

    fn _s_InRawString_enter(&mut self, __e: &CsBodyCloserFsmFrameEvent) {
// Raw string: close when we see N consecutive quotes (where N = raw_quotes)
let n = self.bytes.len();
while self.pos < n {
    if self.bytes[self.pos] == b'"' {
        let mut q: usize = 0;
        let mut p = self.pos;
        while p < n && self.bytes[p] == b'"' { q += 1; p += 1; }
        if q >= self.raw_quotes {
            self.pos = p;
            let mut __compartment = CsBodyCloserFsmCompartment::new("Scanning");
            __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
            self.__transition(__compartment); return;
        }
        self.pos = p;
    } else {
        self.pos += 1;
    }
}
self.error_kind = 4;
self.error_msg = "unterminated raw".to_string();
    }

    fn _s_Scanning_enter(&mut self, __e: &CsBodyCloserFsmFrameEvent) {
let n = self.bytes.len();
while self.pos < n {
    let b = self.bytes[self.pos];
    if b == b'\n' {
        self.pos += 1;
    } else if b == b'/' && self.pos + 1 < n && self.bytes[self.pos + 1] == b'/' {
        self.pos += 2;
        let mut __compartment = CsBodyCloserFsmCompartment::new("InLineComment");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment); return;
    } else if b == b'/' && self.pos + 1 < n && self.bytes[self.pos + 1] == b'*' {
        self.pos += 2;
        let mut __compartment = CsBodyCloserFsmCompartment::new("InBlockComment");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment); return;
    } else if b == b'#' {
        self.pos += 1;
        let mut __compartment = CsBodyCloserFsmCompartment::new("InPreprocessor");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment); return;
    } else if b == b'\'' {
        self.pos += 1;
        let mut __compartment = CsBodyCloserFsmCompartment::new("InCharLiteral");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment); return;
    } else if b == b'@' {
        // @"verbatim" or @$"verbatim interp"
        if self.pos + 1 < n && self.bytes[self.pos + 1] == b'"' {
            self.pos += 2;
            let mut __compartment = CsBodyCloserFsmCompartment::new("InVerbatimString");
            __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
            self.__transition(__compartment); return;
        } else if self.pos + 2 < n && self.bytes[self.pos + 1] == b'$' && self.bytes[self.pos + 2] == b'"' {
            self.pos += 3;
            let mut __compartment = CsBodyCloserFsmCompartment::new("InVerbatimString");
            __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
            self.__transition(__compartment); return;
        } else {
            self.pos += 1;
        }
    } else if b == b'$' {
        // $"interp" or $"""raw""" or $$"..." etc.
        let mut j = self.pos;
        let mut _dollars: usize = 0;
        while j < n && self.bytes[j] == b'$' { _dollars += 1; j += 1; }
        // Check for @$"..." → verbatim interp (handled above via @)
        let mut k = j;
        let mut quotes: usize = 0;
        while k < n && self.bytes[k] == b'"' { quotes += 1; k += 1; }
        if quotes >= 3 {
            // Raw string $"""..."""
            self.raw_quotes = quotes;
            self.pos = k;
            let mut __compartment = CsBodyCloserFsmCompartment::new("InRawString");
            __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
            self.__transition(__compartment); return;
        } else if j < n && self.bytes[j] == b'"' {
            // $"interpolated"
            self.pos = j + 1;
            let mut __compartment = CsBodyCloserFsmCompartment::new("InString");
            __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
            self.__transition(__compartment); return;
        } else {
            self.pos += 1;
        }
    } else if b == b'"' {
        self.pos += 1;
        let mut __compartment = CsBodyCloserFsmCompartment::new("InString");
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

    fn _s_InBlockComment_enter(&mut self, __e: &CsBodyCloserFsmFrameEvent) {
let n = self.bytes.len();
while self.pos + 1 < n {
    if self.bytes[self.pos] == b'*' && self.bytes[self.pos + 1] == b'/' {
        self.pos += 2;
        let mut __compartment = CsBodyCloserFsmCompartment::new("Scanning");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment); return;
    }
    self.pos += 1;
}
self.error_kind = 2;
self.error_msg = "unterminated comment".to_string();
    }

    fn _s_InCharLiteral_enter(&mut self, __e: &CsBodyCloserFsmFrameEvent) {
let n = self.bytes.len();
while self.pos < n {
    if self.bytes[self.pos] == b'\\' {
        self.pos += 2;
        continue;
    }
    if self.bytes[self.pos] == b'\'' {
        self.pos += 1;
        let mut __compartment = CsBodyCloserFsmCompartment::new("Scanning");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment); return;
    }
    self.pos += 1;
}
self.error_kind = 1;
self.error_msg = "unterminated char".to_string();
    }

    fn _s_InPreprocessor_enter(&mut self, __e: &CsBodyCloserFsmFrameEvent) {
let n = self.bytes.len();
while self.pos < n && self.bytes[self.pos] != b'\n' {
    self.pos += 1;
}
let mut __compartment = CsBodyCloserFsmCompartment::new("Scanning");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment); return;
    }

    fn _s_InVerbatimString_enter(&mut self, __e: &CsBodyCloserFsmFrameEvent) {
// Verbatim: "" is escape for literal quote, single " ends
let n = self.bytes.len();
while self.pos < n {
    if self.pos + 1 < n && self.bytes[self.pos] == b'"' && self.bytes[self.pos + 1] == b'"' {
        self.pos += 2;
        continue;
    }
    if self.bytes[self.pos] == b'"' {
        self.pos += 1;
        let mut __compartment = CsBodyCloserFsmCompartment::new("Scanning");
        __compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
        self.__transition(__compartment); return;
    }
    self.pos += 1;
}
self.error_kind = 1;
self.error_msg = "unterminated verbatim string".to_string();
    }

    fn _s_InLineComment_enter(&mut self, __e: &CsBodyCloserFsmFrameEvent) {
let n = self.bytes.len();
while self.pos < n && self.bytes[self.pos] != b'\n' {
    self.pos += 1;
}
let mut __compartment = CsBodyCloserFsmCompartment::new("Scanning");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment); return;
    }
}

