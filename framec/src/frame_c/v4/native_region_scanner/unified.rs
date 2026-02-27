// Unified Native Region Scanner for Frame V4
//
// This module provides a single scanning implementation for all target languages.
// The only language-specific logic is how to skip comments and strings.

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
            if let Some((new_i, kind)) = match_frame_statement_at_sol(skipper, bytes, i, end, indent) {
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
            b'$' if i + 1 < end && bytes[i + 1] == b'.' => {
                if seg_start < i {
                    regions.push(Region::NativeText {
                        span: RegionSpan { start: seg_start, end: i }
                    });
                }
                let var_start = i;
                i += 2; // Skip "$."
                while i < end && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    i += 1;
                }
                // Check for assignment: skip whitespace, look for = (but not ==)
                let mut j = i;
                while j < end && (bytes[j] == b' ' || bytes[j] == b'\t') {
                    j += 1;
                }
                if j < end && bytes[j] == b'=' && (j + 1 >= end || bytes[j + 1] != b'=') {
                    // This is an assignment: $.varName = expr
                    // Capture up to semicolon or newline
                    j += 1; // Skip '='
                    while j < end && bytes[j] != b';' && bytes[j] != b'\n' {
                        // Handle nested parentheses/braces
                        if bytes[j] == b'(' {
                            let mut depth = 1;
                            j += 1;
                            while j < end && depth > 0 {
                                if bytes[j] == b'(' { depth += 1; }
                                if bytes[j] == b')' { depth -= 1; }
                                j += 1;
                            }
                        } else {
                            j += 1;
                        }
                    }
                    // Include the semicolon if present
                    if j < end && bytes[j] == b';' {
                        j += 1;
                    }
                    regions.push(Region::FrameSegment {
                        span: RegionSpan { start: var_start, end: j },
                        kind: FrameSegmentKind::StateVarAssign,
                        indent: 0,
                    });
                    i = j;
                } else {
                    // Just a read access
                    regions.push(Region::FrameSegment {
                        span: RegionSpan { start: var_start, end: i },
                        kind: FrameSegmentKind::StateVar,
                        indent: 0,
                    });
                }
                seg_start = i;
            }

            // System context syntax: @@ variants
            // @@.param - shorthand parameter access
            // @@:return - return value slot
            // @@:event - interface event name
            // @@:data[key] - call-scoped data
            // @@:params[key] - explicit parameter access
            b'@' if i + 1 < end && bytes[i + 1] == b'@' => {
                if seg_start < i {
                    regions.push(Region::NativeText {
                        span: RegionSpan { start: seg_start, end: i }
                    });
                }
                let ctx_start = i;
                i += 2; // Skip "@@"

                if i < end && bytes[i] == b'.' {
                    // @@.param - shorthand parameter access
                    i += 1; // Skip "."
                    while i < end && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                        i += 1;
                    }
                    regions.push(Region::FrameSegment {
                        span: RegionSpan { start: ctx_start, end: i },
                        kind: FrameSegmentKind::ContextParamShorthand,
                        indent: 0,
                    });
                } else if i < end && bytes[i] == b':' {
                    i += 1; // Skip ":"
                    // Check which context field
                    if i + 5 < end && &bytes[i..i + 6] == b"return" {
                        // @@:return - may be assignment or read
                        i += 6;
                        // Check for assignment
                        while i < end && (bytes[i] == b' ' || bytes[i] == b'\t') {
                            i += 1;
                        }
                        if i < end && bytes[i] == b'=' && (i + 1 >= end || bytes[i + 1] != b'=') {
                            // @@:return = <expr>
                            i += 1; // Skip '='
                            // Scan to ';' or newline at depth 0, handling nested parens/brackets
                            let mut depth: i32 = 0;
                            let mut in_string: Option<u8> = None;
                            while i < end {
                                let b = bytes[i];
                                // Handle strings
                                if let Some(q) = in_string {
                                    if b == b'\\' && i + 1 < end {
                                        i += 2;
                                        continue;
                                    }
                                    if b == q {
                                        in_string = None;
                                    }
                                    i += 1;
                                    continue;
                                }
                                if b == b'"' || b == b'\'' {
                                    in_string = Some(b);
                                    i += 1;
                                    continue;
                                }
                                match b {
                                    b'(' | b'[' | b'{' => depth += 1,
                                    b')' | b']' | b'}' => depth = (depth - 1).max(0),
                                    b';' if depth == 0 => {
                                        i += 1; // Include the semicolon
                                        break;
                                    }
                                    b'\n' if depth == 0 => break,
                                    _ => {}
                                }
                                i += 1;
                            }
                        }
                        regions.push(Region::FrameSegment {
                            span: RegionSpan { start: ctx_start, end: i },
                            kind: FrameSegmentKind::ContextReturn,
                            indent: 0,
                        });
                    } else if i + 4 < end && &bytes[i..i + 5] == b"event" {
                        // @@:event
                        i += 5;
                        regions.push(Region::FrameSegment {
                            span: RegionSpan { start: ctx_start, end: i },
                            kind: FrameSegmentKind::ContextEvent,
                            indent: 0,
                        });
                    } else if i + 3 < end && &bytes[i..i + 4] == b"data" {
                        // @@:data[key] or @@:data[key] = expr
                        i += 4;
                        // Must have [key]
                        if i < end && bytes[i] == b'[' {
                            while i < end && bytes[i] != b']' {
                                i += 1;
                            }
                            if i < end {
                                i += 1; // Skip ']'
                            }
                        }
                        // Check if this is an assignment: @@:data[key] = expr
                        let mut j = i;
                        // Skip whitespace
                        while j < end && (bytes[j] == b' ' || bytes[j] == b'\t') {
                            j += 1;
                        }
                        // Check for = but not ==
                        if j < end && bytes[j] == b'=' && (j + 1 >= end || bytes[j + 1] != b'=') {
                            // This is an assignment - scan to end of statement (;)
                            j += 1; // Skip '='
                            // Scan to ';' or newline at depth 0, handling nested parens/brackets and strings
                            let mut depth: i32 = 0;
                            let mut in_string: Option<u8> = None;
                            while j < end {
                                let b = bytes[j];
                                // Handle strings
                                if let Some(q) = in_string {
                                    if b == b'\\' && j + 1 < end {
                                        j += 2;
                                        continue;
                                    }
                                    if b == q {
                                        in_string = None;
                                    }
                                    j += 1;
                                    continue;
                                }
                                if b == b'"' || b == b'\'' {
                                    in_string = Some(b);
                                    j += 1;
                                    continue;
                                }
                                match b {
                                    b'(' | b'[' | b'{' => depth += 1,
                                    b')' | b']' | b'}' => depth = (depth - 1).max(0),
                                    b';' if depth == 0 => {
                                        j += 1; // Include the semicolon
                                        break;
                                    }
                                    b'\n' if depth == 0 => break,
                                    _ => {}
                                }
                                j += 1;
                            }
                            regions.push(Region::FrameSegment {
                                span: RegionSpan { start: ctx_start, end: j },
                                kind: FrameSegmentKind::ContextDataAssign,
                                indent: 0,
                            });
                            i = j;
                        } else {
                            // Read-only access
                            regions.push(Region::FrameSegment {
                                span: RegionSpan { start: ctx_start, end: i },
                                kind: FrameSegmentKind::ContextData,
                                indent: 0,
                            });
                        }
                    } else if i + 5 < end && &bytes[i..i + 6] == b"params" {
                        // @@:params[key]
                        i += 6;
                        // Must have [key]
                        if i < end && bytes[i] == b'[' {
                            while i < end && bytes[i] != b']' {
                                i += 1;
                            }
                            if i < end {
                                i += 1; // Skip ']'
                            }
                        }
                        regions.push(Region::FrameSegment {
                            span: RegionSpan { start: ctx_start, end: i },
                            kind: FrameSegmentKind::ContextParams,
                            indent: 0,
                        });
                    } else {
                        // Unknown @@: variant, treat as native
                        regions.push(Region::NativeText {
                            span: RegionSpan { start: ctx_start, end: i }
                        });
                    }
                } else {
                    // Just @@ without . or :, treat as native
                    regions.push(Region::NativeText {
                        span: RegionSpan { start: ctx_start, end: i }
                    });
                }
                seg_start = i;
            }

            // System return: system.return = <expr> or system.return
            b's' if i + 12 < end && &bytes[i..i + 13] == b"system.return" => {
                if seg_start < i {
                    regions.push(Region::NativeText {
                        span: RegionSpan { start: seg_start, end: i }
                    });
                }
                let start = i;
                i += 13; // Skip "system.return"

                // Skip whitespace
                while i < end && (bytes[i] == b' ' || bytes[i] == b'\t') {
                    i += 1;
                }

                if i < end && bytes[i] == b'=' && (i + 1 >= end || bytes[i + 1] != b'=') {
                    // system.return = <expr>
                    i += 1; // Skip '='
                    i = skipper.find_line_end(bytes, i, end);
                    regions.push(Region::FrameSegment {
                        span: RegionSpan { start, end: i },
                        kind: FrameSegmentKind::SystemReturn,
                        indent: 0,
                    });
                } else {
                    // bare system.return
                    regions.push(Region::FrameSegment {
                        span: RegionSpan { start, end: i },
                        kind: FrameSegmentKind::SystemReturnExpr,
                        indent: 0,
                    });
                }
                seg_start = i;
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
            return Some((after_return, FrameSegmentKind::SystemReturn));
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
