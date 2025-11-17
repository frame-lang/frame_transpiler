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
                    // For exec mode, avoid Python syntax errors on keyword-like tokens by stringifying any token containing '='.
                    let rendered: Vec<String> = state_args.iter().map(|t| {
                        if t.contains('=') && !(t.starts_with('\'') || t.starts_with('"')) { format!("'{}'", t) } else { t.clone() }
                    }).collect();
                    out.push_str(&format!("{}next_compartment.state_args = [{}]\n", pad, rendered.join(", ")));
                }
                out.push_str(&format!("{}self._frame_transition(next_compartment)\n", pad));
                out.push_str(&format!("{}return\n", pad));
                out
            }
            MirItemV3::Forward{ .. } => {
                // Forwards are non-terminal: dispatch to parent and continue
                let mut out = String::new();
                out.push_str(&format!("{}self._frame_router(__e, compartment.parent_compartment)\n", pad));
                out
            }
            MirItemV3::StackPush{ .. } => format!("{}self._frame_stack_push()\n", pad),
            MirItemV3::StackPop{ .. } => format!("{}self._frame_stack_pop()\n", pad),
        }
    }
}

impl FrameStatementExpanderV3 for TsExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize, system_ctx: Option<&str>) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItemV3::Transition{ target, exit_args, enter_args, state_args, .. } => {
                let id = match system_ctx { Some(sys) => format!("__{}_state_{}", sys, target), None => target.clone() };
                // Use a per-state temporary name and wrap each expansion in its own
                // block so we never redeclare the same identifier across cases.
                fn sanitize_ident(s: &str) -> String {
                    let mut out = String::new();
                    for ch in s.chars() {
                        if ch.is_ascii_alphanumeric() || ch == '_' {
                            out.push(ch);
                        } else {
                            out.push('_');
                        }
                    }
                    if out.is_empty() { "_".to_string() } else { out }
                }
                let suffix = sanitize_ident(target);
                let temp_name = format!("__frameNextCompartment_{}", suffix);
                let mut out = String::new();
                out.push_str(&format!("{}{{\n", pad));
                out.push_str(&format!("{}const {} = new FrameCompartment(\"{}\");\n", pad, temp_name, id));
                if !exit_args.is_empty() {
                    out.push_str(&format!("{}compartment.exitArgs = [{}];\n", pad, exit_args.join(", ")));
                }
                if !enter_args.is_empty() {
                    out.push_str(&format!("{}{}.enterArgs = [{}];\n", pad, temp_name, enter_args.join(", ")));
                }
                if !state_args.is_empty() {
                    out.push_str(&format!("{}{}.stateArgs = [{}];\n", pad, temp_name, state_args.join(", ")));
                }
                out.push_str(&format!("{}this._frame_transition({});\n", pad, temp_name));
                out.push_str(&format!("{}return;\n", pad));
                out.push_str(&format!("{}}}\n", pad));
                out
            }
            MirItemV3::Forward{ .. } => {
                // Forwards are non-terminal: dispatch to parent and continue
                let mut out = String::new();
                out.push_str(&format!("{}this._frame_router(__e, compartment.parentCompartment);\n", pad));
                out
            }
            MirItemV3::StackPush{ .. } => format!("{}this._frame_stack_push();\n", pad),
            MirItemV3::StackPop{ .. } => format!("{}this._frame_stack_pop();\n", pad),
        }
    }
}

// Minimal comment-only expanders for other languages
impl FrameStatementExpanderV3 for CExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize, system_ctx: Option<&str>) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItemV3::Transition{ target, state_args: _, .. } => {
                let id_raw = match system_ctx { Some(sys) => format!("__{}_state_{}", sys, target), None => target.clone() };
                let id = format!("\"{}\"", id_raw);
                let mut out = String::new();
                out.push_str(&format!("{}FrameCompartment next_compartment = frame_compartment_new({});\n", pad, id));
                out.push_str(&format!("{}_frame_transition(&next_compartment);\n", pad));
                out.push_str(&format!("{}return;\n", pad));
                out
            }
            MirItemV3::Forward{ .. } => {
                let mut out = String::new();
                out.push_str(&format!("{}_frame_router(0, 0);\n", pad));
                out.push_str(&format!("{}return;\n", pad));
                out
            }
            MirItemV3::StackPush{ .. } => format!("{}_frame_stack_push();\n", pad),
            MirItemV3::StackPop{ .. } => format!("{}_frame_stack_pop();\n", pad),
        }
    }
}

impl FrameStatementExpanderV3 for CppExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize, system_ctx: Option<&str>) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItemV3::Transition{ target, .. } => {
                let id_raw = match system_ctx { Some(sys) => format!("__{}_state_{}", sys, target), None => target.clone() };
                let id = format!("\"{}\"", id_raw);
                let mut out = String::new();
                out.push_str(&format!("{}FrameCompartment next_compartment = frame_compartment_new({});\n", pad, id));
                out.push_str(&format!("{}_frame_transition(&next_compartment);\n", pad));
                out.push_str(&format!("{}return;\n", pad));
                out
            }
            MirItemV3::Forward{ .. } => {
                let mut out = String::new();
                out.push_str(&format!("{}_frame_router(nullptr, nullptr);\n", pad));
                out.push_str(&format!("{}return;\n", pad));
                out
            }
            MirItemV3::StackPush{ .. } => format!("{}_frame_stack_push();\n", pad),
            MirItemV3::StackPop{ .. } => format!("{}_frame_stack_pop();\n", pad),
        }
    }
}

impl FrameStatementExpanderV3 for JavaExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize, system_ctx: Option<&str>) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItemV3::Transition{ target, .. } => {
                let mut out = String::new();
                let id_raw = match system_ctx { Some(sys) => format!("__{}_state_{}", sys, target), None => target.clone() };
                out.push_str(&format!("{}FrameCompartment nextCompartment = new FrameCompartment(\"{}\");\n", pad, id_raw));
                out.push_str(&format!("{}_frame_transition(nextCompartment);\n", pad));
                out.push_str(&format!("{}return;\n", pad));
                out
            }
            MirItemV3::Forward{ .. } => {
                let mut out = String::new();
                out.push_str(&format!("{}_frame_router(null);\n", pad));
                out.push_str(&format!("{}return;\n", pad));
                out
            }
            MirItemV3::StackPush{ .. } => format!("{}_frame_stack_push();\n", pad),
            MirItemV3::StackPop{ .. } => format!("{}_frame_stack_pop();\n", pad),
        }
    }
}

impl FrameStatementExpanderV3 for RustExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize, _system_ctx: Option<&str>) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItemV3::Transition{ target, .. } => {
                let mut out = String::new();
                out.push_str(&format!("{}let next_compartment = FrameCompartment {{ state: StateId::{}, ..Default::default() }};\n", pad, target));
                out.push_str(&format!("{}_frame_transition(&next_compartment);\n", pad));
                out.push_str(&format!("{}return;\n", pad));
                out
            }
            MirItemV3::Forward{ .. } => {
                // Forwards are non-terminal
                format!("{}_frame_router(None);\n", pad)
            }
            MirItemV3::StackPush{ .. } => format!("{}_frame_stack_push();\n", pad),
            MirItemV3::StackPop{ .. } => format!("{}_frame_stack_pop();\n", pad),
        }
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
            MirItemV3::Transition{ target, .. } => {
                // Exec/facade markers for C-like/Java/C# accept only the state name
                format!("{}__frame_transition(\"{}\");\n", pad, target)
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
            MirItemV3::Transition{ target, state_args: _state_args, .. } => {
                // For exec/facade markers we only require the state name; ignore extra args in wrapper calls
                format!("{}__frame_transition(\"{}\");\n", pad, target)
            }
            MirItemV3::Forward{ .. } => format!("{}__frame_forward();\n", pad),
            MirItemV3::StackPush{ .. } => format!("{}__frame_stack_push();\n", pad),
            MirItemV3::StackPop{ .. } => format!("{}__frame_stack_pop();\n", pad),
        }
    }
}
