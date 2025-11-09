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
