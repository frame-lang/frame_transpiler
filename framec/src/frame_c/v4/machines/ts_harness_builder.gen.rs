// NOTE: This file is generated from framec/src/frame_c/v4/machines/ts_harness_builder.frs
// via tools/gen_machines_rs.py using the bootstrap compiler.
// Do not edit directly.

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
enum TsHarnessBuilderReturn { run(()) }

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StateId { Init }

impl Default for StateId { fn default() -> Self { StateId::Init } }

#[derive(Debug, Clone)] struct FrameEvent{ message: String }
#[derive(Debug, Clone, Default)] struct FrameCompartment{ state: StateId, forward_event: Option<FrameEvent>, exit_args: Option<()>, enter_args: Option<()>, parent_compartment: Option<*const FrameCompartment>, state_args: Option<()>, }
struct TsHarnessBuilder {
    compartment: FrameCompartment,
    _stack: Vec<FrameCompartment>,
    _system_return_stack: Vec<TsHarnessBuilderReturn>,
    lines: Vec<String>,
    out_program: String,
}

impl TsHarnessBuilder {
    fn new() -> Self {
        Self {
            compartment: FrameCompartment{ state: StateId::default(), ..Default::default() },
            _stack: Vec::new(),
            _system_return_stack: Vec::<TsHarnessBuilderReturn>::new(),
            lines: Vec::new(),
            out_program: String::new(),
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

impl TsHarnessBuilder {
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
            StateId::Init => {
                
                // Prelude that defines minimal no-op wrappers. Keep this
                // in sync with the Python runner's `_execute_ts_harness_from_spliced`.
                let prelude = [
                "function __frame_transition(state: string, ...args: any[]) {}",
                "function __frame_forward() {}",
                "function __frame_stack_push() {}",
                "function __frame_stack_pop() {}",
                ]
                .join("\n");
                
                // Collect wrapper lines.
                let mut wrappers: Vec<String> = Vec::new();
                for ln in &self.lines {
                let trimmed = ln.trim();
                if trimmed.starts_with("__frame_transition(")
                || trimmed.starts_with("__frame_forward(")
                || trimmed.starts_with("__frame_stack_")
                {
                let mut w = trimmed.to_string();
                if !w.ends_with(';') {
                w.push(';');
                }
                wrappers.push(w);
                }
                }
                
                // Join wrappers into the main body.
                let mut body = String::new();
                for (i, w) in wrappers.iter().enumerate() {
                if i > 0 {
                body.push('\n');
                }
                body.push_str(w);
                }
                
                // Assemble the full program string.
                let mut program = String::new();
                program.push_str(&prelude);
                program.push('\n');
                program.push_str("function main() {\n");
                program.push_str(&body);
                program.push_str("\n}\nmain();\n");
                
                self.out_program = program;
                
            }
            _ => { }
        }
    }
}

impl TsHarnessBuilder {
    fn _set_system_return_for_run(&mut self, value: ()) {
        if let Some(TsHarnessBuilderReturn::run(ref mut v)) = self._system_return_stack.last_mut() {
            *v = value;
        }
    }
    pub fn run(&mut self) -> () {
        let __initial: () = ();
        self._system_return_stack.push(TsHarnessBuilderReturn::run(__initial));
        let __event = FrameEvent{ message: "run".to_string() };
        self._frame_router(Some(__event));
        let __result = match self._system_return_stack.pop() {
            Some(TsHarnessBuilderReturn::run(value)) => value,
            _ => {
                ()
            }
        };
        __result
    }
}

