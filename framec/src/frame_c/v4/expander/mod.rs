use crate::frame_c::v4::mir::MirItem;

pub trait FrameStatementExpander {
    fn expand(&self, mir: &MirItem, indent: usize, system_ctx: Option<&str>) -> String;
}

pub struct PyExpander;
pub struct TsExpander;
pub struct CExpander;
pub struct CppExpander;
pub struct JavaExpander;
pub struct RustExpander;

// Facade-mode expanders (wrapper-call expansions for strict/native validation)
pub struct PyFacadeExpander;
pub struct TsFacadeExpander;
pub struct CFacadeExpander;
pub struct CppFacadeExpander;
pub struct JavaFacadeExpander;
pub struct RustFacadeExpander;

impl FrameStatementExpander for PyExpander {
    fn expand(&self, mir: &MirItem, indent: usize, system_ctx: Option<&str>) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItem::Transition{ target, exit_args, enter_args, state_args, .. } => {
                let id = match system_ctx { Some(sys) => format!("__{}_state_{}", sys, target), None => target.clone() };
                let mut out = String::new();
                out.push_str(&format!("{}next_compartment = FrameCompartment(\"{}\")\n", pad, id));
                if !exit_args.is_empty() {
                    out.push_str(&format!("{}next_compartment.exit_args = ({})\n", pad, exit_args.join(", ")));
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
            MirItem::Forward{ .. } => {
                // Forwards are non-terminal: dispatch to parent and continue
                let mut out = String::new();
                out.push_str(&format!("{}self._frame_router(__e, compartment.parent_compartment)\n", pad));
                out
            }
            MirItem::TransitionForward{ target, .. } => {
                // Transition to state then forward current event
                let id = match system_ctx { Some(sys) => format!("__{}_state_{}", sys, target), None => target.clone() };
                let mut out = String::new();
                out.push_str(&format!("{}next_compartment = FrameCompartment(\"{}\")\n", pad, id));
                out.push_str(&format!("{}self._frame_transition(next_compartment)\n", pad));
                out.push_str(&format!("{}return self._frame_router(__e, self._compartment)\n", pad));
                out
            }
            MirItem::StackPush{ .. } => format!("{}self._frame_stack_push()\n", pad),
            MirItem::StackPop{ .. } => format!("{}self._frame_stack_pop()\n", pad),
            MirItem::SystemReturn{ expr, .. } => {
                if expr.is_empty() {
                    format!("{}return\n", pad)
                } else {
                    format!("{}self._return_value = {}\n{}return\n", pad, expr, pad)
                }
            }
            MirItem::SystemReturnExpr{ .. } => {
                format!("{}self._return_value", pad)
            }
        }
    }
}

impl FrameStatementExpander for TsExpander {
    fn expand(&self, mir: &MirItem, indent: usize, system_ctx: Option<&str>) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItem::Transition{ target, exit_args, enter_args, state_args, .. } => {
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
            MirItem::Forward{ .. } => {
                // Forwards are non-terminal: dispatch to parent and continue
                let mut out = String::new();
                out.push_str(&format!("{}this._frame_router(__e, compartment.parentCompartment);\n", pad));
                out
            }
            MirItem::TransitionForward{ target, .. } => {
                // Transition to state then forward current event
                let id = match system_ctx { Some(sys) => format!("__{}_state_{}", sys, target), None => target.clone() };
                let mut out = String::new();
                out.push_str(&format!("{}{{\n", pad));
                out.push_str(&format!("{}const __frameNextCompartment = new FrameCompartment(\"{}\");\n", pad, id));
                out.push_str(&format!("{}this._frame_transition(__frameNextCompartment);\n", pad));
                out.push_str(&format!("{}return this._frame_router(__e, this._compartment);\n", pad));
                out.push_str(&format!("{}}}\n", pad));
                out
            }
            MirItem::StackPush{ .. } => format!("{}this._frame_stack_push();\n", pad),
            MirItem::StackPop{ .. } => format!("{}this._frame_stack_pop();\n", pad),
            MirItem::SystemReturn{ expr, .. } => {
                if expr.is_empty() {
                    format!("{}return;\n", pad)
                } else {
                    format!("{}this._return_value = {};\n{}return;\n", pad, expr, pad)
                }
            }
            MirItem::SystemReturnExpr{ .. } => {
                format!("{}this._return_value", pad)
            }
        }
    }
}

// Minimal comment-only expanders for other languages
impl FrameStatementExpander for CExpander {
    fn expand(&self, mir: &MirItem, indent: usize, system_ctx: Option<&str>) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItem::Transition{ target, state_args: _, .. } => {
                let id_raw = match system_ctx { Some(sys) => format!("__{}_state_{}", sys, target), None => target.clone() };
                let id = format!("\"{}\"", id_raw);
                let mut out = String::new();
                out.push_str(&format!("{}FrameCompartment next_compartment = frame_compartment_new({});\n", pad, id));
                out.push_str(&format!("{}_frame_transition(&next_compartment);\n", pad));
                out.push_str(&format!("{}return;\n", pad));
                out
            }
            MirItem::Forward{ .. } => {
                let mut out = String::new();
                out.push_str(&format!("{}_frame_router(0, 0);\n", pad));
                out.push_str(&format!("{}return;\n", pad));
                out
            }
            MirItem::TransitionForward{ target, .. } => {
                let id_raw = match system_ctx { Some(sys) => format!("__{}_state_{}", sys, target), None => target.clone() };
                let id = format!("\"{}\"", id_raw);
                let mut out = String::new();
                out.push_str(&format!("{}FrameCompartment next_compartment = frame_compartment_new({});\n", pad, id));
                out.push_str(&format!("{}_frame_transition(&next_compartment);\n", pad));
                out.push_str(&format!("{}return _frame_router(0, 0);\n", pad));
                out
            }
            MirItem::StackPush{ .. } => format!("{}_frame_stack_push();\n", pad),
            MirItem::StackPop{ .. } => format!("{}_frame_stack_pop();\n", pad),
            MirItem::SystemReturn{ expr, .. } => {
                if expr.is_empty() {
                    format!("{}return;\n", pad)
                } else {
                    format!("{}_return_value = {};\n{}return;\n", pad, expr, pad)
                }
            }
            MirItem::SystemReturnExpr{ .. } => {
                format!("{}_return_value", pad)
            }
        }
    }
}

impl FrameStatementExpander for CppExpander {
    fn expand(&self, mir: &MirItem, indent: usize, system_ctx: Option<&str>) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItem::Transition{ target, .. } => {
                let id_raw = match system_ctx { Some(sys) => format!("__{}_state_{}", sys, target), None => target.clone() };
                let id = format!("\"{}\"", id_raw);
                let mut out = String::new();
                out.push_str(&format!("{}FrameCompartment next_compartment = frame_compartment_new({});\n", pad, id));
                out.push_str(&format!("{}_frame_transition(&next_compartment);\n", pad));
                out.push_str(&format!("{}return;\n", pad));
                out
            }
            MirItem::Forward{ .. } => {
                let mut out = String::new();
                out.push_str(&format!("{}_frame_router(nullptr, nullptr);\n", pad));
                out.push_str(&format!("{}return;\n", pad));
                out
            }
            MirItem::TransitionForward{ target, .. } => {
                let id_raw = match system_ctx { Some(sys) => format!("__{}_state_{}", sys, target), None => target.clone() };
                let id = format!("\"{}\"", id_raw);
                let mut out = String::new();
                out.push_str(&format!("{}FrameCompartment next_compartment = frame_compartment_new({});\n", pad, id));
                out.push_str(&format!("{}_frame_transition(&next_compartment);\n", pad));
                out.push_str(&format!("{}return _frame_router(nullptr, nullptr);\n", pad));
                out
            }
            MirItem::StackPush{ .. } => format!("{}_frame_stack_push();\n", pad),
            MirItem::StackPop{ .. } => format!("{}_frame_stack_pop();\n", pad),
            MirItem::SystemReturn{ expr, .. } => {
                if expr.is_empty() {
                    format!("{}return;\n", pad)
                } else {
                    format!("{}_return_value = {};\n{}return;\n", pad, expr, pad)
                }
            }
            MirItem::SystemReturnExpr{ .. } => {
                format!("{}_return_value", pad)
            }
        }
    }
}

impl FrameStatementExpander for JavaExpander {
    fn expand(&self, mir: &MirItem, indent: usize, system_ctx: Option<&str>) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItem::Transition{ target, .. } => {
                let mut out = String::new();
                let id_raw = match system_ctx { Some(sys) => format!("__{}_state_{}", sys, target), None => target.clone() };
                out.push_str(&format!("{}FrameCompartment nextCompartment = new FrameCompartment(\"{}\");\n", pad, id_raw));
                out.push_str(&format!("{}_frame_transition(nextCompartment);\n", pad));
                out.push_str(&format!("{}return;\n", pad));
                out
            }
            MirItem::Forward{ .. } => {
                let mut out = String::new();
                out.push_str(&format!("{}_frame_router(null);\n", pad));
                out.push_str(&format!("{}return;\n", pad));
                out
            }
            MirItem::TransitionForward{ target, .. } => {
                let id_raw = match system_ctx { Some(sys) => format!("__{}_state_{}", sys, target), None => target.clone() };
                let mut out = String::new();
                out.push_str(&format!("{}FrameCompartment nextCompartment = new FrameCompartment(\"{}\");\n", pad, id_raw));
                out.push_str(&format!("{}_frame_transition(nextCompartment);\n", pad));
                out.push_str(&format!("{}return _frame_router(null);\n", pad));
                out
            }
            MirItem::StackPush{ .. } => format!("{}_frame_stack_push();\n", pad),
            MirItem::StackPop{ .. } => format!("{}_frame_stack_pop();\n", pad),
            MirItem::SystemReturn{ expr, .. } => {
                if expr.is_empty() {
                    format!("{}return;\n", pad)
                } else {
                    format!("{}_returnValue = {};\n{}return;\n", pad, expr, pad)
                }
            }
            MirItem::SystemReturnExpr{ .. } => {
                format!("{}_returnValue", pad)
            }
        }
    }
}

impl FrameStatementExpander for RustExpander {
    fn expand(&self, mir: &MirItem, indent: usize, system_ctx: Option<&str>) -> String {
        let pad = " ".repeat(indent);
        // When we know the system context (module path), expand against methods
        // on the generated system struct. Otherwise, keep the legacy free-function
        // calls used by demo/exec harnesses.
        let use_methods = system_ctx.is_some();
        match mir {
            MirItem::Transition{ target, .. } => {
                let mut out = String::new();
                out.push_str(&format!(
                    "{}let next_compartment = FrameCompartment {{ state: StateId::{}, ..Default::default() }};\n",
                    pad, target
                ));
                if use_methods {
                    out.push_str(&format!("{}self._frame_transition(&next_compartment);\n", pad));
                } else {
                    out.push_str(&format!("{}_frame_transition(&next_compartment);\n", pad));
                }
                out.push_str(&format!("{}return;\n", pad));
                out
            }
            MirItem::Forward{ .. } => {
                // Forwards are non-terminal
                if use_methods {
                    format!("{}self._frame_router(None);\n", pad)
                } else {
                    format!("{}_frame_router(None);\n", pad)
                }
            }
            MirItem::TransitionForward{ target, .. } => {
                let mut out = String::new();
                out.push_str(&format!(
                    "{}let next_compartment = FrameCompartment {{ state: StateId::{}, ..Default::default() }};\n",
                    pad, target
                ));
                if use_methods {
                    out.push_str(&format!("{}self._frame_transition(&next_compartment);\n", pad));
                    out.push_str(&format!("{}return self._frame_router(None);\n", pad));
                } else {
                    out.push_str(&format!("{}_frame_transition(&next_compartment);\n", pad));
                    out.push_str(&format!("{}return _frame_router(None);\n", pad));
                }
                out
            }
            MirItem::StackPush{ .. } => {
                if use_methods {
                    format!("{}self._frame_stack_push();\n", pad)
                } else {
                    format!("{}_frame_stack_push();\n", pad)
                }
            }
            MirItem::StackPop{ .. } => {
                if use_methods {
                    format!("{}self._frame_stack_pop();\n", pad)
                } else {
                    format!("{}_frame_stack_pop();\n", pad)
                }
            }
            MirItem::SystemReturn{ expr, .. } => {
                if expr.is_empty() {
                    format!("{}return;\n", pad)
                } else {
                    if use_methods {
                        format!("{}self._return_value = {};\n{}return;\n", pad, expr, pad)
                    } else {
                        format!("{}_return_value = {};\n{}return;\n", pad, expr, pad)
                    }
                }
            }
            MirItem::SystemReturnExpr{ .. } => {
                if use_methods {
                    format!("{}self._return_value", pad)
                } else {
                    format!("{}_return_value", pad)
                }
            }
        }
    }
}

// Python wrapper-call expansion (valid Python statements; unknown name is fine for parse)
impl FrameStatementExpander for PyFacadeExpander {
    fn expand(&self, mir: &MirItem, indent: usize, _system_ctx: Option<&str>) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItem::Transition{ target, state_args, .. } => {
                let arglist = if state_args.is_empty() { String::new() } else { format!(", {}", state_args.join(", ")) };
                format!("{}__frame_transition('{}'{} )\n", pad, target, arglist)
            }
            MirItem::Forward{ .. } => format!("{}__frame_forward()\n", pad),
            MirItem::TransitionForward{ target, .. } => {
                format!("{}__frame_transition_forward('{}')\n", pad, target)
            }
            MirItem::StackPush{ .. } => format!("{}__frame_stack_push()\n", pad),
            MirItem::StackPop{ .. } => format!("{}__frame_stack_pop()\n", pad),
            MirItem::SystemReturn{ expr, .. } => {
                if expr.is_empty() {
                    format!("{}return\n", pad)
                } else {
                    format!("{}__frame_return({})\n", pad, expr)
                }
            }
            MirItem::SystemReturnExpr{ .. } => format!("{}__frame_return_value()", pad),
        }
    }
}

// TypeScript wrapper-call expansion
impl FrameStatementExpander for TsFacadeExpander {
    fn expand(&self, mir: &MirItem, indent: usize, _system_ctx: Option<&str>) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItem::Transition{ target, state_args, .. } => {
                let arglist = if state_args.is_empty() { String::new() } else { format!(", {}", state_args.join(", ")) };
                format!("{}__frame_transition('{}'{});\n", pad, target, arglist)
            }
            MirItem::Forward{ .. } => format!("{}__frame_forward();\n", pad),
            MirItem::TransitionForward{ target, .. } => {
                format!("{}__frame_transition_forward('{}');\n", pad, target)
            }
            MirItem::StackPush{ .. } => format!("{}__frame_stack_push();\n", pad),
            MirItem::StackPop{ .. } => format!("{}__frame_stack_pop();\n", pad),
            MirItem::SystemReturn{ expr, .. } => {
                if expr.is_empty() {
                    format!("{}return;\n", pad)
                } else {
                    format!("{}__frame_return({});\n", pad, expr)
                }
            }
            MirItem::SystemReturnExpr{ .. } => format!("{}__frame_return_value()", pad),
        }
    }
}

// C-like wrapper-call expansions (C/C++/Java/C#)
impl FrameStatementExpander for CFacadeExpander {
    fn expand(&self, mir: &MirItem, indent: usize, _system_ctx: Option<&str>) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItem::Transition{ target, .. } => {
                // Exec/facade markers for C-like/Java/C# accept only the state name
                format!("{}__frame_transition(\"{}\");\n", pad, target)
            }
            MirItem::Forward{ .. } => format!("{}__frame_forward();\n", pad),
            MirItem::TransitionForward{ target, .. } => {
                format!("{}__frame_transition_forward(\"{}\");\n", pad, target)
            }
            MirItem::StackPush{ .. } => format!("{}__frame_stack_push();\n", pad),
            MirItem::StackPop{ .. } => format!("{}__frame_stack_pop();\n", pad),
            MirItem::SystemReturn{ expr, .. } => {
                if expr.is_empty() {
                    format!("{}return;\n", pad)
                } else {
                    format!("{}__frame_return({});\n", pad, expr)
                }
            }
            MirItem::SystemReturnExpr{ .. } => format!("{}__frame_return_value()", pad),
        }
    }
}

impl FrameStatementExpander for CppFacadeExpander { fn expand(&self, m:&MirItem, i:usize, s:Option<&str>)->String{ CFacadeExpander.expand(m,i,s) } }
impl FrameStatementExpander for JavaFacadeExpander { fn expand(&self, m:&MirItem, i:usize, s:Option<&str>)->String{ CFacadeExpander.expand(m,i,s) } }
impl FrameStatementExpander for RustFacadeExpander {
    fn expand(&self, mir: &MirItem, indent: usize, _system_ctx: Option<&str>) -> String {
        // Rust also accepts unknown function calls syntactically
        let pad = " ".repeat(indent);
        match mir {
            MirItem::Transition{ target, state_args: _state_args, .. } => {
                // For exec/facade markers we only require the state name; ignore extra args in wrapper calls
                format!("{}__frame_transition(\"{}\");\n", pad, target)
            }
            MirItem::Forward{ .. } => format!("{}__frame_forward();\n", pad),
            MirItem::TransitionForward{ target, .. } => {
                format!("{}__frame_transition_forward(\"{}\");\n", pad, target)
            }
            MirItem::StackPush{ .. } => format!("{}__frame_stack_push();\n", pad),
            MirItem::StackPop{ .. } => format!("{}__frame_stack_pop();\n", pad),
            MirItem::SystemReturn{ expr, .. } => {
                if expr.is_empty() {
                    format!("{}return;\n", pad)
                } else {
                    format!("{}__frame_return({});\n", pad, expr)
                }
            }
            MirItem::SystemReturnExpr{ .. } => format!("{}__frame_return_value()", pad),
        }
    }
}
