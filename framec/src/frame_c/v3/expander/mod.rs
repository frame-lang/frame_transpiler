use crate::frame_c::v3::mir::MirItemV3;

pub trait FrameStatementExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize, system_ctx: Option<&str>) -> String;
}

pub struct PyExpanderV3;
pub struct TsExpanderV3;
pub struct CExpanderV3;
pub struct CppExpanderV3;
pub struct JavaExpanderV3;
pub struct RustExpanderV3;

// Facade-mode expanders (wrapper-call expansions for strict/native validation)
pub struct PyFacadeExpanderV3;
pub struct TsFacadeExpanderV3;
pub struct CFacadeExpanderV3;
pub struct CppFacadeExpanderV3;
pub struct JavaFacadeExpanderV3;
pub struct RustFacadeExpanderV3;

impl FrameStatementExpanderV3 for PyExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize, system_ctx: Option<&str>) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItemV3::Transition{ target, exit_args, enter_args, state_args, .. } => {
                let id = match system_ctx { Some(sys) => format!("__{}_state_{}", sys, target), None => target.clone() };
                let mut out = String::new();
                out.push_str(&format!("{}next_compartment = FrameCompartment(\"{}\")\n", pad, id));
                if !exit_args.is_empty() {
                    out.push_str(&format!("{}compartment.exit_args = ({})\n", pad, exit_args.join(", ")));
                }
                if !enter_args.is_empty() {
                    out.push_str(&format!("{}next_compartment.enter_args = ({})\n", pad, enter_args.join(", ")));
                }
                if !state_args.is_empty() {
                    out.push_str(&format!("{}next_compartment.state_args = ({})\n", pad, state_args.join(", ")));
                }
                out.push_str(&format!("{}self._frame_transition(next_compartment)\n", pad));
                out.push_str(&format!("{}return\n", pad));
                out
            }
            MirItemV3::Forward{ .. } => {
                let mut out = String::new();
                out.push_str(&format!("{}self._frame_router(__e, compartment.parent_compartment)\n", pad));
                out.push_str(&format!("{}return\n", pad));
                out
            }
            MirItemV3::StackPush{ .. } => format!("{}self._frame_stack_push()\n", pad),
            MirItemV3::StackPop{ .. } => format!("{}self._frame_stack_pop()\n{}return\n", pad, pad),
        }
    }
}

impl FrameStatementExpanderV3 for TsExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize, system_ctx: Option<&str>) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItemV3::Transition{ target, exit_args, enter_args, state_args, .. } => {
                let id = match system_ctx { Some(sys) => format!("__{}_state_{}", sys, target), None => target.clone() };
                let mut out = String::new();
                out.push_str(&format!("{}const nextCompartment = new FrameCompartment(\"{}\");\n", pad, id));
                if !exit_args.is_empty() {
                    out.push_str(&format!("{}compartment.exitArgs = [{}];\n", pad, exit_args.join(", ")));
                }
                if !enter_args.is_empty() {
                    out.push_str(&format!("{}nextCompartment.enterArgs = [{}];\n", pad, enter_args.join(", ")));
                }
                if !state_args.is_empty() {
                    out.push_str(&format!("{}nextCompartment.stateArgs = [{}];\n", pad, state_args.join(", ")));
                }
                out.push_str(&format!("{}this._frame_transition(nextCompartment);\n", pad));
                out.push_str(&format!("{}return;\n", pad));
                out
            }
            MirItemV3::Forward{ .. } => {
                // parent forward: dispatch to parent compartment and return
                let mut out = String::new();
                out.push_str(&format!("{}this._frame_router(__e, compartment.parentCompartment);\n", pad));
                out.push_str(&format!("{}return;\n", pad));
                out
            }
            MirItemV3::StackPush{ .. } => format!("{}this._frame_stack_push();\n", pad),
            MirItemV3::StackPop{ .. } => format!("{}this._frame_stack_pop();\n{}return;\n", pad, pad),
        }
    }
}

// Minimal comment-only expanders for other languages
impl FrameStatementExpanderV3 for CExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize, _system_ctx: Option<&str>) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItemV3::Transition{ target, state_args, .. } => format!("{}// frame:transition {}({})\n", pad, target, state_args.join(", ")),
            MirItemV3::Forward{ .. } => format!("{}// frame:forward\n", pad),
            MirItemV3::StackPush{ .. } => format!("{}// frame:stack_push\n", pad),
            MirItemV3::StackPop{ .. } => format!("{}// frame:stack_pop\n", pad),
        }
    }
}

impl FrameStatementExpanderV3 for CppExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize, system_ctx: Option<&str>) -> String {
        CExpanderV3.expand(mir, indent, system_ctx)
    }
}

impl FrameStatementExpanderV3 for JavaExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize, system_ctx: Option<&str>) -> String {
        CExpanderV3.expand(mir, indent, system_ctx)
    }
}

impl FrameStatementExpanderV3 for RustExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize, system_ctx: Option<&str>) -> String {
        CExpanderV3.expand(mir, indent, system_ctx)
    }
}

// Python wrapper-call expansion (valid Python statements; unknown name is fine for parse)
impl FrameStatementExpanderV3 for PyFacadeExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize, _system_ctx: Option<&str>) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItemV3::Transition{ target, state_args, .. } => {
                let arglist = if state_args.is_empty() { String::new() } else { format!(", {}", state_args.join(", ")) };
                format!("{}__frame_transition('{}'{} )\n", pad, target, arglist)
            }
            MirItemV3::Forward{ .. } => format!("{}__frame_forward()\n", pad),
            MirItemV3::StackPush{ .. } => format!("{}__frame_stack_push()\n", pad),
            MirItemV3::StackPop{ .. } => format!("{}__frame_stack_pop()\n", pad),
        }
    }
}

// TypeScript wrapper-call expansion
impl FrameStatementExpanderV3 for TsFacadeExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize, _system_ctx: Option<&str>) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItemV3::Transition{ target, state_args, .. } => {
                let arglist = if state_args.is_empty() { String::new() } else { format!(", {}", state_args.join(", ")) };
                format!("{}__frame_transition('{}'{});\n", pad, target, arglist)
            }
            MirItemV3::Forward{ .. } => format!("{}__frame_forward();\n", pad),
            MirItemV3::StackPush{ .. } => format!("{}__frame_stack_push();\n", pad),
            MirItemV3::StackPop{ .. } => format!("{}__frame_stack_pop();\n", pad),
        }
    }
}

// C-like wrapper-call expansions (C/C++/Java/C#)
impl FrameStatementExpanderV3 for CFacadeExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize, _system_ctx: Option<&str>) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItemV3::Transition{ target, state_args, .. } => {
                let arglist = if state_args.is_empty() { String::new() } else { format!(", {}", state_args.join(", ")) };
                // Use double quotes for string literal in C-like languages
                format!("{}__frame_transition(\"{}\"{});\n", pad, target, arglist)
            }
            MirItemV3::Forward{ .. } => format!("{}__frame_forward();\n", pad),
            MirItemV3::StackPush{ .. } => format!("{}__frame_stack_push();\n", pad),
            MirItemV3::StackPop{ .. } => format!("{}__frame_stack_pop();\n", pad),
        }
    }
}

impl FrameStatementExpanderV3 for CppFacadeExpanderV3 { fn expand(&self, m:&MirItemV3, i:usize, s:Option<&str>)->String{ CFacadeExpanderV3.expand(m,i,s) } }
impl FrameStatementExpanderV3 for JavaFacadeExpanderV3 { fn expand(&self, m:&MirItemV3, i:usize, s:Option<&str>)->String{ CFacadeExpanderV3.expand(m,i,s) } }
impl FrameStatementExpanderV3 for RustFacadeExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize, _system_ctx: Option<&str>) -> String {
        // Rust also accepts unknown function calls syntactically
        let pad = " ".repeat(indent);
        match mir {
            MirItemV3::Transition{ target, state_args, .. } => {
                let arglist = if state_args.is_empty() { String::new() } else { format!(", {}", state_args.join(", ")) };
                // Use double quotes as Rust char literals use single quotes
                format!("{}__frame_transition(\"{}\"{});\n", pad, target, arglist)
            }
            MirItemV3::Forward{ .. } => format!("{}__frame_forward();\n", pad),
            MirItemV3::StackPush{ .. } => format!("{}__frame_stack_push();\n", pad),
            MirItemV3::StackPop{ .. } => format!("{}__frame_stack_pop();\n", pad),
        }
    }
}
