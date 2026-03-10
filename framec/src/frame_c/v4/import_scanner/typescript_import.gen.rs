
// TypeScript import scanner â Frame-generated state machine.
// Scans for import/export statements terminated by semicolons.
// Handles template literals, block/line comments.
//
// Helpers used: starts_kw, is_frame_section_start (from mod.rs)

struct TypeScriptImportScannerFsmFrameEvent {
    message: String,
    parameters: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl Clone for TypeScriptImportScannerFsmFrameEvent {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            parameters: std::collections::HashMap::new(),
        }
    }
}

impl TypeScriptImportScannerFsmFrameEvent {
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

struct TypeScriptImportScannerFsmFrameContext {
    event: TypeScriptImportScannerFsmFrameEvent,
    _return: Option<Box<dyn std::any::Any>>,
    _data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl TypeScriptImportScannerFsmFrameContext {
    fn new(event: TypeScriptImportScannerFsmFrameEvent, default_return: Option<Box<dyn std::any::Any>>) -> Self {
        Self {
            event,
            _return: default_return,
            _data: std::collections::HashMap::new(),
        }
    }
}

#[derive(Clone)]
struct TypeScriptImportScannerFsmCompartment {
    state: String,
    state_vars: std::collections::HashMap<String, i32>,
    enter_args: std::collections::HashMap<String, String>,
    exit_args: std::collections::HashMap<String, String>,
    forward_event: Option<TypeScriptImportScannerFsmFrameEvent>,
    parent_compartment: Option<Box<TypeScriptImportScannerFsmCompartment>>,
}

impl TypeScriptImportScannerFsmCompartment {
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
enum TypeScriptImportScannerFsmStateContext {
    Init,
    Scanning,
    Empty,
}

impl Default for TypeScriptImportScannerFsmStateContext {
    fn default() -> Self {
        TypeScriptImportScannerFsmStateContext::Init
    }
}

pub struct TypeScriptImportScannerFsm {
    _state_stack: Vec<(String, TypeScriptImportScannerFsmStateContext)>,
    __compartment: TypeScriptImportScannerFsmCompartment,
    __next_compartment: Option<TypeScriptImportScannerFsmCompartment>,
    _context_stack: Vec<TypeScriptImportScannerFsmFrameContext>,
    pub bytes: Vec<u8>,
    pub start: usize,
    pub spans: Vec<RegionSpan>,
    pub issues: Vec<ValidationIssue>,
}

impl TypeScriptImportScannerFsm {
    pub fn new() -> Self {
        let mut this = Self {
            _state_stack: vec![],
            _context_stack: vec![],
            bytes: Vec::new(),
            start: 0,
            spans: Vec::new(),
            issues: Vec::new(),
            __compartment: TypeScriptImportScannerFsmCompartment::new("Init"),
            __next_compartment: None,
        };
let __frame_event = TypeScriptImportScannerFsmFrameEvent::new("$>");
this.__kernel(__frame_event);
        this
    }

    fn __kernel(&mut self, __e: TypeScriptImportScannerFsmFrameEvent) {
// Route event to current state
self.__router(&__e);
// Process any pending transition
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    // Exit current state (with exit_args from current compartment)
    let exit_event = TypeScriptImportScannerFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    // Switch to new compartment
    self.__compartment = next_compartment;
    // Enter new state (or forward event)
    if self.__compartment.forward_event.is_none() {
        let enter_event = TypeScriptImportScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        // Forward event to new state
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            // Forwarding enter event - just send it
            self.__router(&forward_event);
        } else {
            // Forwarding other event - send $> first, then forward
            let enter_event = TypeScriptImportScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
    }

    fn __router(&mut self, __e: &TypeScriptImportScannerFsmFrameEvent) {
match self.__compartment.state.as_str() {
    "Init" => self._state_Init(__e),
    "Scanning" => self._state_Scanning(__e),
    _ => {}
}
    }

    fn __transition(&mut self, next_compartment: TypeScriptImportScannerFsmCompartment) {
self.__next_compartment = Some(next_compartment);
    }

    fn _state_stack_push(&mut self) {
let state_context = match self.__compartment.state.as_str() {
    "Init" => TypeScriptImportScannerFsmStateContext::Init,
    "Scanning" => TypeScriptImportScannerFsmStateContext::Scanning,
    _ => TypeScriptImportScannerFsmStateContext::Empty,
};
self._state_stack.push((self.__compartment.state.clone(), state_context));
    }

    fn _state_stack_pop(&mut self) {
let (saved_state, state_context) = self._state_stack.pop().unwrap();
let exit_event = TypeScriptImportScannerFsmFrameEvent::new("$<");
self.__router(&exit_event);
self.__compartment.state = saved_state;
match state_context {
    TypeScriptImportScannerFsmStateContext::Init => {}
    TypeScriptImportScannerFsmStateContext::Scanning => {}
    TypeScriptImportScannerFsmStateContext::Empty => {}
}
    }

    pub fn do_scan(&mut self) {
let mut __e = TypeScriptImportScannerFsmFrameEvent::new("do_scan");
let __ctx = TypeScriptImportScannerFsmFrameContext::new(__e.clone(), None);
self._context_stack.push(__ctx);
match self.__compartment.state.as_str() {
            "Init" => { self._s_Init_do_scan(&__e); }
            _ => {}
        }
while self.__next_compartment.is_some() {
    let next_compartment = self.__next_compartment.take().unwrap();
    let exit_event = TypeScriptImportScannerFsmFrameEvent::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    self.__compartment = next_compartment;
    if self.__compartment.forward_event.is_none() {
        let enter_event = TypeScriptImportScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    } else {
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {
            self.__router(&forward_event);
        } else {
            let enter_event = TypeScriptImportScannerFsmFrameEvent::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }
    }
}
self._context_stack.pop();
    }

    fn _state_Scanning(&mut self, __e: &TypeScriptImportScannerFsmFrameEvent) {
match __e.message.as_str() {
    "$>" => { self._s_Scanning_enter(__e); }
    _ => {}
}
    }

    fn _state_Init(&mut self, __e: &TypeScriptImportScannerFsmFrameEvent) {
match __e.message.as_str() {
    "do_scan" => { self._s_Init_do_scan(__e); }
    _ => {}
}
    }

    fn _s_Scanning_enter(&mut self, __e: &TypeScriptImportScannerFsmFrameEvent) {
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
        if j < n && (starts_kw(&self.bytes, j, b"import") || starts_kw(&self.bytes, j, b"export")) {
            let stmt_start = line_start;
            let mut k = j;
            let mut depth_paren: i32 = 0;
            let mut depth_brace: i32 = 0;
            let mut tmpl_depth: i32 = 0;
            let mut in_s: u8 = 0; // 1 ', 2 ", 3 backtick
            let mut found_semicolon = false;
            while k < n {
                let b = self.bytes[k];
                // Comments outside strings/templates
                if in_s == 0 {
                    if b == b'/' && k + 1 < n && self.bytes[k + 1] == b'/' {
                        k += 2; while k < n && self.bytes[k] != b'\n' { k += 1; }
                        if k < n { k += 1; continue; } else { break; }
                    }
                    if b == b'/' && k + 1 < n && self.bytes[k + 1] == b'*' {
                        k += 2;
                        while k + 1 < n { if self.bytes[k] == b'*' && self.bytes[k + 1] == b'/' { k += 2; break; } k += 1; }
                        continue;
                    }
                }
                match b {
                    b'\'' if in_s == 0 => { in_s = 1; k += 1; }
                    b'"' if in_s == 0 => { in_s = 2; k += 1; }
                    b'`' if in_s == 0 => { in_s = 3; tmpl_depth = 0; k += 1; }
                    b'`' if in_s == 3 && tmpl_depth == 0 => { in_s = 0; k += 1; }
                    b'\\' if in_s == 1 || in_s == 2 => { k += 2; }
                    b'$' if in_s == 3 && k + 1 < n && self.bytes[k + 1] == b'{' => { tmpl_depth += 1; k += 2; }
                    b'}' if in_s == 3 && tmpl_depth > 0 => { tmpl_depth -= 1; k += 1; }
                    b'\'' if in_s == 1 => { in_s = 0; k += 1; }
                    b'"' if in_s == 2 => { in_s = 0; k += 1; }
                    b'(' if in_s == 0 => { depth_paren += 1; k += 1; }
                    b')' if in_s == 0 => { depth_paren -= 1; k += 1; }
                    b'{' if in_s == 0 => { depth_brace += 1; k += 1; }
                    b'}' if in_s == 0 => { depth_brace -= 1; k += 1; }
                    b';' if in_s == 0 && depth_paren == 0 && depth_brace == 0 => {
                        self.spans.push(RegionSpan { start: stmt_start, end: k });
                        found_semicolon = true;
                        k += 1; i = k; at_sol = true; break;
                    }
                    b'\n' => { k += 1; }
                    _ => { k += 1; }
                }
            }
            if k >= n {
                if !found_semicolon || in_s != 0 || depth_paren != 0 || depth_brace != 0 || (in_s == 3 && tmpl_depth != 0) {
                    self.issues.push(ValidationIssue { message: "E110: unterminated import/export statement".into() });
                }
                self.spans.push(RegionSpan { start: stmt_start, end: n });
                break;
            }
            continue;
        }
        // Not import/export at SOL, advance to next line
        while i < n && self.bytes[i] != b'\n' { i += 1; }
        if i < n { i += 1; }
        at_sol = true;
        continue;
    } else {
        if self.bytes[i] == b'\n' { at_sol = true; i += 1; } else { i += 1; }
    }
}
    }

    fn _s_Init_do_scan(&mut self, __e: &TypeScriptImportScannerFsmFrameEvent) {
let mut __compartment = TypeScriptImportScannerFsmCompartment::new("Scanning");
__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));
self.__transition(__compartment);
return;
    }
}
