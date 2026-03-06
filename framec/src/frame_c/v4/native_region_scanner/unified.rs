// Unified Native Region Scanner for Frame V4
//
// This module provides a single scanning implementation for all target languages.
// The only language-specific logic is how to skip comments and strings.
//
// Sub-machines (hierarchical state manager pattern):
//   ExprScannerFsm   — PDA for scanning assignment RHS expressions
//   ContextParserFsm — FSM for parsing @@ context constructs
//   StateVarParserFsm — FSM for parsing $.varName access/assignment

#[allow(unreachable_patterns)]
#[allow(unused_mut)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused_variables)]
mod _expr_scanner { include!("expr_scanner.gen.rs"); }

#[allow(unreachable_patterns)]
#[allow(unused_mut)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused_variables)]
mod _context_parser { include!("context_parser.gen.rs"); }

#[allow(unreachable_patterns)]
#[allow(unused_mut)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused_variables)]
mod _state_var_parser { include!("state_var_parser.gen.rs"); }

use _context_parser::ContextParserFsm;
use _state_var_parser::StateVarParserFsm;

use super::*;
use crate::frame_c::v4::body_closer::BodyCloser;

/// Language-specific syntax skipper trait.
/// Each language only needs to implement how to skip its comments and strings.
pub trait SyntaxSkipper {
    /// Get the body closer for this language
    fn body_closer(&self) -> Box<dyn BodyCloser>;

    /// Try to skip a comment starting at position i.
    /// Returns Some(new_position) if a comment was skipped, None otherwise.
    fn skip_comment(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize>;

    /// Try to skip a string literal starting at position i.
    /// Returns Some(new_position) if a string was skipped, None otherwise.
    fn skip_string(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize>;

    /// Find the end of a Frame statement line, respecting language-specific string syntax.
    /// Stops at newline, semicolon, or comment start.
    fn find_line_end(&self, bytes: &[u8], start: usize, end: usize) -> usize;

    /// Try to find matching close paren, respecting language-specific string syntax.
    /// Returns Some(position after ')') if balanced, None otherwise.
    fn balanced_paren_end(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize>;
}

/// Unified scanner that works with any language via the SyntaxSkipper trait
pub fn scan_native_regions<S: SyntaxSkipper>(
    skipper: &S,
    bytes: &[u8],
    open_brace_index: usize,
) -> Result<ScanResult, ScanError> {
    let mut closer = skipper.body_closer();
    let close = closer.close_byte(bytes, open_brace_index)
        .map_err(|e| ScanError {
            kind: ScanErrorKind::UnterminatedProtected,
            message: format!("{:?}", e)
        })?;

    let mut regions: Vec<Region> = Vec::new();
    let mut i = open_brace_index + 1;
    let end = close;
    let mut seg_start = i;
    let mut at_sol = true;
    let mut indent = 0usize;

    while i < end {
        let b = bytes[i];

        // Track indentation at start of line
        if at_sol {
            if b == b' ' || b == b'\t' {
                indent += 1;
                i += 1;
                continue;
            }

            // ===== FRAME STATEMENT DETECTION (unified for all languages) =====

            // Try to match Frame statements at start of line
            if let Some((_new_i, kind)) = match_frame_statement_at_sol(skipper, bytes, i, end, indent) {
                // Emit any preceding native text (excluding indentation)
                let native_end = i.saturating_sub(indent);
                if seg_start < native_end {
                    regions.push(Region::NativeText {
                        span: RegionSpan { start: seg_start, end: native_end }
                    });
                }

                // Find end of Frame statement
                let stmt_end = skipper.find_line_end(bytes, i, end);

                regions.push(Region::FrameSegment {
                    span: RegionSpan { start: i, end: stmt_end },
                    kind,
                    indent,
                });

                i = stmt_end;
                seg_start = i;
                at_sol = true;
                indent = 0;
                continue;
            }

            at_sol = false;
            indent = 0;
        }

        // ===== HANDLE CURRENT CHARACTER =====

        match b {
            b'\n' => {
                at_sol = true;
                indent = 0;
                i += 1;
            }

            // Try language-specific comment skip
            _ if skipper.skip_comment(bytes, i, end).is_some() => {
                i = skipper.skip_comment(bytes, i, end).unwrap();
            }

            // Try language-specific string skip
            _ if skipper.skip_string(bytes, i, end).is_some() => {
                i = skipper.skip_string(bytes, i, end).unwrap();
            }

            // ===== MID-LINE FRAME CONSTRUCTS =====

            // State variable reference: $.varName or assignment: $.varName = expr
            // Delegates to StateVarParserFsm (hierarchical state manager pattern)
            b'$' if i + 1 < end && bytes[i + 1] == b'.' => {
                if seg_start < i {
                    regions.push(Region::NativeText {
                        span: RegionSpan { start: seg_start, end: i }
                    });
                }
                let var_start = i;
                let mut parser = StateVarParserFsm::new();
                parser.bytes = bytes[..end].to_vec();
                parser.pos = i;
                parser.end = end;
                parser.do_parse();

                let kind = if parser.is_assignment {
                    FrameSegmentKind::StateVarAssign
                } else {
                    FrameSegmentKind::StateVar
                };
                regions.push(Region::FrameSegment {
                    span: RegionSpan { start: var_start, end: parser.result_end },
                    kind,
                    indent: 0,
                });
                i = parser.result_end;
                seg_start = i;
                // parser is destroyed here (state manager pattern)
            }

            // System context syntax: @@ variants
            // Delegates to ContextParserFsm (hierarchical state manager pattern)
            b'@' if i + 1 < end && bytes[i + 1] == b'@' => {
                if seg_start < i {
                    regions.push(Region::NativeText {
                        span: RegionSpan { start: seg_start, end: i }
                    });
                }
                let ctx_start = i;

                // For @@SystemName(), pre-compute balanced_paren_end via the
                // language-specific SyntaxSkipper (the FSM can't call traits).
                let after_at = i + 2;
                let mut precomputed_paren_end: usize = 0;
                if after_at < end && bytes[after_at].is_ascii_uppercase() {
                    let mut name_end = after_at;
                    while name_end < end && (bytes[name_end].is_ascii_alphanumeric() || bytes[name_end] == b'_') {
                        name_end += 1;
                    }
                    if name_end < end && bytes[name_end] == b'(' {
                        if let Some(pe) = skipper.balanced_paren_end(bytes, name_end, end) {
                            precomputed_paren_end = pe;
                        }
                    }
                }

                let mut parser = ContextParserFsm::new();
                parser.bytes = bytes[..end].to_vec();
                parser.pos = after_at; // position after @@
                parser.end = end;
                parser.paren_end = precomputed_paren_end;
                parser.do_parse();

                if parser.has_result {
                    let kind = match parser.result_kind {
                        1 => FrameSegmentKind::ContextParamShorthand,
                        2 => FrameSegmentKind::ContextReturn,
                        3 => FrameSegmentKind::ContextEvent,
                        4 => FrameSegmentKind::ContextData,
                        5 => FrameSegmentKind::ContextDataAssign,
                        6 => FrameSegmentKind::ContextParams,
                        7 => FrameSegmentKind::TaggedInstantiation,
                        _ => FrameSegmentKind::ContextReturn, // shouldn't happen
                    };
                    regions.push(Region::FrameSegment {
                        span: RegionSpan { start: ctx_start, end: parser.result_end },
                        kind,
                        indent: 0,
                    });
                } else {
                    // No match — treat as native text
                    regions.push(Region::NativeText {
                        span: RegionSpan { start: ctx_start, end: parser.result_end }
                    });
                }
                i = parser.result_end;
                seg_start = i;
                // parser is destroyed here (state manager pattern)
            }

            _ => {
                i += 1;
            }
        }
    }

    // Emit any remaining native text
    if seg_start < end {
        regions.push(Region::NativeText {
            span: RegionSpan { start: seg_start, end }
        });
    }

    Ok(ScanResult { close_byte: close, regions })
}

/// Match Frame statements at start of line.
/// Returns Some((new_position, kind)) if matched, None otherwise.
///
/// Handles both:
/// - Direct Frame statements: `-> $State`, `push$`, etc.
/// - Backtick-prefixed statements: `` `push$ ``, `` `-> pop$ `` (V4 embedded syntax)
fn match_frame_statement_at_sol<S: SyntaxSkipper>(
    skipper: &S,
    bytes: &[u8],
    i: usize,
    end: usize,
    _indent: usize,
) -> Option<(usize, FrameSegmentKind)> {
    let mut pos = i;

    // Check for backtick prefix (V4 embedded Frame statement syntax)
    // e.g., `push$, `-> pop$, `-> $State
    if pos < end && bytes[pos] == b'`' {
        pos += 1;
        // Skip optional whitespace after backtick
        while pos < end && (bytes[pos] == b' ' || bytes[pos] == b'\t') {
            pos += 1;
        }
    }

    if pos >= end {
        return None;
    }

    let b = bytes[pos];

    // Transition variants: -> $State, -> (args) $State, -> pop$, -> => $State
    if b == b'-' && pos + 1 < end && bytes[pos + 1] == b'>' {
        let mut k = pos + 2;

        // Skip whitespace after ->
        while k < end && (bytes[k] == b' ' || bytes[k] == b'\t') {
            k += 1;
        }

        // Check for -> => $State (transition forward)
        if k + 1 < end && bytes[k] == b'=' && bytes[k + 1] == b'>' {
            k += 2;
            while k < end && (bytes[k] == b' ' || bytes[k] == b'\t') {
                k += 1;
            }
            if k < end && bytes[k] == b'$' {
                return Some((k, FrameSegmentKind::TransitionForward));
            }
        }

        // Check for -> pop$ (pop transition)
        if k + 3 < end && bytes[k] == b'p' && bytes[k + 1] == b'o' && bytes[k + 2] == b'p' && bytes[k + 3] == b'$' {
            return Some((k + 4, FrameSegmentKind::StackPop));
        }

        // Check for optional enter args: -> (args) $State
        if k < end && bytes[k] == b'(' {
            if let Some(k2) = skipper.balanced_paren_end(bytes, k, end) {
                k = k2;
                while k < end && (bytes[k] == b' ' || bytes[k] == b'\t') {
                    k += 1;
                }
            }
        }

        // Regular transition: -> $State
        if k < end && bytes[k] == b'$' {
            return Some((k, FrameSegmentKind::Transition));
        }
    }

    // Forward: => $^
    if b == b'=' && pos + 3 < end && bytes[pos + 1] == b'>' && bytes[pos + 2] == b' ' && bytes[pos + 3] == b'$' {
        return Some((pos + 4, FrameSegmentKind::Forward));
    }

    // Transition with leading exit args: (exit_args) -> (enter_args) $State
    if b == b'(' {
        if let Some(mut k) = skipper.balanced_paren_end(bytes, pos, end) {
            while k < end && (bytes[k] == b' ' || bytes[k] == b'\t') {
                k += 1;
            }
            if k + 1 < end && bytes[k] == b'-' && bytes[k + 1] == b'>' {
                k += 2;
                while k < end && (bytes[k] == b' ' || bytes[k] == b'\t') {
                    k += 1;
                }
                // Optional enter args
                if k < end && bytes[k] == b'(' {
                    if let Some(k2) = skipper.balanced_paren_end(bytes, k, end) {
                        k = k2;
                        while k < end && (bytes[k] == b' ' || bytes[k] == b'\t') {
                            k += 1;
                        }
                    }
                }
                if k < end && bytes[k] == b'$' {
                    return Some((k, FrameSegmentKind::Transition));
                }
            }
        }
    }

    // Stack push: push$
    if b == b'p' && pos + 4 < end
        && bytes[pos + 1] == b'u'
        && bytes[pos + 2] == b's'
        && bytes[pos + 3] == b'h'
        && bytes[pos + 4] == b'$'
    {
        return Some((pos + 5, FrameSegmentKind::StackPush));
    }

    // Stack pop (standalone): pop$
    if b == b'p' && pos + 3 < end
        && bytes[pos + 1] == b'o'
        && bytes[pos + 2] == b'p'
        && bytes[pos + 3] == b'$'
    {
        return Some((pos + 4, FrameSegmentKind::StackPop));
    }

    // Return sugar: return <expr> at start of line
    if b == b'r' && pos + 6 <= end && &bytes[pos..pos + 6] == b"return" {
        let after_return = pos + 6;
        if after_return < end && (bytes[after_return] == b' ' || bytes[after_return] == b'\t') {
            return Some((after_return, FrameSegmentKind::ReturnSugar));
        }
    }

    None
}

// ============================================================================
// Common helper implementations for C-like languages
// ============================================================================

/// Skip C-style line comment: // ...
pub fn skip_line_comment(bytes: &[u8], i: usize, end: usize) -> Option<usize> {
    if i + 1 < end && bytes[i] == b'/' && bytes[i + 1] == b'/' {
        let mut j = i + 2;
        while j < end && bytes[j] != b'\n' {
            j += 1;
        }
        Some(j)
    } else {
        None
    }
}

/// Skip C-style block comment: /* ... */
pub fn skip_block_comment(bytes: &[u8], i: usize, end: usize) -> Option<usize> {
    if i + 1 < end && bytes[i] == b'/' && bytes[i + 1] == b'*' {
        let mut j = i + 2;
        while j + 1 < end {
            if bytes[j] == b'*' && bytes[j + 1] == b'/' {
                return Some(j + 2);
            }
            j += 1;
        }
        Some(end) // Unterminated, consume rest
    } else {
        None
    }
}

/// Skip Python-style comment: # ...
pub fn skip_hash_comment(bytes: &[u8], i: usize, end: usize) -> Option<usize> {
    if bytes[i] == b'#' {
        let mut j = i + 1;
        while j < end && bytes[j] != b'\n' {
            j += 1;
        }
        Some(j)
    } else {
        None
    }
}

/// Skip simple string: 'x' or "x" with backslash escapes
pub fn skip_simple_string(bytes: &[u8], i: usize, end: usize) -> Option<usize> {
    let b = bytes[i];
    if b == b'\'' || b == b'"' {
        let q = b;
        let mut j = i + 1;
        while j < end {
            if bytes[j] == b'\\' {
                j += 2;
                continue;
            }
            if bytes[j] == q {
                return Some(j + 1);
            }
            j += 1;
        }
        Some(end) // Unterminated
    } else {
        None
    }
}

/// Skip Python triple-quoted string: '''x''' or """x"""
pub fn skip_triple_string(bytes: &[u8], i: usize, end: usize) -> Option<usize> {
    let b = bytes[i];
    if (b == b'\'' || b == b'"') && i + 2 < end && bytes[i + 1] == b && bytes[i + 2] == b {
        let q = b;
        let mut j = i + 3;
        while j + 2 < end {
            if bytes[j] == q && bytes[j + 1] == q && bytes[j + 2] == q {
                return Some(j + 3);
            }
            j += 1;
        }
        Some(end) // Unterminated
    } else {
        None
    }
}

/// Skip TypeScript/JS template literal: `...${...}...`
pub fn skip_template_literal(bytes: &[u8], i: usize, end: usize) -> Option<usize> {
    if bytes[i] == b'`' {
        let mut j = i + 1;
        let mut brace_depth = 0i32;
        while j < end {
            if bytes[j] == b'`' && brace_depth == 0 {
                return Some(j + 1);
            }
            if bytes[j] == b'\\' {
                j += 2;
                continue;
            }
            if bytes[j] == b'$' && j + 1 < end && bytes[j + 1] == b'{' {
                brace_depth += 1;
                j += 2;
                continue;
            }
            if bytes[j] == b'}' && brace_depth > 0 {
                brace_depth -= 1;
            }
            j += 1;
        }
        Some(end) // Unterminated
    } else {
        None
    }
}

/// Skip Rust raw string: r#"..."#
pub fn skip_rust_raw_string(bytes: &[u8], i: usize, end: usize) -> Option<usize> {
    if bytes[i] != b'r' {
        return None;
    }
    let mut j = i + 1;
    let mut hashes = 0usize;

    // Count leading #s
    while j < end && bytes[j] == b'#' {
        hashes += 1;
        j += 1;
    }

    // Must have opening "
    if j >= end || bytes[j] != b'"' {
        return None;
    }
    j += 1;

    // Find closing "###
    while j < end {
        if bytes[j] == b'"' {
            let mut k = j + 1;
            let mut matched = 0usize;
            while matched < hashes && k < end && bytes[k] == b'#' {
                matched += 1;
                k += 1;
            }
            if matched == hashes {
                return Some(k);
            }
        }
        j += 1;
    }
    Some(end) // Unterminated
}

/// Find end of Frame statement line for C-like languages
pub fn find_line_end_c_like(bytes: &[u8], mut j: usize, end: usize) -> usize {
    let mut in_string: Option<u8> = None;

    while j < end {
        let b = bytes[j];

        if b == b'\n' {
            break;
        }

        if let Some(q) = in_string {
            if b == b'\\' {
                j += 2;
                continue;
            }
            if b == q {
                in_string = None;
            }
            j += 1;
            continue;
        }

        if b == b';' {
            break;
        }
        if b == b'/' && j + 1 < end && (bytes[j + 1] == b'/' || bytes[j + 1] == b'*') {
            break;
        }
        if b == b'\'' || b == b'"' {
            in_string = Some(b);
        }
        j += 1;
    }
    j
}

/// Find end of Frame statement line for Python
pub fn find_line_end_python(bytes: &[u8], mut j: usize, end: usize) -> usize {
    let mut in_string: Option<u8> = None;

    while j < end {
        let b = bytes[j];

        if b == b'\n' {
            break;
        }

        if let Some(q) = in_string {
            if b == b'\\' {
                j += 2;
                continue;
            }
            if b == q {
                in_string = None;
            }
            j += 1;
            continue;
        }

        if b == b'#' || b == b';' {
            break;
        }
        if b == b'\'' || b == b'"' {
            in_string = Some(b);
        }
        j += 1;
    }
    j
}

/// Find balanced paren end for C-like languages
pub fn balanced_paren_end_c_like(bytes: &[u8], mut i: usize, end: usize) -> Option<usize> {
    if i >= end || bytes[i] != b'(' {
        return None;
    }

    let mut depth = 0i32;
    let mut in_string: Option<u8> = None;

    while i < end {
        let b = bytes[i];

        if let Some(q) = in_string {
            if b == b'\\' {
                i += 2;
                continue;
            }
            if b == q {
                in_string = None;
            }
            i += 1;
            continue;
        }

        match b {
            b'\'' | b'"' => {
                in_string = Some(b);
                i += 1;
            }
            b'(' => {
                depth += 1;
                i += 1;
            }
            b')' => {
                depth -= 1;
                i += 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {
                i += 1;
            }
        }
    }
    None
}
