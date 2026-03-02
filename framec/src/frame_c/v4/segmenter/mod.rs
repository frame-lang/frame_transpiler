//! Source Segmenter (Stage 0)
//!
//! The Segmenter is the **only stage that implements the Oceans Model**. It scans raw source
//! bytes and produces a `SourceMap` — an ordered list of segments that partitions the entire
//! source file into typed regions: native code, pragmas, and @@system blocks.
//!
//! ## Architecture
//!
//! - Uses `SyntaxSkipper` to avoid false `@@` detection inside strings/comments
//! - Uses `BodyCloser` to find matching close brace for `@@system` blocks
//! - Downstream stages (Lexer, Parser, etc.) receive only their segment — they never
//!   see raw source bytes for boundary detection
//!
//! ## Reuse
//!
//! This consolidates and replaces:
//! - `PragmaScanner` (existing)
//! - `skip_native_preamble()` in frame_parser.rs
//! - `skip_pragmas()` in frame_parser.rs
//! - `skip_pragmas_keep_native()` in compiler.rs
//! - `extract_native_code()` in compiler.rs

use std::convert::TryFrom;

use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v4::native_region_scanner::unified::SyntaxSkipper;
use crate::frame_c::v4::native_region_scanner::RegionSpan;

// ============================================================================
// SourceMap and Segment types
// ============================================================================

/// Span within the source bytes
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }
}

/// The kind of Frame pragma
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PragmaKind {
    /// @@target <language>
    Target,
    /// @@persist (optionally with library)
    Persist,
    /// @@codegen { ... }
    Codegen,
    /// @@run-expect <pattern>
    RunExpect,
    /// @@skip-if <condition>
    SkipIf,
    /// @@timeout <seconds>
    Timeout,
    /// @@async
    Async,
    /// Unknown or future pragma
    Other,
}

/// A segment in the source file
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Segment {
    /// Native code region — passes through verbatim to output
    Native {
        span: Span,
    },
    /// Pragma line (@@target, @@persist, @@run-expect, etc.)
    Pragma {
        kind: PragmaKind,
        span: Span,
        /// Parsed pragma value (e.g., "python_3" for @@target)
        value: Option<String>,
    },
    /// @@system block — contents will be lexed, parsed, and compiled
    System {
        /// Span of the entire block including `@@system Name { ... }`
        outer_span: Span,
        /// Span of just the system body (between { and }, exclusive of braces)
        body_span: Span,
        /// System name extracted during segmentation
        name: String,
    },
}

/// The result of source segmentation — an ordered, non-overlapping partition of the source
#[derive(Debug)]
pub struct SourceMap {
    /// Ordered, non-overlapping segments covering the entire source
    pub segments: Vec<Segment>,
    /// Original source bytes (owned, for content extraction by Lexer)
    pub source: Vec<u8>,
    /// Target language (from @@target pragma, if present)
    pub target: Option<TargetLanguage>,
}

impl SourceMap {
    /// Get all system segments
    pub fn systems(&self) -> Vec<&Segment> {
        self.segments.iter().filter(|s| matches!(s, Segment::System { .. })).collect()
    }

    /// Get all native segments
    pub fn natives(&self) -> Vec<&Segment> {
        self.segments.iter().filter(|s| matches!(s, Segment::Native { .. })).collect()
    }

    /// Get all pragma segments
    pub fn pragmas(&self) -> Vec<&Segment> {
        self.segments.iter().filter(|s| matches!(s, Segment::Pragma { .. })).collect()
    }

    /// Extract text content for a span
    pub fn text(&self, span: &Span) -> &str {
        let start = span.start.min(self.source.len());
        let end = span.end.min(self.source.len());
        std::str::from_utf8(&self.source[start..end]).unwrap_or("")
    }

    /// Get the @@persist pragma if present
    pub fn persist_pragma(&self) -> Option<&Segment> {
        self.segments.iter().find(|s| matches!(s, Segment::Pragma { kind: PragmaKind::Persist, .. }))
    }

    /// Get the @@run-expect value if present
    pub fn run_expect(&self) -> Option<&str> {
        self.segments.iter().find_map(|s| {
            if let Segment::Pragma { kind: PragmaKind::RunExpect, value: Some(v), .. } = s {
                Some(v.as_str())
            } else {
                None
            }
        })
    }
}

// ============================================================================
// Segmentation errors
// ============================================================================

/// Error during segmentation
#[derive(Debug)]
pub enum SegmentError {
    /// Unterminated @@system block (no matching close brace)
    UnterminatedSystem { name: String, open_brace_pos: usize },
    /// Unterminated string literal found during scanning
    UnterminatedString { pos: usize },
    /// Unterminated comment found during scanning
    UnterminatedComment { pos: usize },
    /// Invalid @@target value
    InvalidTarget { value: String, pos: usize },
}

impl std::fmt::Display for SegmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SegmentError::UnterminatedSystem { name, open_brace_pos } => {
                write!(f, "Unterminated @@system block '{}' (opening brace at byte {})", name, open_brace_pos)
            }
            SegmentError::UnterminatedString { pos } => {
                write!(f, "Unterminated string literal at byte {}", pos)
            }
            SegmentError::UnterminatedComment { pos } => {
                write!(f, "Unterminated comment at byte {}", pos)
            }
            SegmentError::InvalidTarget { value, pos } => {
                write!(f, "Invalid @@target value '{}' at byte {}", value, pos)
            }
        }
    }
}

// ============================================================================
// Segmenter implementation
// ============================================================================

/// Segment raw source bytes into native code, pragmas, and @@system blocks.
///
/// This is the entry point for Stage 0 of the pipeline. It uses the given
/// `SyntaxSkipper` to avoid false `@@` detection inside strings and comments.
pub fn segment<S: SyntaxSkipper>(
    skipper: &S,
    source: &[u8],
) -> Result<SourceMap, SegmentError> {
    let mut segments: Vec<Segment> = Vec::new();
    let n = source.len();
    let mut i = 0usize;
    let mut seg_start = 0usize;
    let mut at_sol = true;
    let mut target: Option<TargetLanguage> = None;

    while i < n {
        // At start of line, check for @@ pragma
        if at_sol {
            // Skip leading whitespace
            let line_start = i;
            while i < n && (source[i] == b' ' || source[i] == b'\t') {
                i += 1;
            }

            // Check for @@ pragma
            if i + 1 < n && source[i] == b'@' && source[i + 1] == b'@' {
                // Found pragma — emit any preceding native text
                if seg_start < line_start {
                    segments.push(Segment::Native {
                        span: Span { start: seg_start, end: line_start },
                    });
                }

                let pragma_start = i;
                let (kind, pragma_value) = identify_pragma(source, i);

                match kind {
                    PragmaKind::Target => {
                        // Parse and validate target language
                        if let Some(ref val) = pragma_value {
                            match TargetLanguage::try_from(val.as_str()) {
                                Ok(lang) => target = Some(lang),
                                Err(_) => {
                                    return Err(SegmentError::InvalidTarget {
                                        value: val.clone(),
                                        pos: pragma_start,
                                    });
                                }
                            }
                        }
                        let line_end = find_line_end(source, pragma_start);
                        segments.push(Segment::Pragma {
                            kind: PragmaKind::Target,
                            span: Span { start: pragma_start, end: line_end },
                            value: pragma_value,
                        });
                        i = line_end;
                        seg_start = i;
                        at_sol = true;
                        continue;
                    }
                    PragmaKind::Codegen => {
                        // @@codegen { ... } — find closing brace
                        let block_end = find_pragma_block_end(skipper, source, pragma_start)?;
                        segments.push(Segment::Pragma {
                            kind: PragmaKind::Codegen,
                            span: Span { start: pragma_start, end: block_end },
                            value: pragma_value,
                        });
                        i = block_end;
                        seg_start = i;
                        at_sol = true;
                        continue;
                    }
                    PragmaKind::Persist | PragmaKind::Async => {
                        // These can optionally have a block body
                        // Check if there's a { on the same line
                        let line_end_pos = find_line_end(source, pragma_start);
                        let has_brace = source[pragma_start..line_end_pos].iter().any(|&b| b == b'{');
                        let end = if has_brace {
                            find_pragma_block_end(skipper, source, pragma_start)?
                        } else {
                            line_end_pos
                        };
                        segments.push(Segment::Pragma {
                            kind: kind.clone(),
                            span: Span { start: pragma_start, end },
                            value: pragma_value,
                        });
                        i = end;
                        seg_start = i;
                        at_sol = true;
                        continue;
                    }
                    _ if kind == PragmaKind::Other && is_system_pragma(source, i) => {
                        // @@system Name { ... }
                        let (system_name, name_end) = extract_system_name(source, i);

                        // Find the opening brace
                        let mut brace_pos = name_end;
                        while brace_pos < n && source[brace_pos] != b'{' && source[brace_pos] != b'\n' {
                            brace_pos += 1;
                        }

                        if brace_pos >= n || source[brace_pos] != b'{' {
                            return Err(SegmentError::UnterminatedSystem {
                                name: system_name,
                                open_brace_pos: brace_pos,
                            });
                        }

                        // Use BodyCloser to find matching close brace
                        let mut closer = skipper.body_closer();
                        let close_pos = match closer.close_byte(source, brace_pos) {
                            Ok(pos) => pos,
                            Err(_) => {
                                return Err(SegmentError::UnterminatedSystem {
                                    name: system_name,
                                    open_brace_pos: brace_pos,
                                });
                            }
                        };

                        // outer_span: from @@system to after closing }
                        let mut outer_end = close_pos + 1;
                        if outer_end < n && source[outer_end] == b'\n' {
                            outer_end += 1;
                        }

                        // body_span: between { and } (exclusive)
                        let body_start = brace_pos + 1;
                        let body_end = close_pos;

                        segments.push(Segment::System {
                            outer_span: Span { start: pragma_start, end: outer_end },
                            body_span: Span { start: body_start, end: body_end },
                            name: system_name,
                        });

                        i = outer_end;
                        seg_start = i;
                        at_sol = true;
                        continue;
                    }
                    _ => {
                        // Simple pragma (@@run-expect, @@skip-if, @@timeout, etc.)
                        let line_end = find_line_end(source, pragma_start);
                        segments.push(Segment::Pragma {
                            kind,
                            span: Span { start: pragma_start, end: line_end },
                            value: pragma_value,
                        });
                        i = line_end;
                        seg_start = i;
                        at_sol = true;
                        continue;
                    }
                }
            }

            // Not a pragma — continue scanning as native code
            at_sol = false;
            i = line_start; // Reset to line start for normal scanning
        }

        // Try to skip protected regions (strings/comments)
        if let Some(new_i) = skipper.skip_comment(source, i, n) {
            i = new_i;
            continue;
        }
        if let Some(new_i) = skipper.skip_string(source, i, n) {
            i = new_i;
            continue;
        }

        // Track newlines for SOL detection
        if source[i] == b'\n' {
            at_sol = true;
            i += 1;
            continue;
        }

        i += 1;
    }

    // Emit any remaining native text
    if seg_start < n {
        segments.push(Segment::Native {
            span: Span { start: seg_start, end: n },
        });
    }

    Ok(SourceMap {
        segments,
        source: source.to_vec(),
        target,
    })
}

// ============================================================================
// Helper functions
// ============================================================================

/// Check if position is at `@@system` (not just any `@@` pragma)
fn is_system_pragma(bytes: &[u8], pos: usize) -> bool {
    let remaining = &bytes[pos..];
    remaining.starts_with(b"@@system") &&
        (remaining.len() <= 8 || !remaining[8].is_ascii_alphanumeric())
}

/// Identify the kind and value of a pragma at the given position.
///
/// The position should be at the first `@` of `@@keyword`.
fn identify_pragma(bytes: &[u8], start: usize) -> (PragmaKind, Option<String>) {
    let n = bytes.len();
    let mut i = start + 2; // Skip @@

    // Extract keyword
    let kw_start = i;
    while i < n && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'-') {
        i += 1;
    }
    let keyword = &bytes[kw_start..i];

    // Skip whitespace after keyword
    while i < n && (bytes[i] == b' ' || bytes[i] == b'\t') {
        i += 1;
    }

    // Extract value (rest of line, trimmed)
    let value_start = i;
    let line_end = {
        let mut j = i;
        while j < n && bytes[j] != b'\n' {
            j += 1;
        }
        j
    };
    // Trim trailing whitespace from value
    let mut value_end = line_end;
    while value_end > value_start && (bytes[value_end - 1] == b' ' || bytes[value_end - 1] == b'\t' || bytes[value_end - 1] == b'\r') {
        value_end -= 1;
    }
    let value = if value_start < value_end {
        Some(String::from_utf8_lossy(&bytes[value_start..value_end]).to_string())
    } else {
        None
    };

    let kind = match keyword {
        b"target" => PragmaKind::Target,
        b"codegen" => PragmaKind::Codegen,
        b"system" => return (PragmaKind::Other, value), // @@system handled separately
        b"persist" => PragmaKind::Persist,
        b"async" => PragmaKind::Async,
        b"run-expect" => PragmaKind::RunExpect,
        b"skip-if" => PragmaKind::SkipIf,
        b"timeout" => PragmaKind::Timeout,
        _ => PragmaKind::Other,
    };

    (kind, value)
}

/// Extract system name from `@@system <Name> ...`
fn extract_system_name(bytes: &[u8], start: usize) -> (String, usize) {
    let n = bytes.len();
    let mut i = start + 8; // Skip "@@system"

    // Skip whitespace
    while i < n && (bytes[i] == b' ' || bytes[i] == b'\t') {
        i += 1;
    }

    // Extract identifier
    let name_start = i;
    while i < n && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
        i += 1;
    }

    let name = String::from_utf8_lossy(&bytes[name_start..i]).to_string();
    (name, i)
}

/// Find end of line (including the newline character)
fn find_line_end(bytes: &[u8], start: usize) -> usize {
    let n = bytes.len();
    let mut i = start;
    while i < n && bytes[i] != b'\n' {
        i += 1;
    }
    if i < n && bytes[i] == b'\n' {
        i += 1; // Include the newline
    }
    i
}

/// Find end of a pragma block (@@codegen { ... } or @@persist { ... })
fn find_pragma_block_end<S: SyntaxSkipper>(
    skipper: &S,
    bytes: &[u8],
    start: usize,
) -> Result<usize, SegmentError> {
    let n = bytes.len();
    let mut i = start;

    // Find opening brace
    while i < n && bytes[i] != b'{' {
        i += 1;
    }

    if i >= n {
        // No opening brace — treat as simple line pragma
        return Ok(find_line_end(bytes, start));
    }

    // Use BodyCloser to find matching brace
    let mut closer = skipper.body_closer();
    match closer.close_byte(bytes, i) {
        Ok(close_pos) => {
            let mut end = close_pos + 1;
            // Include trailing newline if present
            if end < n && bytes[end] == b'\n' {
                end += 1;
            }
            Ok(end)
        }
        Err(_) => {
            Err(SegmentError::UnterminatedComment { pos: start })
        }
    }
}

// ============================================================================
// Convenience: get SyntaxSkipper for a TargetLanguage
// ============================================================================

/// Get the appropriate SyntaxSkipper for the given target language
/// and run the segmenter.
///
/// This is the top-level entry point that most callers should use.
pub fn segment_source(source: &[u8], lang: TargetLanguage) -> Result<SourceMap, SegmentError> {
    match lang {
        TargetLanguage::Python3 => {
            use crate::frame_c::v4::native_region_scanner::python::PythonSkipper;
            segment(&PythonSkipper, source)
        }
        TargetLanguage::TypeScript => {
            use crate::frame_c::v4::native_region_scanner::typescript::TypeScriptSkipper;
            segment(&TypeScriptSkipper, source)
        }
        TargetLanguage::Rust => {
            use crate::frame_c::v4::native_region_scanner::rust::RustSkipper;
            segment(&RustSkipper, source)
        }
        TargetLanguage::C => {
            use crate::frame_c::v4::native_region_scanner::c::CSkipper;
            segment(&CSkipper, source)
        }
        TargetLanguage::Cpp => {
            use crate::frame_c::v4::native_region_scanner::cpp::CppSkipper;
            segment(&CppSkipper, source)
        }
        TargetLanguage::Java => {
            use crate::frame_c::v4::native_region_scanner::java::JavaSkipper;
            segment(&JavaSkipper, source)
        }
        TargetLanguage::CSharp => {
            use crate::frame_c::v4::native_region_scanner::csharp::CSharpSkipper;
            segment(&CSharpSkipper, source)
        }
        _ => {
            // Fallback: use Python skipper (safe for basic brace/string detection)
            use crate::frame_c::v4::native_region_scanner::python::PythonSkipper;
            segment(&PythonSkipper, source)
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame_c::v4::native_region_scanner::python::PythonSkipper;

    fn segment_py(source: &str) -> SourceMap {
        segment(&PythonSkipper, source.as_bytes()).unwrap()
    }

    #[test]
    fn test_empty_source() {
        let map = segment_py("");
        assert!(map.segments.is_empty());
        assert!(map.target.is_none());
    }

    #[test]
    fn test_native_only() {
        let source = "import os\ndef foo(): pass\n";
        let map = segment_py(source);
        assert_eq!(map.segments.len(), 1);
        assert!(matches!(&map.segments[0], Segment::Native { .. }));
        assert_eq!(map.text(&Span { start: 0, end: source.len() }), source);
    }

    #[test]
    fn test_target_pragma() {
        let source = "@@target python_3\n";
        let map = segment_py(source);
        assert_eq!(map.segments.len(), 1);
        match &map.segments[0] {
            Segment::Pragma { kind: PragmaKind::Target, value, .. } => {
                assert_eq!(value.as_deref(), Some("python_3"));
            }
            other => panic!("Expected Target pragma, got: {:?}", other),
        }
        assert_eq!(map.target, Some(TargetLanguage::Python3));
    }

    #[test]
    fn test_simple_system() {
        let source = "@@target python_3\n\n@@system Foo {\n    machine:\n        $A {\n        }\n}\n";
        let map = segment_py(source);

        // Should have: Target pragma, Native (blank line), System
        let systems: Vec<_> = map.systems();
        assert_eq!(systems.len(), 1);
        match systems[0] {
            Segment::System { name, .. } => {
                assert_eq!(name, "Foo");
            }
            _ => panic!("Expected System segment"),
        }
    }

    #[test]
    fn test_native_before_and_after_system() {
        let source = "@@target python_3\nimport os\n\n@@system S {\n    machine:\n        $A { }\n}\n\ndef main():\n    pass\n";
        let map = segment_py(source);

        // Count segment types
        let native_count = map.natives().len();
        let system_count = map.systems().len();
        assert_eq!(system_count, 1);
        assert!(native_count >= 2, "Expected at least 2 native segments (before and after system), got {}", native_count);
    }

    #[test]
    fn test_multi_system() {
        let source = "@@target python_3\n\n@@system A {\n    machine:\n        $S1 { }\n}\n\n@@system B {\n    machine:\n        $S2 { }\n}\n";
        let map = segment_py(source);

        let systems: Vec<_> = map.systems();
        assert_eq!(systems.len(), 2);

        let names: Vec<&str> = systems.iter().map(|s| match s {
            Segment::System { name, .. } => name.as_str(),
            _ => "",
        }).collect();
        assert_eq!(names, vec!["A", "B"]);
    }

    #[test]
    fn test_system_body_span() {
        let source = "@@system Foo {\n    machine:\n        $A { }\n}\n";
        let map = segment_py(source);

        let systems = map.systems();
        assert_eq!(systems.len(), 1);
        match systems[0] {
            Segment::System { body_span, name, .. } => {
                assert_eq!(name, "Foo");
                let body = map.text(body_span);
                assert!(body.contains("machine:"), "Body should contain 'machine:', got: '{}'", body);
                assert!(!body.contains("@@system"), "Body should not contain '@@system'");
            }
            _ => panic!("Expected System segment"),
        }
    }

    #[test]
    fn test_pragma_in_string_ignored() {
        let source = "x = \"@@system Fake { }\"\n";
        let map = segment_py(source);
        assert_eq!(map.segments.len(), 1);
        assert!(matches!(&map.segments[0], Segment::Native { .. }));
    }

    #[test]
    fn test_pragma_in_comment_ignored() {
        let source = "# @@system Fake { }\n";
        let map = segment_py(source);
        assert_eq!(map.segments.len(), 1);
        assert!(matches!(&map.segments[0], Segment::Native { .. }));
    }

    #[test]
    fn test_run_expect_pragma() {
        let source = "@@target python_3\n@@run-expect SUCCESS\n@@system S {\n    machine:\n        $A { }\n}\n";
        let map = segment_py(source);

        assert_eq!(map.run_expect(), Some("SUCCESS"));
    }

    #[test]
    fn test_invalid_target_returns_error() {
        let source = "@@target invalid_lang\n";
        let result = segment(&PythonSkipper, source.as_bytes());
        assert!(result.is_err());
        match result.unwrap_err() {
            SegmentError::InvalidTarget { value, .. } => {
                assert_eq!(value, "invalid_lang");
            }
            other => panic!("Expected InvalidTarget error, got: {:?}", other),
        }
    }

    #[test]
    fn test_segment_source_convenience() {
        let source = b"@@target python_3\n@@system S {\n    machine:\n        $A { }\n}\n";
        let map = segment_source(source, TargetLanguage::Python3).unwrap();
        assert_eq!(map.systems().len(), 1);
        assert_eq!(map.target, Some(TargetLanguage::Python3));
    }
}
