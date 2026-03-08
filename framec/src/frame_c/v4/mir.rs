use crate::frame_c::v4::native_region_scanner::RegionSpan;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MirItem {
    // Transition with full argument buckets
    // Syntax (Frame): (exit_args)? -> (enter_args)? label? $State(state_params?)
    Transition {
        target: String,
        exit_args: Vec<String>,
        enter_args: Vec<String>,
        state_args: Vec<String>,
        /// Optional user-provided label (e.g., -> "Path A" $State)
        label: Option<String>,
        span: RegionSpan,
    },
    Forward { span: RegionSpan },
    // Transition then forward event: -> => $State
    // Transitions to target state, then dispatches current event to the new state
    TransitionForward { target: String, span: RegionSpan },
    StackPush { span: RegionSpan },
    StackPop { span: RegionSpan },
    // System return: system.return = <expr> or ^ <expr>
    // Sets the return value and returns from handler
    SystemReturn { expr: String, span: RegionSpan },
    // System return expression read: bare system.return
    // Returns the current return value
    SystemReturnExpr { span: RegionSpan },
}
