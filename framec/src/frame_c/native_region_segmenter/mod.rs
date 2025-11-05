/// NativeRegionSegmenter: splits a native region into ordered segments without reordering.
/// - Native segments are emitted verbatim by visitors.
/// - Directive segments represent Frame control directives recognized at top level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectiveKind {
    Transition,
    Forward,
    StackPush,
    StackPop,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BodySegment {
    Native {
        text: String,
        start_line: usize,
        end_line: usize,
    },
    Directive {
        kind: DirectiveKind,
        frame_line: usize,
        line_text: String,
    },
}

pub mod typescript;
