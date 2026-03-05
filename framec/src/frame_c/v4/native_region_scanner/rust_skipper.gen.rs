
// Rust syntax skipper â Frame-generated state machine.
// Handles nested block comments, raw strings r#"..."#, and simple strings.

struct RustSyntaxSkipperFsmFrameEvent {
    message: String,
    parameters: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl Clone for RustSyntaxSkipperFsmFrameEvent {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            parameters: std::collections::HashMap::new(),
        }
    }
}

impl RustSyntaxSkipperFsmFrameEvent {
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

struct RustSyntaxSkipperFsmFrameContext {
    event: RustSyntaxSkipperFsmFrameEvent,
    _return: Option<Box<dyn std::any::Any>>,
    _data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl RustSyntaxSkipperFsmFrameContext {
    fn new(event: RustSyntaxSkipperFsmFrameEvent, default_return: Option<Box<dyn std::any::Any>>) -> Self {
        Self {
            event,
            _return: default_return,
            _data: std::collections::HashMap::new(),
        }
    }
}

#[derive(Clone)]
struct RustSyntaxSkipperFsmCompartment {
    state: String,
    state_vars: std::collections::HashMap<String, i32>,
    enter_args: std::collections::HashMap<String, String>,
    exit_args: std::collections::HashMap<String, String>,
    forward_event: Option<RustSyntaxSkipperFsmFrameEvent>,
    parent_compartment: Option<Box<RustSyntaxSkipperFsmCompartment>>,
}

impl RustSyntaxSkipperFsmCompartment {
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
enum RustSyntaxSkipperFsmStateContext {
    Init,
    SkipComment,
    SkipString,
    FindLineEnd,
    BalancedParenEnd,
    Empty,
}

impl Default for RustSyntaxSkipperFsmStateContext {
    fn default() -> Self {
        RustSyntaxSkipperFsmStateContext::Init
    }
}

pub struct RustSyntaxSkipperFsm {
    _state_stack: Vec<(String, RustSyntaxSkipperFsmStateContext)>,
    __compartment: RustSyntaxSkipperFsmCompartment,
    __next_compartment: Option<RustSyntaxSkipperFsmCompartment>,
    _context_stack: Vec<RustSyntaxSkipperFsmFrameContext>,
    pub bytes: Vec<u8>,
    pub pos: usize,
    pub end: usize,
    pub result_pos: usize,
    pub success: usize,
}

impl RustSyntaxSkipperFsm {
    pub fn new() -> Self {
        let mut this = Self {
            _state_stack: vec![],
            _context_stack: vec![],
            bytes: Vec::new(),
            pos: 0,
            end: 0,
            result_pos: 0,
            success: 1,
            __compartment: RustSyntaxSkipperFsmCompartment::new("Init"),
            __next_compartment: None,
        };
let __frame_event = RustSyntaxSkipperFsmFrameEvent::new("$>");
this.__kernel(__frame_event);
        this
    }

    fn __kernel(&mut self, __e: RustSyntaxSkipperFsmFrameEvent) {
// Route event to current state
self.__router(&__e);
// Process any pending transition
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    // Exit current state (with exit_args from current compartment)
    let exit_event = RustSyntaxSkipperFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    // Switch to new compartment
    self.__compartment = next_compartment;
    // Enter new state (or forward event)
    if self.__compartment.forward_event.is_none() {
        let enter_event = RustSyntaxSkipperFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        // Forward event to new state
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            // Forwarding enter event - just send it
            self.__router(&forward_event);
        } else {
            // Forwarding other event - send $> first, then forward
            let enter_event = RustSyntaxSkipperFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
    }

    fn __router(&mut self, __e: &RustSyntaxSkipperFsmFrameEvent) {
match self.__compartment.state.as_str() {
    "Init" => self._state_Init(__e),
    "SkipComment" => self._state_SkipComment(__e),
    "SkipString" => self._state_SkipString(__e),
    "FindLineEnd" => self._state_FindLineEnd(__e),
    "BalancedParenEnd" => self._state_BalancedParenEnd(__e),
    _ => {}
}
    }

    fn __transition(&mut self, next_compartment: RustSyntaxSkipperFsmCompartment) {
self.__next_compartment = Some(next_compartment);
    }

    fn _state_stack_push(&mut self) {
let state_context = match self.__compartment.state.as_str() {
    "Init" => RustSyntaxSkipperFsmStateContext::Init,
    "SkipComment" => RustSyntaxSkipperFsmStateContext::SkipComment,
    "SkipString" => RustSyntaxSkipperFsmStateContext::SkipString,
    "FindLineEnd" => RustSyntaxSkipperFsmStateContext::FindLineEnd,
    "BalancedParenEnd" => RustSyntaxSkipperFsmStateContext::BalancedParenEnd,
    _ => RustSyntaxSkipperFsmStateContext::Empty,
};
self._state_stack.push((self.__compartment.state.clone(), state_context));
    }

    fn _state_stack_pop(&mut self) {
let (saved_state, state_context) = self._state_stack.pop().unwrap();
let exit_event = RustSyntaxSkipperFsmFrameEvent::new("$<");
self.__router(&exit_event);
self.__compartment.state = saved_state;
match state_context {
    RustSyntaxSkipperFsmStateContext::Init => {}
    RustSyntaxSkipperFsmStateContext::SkipComment => {}
    RustSyntaxSkipperFsmStateContext::SkipString => {}
    RustSyntaxSkipperFsmStateContext::FindLineEnd => {}
    RustSyntaxSkipperFsmStateContext::BalancedParenEnd => {}
    RustSyntaxSkipperFsmStateContext::Empty => {}
}
    }

    pub fn do_skip_comment(&mut self) {
let mut __e = RustSyntaxSkipperFsmFrameEvent::new("do_skip_comment");
let __ctx = RustSyntaxSkipperFsmFrameContext::new(__e.clone(), None);
self._context_stack.push(__ctx);
match self.__compartment.state.as_str() {
            "Init" => { self._s_Init_do_skip_comment(&__e); }
            _ => {}
        }
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    let exit_event = RustSyntaxSkipperFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    self.__compartment = next_compartment;
    if self.__compartment.forward_event.is_none() {
        let enter_event = RustSyntaxSkipperFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            self.__router(&forward_event);
        } else {
            let enter_event = RustSyntaxSkipperFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
self._context_stack.pop();
    }

    pub fn do_skip_string(&mut self) {
let mut __e = RustSyntaxSkipperFsmFrameEvent::new("do_skip_string");
let __ctx = RustSyntaxSkipperFsmFrameContext::new(__e.clone(), None);
self._context_stack.push(__ctx);
match self.__compartment.state.as_str() {
            "Init" => { self._s_Init_do_skip_string(&__e); }
            _ => {}
        }
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    let exit_event = RustSyntaxSkipperFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    self.__compartment = next_compartment;
    if self.__compartment.forward_event.is_none() {
        let enter_event = RustSyntaxSkipperFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            self.__router(&forward_event);
        } else {
            let enter_event = RustSyntaxSkipperFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
self._context_stack.pop();
    }

    pub fn do_find_line_end(&mut self) {
let mut __e = RustSyntaxSkipperFsmFrameEvent::new("do_find_line_end");
let __ctx = RustSyntaxSkipperFsmFrameContext::new(__e.clone(), None);
self._context_stack.push(__ctx);
match self.__compartment.state.as_str() {
            "Init" => { self._s_Init_do_find_line_end(&__e); }
            _ => {}
        }
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    let exit_event = RustSyntaxSkipperFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    self.__compartment = next_compartment;
    if self.__compartment.forward_event.is_none() {
        let enter_event = RustSyntaxSkipperFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            self.__router(&forward_event);
        } else {
            let enter_event = RustSyntaxSkipperFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
self._context_stack.pop();
    }

    pub fn do_balanced_paren_end(&mut self) {
let mut __e = RustSyntaxSkipperFsmFrameEvent::new("do_balanced_paren_end");
let __ctx = RustSyntaxSkipperFsmFrameContext::new(__e.clone(), None);
self._context_stack.push(__ctx);
match self.__compartment.state.as_str() {
            "Init" => { self._s_Init_do_balanced_paren_end(&__e); }
            _ => {}
        }
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    let exit_event = RustSyntaxSkipperFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    self.__compartment = next_compartment;
    if self.__compartment.forward_event.is_none() {
        let enter_event = RustSyntaxSkipperFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            self.__router(&forward_event);
        } else {
            let enter_event = RustSyntaxSkipperFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
self._context_stack.pop();
    }

    fn _state_SkipComment(&mut self, __e: &RustSyntaxSkipperFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_SkipComment_enter(__e); }
    _ => {}
}
    }

    fn _state_Init(&mut self, __e: &RustSyntaxSkipperFsmFrameEvent) {
match __e.message.as_str() {
    "do_balanced_paren_end" => { self._s_Init_do_balanced_paren_end(__e); }
    "do_find_line_end" => { self._s_Init_do_find_line_end(__e); }
    "do_skip_comment" => { self._s_Init_do_skip_comment(__e); }
    "do_skip_string" => { self._s_Init_do_skip_string(__e); }
    _ => {}
}
    }

    fn _state_SkipString(&mut self, __e: &RustSyntaxSkipperFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_SkipString_enter(__e); }
    _ => {}
}
    }

    fn _state_BalancedParenEnd(&mut self, __e: &RustSyntaxSkipperFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_BalancedParenEnd_enter(__e); }
    _ => {}
}
    }

    fn _state_FindLineEnd(&mut self, __e: &RustSyntaxSkipperFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_FindLineEnd_enter(__e); }
    _ => {}
}
    }

    fn _s_SkipComment_enter(&mut self, __e: &RustSyntaxSkipperFsmFrameEvent) {
let i = self.pos;
let end = self.end;
let bytes = &self.bytes;
// Line comment
if i + 1 < end && bytes[i] == b'/' && bytes[i + 1] == b'/' {
    let mut j = i + 2;
    while j < end && bytes[j] != b'\n' { j += 1; }
    self.result_pos = j;
    self.success = 1;
    return
}
// Rust nested block comment: /* ... /* ... */ ... */
if i + 1 < end && bytes[i] == b'/' && bytes[i + 1] == b'*' {
    let mut j = i + 2;
    let mut depth: i32 = 1;
    while j + 1 < end && depth > 0 {
        if bytes[j] == b'/' && bytes[j + 1] == b'*' {
            depth += 1;
            j += 2;
            continue;
        }
        if bytes[j] == b'*' && bytes[j + 1] == b'/' {
            depth -= 1;
            j += 2;
            continue;
        }
        j += 1;
    }
    self.result_pos = j;
    self.success = 1;
    return
}
self.success = 0;
    }

    fn _s_Init_do_skip_string(&mut self, __e: &RustSyntaxSkipperFsmFrameEvent) {
let mut __compartment = RustSyntaxSkipperFsmCompartment::new("SkipString");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }

    fn _s_Init_do_find_line_end(&mut self, __e: &RustSyntaxSkipperFsmFrameEvent) {
let mut __compartment = RustSyntaxSkipperFsmCompartment::new("FindLineEnd");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }

    fn _s_Init_do_skip_comment(&mut self, __e: &RustSyntaxSkipperFsmFrameEvent) {
let mut __compartment = RustSyntaxSkipperFsmCompartment::new("SkipComment");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }

    fn _s_Init_do_balanced_paren_end(&mut self, __e: &RustSyntaxSkipperFsmFrameEvent) {
let mut __compartment = RustSyntaxSkipperFsmCompartment::new("BalancedParenEnd");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }

    fn _s_SkipString_enter(&mut self, __e: &RustSyntaxSkipperFsmFrameEvent) {
let i = self.pos;
let end = self.end;
let bytes = &self.bytes;
// Raw string: r#"..."# (with 0+ hashes)
if bytes[i] == b'r' {
    let mut j = i + 1;
    let mut hashes: usize = 0;
    while j < end && bytes[j] == b'#' {
        hashes += 1;
        j += 1;
    }
    if j < end && bytes[j] == b'"' {
        j += 1; // skip opening "
        while j < end {
            if bytes[j] == b'"' {
                let mut k = j + 1;
                let mut matched: usize = 0;
                while matched < hashes && k < end && bytes[k] == b'#' {
                    matched += 1;
                    k += 1;
                }
                if matched == hashes {
                    self.result_pos = k;
                    self.success = 1;
                    return
                }
            }
            j += 1;
        }
        self.result_pos = end;
        self.success = 1;
        return
    }
}
// Simple string
let b = bytes[i];
if b == b'\'' || b == b'"' {
    let q = b;
    let mut j = i + 1;
    while j < end {
        if bytes[j] == b'\\' { j += 2; continue; }
        if bytes[j] == q {
            self.result_pos = j + 1;
            self.success = 1;
            return
        }
        j += 1;
    }
    self.result_pos = end;
    self.success = 1;
    return
}
self.success = 0;
    }

    fn _s_BalancedParenEnd_enter(&mut self, __e: &RustSyntaxSkipperFsmFrameEvent) {
// Balanced parens, respecting strings (same as C-like)
let end = self.end;
let bytes = &self.bytes;
let mut i = self.pos;
if i >= end || bytes[i] != b'(' {
    self.success = 0;
    return
}
let mut depth: i32 = 0;
let mut in_string: u8 = 0;
while i < end {
    let b = bytes[i];
    if in_string != 0 {
        if b == b'\\' { i += 2; continue; }
        if b == in_string { in_string = 0; }
        i += 1;
        continue;
    }
    if b == b'\'' || b == b'"' { in_string = b; i += 1; }
    else if b == b'(' { depth += 1; i += 1; }
    else if b == b')' {
        depth -= 1; i += 1;
        if depth == 0 { self.result_pos = i; self.success = 1; return }
    } else { i += 1; }
}
self.success = 0;
    }

    fn _s_FindLineEnd_enter(&mut self, __e: &RustSyntaxSkipperFsmFrameEvent) {
// Rust: handle raw strings r#"..."# during line scanning
let end = self.end;
let bytes = &self.bytes;
let mut j = self.pos;
let mut in_string: u8 = 0;
let mut raw_hashes: usize = 0;

while j < end {
    let b = bytes[j];
    if b == b'\n' { break; }

    // Inside raw string
    if raw_hashes > 0 {
        if b == b'"' {
            let mut k = j + 1;
            let mut matched: usize = 0;
            while k < end && matched < raw_hashes && bytes[k] == b'#' {
                matched += 1;
                k += 1;
            }
            if matched == raw_hashes {
                j = k;
                raw_hashes = 0;
                continue;
            }
        }
        j += 1;
        continue;
    }

    // Inside regular string
    if in_string != 0 {
        if b == b'\\' { j += 2; continue; }
        if b == in_string { in_string = 0; }
        j += 1;
        continue;
    }

    // Terminators
    if b == b';' { break; }
    if b == b'/' && j + 1 < end && (bytes[j + 1] == b'/' || bytes[j + 1] == b'*') { break; }

    // String starts
    if b == b'\'' || b == b'"' {
        in_string = b;
        j += 1;
        continue;
    }

    // Raw string start
    if b == b'r' {
        let mut k = j + 1;
        let mut hashes: usize = 0;
        while k < end && bytes[k] == b'#' {
            hashes += 1;
            k += 1;
        }
        if k < end && bytes[k] == b'"' {
            raw_hashes = hashes;
            j = k + 1;
            continue;
        }
    }

    j += 1;
}
self.result_pos = j;
    }
}
