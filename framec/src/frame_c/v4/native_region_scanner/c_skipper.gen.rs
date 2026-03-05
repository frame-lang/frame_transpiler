
// C language syntax skipper â Frame-generated state machine.
// Implements skip_comment, skip_string, find_line_end, balanced_paren_end
// by delegating to unified helpers from the native_region_scanner module.
//
// Each interface method creates a fresh scan via state transitions.
// Results are read from domain vars after the method returns.

struct CSyntaxSkipperFsmFrameEvent {
    message: String,
    parameters: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl Clone for CSyntaxSkipperFsmFrameEvent {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            parameters: std::collections::HashMap::new(),
        }
    }
}

impl CSyntaxSkipperFsmFrameEvent {
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

struct CSyntaxSkipperFsmFrameContext {
    event: CSyntaxSkipperFsmFrameEvent,
    _return: Option<Box<dyn std::any::Any>>,
    _data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl CSyntaxSkipperFsmFrameContext {
    fn new(event: CSyntaxSkipperFsmFrameEvent, default_return: Option<Box<dyn std::any::Any>>) -> Self {
        Self {
            event,
            _return: default_return,
            _data: std::collections::HashMap::new(),
        }
    }
}

#[derive(Clone)]
struct CSyntaxSkipperFsmCompartment {
    state: String,
    state_vars: std::collections::HashMap<String, i32>,
    enter_args: std::collections::HashMap<String, String>,
    exit_args: std::collections::HashMap<String, String>,
    forward_event: Option<CSyntaxSkipperFsmFrameEvent>,
    parent_compartment: Option<Box<CSyntaxSkipperFsmCompartment>>,
}

impl CSyntaxSkipperFsmCompartment {
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
enum CSyntaxSkipperFsmStateContext {
    Init,
    SkipComment,
    SkipString,
    FindLineEnd,
    BalancedParenEnd,
    Empty,
}

impl Default for CSyntaxSkipperFsmStateContext {
    fn default() -> Self {
        CSyntaxSkipperFsmStateContext::Init
    }
}

pub struct CSyntaxSkipperFsm {
    _state_stack: Vec<(String, CSyntaxSkipperFsmStateContext)>,
    __compartment: CSyntaxSkipperFsmCompartment,
    __next_compartment: Option<CSyntaxSkipperFsmCompartment>,
    _context_stack: Vec<CSyntaxSkipperFsmFrameContext>,
    pub bytes: Vec<u8>,
    pub pos: usize,
    pub end: usize,
    pub result_pos: usize,
    pub success: usize,
}

impl CSyntaxSkipperFsm {
    pub fn new() -> Self {
        let mut this = Self {
            _state_stack: vec![],
            _context_stack: vec![],
            bytes: Vec::new(),
            pos: 0,
            end: 0,
            result_pos: 0,
            success: 1,
            __compartment: CSyntaxSkipperFsmCompartment::new("Init"),
            __next_compartment: None,
        };
let __frame_event = CSyntaxSkipperFsmFrameEvent::new("$>");
this.__kernel(__frame_event);
        this
    }

    fn __kernel(&mut self, __e: CSyntaxSkipperFsmFrameEvent) {
// Route event to current state
self.__router(&__e);
// Process any pending transition
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    // Exit current state (with exit_args from current compartment)
    let exit_event = CSyntaxSkipperFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    // Switch to new compartment
    self.__compartment = next_compartment;
    // Enter new state (or forward event)
    if self.__compartment.forward_event.is_none() {
        let enter_event = CSyntaxSkipperFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        // Forward event to new state
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            // Forwarding enter event - just send it
            self.__router(&forward_event);
        } else {
            // Forwarding other event - send $> first, then forward
            let enter_event = CSyntaxSkipperFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
    }

    fn __router(&mut self, __e: &CSyntaxSkipperFsmFrameEvent) {
match self.__compartment.state.as_str() {
    "Init" => self._state_Init(__e),
    "SkipComment" => self._state_SkipComment(__e),
    "SkipString" => self._state_SkipString(__e),
    "FindLineEnd" => self._state_FindLineEnd(__e),
    "BalancedParenEnd" => self._state_BalancedParenEnd(__e),
    _ => {}
}
    }

    fn __transition(&mut self, next_compartment: CSyntaxSkipperFsmCompartment) {
self.__next_compartment = Some(next_compartment);
    }

    fn _state_stack_push(&mut self) {
let state_context = match self.__compartment.state.as_str() {
    "Init" => CSyntaxSkipperFsmStateContext::Init,
    "SkipComment" => CSyntaxSkipperFsmStateContext::SkipComment,
    "SkipString" => CSyntaxSkipperFsmStateContext::SkipString,
    "FindLineEnd" => CSyntaxSkipperFsmStateContext::FindLineEnd,
    "BalancedParenEnd" => CSyntaxSkipperFsmStateContext::BalancedParenEnd,
    _ => CSyntaxSkipperFsmStateContext::Empty,
};
self._state_stack.push((self.__compartment.state.clone(), state_context));
    }

    fn _state_stack_pop(&mut self) {
let (saved_state, state_context) = self._state_stack.pop().unwrap();
let exit_event = CSyntaxSkipperFsmFrameEvent::new("$<");
self.__router(&exit_event);
self.__compartment.state = saved_state;
match state_context {
    CSyntaxSkipperFsmStateContext::Init => {}
    CSyntaxSkipperFsmStateContext::SkipComment => {}
    CSyntaxSkipperFsmStateContext::SkipString => {}
    CSyntaxSkipperFsmStateContext::FindLineEnd => {}
    CSyntaxSkipperFsmStateContext::BalancedParenEnd => {}
    CSyntaxSkipperFsmStateContext::Empty => {}
}
    }

    pub fn do_skip_comment(&mut self) {
let mut __e = CSyntaxSkipperFsmFrameEvent::new("do_skip_comment");
let __ctx = CSyntaxSkipperFsmFrameContext::new(__e.clone(), None);
self._context_stack.push(__ctx);
match self.__compartment.state.as_str() {
            "Init" => { self._s_Init_do_skip_comment(&__e); }
            _ => {}
        }
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    let exit_event = CSyntaxSkipperFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    self.__compartment = next_compartment;
    if self.__compartment.forward_event.is_none() {
        let enter_event = CSyntaxSkipperFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            self.__router(&forward_event);
        } else {
            let enter_event = CSyntaxSkipperFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
self._context_stack.pop();
    }

    pub fn do_skip_string(&mut self) {
let mut __e = CSyntaxSkipperFsmFrameEvent::new("do_skip_string");
let __ctx = CSyntaxSkipperFsmFrameContext::new(__e.clone(), None);
self._context_stack.push(__ctx);
match self.__compartment.state.as_str() {
            "Init" => { self._s_Init_do_skip_string(&__e); }
            _ => {}
        }
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    let exit_event = CSyntaxSkipperFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    self.__compartment = next_compartment;
    if self.__compartment.forward_event.is_none() {
        let enter_event = CSyntaxSkipperFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            self.__router(&forward_event);
        } else {
            let enter_event = CSyntaxSkipperFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
self._context_stack.pop();
    }

    pub fn do_find_line_end(&mut self) {
let mut __e = CSyntaxSkipperFsmFrameEvent::new("do_find_line_end");
let __ctx = CSyntaxSkipperFsmFrameContext::new(__e.clone(), None);
self._context_stack.push(__ctx);
match self.__compartment.state.as_str() {
            "Init" => { self._s_Init_do_find_line_end(&__e); }
            _ => {}
        }
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    let exit_event = CSyntaxSkipperFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    self.__compartment = next_compartment;
    if self.__compartment.forward_event.is_none() {
        let enter_event = CSyntaxSkipperFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            self.__router(&forward_event);
        } else {
            let enter_event = CSyntaxSkipperFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
self._context_stack.pop();
    }

    pub fn do_balanced_paren_end(&mut self) {
let mut __e = CSyntaxSkipperFsmFrameEvent::new("do_balanced_paren_end");
let __ctx = CSyntaxSkipperFsmFrameContext::new(__e.clone(), None);
self._context_stack.push(__ctx);
match self.__compartment.state.as_str() {
            "Init" => { self._s_Init_do_balanced_paren_end(&__e); }
            _ => {}
        }
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    let exit_event = CSyntaxSkipperFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    self.__compartment = next_compartment;
    if self.__compartment.forward_event.is_none() {
        let enter_event = CSyntaxSkipperFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            self.__router(&forward_event);
        } else {
            let enter_event = CSyntaxSkipperFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
self._context_stack.pop();
    }

    fn _state_Init(&mut self, __e: &CSyntaxSkipperFsmFrameEvent) {
match __e.message.as_str() {
    "do_balanced_paren_end" => { self._s_Init_do_balanced_paren_end(__e); }
    "do_find_line_end" => { self._s_Init_do_find_line_end(__e); }
    "do_skip_comment" => { self._s_Init_do_skip_comment(__e); }
    "do_skip_string" => { self._s_Init_do_skip_string(__e); }
    _ => {}
}
    }

    fn _state_FindLineEnd(&mut self, __e: &CSyntaxSkipperFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_FindLineEnd_enter(__e); }
    _ => {}
}
    }

    fn _state_SkipComment(&mut self, __e: &CSyntaxSkipperFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_SkipComment_enter(__e); }
    _ => {}
}
    }

    fn _state_SkipString(&mut self, __e: &CSyntaxSkipperFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_SkipString_enter(__e); }
    _ => {}
}
    }

    fn _state_BalancedParenEnd(&mut self, __e: &CSyntaxSkipperFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_BalancedParenEnd_enter(__e); }
    _ => {}
}
    }

    fn _s_Init_do_skip_comment(&mut self, __e: &CSyntaxSkipperFsmFrameEvent) {
let mut __compartment = CSyntaxSkipperFsmCompartment::new("SkipComment");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }

    fn _s_Init_do_skip_string(&mut self, __e: &CSyntaxSkipperFsmFrameEvent) {
let mut __compartment = CSyntaxSkipperFsmCompartment::new("SkipString");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }

    fn _s_Init_do_find_line_end(&mut self, __e: &CSyntaxSkipperFsmFrameEvent) {
let mut __compartment = CSyntaxSkipperFsmCompartment::new("FindLineEnd");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }

    fn _s_Init_do_balanced_paren_end(&mut self, __e: &CSyntaxSkipperFsmFrameEvent) {
let mut __compartment = CSyntaxSkipperFsmCompartment::new("BalancedParenEnd");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }

    fn _s_FindLineEnd_enter(&mut self, __e: &CSyntaxSkipperFsmFrameEvent) {
// C-like: stop at newline, semicolon, or comment start
let end = self.end;
let bytes = &self.bytes;
let mut j = self.pos;
let mut in_string: u8 = 0;  // 0 = not in string, else quote char

while j < end {
    let b = bytes[j];
    if b == b'\n' {
        break;
    }
    if in_string != 0 {
        if b == b'\\' {
            j += 2;
            continue;
        }
        if b == in_string {
            in_string = 0;
        }
        j += 1;
        continue;
    }
    if b == b';' {
        break;
    }
    if b == b'/' && j + 1 < end && (bytes[j + 1] == b'/' || bytes[j + 1] == b'*') {
        break;
    }
    if b == b'\'' || b == b'"' {
        in_string = b;
    }
    j += 1;
}
self.result_pos = j;
    }

    fn _s_SkipComment_enter(&mut self, __e: &CSyntaxSkipperFsmFrameEvent) {
// C has // line comments and /* */ block comments
let i = self.pos;
let end = self.end;
let bytes = &self.bytes;
if i + 1 < end && bytes[i] == b'/' && bytes[i + 1] == b'/' {
    // Line comment — scan to newline
    let mut j = i + 2;
    while j < end && bytes[j] != b'\n' {
        j += 1;
    }
    self.result_pos = j;
    self.success = 1;
    return
}
if i + 1 < end && bytes[i] == b'/' && bytes[i + 1] == b'*' {
    // Block comment — scan to */
    let mut j = i + 2;
    while j + 1 < end {
        if bytes[j] == b'*' && bytes[j + 1] == b'/' {
            self.result_pos = j + 2;
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

    fn _s_SkipString_enter(&mut self, __e: &CSyntaxSkipperFsmFrameEvent) {
// C has simple strings: "..." and '...' with backslash escapes
let i = self.pos;
let end = self.end;
let bytes = &self.bytes;
let b = bytes[i];
if b == b'\'' || b == b'"' {
    let q = b;
    let mut j = i + 1;
    while j < end {
        if bytes[j] == b'\\' {
            j += 2;
            continue;
        }
        if bytes[j] == q {
            self.result_pos = j + 1;
            self.success = 1;
            return
        }
        j += 1;
    }
    // Unterminated — consume rest
    self.result_pos = end;
    self.success = 1;
    return
}
self.success = 0;
    }

    fn _s_BalancedParenEnd_enter(&mut self, __e: &CSyntaxSkipperFsmFrameEvent) {
// C-like: find matching ), respecting strings
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
        if b == b'\\' {
            i += 2;
            continue;
        }
        if b == in_string {
            in_string = 0;
        }
        i += 1;
        continue;
    }
    if b == b'\'' || b == b'"' {
        in_string = b;
        i += 1;
    } else if b == b'(' {
        depth += 1;
        i += 1;
    } else if b == b')' {
        depth -= 1;
        i += 1;
        if depth == 0 {
            self.result_pos = i;
            self.success = 1;
            return
        }
    } else {
        i += 1;
    }
}
self.success = 0;
    }
}
