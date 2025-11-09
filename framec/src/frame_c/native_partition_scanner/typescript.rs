use crate::frame_c::native_region_segmenter::{BodySegment, FrameStmtKind};

/// Stream a TypeScript body starting at the line containing the opening '{'.
/// Returns (segments, close_line) where close_line is the 1-based line number
/// containing the matching closing '}' at top-level depth.
pub fn scan_body_to_segments(source: &str, body_start_line: usize) -> (Vec<BodySegment>, usize) {
    let all_lines: Vec<&str> = source.lines().collect();
    let mut segments: Vec<BodySegment> = Vec::new();

    // DPDA state across lines
    let mut in_squote = false;
    let mut in_dquote = false;
    let mut in_block_comment = false;
    let mut in_template = false;
    let mut tpl_expr_depth: i32 = 0;
    let mut brace_depth: i32 = 1; // opening '{' already consumed above body_start_line
    let mut close_line = body_start_line;

    // Native slice management
    let mut native_start: Option<usize> = None; // frame line start of current native run

    fn flush_native_into(
        segments: &mut Vec<BodySegment>,
        all_lines: &[&str],
        start_line: usize,
        end_line: usize,
    ) {
        if end_line < start_line { return; }
        let start0 = start_line.saturating_sub(1).min(all_lines.len());
        let end0 = end_line.saturating_sub(1).min(all_lines.len().saturating_sub(1));
        if start0 > end0 || start0 >= all_lines.len() { return; }
        let mut text = String::new();
        for ln in start0..=end0 {
            text.push_str(all_lines[ln]);
            if !all_lines[ln].ends_with('\n') { text.push('\n'); }
        }
        segments.push(BodySegment::Native { text, start_line, end_line });
    }

    // Iterate lines starting from the next line after '{'
    let mut i = body_start_line; // 1-based
    while i <= all_lines.len() {
        let frame_ln = i;
        let line = match all_lines.get(i.saturating_sub(1)) { Some(s) => *s, None => break };
        let bytes = line.as_bytes();
        let mut j = 0usize;
        let mut in_line_comment = false;

        // Quick fast path: if line has no quotes/backticks and SOL cannot start a directive,
        // treat it as native without per-char scanning.
        if !in_template && !in_squote && !in_dquote && !in_block_comment {
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
                    // Still need to look for closing '}' on this line at top-level
                    // Fall through to per-char scan to maintain brace_depth.
                }
            } else {
                if native_start.is_none() { native_start = Some(frame_ln); }
            }
        }

        while j < bytes.len() {
            let ch = bytes[j] as char;
            // Handle comments at top
            if !in_template && !in_squote && !in_dquote && !in_block_comment {
                if ch == '/' && j + 1 < bytes.len() && bytes[j + 1] as char == '/' {
                    in_line_comment = true; break;
                }
                if ch == '/' && j + 1 < bytes.len() && bytes[j + 1] as char == '*' {
                    in_block_comment = true; j += 2; continue;
                }
            }

            if in_block_comment {
                if ch == '*' && j + 1 < bytes.len() && bytes[j + 1] as char == '/' { in_block_comment = false; j += 2; continue; }
                j += 1; continue;
            }
            if in_line_comment { break; }

            // String/template states
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
                    if ch == '$' && j + 1 < bytes.len() && bytes[j + 1] as char == '{' { tpl_expr_depth += 1; j += 2; continue; }
                    if ch == '}' && tpl_expr_depth > 0 { tpl_expr_depth -= 1; j += 1; continue; }
                    if ch == '`' && tpl_expr_depth == 0 { in_template = false; j += 1; continue; }
                    j += 1; continue;
                }
            }

            // Top-level depth tracking (outside protected regions)
            if ch == '{' { brace_depth += 1; j += 1; continue; }
            if ch == '}' {
                brace_depth -= 1;
                if brace_depth == 0 { close_line = frame_ln; break; }
                j += 1; continue;
            }

            // Detect directives at SOL (first non-whitespace)
            if brace_depth == 1 {
                if let Some(col0) = line.find(|c: char| !c.is_whitespace()) {
                    if j == col0 {
                        // Transition
                        if ch == '-' && j + 1 < bytes.len() && bytes[j + 1] as char == '>' {
                            // optional whitespace then '$'
                            let mut k = j + 2; while k < bytes.len() && (bytes[k] as char).is_whitespace() { k += 1; }
                            if k < bytes.len() && bytes[k] as char == '$' {
                                if let Some(ns) = native_start { flush_native_into(&mut segments, &all_lines, ns, frame_ln.saturating_sub(1)); }
                                native_start = None;
                                segments.push(BodySegment::FrameStmt { kind: FrameStmtKind::Transition, frame_line: frame_ln, line_text: line.to_string() });
                                break;
                            }
                        }
                        // Parent forward
                        if ch == '=' && j + 1 < bytes.len() && bytes[j + 1] as char == '>' {
                            let mut k = j + 2; while k < bytes.len() && (bytes[k] as char).is_whitespace() { k += 1; }
                            if k + 1 < bytes.len() && bytes[k] as char == '$' && bytes[k + 1] as char == '^' {
                                if let Some(ns) = native_start { flush_native_into(&mut segments, &all_lines, ns, frame_ln.saturating_sub(1)); }
                                native_start = None;
                                segments.push(BodySegment::FrameStmt { kind: FrameStmtKind::Forward, frame_line: frame_ln, line_text: line.to_string() });
                                break;
                            }
                        }
                        // Stack ops
                        if ch == '$' && j + 4 < bytes.len() && bytes[j + 1] as char == '$' && bytes[j + 2] as char == '[' {
                            let sign = bytes[j + 3] as char;
                            if (sign == '+' || sign == '-') && bytes[j + 4] as char == ']' {
                                if let Some(ns) = native_start { flush_native_into(&mut segments, &all_lines, ns, frame_ln.saturating_sub(1)); }
                                native_start = None;
                                let kind = if sign == '+' { FrameStmtKind::StackPush } else { FrameStmtKind::StackPop };
                                segments.push(BodySegment::FrameStmt { kind, frame_line: frame_ln, line_text: line.to_string() });
                                break;
                            }
                        }
                    }
                }
            }

            j += 1;
        }

        // Reached end of line without pushing a directive; open native run if needed
        if !in_line_comment && brace_depth > 0 {
            let line_has_stmt = segments.last().map(|s| matches!(s, BodySegment::FrameStmt { frame_line, .. } if *frame_line == frame_ln)).unwrap_or(false);
            if !line_has_stmt {
                if native_start.is_none() { native_start = Some(frame_ln); }
            }
        }

        if brace_depth == 0 { break; }
        i += 1;
    }

    // Flush trailing native slice up to the line before '}'
    if let Some(ns) = native_start { if close_line > 0 { flush_native_into(&mut segments, &all_lines, ns, close_line.saturating_sub(1)); } }
    if close_line == body_start_line { close_line = i; }
    (segments, close_line)
}
