use super::{BodySegment, DirectiveKind};

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

    // State flags
    let mut in_squote = false;
    let mut in_dquote = false;
    // line-comment flag is scoped per line
    let mut in_block_comment = false;
    let mut in_template = false; // inside backtick string
    let mut tpl_expr_depth = 0; // ${ ... } nesting
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
                if ch == '\'' { in_squote = true; j += 1; continue; }
                if ch == '"' { in_dquote = true; j += 1; continue; }
                if ch == '`' { in_template = true; j += 1; continue; }
            } else {
                if in_squote {
                    if ch == '\\' { j += 2; continue; }
                    if ch == '\'' { in_squote = false; j += 1; continue; }
                    j += 1; continue;
                }
                if in_dquote {
                    if ch == '\\' { j += 2; continue; }
                    if ch == '"' { in_dquote = false; j += 1; continue; }
                    j += 1; continue;
                }
                if in_template {
                    if ch == '$' && j + 1 < bytes.len() && bytes[j + 1] as char == '{' {
                        tpl_expr_depth += 1; j += 2; continue;
                    }
                    if ch == '}' && tpl_expr_depth > 0 { tpl_expr_depth -= 1; j += 1; continue; }
                    if ch == '`' && tpl_expr_depth == 0 { in_template = false; j += 1; continue; }
                    j += 1; continue;
                }
            }

            // Update brace depth outside strings/comments/template text
            if ch == '{' { brace_depth += 1; j += 1; continue; }
            if ch == '}' { brace_depth -= 1; j += 1; continue; }

            // Detect directives at top level
            if brace_depth == 0 {
                // Transition: "->"
                if ch == '-' && j + 1 < bytes.len() && bytes[j + 1] as char == '>' {
                    // Flush native up to previous line
                    flush_native(&mut segments, region_lines, start_line, native_start, frame_ln.saturating_sub(1));
                    native_start = None;
                    segments.push(BodySegment::Directive { kind: DirectiveKind::Transition, frame_line: frame_ln, line_text: line.to_string() });
                    // Consume rest of line
                    break;
                }
                // Forward: "=> $^" (avoid TS arrow false positives)
                if ch == '=' && j + 1 < bytes.len() && bytes[j + 1] as char == '>' {
                    // lookahead for $^
                    let mut k = j + 2;
                    while k < bytes.len() && (bytes[k] as char).is_whitespace() { k += 1; }
                    if k + 1 < bytes.len() && bytes[k] as char == '$' && bytes[k + 1] as char == '^' {
                        flush_native(&mut segments, region_lines, start_line, native_start, frame_ln.saturating_sub(1));
                        native_start = None;
                        segments.push(BodySegment::Directive { kind: DirectiveKind::Forward, frame_line: frame_ln, line_text: line.to_string() });
                        break;
                    }
                }
                // Stack push/pop
                if ch == '$' && j + 4 < bytes.len() && bytes[j + 1] as char == '$' && bytes[j + 2] as char == '[' {
                    let sign = bytes[j + 3] as char;
                    if (sign == '+' || sign == '-') && bytes[j + 4] as char == ']' {
                        flush_native(&mut segments, region_lines, start_line, native_start, frame_ln.saturating_sub(1));
                        native_start = None;
                        let kind = if sign == '+' { DirectiveKind::StackPush } else { DirectiveKind::StackPop };
                        segments.push(BodySegment::Directive { kind, frame_line: frame_ln, line_text: line.to_string() });
                        break;
                    }
                }
            }

            j += 1;
        }

        // If we didn't emit a directive on this line, ensure a native run is open
        if !segments
            .last()
            .map(|s| match s {
                BodySegment::Directive { frame_line, .. } if *frame_line == frame_ln => true,
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
        flush_native(&mut segments, region_lines, start_line, Some(start), end_line);
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
            BodySegment::Native { text, start_line, end_line } => {
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
        assert!(matches!(segs.last().unwrap(), BodySegment::Directive { .. }));
    }
}
