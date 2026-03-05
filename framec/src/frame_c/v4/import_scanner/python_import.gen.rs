
// Python import scanner â Frame-generated state machine.
// Scans for import/from statements with paren/backslash continuation.
// Handles triple-quoted strings and hash comments.
//
// Helpers used: starts_kw, is_frame_section_start (from mod.rs)

struct PythonImportScannerFsmFrameEvent {
    message: String,
    parameters: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl Clone for PythonImportScannerFsmFrameEvent {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            parameters: std::collections::HashMap::new(),
        }
    }
}

impl PythonImportScannerFsmFrameEvent {
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

struct PythonImportScannerFsmFrameContext {
    event: PythonImportScannerFsmFrameEvent,
    _return: Option<Box<dyn std::any::Any>>,
    _data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl PythonImportScannerFsmFrameContext {
    fn new(event: PythonImportScannerFsmFrameEvent, default_return: Option<Box<dyn std::any::Any>>) -> Self {
        Self {
            event,
            _return: default_return,
            _data: std::collections::HashMap::new(),
        }
    }
}

#[derive(Clone)]
struct PythonImportScannerFsmCompartment {
    state: String,
    state_vars: std::collections::HashMap<String, i32>,
    enter_args: std::collections::HashMap<String, String>,
    exit_args: std::collections::HashMap<String, String>,
    forward_event: Option<PythonImportScannerFsmFrameEvent>,
    parent_compartment: Option<Box<PythonImportScannerFsmCompartment>>,
}

impl PythonImportScannerFsmCompartment {
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
enum PythonImportScannerFsmStateContext {
    Init,
    Scanning,
    Empty,
}

impl Default for PythonImportScannerFsmStateContext {
    fn default() -> Self {
        PythonImportScannerFsmStateContext::Init
    }
}

pub struct PythonImportScannerFsm {
    _state_stack: Vec<(String, PythonImportScannerFsmStateContext)>,
    __compartment: PythonImportScannerFsmCompartment,
    __next_compartment: Option<PythonImportScannerFsmCompartment>,
    _context_stack: Vec<PythonImportScannerFsmFrameContext>,
    pub bytes: Vec<u8>,
    pub start: usize,
    pub spans: Vec<RegionSpan>,
    pub issues: Vec<ValidationIssue>,
}

impl PythonImportScannerFsm {
    pub fn new() -> Self {
        let mut this = Self {
            _state_stack: vec![],
            _context_stack: vec![],
            bytes: Vec::new(),
            start: 0,
            spans: Vec::new(),
            issues: Vec::new(),
            __compartment: PythonImportScannerFsmCompartment::new("Init"),
            __next_compartment: None,
        };
let __frame_event = PythonImportScannerFsmFrameEvent::new("$>");
this.__kernel(__frame_event);
        this
    }

    fn __kernel(&mut self, __e: PythonImportScannerFsmFrameEvent) {
// Route event to current state
self.__router(&__e);
// Process any pending transition
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    // Exit current state (with exit_args from current compartment)
    let exit_event = PythonImportScannerFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    // Switch to new compartment
    self.__compartment = next_compartment;
    // Enter new state (or forward event)
    if self.__compartment.forward_event.is_none() {
        let enter_event = PythonImportScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        // Forward event to new state
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            // Forwarding enter event - just send it
            self.__router(&forward_event);
        } else {
            // Forwarding other event - send $> first, then forward
            let enter_event = PythonImportScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
    }

    fn __router(&mut self, __e: &PythonImportScannerFsmFrameEvent) {
match self.__compartment.state.as_str() {
    "Init" => self._state_Init(__e),
    "Scanning" => self._state_Scanning(__e),
    _ => {}
}
    }

    fn __transition(&mut self, next_compartment: PythonImportScannerFsmCompartment) {
self.__next_compartment = Some(next_compartment);
    }

    fn _state_stack_push(&mut self) {
let state_context = match self.__compartment.state.as_str() {
    "Init" => PythonImportScannerFsmStateContext::Init,
    "Scanning" => PythonImportScannerFsmStateContext::Scanning,
    _ => PythonImportScannerFsmStateContext::Empty,
};
self._state_stack.push((self.__compartment.state.clone(), state_context));
    }

    fn _state_stack_pop(&mut self) {
let (saved_state, state_context) = self._state_stack.pop().unwrap();
let exit_event = PythonImportScannerFsmFrameEvent::new("$<");
self.__router(&exit_event);
self.__compartment.state = saved_state;
match state_context {
    PythonImportScannerFsmStateContext::Init => {}
    PythonImportScannerFsmStateContext::Scanning => {}
    PythonImportScannerFsmStateContext::Empty => {}
}
    }

    pub fn do_scan(&mut self) {
let mut __e = PythonImportScannerFsmFrameEvent::new("do_scan");
let __ctx = PythonImportScannerFsmFrameContext::new(__e.clone(), None);
self._context_stack.push(__ctx);
match self.__compartment.state.as_str() {
            "Init" => { self._s_Init_do_scan(&__e); }
            _ => {}
        }
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    let exit_event = PythonImportScannerFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    self.__compartment = next_compartment;
    if self.__compartment.forward_event.is_none() {
        let enter_event = PythonImportScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            self.__router(&forward_event);
        } else {
            let enter_event = PythonImportScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
self._context_stack.pop();
    }

    fn _state_Init(&mut self, __e: &PythonImportScannerFsmFrameEvent) {
match __e.message.as_str() {
    "do_scan" => { self._s_Init_do_scan(__e); }
    _ => {}
}
    }

    fn _state_Scanning(&mut self, __e: &PythonImportScannerFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_Scanning_enter(__e); }
    _ => {}
}
    }

    fn _s_Init_do_scan(&mut self, __e: &PythonImportScannerFsmFrameEvent) {
let mut __compartment = PythonImportScannerFsmCompartment::new("Scanning");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }

    fn _s_Scanning_enter(&mut self, __e: &PythonImportScannerFsmFrameEvent) {
let n = self.bytes.len();
let mut i = self.start;
let mut at_sol = true;
while i < n {
    if at_sol {
        let line_start = i;
        let mut j = i;
        while j < n && (self.bytes[j] == b' ' || self.bytes[j] == b'\t') { j += 1; }
        // Stop at Frame section keywords
        if j < n && is_frame_section_start(&self.bytes, j) {
            break;
        }
        if j < n && (starts_kw(&self.bytes, j, b"import") || starts_kw(&self.bytes, j, b"from")) {
            let stmt_start = line_start;
            let mut k = j;
            let mut paren: i32 = 0;
            let mut in_s: u8 = 0; // 0 none, 1 ', 2 "
            let mut in_triple: u8 = 0; // 1 ''', 2 """
            while k < n {
                let b = self.bytes[k];
                // Inside triple-quoted string
                if in_triple == 1 {
                    if k + 2 < n && self.bytes[k] == b'\'' && self.bytes[k + 1] == b'\'' && self.bytes[k + 2] == b'\'' {
                        k += 3; in_triple = 0; continue;
                    }
                    k += 1; continue;
                } else if in_triple == 2 {
                    if k + 2 < n && self.bytes[k] == b'"' && self.bytes[k + 1] == b'"' && self.bytes[k + 2] == b'"' {
                        k += 3; in_triple = 0; continue;
                    }
                    k += 1; continue;
                }
                match b {
                    b'\'' if in_s == 0 => {
                        if k + 2 < n && self.bytes[k + 1] == b'\'' && self.bytes[k + 2] == b'\'' {
                            in_triple = 1; k += 3; continue;
                        }
                        in_s = 1; k += 1; continue;
                    }
                    b'"' if in_s == 0 => {
                        if k + 2 < n && self.bytes[k + 1] == b'"' && self.bytes[k + 2] == b'"' {
                            in_triple = 2; k += 3; continue;
                        }
                        in_s = 2; k += 1; continue;
                    }
                    b'\'' if in_s == 1 => { in_s = 0; k += 1; continue; }
                    b'"' if in_s == 2 => { in_s = 0; k += 1; continue; }
                    b'(' => { if in_s == 0 { paren += 1; } k += 1; continue; }
                    b')' => { if in_s == 0 { paren -= 1; } k += 1; continue; }
                    b'#' => {
                        // Comment to end of physical line
                        while k < n && self.bytes[k] != b'\n' { k += 1; }
                        // Fallthrough to newline handling
                    }
                    b'\n' => {
                        // Check continuation: paren > 0 or trailing backslash
                        let mut p = k;
                        if p > 0 { p -= 1; }
                        while p > stmt_start && (self.bytes[p] == b' ' || self.bytes[p] == b'\t' || self.bytes[p] == b'\r') {
                            if p == 0 { break; }
                            p -= 1;
                        }
                        let backslash_cont = self.bytes.get(p).copied() == Some(b'\\');
                        if paren > 0 || backslash_cont {
                            k += 1; continue; // Continue logical line
                        } else {
                            let stmt_end = k;
                            self.spans.push(RegionSpan { start: stmt_start, end: stmt_end });
                            k += 1; i = k; at_sol = true; break;
                        }
                    }
                    _ => { k += 1; continue; }
                }
            }
            if k >= n {
                if paren > 0 || in_s != 0 || in_triple != 0 {
                    self.issues.push(ValidationIssue { message: "E110: unterminated import statement".into() });
                }
                self.spans.push(RegionSpan { start: stmt_start, end: n });
                break;
            }
            continue;
        }
        // Not an import line; advance to end of line
        while i < n && self.bytes[i] != b'\n' { i += 1; }
        if i < n { i += 1; }
        at_sol = true;
        continue;
    } else {
        if self.bytes[i] == b'\n' { at_sol = true; i += 1; } else { i += 1; }
    }
}
    }
}
