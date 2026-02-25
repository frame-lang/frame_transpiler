#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameSegmentKind {
    Transition,
    TransitionForward, // -> => $State - transition then forward event
    Forward,
    StackPush,
    StackPop,
    StateVar,          // $.varName (read access)
    StateVarAssign,    // $.varName = expr (assignment)
    SystemReturn,      // system.return = <expr> or return <expr> (sugar)
    SystemReturnExpr,  // bare system.return (read current value)
    // System context syntax (@@)
    ContextParamShorthand, // @@.param - shorthand for parameter access
    ContextReturn,         // @@:return - return value slot (assignment or read)
    ContextEvent,          // @@:event - interface event name
    ContextData,           // @@:data[key] - call-scoped data
    ContextParams,         // @@:params[key] - explicit parameter access
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionSpan { pub start: usize, pub end: usize }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Region {
    NativeText { span: RegionSpan },
    FrameSegment { span: RegionSpan, kind: FrameSegmentKind, indent: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanResult { pub close_byte: usize, pub regions: Vec<Region> }

#[derive(Debug)]
pub enum ScanErrorKind { UnterminatedProtected, Internal }

#[derive(Debug)]
pub struct ScanError { pub kind: ScanErrorKind, pub message: String }

impl ScanError { pub fn internal(msg: &str) -> Self { Self{ kind: ScanErrorKind::Internal, message: msg.to_string() } } }

pub trait NativeRegionScanner {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResult, ScanError>;
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

