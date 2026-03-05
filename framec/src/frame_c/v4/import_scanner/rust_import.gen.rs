
// Rust import scanner â Frame-generated state machine.
// Scans for use/extern crate statements terminated by semicolons.
// Handles raw strings r#"..."# and block comments /* */.
//
// Helpers used: starts_kw, is_frame_section_start (from mod.rs)

struct RustImportScannerFsmFrameEvent {
    message: String,
    parameters: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl Clone for RustImportScannerFsmFrameEvent {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            parameters: std::collections::HashMap::new(),
        }
    }
}

impl RustImportScannerFsmFrameEvent {
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

struct RustImportScannerFsmFrameContext {
    event: RustImportScannerFsmFrameEvent,
    _return: Option<Box<dyn std::any::Any>>,
    _data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl RustImportScannerFsmFrameContext {
    fn new(event: RustImportScannerFsmFrameEvent, default_return: Option<Box<dyn std::any::Any>>) -> Self {
        Self {
            event,
            _return: default_return,
            _data: std::collections::HashMap::new(),
        }
    }
}

#[derive(Clone)]
struct RustImportScannerFsmCompartment {
    state: String,
    state_vars: std::collections::HashMap<String, i32>,
    enter_args: std::collections::HashMap<String, String>,
    exit_args: std::collections::HashMap<String, String>,
    forward_event: Option<RustImportScannerFsmFrameEvent>,
    parent_compartment: Option<Box<RustImportScannerFsmCompartment>>,
}

impl RustImportScannerFsmCompartment {
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
enum RustImportScannerFsmStateContext {
    Init,
    Scanning,
    Empty,
}

impl Default for RustImportScannerFsmStateContext {
    fn default() -> Self {
        RustImportScannerFsmStateContext::Init
    }
}

pub struct RustImportScannerFsm {
    _state_stack: Vec<(String, RustImportScannerFsmStateContext)>,
    __compartment: RustImportScannerFsmCompartment,
    __next_compartment: Option<RustImportScannerFsmCompartment>,
    _context_stack: Vec<RustImportScannerFsmFrameContext>,
    pub bytes: Vec<u8>,
    pub start: usize,
    pub spans: Vec<RegionSpan>,
    pub issues: Vec<ValidationIssue>,
}

impl RustImportScannerFsm {
    pub fn new() -> Self {
        let mut this = Self {
            _state_stack: vec![],
            _context_stack: vec![],
            bytes: Vec::new(),
            start: 0,
            spans: Vec::new(),
            issues: Vec::new(),
            __compartment: RustImportScannerFsmCompartment::new("Init"),
            __next_compartment: None,
        };
let __frame_event = RustImportScannerFsmFrameEvent::new("$>");
this.__kernel(__frame_event);
        this
    }

    fn __kernel(&mut self, __e: RustImportScannerFsmFrameEvent) {
// Route event to current state
self.__router(&__e);
// Process any pending transition
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    // Exit current state (with exit_args from current compartment)
    let exit_event = RustImportScannerFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    // Switch to new compartment
    self.__compartment = next_compartment;
    // Enter new state (or forward event)
    if self.__compartment.forward_event.is_none() {
        let enter_event = RustImportScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        // Forward event to new state
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            // Forwarding enter event - just send it
            self.__router(&forward_event);
        } else {
            // Forwarding other event - send $> first, then forward
            let enter_event = RustImportScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
    }

    fn __router(&mut self, __e: &RustImportScannerFsmFrameEvent) {
match self.__compartment.state.as_str() {
    "Init" => self._state_Init(__e),
    "Scanning" => self._state_Scanning(__e),
    _ => {}
}
    }

    fn __transition(&mut self, next_compartment: RustImportScannerFsmCompartment) {
self.__next_compartment = Some(next_compartment);
    }

    fn _state_stack_push(&mut self) {
let state_context = match self.__compartment.state.as_str() {
    "Init" => RustImportScannerFsmStateContext::Init,
    "Scanning" => RustImportScannerFsmStateContext::Scanning,
    _ => RustImportScannerFsmStateContext::Empty,
};
self._state_stack.push((self.__compartment.state.clone(), state_context));
    }

    fn _state_stack_pop(&mut self) {
let (saved_state, state_context) = self._state_stack.pop().unwrap();
let exit_event = RustImportScannerFsmFrameEvent::new("$<");
self.__router(&exit_event);
self.__compartment.state = saved_state;
match state_context {
    RustImportScannerFsmStateContext::Init => {}
    RustImportScannerFsmStateContext::Scanning => {}
    RustImportScannerFsmStateContext::Empty => {}
}
    }

    pub fn do_scan(&mut self) {
let mut __e = RustImportScannerFsmFrameEvent::new("do_scan");
let __ctx = RustImportScannerFsmFrameContext::new(__e.clone(), None);
self._context_stack.push(__ctx);
match self.__compartment.state.as_str() {
            "Init" => { self._s_Init_do_scan(&__e); }
            _ => {}
        }
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    let exit_event = RustImportScannerFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    self.__compartment = next_compartment;
    if self.__compartment.forward_event.is_none() {
        let enter_event = RustImportScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            self.__router(&forward_event);
        } else {
            let enter_event = RustImportScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
self._context_stack.pop();
    }

    fn _state_Init(&mut self, __e: &RustImportScannerFsmFrameEvent) {
match __e.message.as_str() {
    "do_scan" => { self._s_Init_do_scan(__e); }
    _ => {}
}
    }

    fn _state_Scanning(&mut self, __e: &RustImportScannerFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_Scanning_enter(__e); }
    _ => {}
}
    }

    fn _s_Init_do_scan(&mut self, __e: &RustImportScannerFsmFrameEvent) {
let mut __compartment = RustImportScannerFsmCompartment::new("Scanning");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }

    fn _s_Scanning_enter(&mut self, __e: &RustImportScannerFsmFrameEvent) {
let n = self.bytes.len();
let mut i = self.start;
let mut at_sol = true;
while i < n {
    if at_sol {
        if self.bytes[i] == b'\n' || self.bytes[i] == b'\r' { i += 1; continue; }
        let line_start = i;
        let mut j = i;
        while j < n && (self.bytes[j] == b' ' || self.bytes[j] == b'\t') { j += 1; }
        // Stop at Frame section keywords
        if j < n && is_frame_section_start(&self.bytes, j) {
            break;
        }
        if j < n && (starts_kw(&self.bytes, j, b"use") || starts_kw(&self.bytes, j, b"extern")) {
            let stmt_start = line_start;
            let mut k = j;
            let mut in_s = false;
            let mut raw_hashes: usize = 0;
            let mut block = false;
            let mut esc = false;
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
                // Raw string r#"..."#
                if !in_s && self.bytes[k] == b'r' {
                    let mut p = k + 1;
                    let mut hashes: usize = 0;
                    if p < n && self.bytes[p] == b'#' {
                        while p < n && self.bytes[p] == b'#' { hashes += 1; p += 1; }
                    }
                    if p < n && self.bytes[p] == b'"' {
                        in_s = true;
                        raw_hashes = hashes;
                        k = p + 1;
                        continue;
                    }
                }
                if in_s {
                    if raw_hashes > 0 {
                        if self.bytes[k] == b'"' {
                            let mut p = k + 1;
                            let mut cnt: usize = 0;
                            while p < n && cnt < raw_hashes && self.bytes[p] == b'#' { cnt += 1; p += 1; }
                            if cnt == raw_hashes { in_s = false; k = p; continue; }
                        }
                        k += 1; continue;
                    } else {
                        if esc { esc = false; k += 1; continue; }
                        if self.bytes[k] == b'\\' { esc = true; k += 1; continue; }
                        if self.bytes[k] == b'"' { in_s = false; k += 1; continue; }
                        k += 1; continue;
                    }
                }
                if self.bytes[k] == b';' {
                    self.spans.push(RegionSpan { start: stmt_start, end: k });
                    found_semicolon = true;
                    k += 1; i = k; break;
                }
                if self.bytes[k] == b'\n' { k += 1; } else { k += 1; }
            }
            if i == line_start {
                if !found_semicolon || in_s || block {
                    self.issues.push(ValidationIssue { message: "E110: unterminated use/extern statement".into() });
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
