// NOTE: This file is generated from framec/src/frame_c/v3/machines/indent_normalizer.frs
// via tools/gen_v3_machines_rs.py using the bootstrap compiler.
// Do not edit directly.

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
enum IndentNormalizerReturn { run(()) }

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StateId { ComputeBase }

impl Default for StateId { fn default() -> Self { StateId::ComputeBase } }

#[derive(Debug, Clone)] struct FrameEvent{ message: String }
#[derive(Debug, Clone, Default)] struct FrameCompartment{ state: StateId, forward_event: Option<FrameEvent>, exit_args: Option<()>, enter_args: Option<()>, parent_compartment: Option<*const FrameCompartment>, state_args: Option<()>, }
struct IndentNormalizer {
    compartment: FrameCompartment,
    _stack: Vec<FrameCompartment>,
    _system_return_stack: Vec<IndentNormalizerReturn>,
    lines: Vec<String>,
    flags_is_expansion: Vec<bool>,
    flags_is_comment: Vec<bool>,
    pad: String,
    base_indent: i32,
    prev_indent_norm: i32,
    has_prev_indent: bool,
    last_line_ended_with_colon: bool,
    has_non_comment: bool,
    out_lines: Vec<String>,
    i: usize,
}

impl IndentNormalizer {
    fn new() -> Self {
        Self {
            compartment: FrameCompartment{ state: StateId::ComputeBase, ..Default::default() },
            _stack: Vec::new(),
            _system_return_stack: Vec::<IndentNormalizerReturn>::new(),
            lines: Vec::new(),
            flags_is_expansion: Vec::new(),
            flags_is_comment: Vec::new(),
            pad: String::new(),
            base_indent: 0,
            prev_indent_norm: 0,
            has_prev_indent: false,
            last_line_ended_with_colon: false,
            has_non_comment: false,
            out_lines: Vec::new(),
            i: 0,
        }
    }
    fn _frame_transition(&mut self, next: &FrameCompartment){
        // Basic transition: update the active state id; other fields remain unchanged for now.
        self.compartment.state = next.state;
    }
    fn _frame_stack_push(&mut self){
        self._stack.push(self.compartment.clone());
    }
    fn _frame_stack_pop(&mut self){
        if let Some(prev) = self._stack.pop() {
            self._frame_transition(&prev);
        }
    }
}

impl IndentNormalizer {
    fn _frame_router(&mut self, e: Option<FrameEvent>) {
        if let Some(ev) = e {
            match ev.message.as_str() {
                "run" => self._event_run(),
                _ => { }
            }
        }
    }
    fn _event_run(&mut self) {
        match self.compartment.state {
            StateId::ComputeBase => {
                
                // Compute base_indent: minimal non-blank indent across lines.
                self.base_indent = 0;
                let mut base_opt: Option<usize> = None;
                for ln in &self.lines {
                let trimmed = ln.trim();
                if trimmed.is_empty() {
                continue;
                }
                let indent = ln.chars().take_while(|c| *c == ' ' || *c == '\t').count();
                base_opt = Some(base_opt.map_or(indent, |m| m.min(indent)));
                }
                self.base_indent = base_opt.unwrap_or(0) as i32;
                
                // Reset normalization state.
                self.prev_indent_norm = 0;
                self.has_prev_indent = false;
                self.last_line_ended_with_colon = false;
                self.has_non_comment = false;
                self.out_lines.clear();
                self.i = 0;
                
                // Normalize each line according to the Stage 14 algorithm.
                while (self.i as usize) < self.lines.len() {
                let ln = self.lines[self.i as usize].to_string();
                let is_expansion = self.flags_is_expansion[self.i as usize];
                let is_comment_flag = self.flags_is_comment[self.i as usize];
                self.i = self.i + 1;
                
                let raw = ln.trim_end().to_string();
                if raw.trim().is_empty() {
                self.out_lines.push(format!("{}\n", self.pad));
                continue;
                }
                
                // Rewrite system.return to stack access (Phase A: no-op in example).
                let mut t = raw.clone();
                
                let indent_orig = t.chars().take_while(|c| *c == ' ' || *c == '\t').count() as i32;
                let content = t[indent_orig as usize..].to_string();
                let trimmed = content.trim_start().to_string();
                if trimmed.is_empty() {
                self.out_lines.push(format!("{}\n", self.pad));
                continue;
                }
                
                if !is_comment_flag {
                self.has_non_comment = true;
                }
                
                // Choose normalized indent.
                let mut indent_norm = if self.last_line_ended_with_colon {
                (if self.has_prev_indent { self.prev_indent_norm } else { self.base_indent }) + 4
                } else if is_expansion {
                if self.has_prev_indent { self.prev_indent_norm } else { indent_orig }
                } else {
                indent_orig
                };
                if indent_norm < self.base_indent {
                indent_norm = self.base_indent;
                }
                let extra_width = (indent_norm - self.base_indent).max(0) as usize;
                let extra = " ".repeat(extra_width);
                
                // Handler-only sugar: `return expr` => system.return = expr; return.
                if trimmed.starts_with("return ")
                && trimmed != "return"
                && trimmed != "return:"
                {
                let expr = trimmed["return ".len()..].trim_end_matches(':').trim().to_string();
                if !expr.is_empty() {
                self.out_lines.push(format!("{}{}self._system_return_stack[-1] = {}\n", self.pad, extra, expr));
                self.out_lines.push(format!("{}{}return\n", self.pad, extra));
                self.prev_indent_norm = indent_norm;
                self.has_prev_indent = true;
                self.last_line_ended_with_colon = false;
                continue;
                }
                }
                
                self.out_lines.push(format!("{}{}{}\n", self.pad, extra, trimmed));
                self.prev_indent_norm = indent_norm;
                self.has_prev_indent = true;
                self.last_line_ended_with_colon = trimmed.ends_with(':');
                }
                
                if !self.has_non_comment {
                self.out_lines.push(format!("{}pass\n", self.pad));
                }
                
            }
            _ => { }
        }
    }
}

impl IndentNormalizer {
    fn _set_system_return_for_run(&mut self, value: ()) {
        if let Some(IndentNormalizerReturn::run(ref mut v)) = self._system_return_stack.last_mut() {
            *v = value;
        }
    }
    pub fn run(&mut self) -> () {
        let __initial: () = ();
        self._system_return_stack.push(IndentNormalizerReturn::run(__initial));
        let __event = FrameEvent{ message: "run".to_string() };
        self._frame_router(Some(__event));
        let __result = match self._system_return_stack.pop() {
            Some(IndentNormalizerReturn::run(value)) => value,
            _ => {
                ()
            }
        };
        __result
    }
}
