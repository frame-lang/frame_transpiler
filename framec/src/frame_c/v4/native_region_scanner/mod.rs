#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameSegmentKindV3 {
    Transition,
    TransitionForward, // -> => $State - transition then forward event
    Forward,
    StackPush,
    StackPop,
    StateVar,
    ParentStateVar,    // $^.varName - access parent state variable
    SystemReturn,      // system.return = <expr> or return <expr> (sugar)
    SystemReturnExpr,  // bare system.return (read current value)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionSpan { pub start: usize, pub end: usize }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegionV3 {
    NativeText { span: RegionSpan },
    FrameSegment { span: RegionSpan, kind: FrameSegmentKindV3, indent: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanResultV3 { pub close_byte: usize, pub regions: Vec<RegionV3> }

#[derive(Debug)]
pub enum ScanErrorV3Kind { UnterminatedProtected, Internal }

#[derive(Debug)]
pub struct ScanErrorV3 { pub kind: ScanErrorV3Kind, pub message: String }

impl ScanErrorV3 { pub fn internal(msg: &str) -> Self { Self{ kind: ScanErrorV3Kind::Internal, message: msg.to_string() } } }

pub trait NativeRegionScannerV3 {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResultV3, ScanErrorV3>;
}

// Unified scanner architecture - Frame statement detection is shared,
// only language-specific syntax skipping differs
pub mod unified;

pub mod python;
pub mod typescript;
pub mod csharp;
pub mod c;
pub mod cpp;
pub mod java;
pub mod rust;

