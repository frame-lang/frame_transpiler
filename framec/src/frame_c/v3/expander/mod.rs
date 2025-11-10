use crate::frame_c::v3::mir::MirItemV3;

pub trait FrameStatementExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize) -> String;
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
    fn expand(&self, mir: &MirItemV3, indent: usize) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItemV3::Transition{ target, args, .. } => format!("{}# frame:transition {}({})\n", pad, target, args.join(", ")),
            MirItemV3::Forward{ .. } => format!("{}# frame:forward\n", pad),
            MirItemV3::StackPush{ .. } => format!("{}# frame:stack_push\n", pad),
            MirItemV3::StackPop{ .. } => format!("{}# frame:stack_pop\n", pad),
        }
    }
}

impl FrameStatementExpanderV3 for TsExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItemV3::Transition{ target, args, .. } => format!("{}// frame:transition {}({})\n", pad, target, args.join(", ")),
            MirItemV3::Forward{ .. } => format!("{}// frame:forward\n", pad),
            MirItemV3::StackPush{ .. } => format!("{}// frame:stack_push\n", pad),
            MirItemV3::StackPop{ .. } => format!("{}// frame:stack_pop\n", pad),
        }
    }
}

// Minimal comment-only expanders for other languages
impl FrameStatementExpanderV3 for CExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItemV3::Transition{ target, args, .. } => format!("{}// frame:transition {}({})\n", pad, target, args.join(", ")),
            MirItemV3::Forward{ .. } => format!("{}// frame:forward\n", pad),
            MirItemV3::StackPush{ .. } => format!("{}// frame:stack_push\n", pad),
            MirItemV3::StackPop{ .. } => format!("{}// frame:stack_pop\n", pad),
        }
    }
}

impl FrameStatementExpanderV3 for CppExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize) -> String {
        CExpanderV3.expand(mir, indent)
    }
}

impl FrameStatementExpanderV3 for JavaExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize) -> String {
        CExpanderV3.expand(mir, indent)
    }
}

impl FrameStatementExpanderV3 for RustExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize) -> String {
        CExpanderV3.expand(mir, indent)
    }
}

// Python wrapper-call expansion (valid Python statements; unknown name is fine for parse)
impl FrameStatementExpanderV3 for PyFacadeExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItemV3::Transition{ target, args, .. } => {
                let arglist = if args.is_empty() { String::new() } else { format!(", {}", args.join(", ")) };
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
    fn expand(&self, mir: &MirItemV3, indent: usize) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItemV3::Transition{ target, args, .. } => {
                let arglist = if args.is_empty() { String::new() } else { format!(", {}", args.join(", ")) };
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
    fn expand(&self, mir: &MirItemV3, indent: usize) -> String {
        let pad = " ".repeat(indent);
        match mir {
            MirItemV3::Transition{ target, args, .. } => {
                let arglist = if args.is_empty() { String::new() } else { format!(", {}", args.join(", ")) };
                // Use double quotes for string literal in C-like languages
                format!("{}__frame_transition(\"{}\"{});\n", pad, target, arglist)
            }
            MirItemV3::Forward{ .. } => format!("{}__frame_forward();\n", pad),
            MirItemV3::StackPush{ .. } => format!("{}__frame_stack_push();\n", pad),
            MirItemV3::StackPop{ .. } => format!("{}__frame_stack_pop();\n", pad),
        }
    }
}

impl FrameStatementExpanderV3 for CppFacadeExpanderV3 { fn expand(&self, m:&MirItemV3, i:usize)->String{ CFacadeExpanderV3.expand(m,i) } }
impl FrameStatementExpanderV3 for JavaFacadeExpanderV3 { fn expand(&self, m:&MirItemV3, i:usize)->String{ CFacadeExpanderV3.expand(m,i) } }
impl FrameStatementExpanderV3 for RustFacadeExpanderV3 {
    fn expand(&self, mir: &MirItemV3, indent: usize) -> String {
        // Rust also accepts unknown function calls syntactically
        let pad = " ".repeat(indent);
        match mir {
            MirItemV3::Transition{ target, args, .. } => {
                let arglist = if args.is_empty() { String::new() } else { format!(", {}", args.join(", ")) };
                // Use double quotes as Rust char literals use single quotes
                format!("{}__frame_transition(\"{}\"{});\n", pad, target, arglist)
            }
            MirItemV3::Forward{ .. } => format!("{}__frame_forward();\n", pad),
            MirItemV3::StackPush{ .. } => format!("{}__frame_stack_push();\n", pad),
            MirItemV3::StackPop{ .. } => format!("{}__frame_stack_pop();\n", pad),
        }
    }
}
