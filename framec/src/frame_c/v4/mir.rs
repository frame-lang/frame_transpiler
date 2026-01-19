use crate::frame_c::v4::native_region_scanner::RegionSpan;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MirItemV3 {
    // Transition with full argument buckets
    // Syntax (Frame): (exit_args)? -> (enter_args)? $State(state_params?)
    Transition {
        target: String,
        exit_args: Vec<String>,
        enter_args: Vec<String>,
        state_args: Vec<String>,
        span: RegionSpan,
    },
    Forward { span: RegionSpan },
    StackPush { span: RegionSpan },
    StackPop { span: RegionSpan },
}
