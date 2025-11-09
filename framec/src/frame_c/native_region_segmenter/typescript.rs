use super::{BodySegment, FrameStmtKind};
use crate::frame_c::region_scanner::{RegionScanner, ScanResult, TsTemplateScanner};

/// Segment a TypeScript native region into Native and Directive segments.
/// Top-level detection uses a small state machine for strings/comments/template literals.
pub fn segment_ts_body(source: &str, start_line: usize, end_line: usize) -> Vec<BodySegment> {
    if start_line == 0 || end_line == 0 || end_line < start_line {
        return vec![];
    }

    // Extract the region lines (1-based input)
    let all_lines: Vec<&str> = source.lines().collect();
    let s_idx = start_line.saturating_sub(1).min(all_lines.len());
    let e_idx = end_line.saturating_sub(1).min(all_lines.len());
    if s_idx >= all_lines.len() || s_idx > e_idx {
        return vec![];
    }
    let region_lines = &all_lines[s_idx..=e_idx];

    // Precompute char-offsets for each line start to allow RegionScanner (char-based)
    // to map offsets back to (line, column).
    fn build_byte_line_offsets(src: &str) -> Vec<usize> {
        let mut v = Vec::new();
        v.push(0);
        for (i, b) in src.as_bytes().iter().enumerate() { if *b == b'\n' { v.push(i + 1); } }
        v
    }
    fn byte_offset_of(line_1: usize, col_bytes: usize, byte_line_offsets: &[usize]) -> usize {
        let base = *byte_line_offsets.get(line_1.saturating_sub(1)).unwrap_or(&0);
        base + col_bytes
    }
    fn line_col_from_byte_offset(byte_line_offsets: &[usize], target: usize) -> (usize, usize) {
        let mut lo = 0usize; let mut hi = byte_line_offsets.len();
        while lo + 1 < hi {
            let mid = (lo + hi) / 2;
            if byte_line_offsets[mid] <= target { lo = mid; } else { hi = mid; }
        }
        let line_1 = lo + 1; let start = byte_line_offsets[lo];
        (line_1, target.saturating_sub(start))
    }

    let byte_line_offsets = build_byte_line_offsets(source);

    // State flags
    let mut in_squote = false;
    let mut in_dquote = false;
    // line-comment flag is scoped per line
    let mut in_block_comment = false;
    let mut in_template = false; // inside backtick string (skipped via TsTemplateScanner)
    let mut brace_depth: i32 = 0; // top-level within region when 0

    let mut segments: Vec<BodySegment> = Vec::new();
    let mut native_start: Option<usize> = None; // frame line start of current native run

    // Helper (function) to flush a native run
    fn flush_native(
        segments: &mut Vec<BodySegment>,
        region_lines: &[&str],
        start_line: usize,
        start: Option<usize>,
        end_line_inclusive: usize,
    ) {
        if let Some(start_ln) = start {
            if end_line_inclusive >= start_ln {
                // Join slice of lines
                let start0 = (start_ln - start_line) as usize; // 0-based within region
                let end0 = (end_line_inclusive - start_line) as usize;
                if start0 < region_lines.len() && start0 <= end0 && end0 < region_lines.len() {
                    let mut text = String::new();
                    for ln in start0..=end0 {
                        text.push_str(region_lines[ln]);
                        if !region_lines[ln].ends_with('\n') {
                            text.push('\n');
                        }
                    }
                    segments.push(BodySegment::Native {
                        text,
                        start_line: start_ln,
                        end_line: end_line_inclusive,
                    });
                }
            }
        }
    }

    for i in 0..region_lines.len() {
        let frame_ln = start_line + i; // actual frame line for this region line
        let line = region_lines[i];

        // Fast path: if not inside any state and the first non-whitespace
        // character cannot begin a Frame directive, and the line contains no
        // quotes/backticks or comment markers, treat the whole line as native.
        if !in_template && !in_squote && !in_dquote {
            if let Some(col0) = line.find(|c: char| !c.is_whitespace()) {
                let head = &line[col0..];
                let starts_ok = !(head.starts_with("->")
                    || head.starts_with("=>")
                    || head.starts_with("$$")
                    || head.starts_with("system.return"));
                let has_string_or_comment = line.as_bytes().contains(&b'\'')
                    || line.as_bytes().contains(&b'"')
                    || line.as_bytes().contains(&b'`')
                    || head.starts_with("//");
                if starts_ok && !has_string_or_comment {
                    if native_start.is_none() { native_start = Some(frame_ln); }
                    continue;
                }
            } else {
                if native_start.is_none() { native_start = Some(frame_ln); }
                continue;
            }
        }

        // Reset single-line comment per line
        let mut in_line_comment = false;

        let mut j = 0;
        let bytes = line.as_bytes();
        while j < bytes.len() {
            let ch = bytes[j] as char;

            // Handle line comment
            if !in_squote && !in_dquote && !in_block_comment && !in_template {
                if ch == '/' && j + 1 < bytes.len() && bytes[j + 1] as char == '/' {
                    in_line_comment = true;
                }
                if ch == '/' && j + 1 < bytes.len() && bytes[j + 1] as char == '*' {
                    in_block_comment = true;
                    j += 2;
                    continue;
                }
            }

            if in_line_comment {
                break; // ignore rest of the line
            }

            // End of block comment
            if in_block_comment {
                if ch == '*' && j + 1 < bytes.len() && bytes[j + 1] as char == '/' {
                    in_block_comment = false;
                    j += 2;
                    continue;
                }
                j += 1;
                continue;
            }

            // String states
            if !in_template && !in_squote && !in_dquote {
                if ch == '\'' {
                    in_squote = true;
                    j += 1;
                    continue;
                }
                if ch == '"' {
                    in_dquote = true;
                    j += 1;
                    continue;
                }
                if ch == '`' {
                    // Use RegionScanner to skip template literal including nested ${...}
                    let scanner = TsTemplateScanner::new();
                    let abs_byte_off = byte_offset_of(frame_ln, j, &byte_line_offsets);
                    match scanner.scan(source, abs_byte_off, frame_ln) {
                        ScanResult::Ok(env) => {
                            // Jump to just after the closing backtick
                            let (end_l1, end_col_bytes) = line_col_from_byte_offset(&byte_line_offsets, env.end_offset + 1);
                            // Map to region indices
                            let new_i = end_l1.saturating_sub(start_line);
                            if new_i >= 1 && new_i.saturating_sub(1) < i {
                                // no-op safeguard
                            }
                            // Flush any native run up to current line-1 if needed before jumping lines
                            // (no flush here; we are just skipping content within native)
                            // Update loop counters to new location
                            if new_i == i { // same line
                                j = end_col_bytes;
                            } else {
                                // Different line: finish this line; outer loop will proceed to next line
                                j = bytes.len();
                            }
                            // Continue scanning after skipped template
                            continue;
                        }
                        ScanResult::Failure(_) => {
                            // Fall back: treat remaining as native and break this line
                            in_template = true;
                            j += 1;
                            continue;
                        }
                    }
                }
            } else {
                if in_squote {
                    if ch == '\\' {
                        j += 2;
                        continue;
                    }
                    if ch == '\'' {
                        in_squote = false;
                        j += 1;
                        continue;
                    }
                    j += 1;
                    continue;
                }
                if in_dquote {
                    if ch == '\\' {
                        j += 2;
                        continue;
                    }
                    if ch == '"' {
                        in_dquote = false;
                        j += 1;
                        continue;
                    }
                    j += 1;
                    continue;
                }
                if in_template {
                    // Should not be reached when RegionScanner is used; but keep as safety net
                    if ch == '`' { in_template = false; j += 1; continue; }
                    j += 1;
                    continue;
                }
            }

            // Update brace depth outside strings/comments/template text
            if ch == '{' {
                brace_depth += 1;
                j += 1;
                continue;
            }
            if ch == '}' {
                brace_depth -= 1;
                j += 1;
                continue;
            }

            // Detect directives at top level and only at first non-whitespace column
            if brace_depth == 0 {
                if let Some(col0) = line.find(|c: char| !c.is_whitespace()) {
                    if j == col0 {
                        // Transition: require '->' followed by optional WS and then '$'
                        if ch == '-' && j + 1 < bytes.len() && bytes[j + 1] as char == '>' {
                            let mut k = j + 2;
                            while k < bytes.len() && (bytes[k] as char).is_whitespace() {
                                k += 1;
                            }
                            if k < bytes.len() && bytes[k] as char == '$' {
                                flush_native(
                                    &mut segments,
                                    region_lines,
                                    start_line,
                                    native_start,
                                    frame_ln.saturating_sub(1),
                                );
                                native_start = None;
                                segments.push(BodySegment::FrameStmt {
                                    kind: FrameStmtKind::Transition,
                                    frame_line: frame_ln,
                                    line_text: line.to_string(),
                                });
                                break;
                            }
                        }
                        // Forward: '=>' followed by optional WS and then '$^'
                        if ch == '=' && j + 1 < bytes.len() && bytes[j + 1] as char == '>' {
                            let mut k = j + 2;
                            while k < bytes.len() && (bytes[k] as char).is_whitespace() {
                                k += 1;
                            }
                            if k + 1 < bytes.len()
                                && bytes[k] as char == '$'
                                && bytes[k + 1] as char == '^'
                            {
                                flush_native(
                                    &mut segments,
                                    region_lines,
                                    start_line,
                                    native_start,
                                    frame_ln.saturating_sub(1),
                                );
                                native_start = None;
                                segments.push(BodySegment::FrameStmt {
                                    kind: FrameStmtKind::Forward,
                                    frame_line: frame_ln,
                                    line_text: line.to_string(),
                                });
                                break;
                            }
                        }
                        // Stack push/pop: $$[+]/$$[-]
                        if ch == '$'
                            && j + 4 < bytes.len()
                            && bytes[j + 1] as char == '$'
                            && bytes[j + 2] as char == '['
                        {
                            let sign = bytes[j + 3] as char;
                            if (sign == '+' || sign == '-') && bytes[j + 4] as char == ']' {
                                flush_native(
                                    &mut segments,
                                    region_lines,
                                    start_line,
                                    native_start,
                                    frame_ln.saturating_sub(1),
                                );
                                native_start = None;
                                let kind = if sign == '+' {
                                    FrameStmtKind::StackPush
                                } else {
                                    FrameStmtKind::StackPop
                                };
                                segments.push(BodySegment::FrameStmt {
                                    kind,
                                    frame_line: frame_ln,
                                    line_text: line.to_string(),
                                });
                                break;
                            }
                        }
                    }
                }
            }

            j += 1;
        }

        // If we didn't emit a directive on this line, ensure a native run is open
        if !segments
            .last()
            .map(|s| match s {
                BodySegment::FrameStmt { frame_line, .. } if *frame_line == frame_ln => true,
                _ => false,
            })
            .unwrap_or(false)
        {
            if native_start.is_none() {
                native_start = Some(frame_ln);
            }
        }
    }

    // Flush trailing native run
    if let Some(start) = native_start {
        flush_native(
            &mut segments,
            region_lines,
            start_line,
            Some(start),
            end_line,
        );
    }

    segments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segments_single_native_line() {
        let src = r#"@target typescript
system NativeLine {
    operations:
    op1() {
        const x = 1;
    }
    interface:
    machine:
    actions:
    domain:
}
"#;
        // body lines for op1() { ... } are 5..5 in this formatting
        let segs = segment_ts_body(src, 5, 5);
        assert_eq!(segs.len(), 1);
        match &segs[0] {
            BodySegment::Native {
                text,
                start_line,
                end_line,
            } => {
                assert!(text.contains("const x = 1;"));
                assert_eq!((*start_line, *end_line), (5, 5));
            }
            _ => panic!("expected Native segment"),
        }
    }

    #[test]
    fn detects_transition_directive_at_top_level() {
        let src = r#"@target typescript
system S {
    operations:
    op1() {
        -> $Next
    }
    interface:
    machine:
    actions:
    domain:
}
"#;
        let segs = segment_ts_body(src, 5, 5);
        assert!(matches!(
            segs.last().unwrap(),
            BodySegment::FrameStmt { .. }
        ));
    }
}
