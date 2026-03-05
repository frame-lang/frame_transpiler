
// Java import scanner â Frame-generated state machine.
// Scans for import/package statements terminated by semicolons.
//
// Helpers used: starts_kw (from mod.rs)

struct JavaImportScannerFsmFrameEvent {
    message: String,
    parameters: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl Clone for JavaImportScannerFsmFrameEvent {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            parameters: std::collections::HashMap::new(),
        }
    }
}

impl JavaImportScannerFsmFrameEvent {
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

struct JavaImportScannerFsmFrameContext {
    event: JavaImportScannerFsmFrameEvent,
    _return: Option<Box<dyn std::any::Any>>,
    _data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl JavaImportScannerFsmFrameContext {
    fn new(event: JavaImportScannerFsmFrameEvent, default_return: Option<Box<dyn std::any::Any>>) -> Self {
        Self {
            event,
            _return: default_return,
            _data: std::collections::HashMap::new(),
        }
    }
}

#[derive(Clone)]
struct JavaImportScannerFsmCompartment {
    state: String,
    state_vars: std::collections::HashMap<String, i32>,
    enter_args: std::collections::HashMap<String, String>,
    exit_args: std::collections::HashMap<String, String>,
    forward_event: Option<JavaImportScannerFsmFrameEvent>,
    parent_compartment: Option<Box<JavaImportScannerFsmCompartment>>,
}

impl JavaImportScannerFsmCompartment {
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
enum JavaImportScannerFsmStateContext {
    Init,
    Scanning,
    Empty,
}

impl Default for JavaImportScannerFsmStateContext {
    fn default() -> Self {
        JavaImportScannerFsmStateContext::Init
    }
}

pub struct JavaImportScannerFsm {
    _state_stack: Vec<(String, JavaImportScannerFsmStateContext)>,
    __compartment: JavaImportScannerFsmCompartment,
    __next_compartment: Option<JavaImportScannerFsmCompartment>,
    _context_stack: Vec<JavaImportScannerFsmFrameContext>,
    pub bytes: Vec<u8>,
    pub start: usize,
    pub spans: Vec<RegionSpan>,
    pub issues: Vec<ValidationIssue>,
}

impl JavaImportScannerFsm {
    pub fn new() -> Self {
        let mut this = Self {
            _state_stack: vec![],
            _context_stack: vec![],
            bytes: Vec::new(),
            start: 0,
            spans: Vec::new(),
            issues: Vec::new(),
            __compartment: JavaImportScannerFsmCompartment::new("Init"),
            __next_compartment: None,
        };
let __frame_event = JavaImportScannerFsmFrameEvent::new("$>");
this.__kernel(__frame_event);
        this
    }

    fn __kernel(&mut self, __e: JavaImportScannerFsmFrameEvent) {
// Route event to current state
self.__router(&__e);
// Process any pending transition
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    // Exit current state (with exit_args from current compartment)
    let exit_event = JavaImportScannerFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    // Switch to new compartment
    self.__compartment = next_compartment;
    // Enter new state (or forward event)
    if self.__compartment.forward_event.is_none() {
        let enter_event = JavaImportScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        // Forward event to new state
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            // Forwarding enter event - just send it
            self.__router(&forward_event);
        } else {
            // Forwarding other event - send $> first, then forward
            let enter_event = JavaImportScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
    }

    fn __router(&mut self, __e: &JavaImportScannerFsmFrameEvent) {
match self.__compartment.state.as_str() {
    "Init" => self._state_Init(__e),
    "Scanning" => self._state_Scanning(__e),
    _ => {}
}
    }

    fn __transition(&mut self, next_compartment: JavaImportScannerFsmCompartment) {
self.__next_compartment = Some(next_compartment);
    }

    fn _state_stack_push(&mut self) {
let state_context = match self.__compartment.state.as_str() {
    "Init" => JavaImportScannerFsmStateContext::Init,
    "Scanning" => JavaImportScannerFsmStateContext::Scanning,
    _ => JavaImportScannerFsmStateContext::Empty,
};
self._state_stack.push((self.__compartment.state.clone(), state_context));
    }

    fn _state_stack_pop(&mut self) {
let (saved_state, state_context) = self._state_stack.pop().unwrap();
let exit_event = JavaImportScannerFsmFrameEvent::new("$<");
self.__router(&exit_event);
self.__compartment.state = saved_state;
match state_context {
    JavaImportScannerFsmStateContext::Init => {}
    JavaImportScannerFsmStateContext::Scanning => {}
    JavaImportScannerFsmStateContext::Empty => {}
}
    }

    pub fn do_scan(&mut self) {
let mut __e = JavaImportScannerFsmFrameEvent::new("do_scan");
let __ctx = JavaImportScannerFsmFrameContext::new(__e.clone(), None);
self._context_stack.push(__ctx);
match self.__compartment.state.as_str() {
            "Init" => { self._s_Init_do_scan(&__e); }
            _ => {}
        }
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    let exit_event = JavaImportScannerFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    self.__compartment = next_compartment;
    if self.__compartment.forward_event.is_none() {
        let enter_event = JavaImportScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            self.__router(&forward_event);
        } else {
            let enter_event = JavaImportScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
self._context_stack.pop();
    }

    fn _state_Init(&mut self, __e: &JavaImportScannerFsmFrameEvent) {
match __e.message.as_str() {
    "do_scan" => { self._s_Init_do_scan(__e); }
    _ => {}
}
    }

    fn _state_Scanning(&mut self, __e: &JavaImportScannerFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_Scanning_enter(__e); }
    _ => {}
}
    }

    fn _s_Init_do_scan(&mut self, __e: &JavaImportScannerFsmFrameEvent) {
let mut __compartment = JavaImportScannerFsmCompartment::new("Scanning");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }

    fn _s_Scanning_enter(&mut self, __e: &JavaImportScannerFsmFrameEvent) {
let n = self.bytes.len();
let mut i = self.start;
let mut at_sol = true;
while i < n {
    if at_sol {
        if self.bytes[i] == b'\n' || self.bytes[i] == b'\r' { i += 1; continue; }
        let line_start = i;
        let mut j = i;
        while j < n && (self.bytes[j] == b' ' || self.bytes[j] == b'\t') { j += 1; }
        if j < n && (starts_kw(&self.bytes, j, b"package") || starts_kw(&self.bytes, j, b"import")) {
            let stmt_start = line_start;
            let mut k = j;
            let mut in_s = false;
            let mut esc = false;
            let mut block = false;
            let mut found_semicolon = false;
            while k < n {
                // Block comment
                if !in_s && !block && k + 1 < n && self.bytes[k] == b'/' && self.bytes[k + 1] == b'*' {
                    block = true; k += 2; continue;
                }
                if block {
                    if k + 1 < n && self.bytes[k] == b'*' && self.bytes[k + 1] == b'/' { block = false; k += 2; continue; }
                    k += 1; continue;
                }
                // Line comment
                if !in_s && k + 1 < n && self.bytes[k] == b'/' && self.bytes[k + 1] == b'/' {
                    k += 2; while k < n && self.bytes[k] != b'\n' { k += 1; } continue;
                }
                let b = self.bytes[k];
                if in_s {
                    if esc { esc = false; k += 1; continue; }
                    if b == b'\\' { esc = true; k += 1; continue; }
                    if b == b'"' { in_s = false; k += 1; continue; }
                    k += 1; continue;
                }
                match b {
                    b'"' => { in_s = true; k += 1; }
                    b';' => {
                        self.spans.push(RegionSpan { start: stmt_start, end: k });
                        found_semicolon = true;
                        k += 1; i = k; break;
                    }
                    b'\n' => { k += 1; }
                    _ => { k += 1; }
                }
            }
            if k >= n {
                if !found_semicolon || in_s || block {
                    self.issues.push(ValidationIssue { message: "E110: unterminated package/import statement".into() });
                }
                self.spans.push(RegionSpan { start: stmt_start, end: n });
                i = n;
            }
            continue;
        }
        break;
    } else {
        if self.bytes[i] == b'\n' { at_sol = true; i += 1; } else { i += 1; }
    }
}
    }
}
