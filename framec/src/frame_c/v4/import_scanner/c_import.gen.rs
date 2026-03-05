
// C import scanner â Frame-generated state machine.
// Scans for #include directives at SOL, with backslash continuation.
//
// Helpers used: starts_kw (from mod.rs)

struct CImportScannerFsmFrameEvent {
    message: String,
    parameters: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl Clone for CImportScannerFsmFrameEvent {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            parameters: std::collections::HashMap::new(),
        }
    }
}

impl CImportScannerFsmFrameEvent {
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

struct CImportScannerFsmFrameContext {
    event: CImportScannerFsmFrameEvent,
    _return: Option<Box<dyn std::any::Any>>,
    _data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl CImportScannerFsmFrameContext {
    fn new(event: CImportScannerFsmFrameEvent, default_return: Option<Box<dyn std::any::Any>>) -> Self {
        Self {
            event,
            _return: default_return,
            _data: std::collections::HashMap::new(),
        }
    }
}

#[derive(Clone)]
struct CImportScannerFsmCompartment {
    state: String,
    state_vars: std::collections::HashMap<String, i32>,
    enter_args: std::collections::HashMap<String, String>,
    exit_args: std::collections::HashMap<String, String>,
    forward_event: Option<CImportScannerFsmFrameEvent>,
    parent_compartment: Option<Box<CImportScannerFsmCompartment>>,
}

impl CImportScannerFsmCompartment {
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
enum CImportScannerFsmStateContext {
    Init,
    Scanning,
    Empty,
}

impl Default for CImportScannerFsmStateContext {
    fn default() -> Self {
        CImportScannerFsmStateContext::Init
    }
}

pub struct CImportScannerFsm {
    _state_stack: Vec<(String, CImportScannerFsmStateContext)>,
    __compartment: CImportScannerFsmCompartment,
    __next_compartment: Option<CImportScannerFsmCompartment>,
    _context_stack: Vec<CImportScannerFsmFrameContext>,
    pub bytes: Vec<u8>,
    pub start: usize,
    pub spans: Vec<RegionSpan>,
    pub issues: Vec<ValidationIssue>,
}

impl CImportScannerFsm {
    pub fn new() -> Self {
        let mut this = Self {
            _state_stack: vec![],
            _context_stack: vec![],
            bytes: Vec::new(),
            start: 0,
            spans: Vec::new(),
            issues: Vec::new(),
            __compartment: CImportScannerFsmCompartment::new("Init"),
            __next_compartment: None,
        };
let __frame_event = CImportScannerFsmFrameEvent::new("$>");
this.__kernel(__frame_event);
        this
    }

    fn __kernel(&mut self, __e: CImportScannerFsmFrameEvent) {
// Route event to current state
self.__router(&__e);
// Process any pending transition
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    // Exit current state (with exit_args from current compartment)
    let exit_event = CImportScannerFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    // Switch to new compartment
    self.__compartment = next_compartment;
    // Enter new state (or forward event)
    if self.__compartment.forward_event.is_none() {
        let enter_event = CImportScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        // Forward event to new state
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            // Forwarding enter event - just send it
            self.__router(&forward_event);
        } else {
            // Forwarding other event - send $> first, then forward
            let enter_event = CImportScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
    }

    fn __router(&mut self, __e: &CImportScannerFsmFrameEvent) {
match self.__compartment.state.as_str() {
    "Init" => self._state_Init(__e),
    "Scanning" => self._state_Scanning(__e),
    _ => {}
}
    }

    fn __transition(&mut self, next_compartment: CImportScannerFsmCompartment) {
self.__next_compartment = Some(next_compartment);
    }

    fn _state_stack_push(&mut self) {
let state_context = match self.__compartment.state.as_str() {
    "Init" => CImportScannerFsmStateContext::Init,
    "Scanning" => CImportScannerFsmStateContext::Scanning,
    _ => CImportScannerFsmStateContext::Empty,
};
self._state_stack.push((self.__compartment.state.clone(), state_context));
    }

    fn _state_stack_pop(&mut self) {
let (saved_state, state_context) = self._state_stack.pop().unwrap();
let exit_event = CImportScannerFsmFrameEvent::new("$<");
self.__router(&exit_event);
self.__compartment.state = saved_state;
match state_context {
    CImportScannerFsmStateContext::Init => {}
    CImportScannerFsmStateContext::Scanning => {}
    CImportScannerFsmStateContext::Empty => {}
}
    }

    pub fn do_scan(&mut self) {
let mut __e = CImportScannerFsmFrameEvent::new("do_scan");
let __ctx = CImportScannerFsmFrameContext::new(__e.clone(), None);
self._context_stack.push(__ctx);
match self.__compartment.state.as_str() {
            "Init" => { self._s_Init_do_scan(&__e); }
            _ => {}
        }
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    let exit_event = CImportScannerFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    self.__compartment = next_compartment;
    if self.__compartment.forward_event.is_none() {
        let enter_event = CImportScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            self.__router(&forward_event);
        } else {
            let enter_event = CImportScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
self._context_stack.pop();
    }

    fn _state_Scanning(&mut self, __e: &CImportScannerFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_Scanning_enter(__e); }
    _ => {}
}
    }

    fn _state_Init(&mut self, __e: &CImportScannerFsmFrameEvent) {
match __e.message.as_str() {
    "do_scan" => { self._s_Init_do_scan(__e); }
    _ => {}
}
    }

    fn _s_Scanning_enter(&mut self, __e: &CImportScannerFsmFrameEvent) {
let n = self.bytes.len();
let mut i = self.start;
let mut at_sol = true;
while i < n {
    if at_sol {
        if self.bytes[i] == b'\n' || self.bytes[i] == b'\r' { i += 1; continue; }
        let line_start = i;
        let mut j = i;
        while j < n && (self.bytes[j] == b' ' || self.bytes[j] == b'\t') { j += 1; }
        if j < n && self.bytes[j] == b'#' {
            // Check for #include
            let mut k = j + 1;
            while k < n && (self.bytes[k] == b' ' || self.bytes[k] == b'\t') { k += 1; }
            let key_start = k;
            while k < n && self.bytes[k].is_ascii_alphabetic() { k += 1; }
            let kw = &self.bytes[key_start..k];
            if kw == b"include" {
                // Logical line: handle backslash continuation
                let mut p = k;
                loop {
                    while p < n && self.bytes[p] != b'\n' { p += 1; }
                    // Check backslash continuation
                    let mut q = if p > 0 { p - 1 } else { 0 };
                    while q > line_start && (self.bytes[q] == b' ' || self.bytes[q] == b'\t' || self.bytes[q] == b'\r') { q -= 1; }
                    let cont = self.bytes.get(q).copied() == Some(b'\\');
                    if cont {
                        if p < n { p += 1; continue; } else { break; }
                    } else {
                        break;
                    }
                }
                // Basic closure check for <...> or "..."
                let mut has_closure = false;
                let mut s = k;
                while s < p && (self.bytes[s] == b' ' || self.bytes[s] == b'\t') { s += 1; }
                if s < p {
                    if self.bytes[s] == b'<' {
                        let mut t = s + 1;
                        while t < p { if self.bytes[t] == b'>' { has_closure = true; break; } t += 1; }
                    } else if self.bytes[s] == b'"' {
                        let mut t = s + 1;
                        let mut esc = false;
                        while t < p {
                            let b = self.bytes[t];
                            if esc { esc = false; t += 1; continue; }
                            if b == b'\\' { esc = true; t += 1; continue; }
                            if b == b'"' { has_closure = true; break; }
                            t += 1;
                        }
                    }
                }
                if !has_closure {
                    self.issues.push(ValidationIssue { message: "E110: unterminated include directive".into() });
                }
                self.spans.push(RegionSpan { start: line_start, end: p.min(n) });
                i = p.min(n);
                at_sol = true;
                continue;
            } else {
                break; // End of import section
            }
        }
        break; // Not a directive -> end imports
    } else {
        if self.bytes[i] == b'\n' { at_sol = true; i += 1; } else { i += 1; }
    }
}
    }

    fn _s_Init_do_scan(&mut self, __e: &CImportScannerFsmFrameEvent) {
let mut __compartment = CImportScannerFsmCompartment::new("Scanning");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }
}
