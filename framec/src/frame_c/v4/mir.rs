use crate::frame_c::v4::native_region_scanner::RegionSpan;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MirItem {
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
    // Transition then forward event: -> => $State
    // Transitions to target state, then dispatches current event to the new state
    TransitionForward { target: String, span: RegionSpan },
    StackPush { span: RegionSpan },
    StackPop { span: RegionSpan },
    // Return sugar: return <expr>
    // Sets the return value and returns from handler
    ReturnSugar { expr: String, span: RegionSpan },
}
