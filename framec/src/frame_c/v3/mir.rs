use crate::frame_c::v3::native_region_scanner::RegionSpan;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MirItemV3 {
    Transition { target: String, args: Vec<String>, span: RegionSpan },
    Forward { span: RegionSpan },
    StackPush { span: RegionSpan },
    StackPop { span: RegionSpan },
}

