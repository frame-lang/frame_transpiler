use super::{BodySegment, FrameStmtKind};

/// Segment a Python native region into Native and FrameStmt segments.
/// Top-level detection ignores strings and comments, and recognizes:
///  - Transition: "-> $State"
///  - Parent forward: "=> $^"
///  - Stack push/pop: "$$[+]" / "$$[-]"
pub fn segment_py_body(source: &str, start_line: usize, end_line: usize) -> Vec<BodySegment> {
    if start_line == 0 || end_line == 0 || end_line < start_line {
        return vec![];
    }

    let all_lines: Vec<&str> = source.lines().collect();
    let len0 = all_lines.len();
    if len0 == 0 {
        return vec![];
    }
    // Convert 1-based line numbers to 0-based indices and clamp safely.
    let s_raw = start_line.saturating_sub(1);
    if s_raw >= len0 {
        return vec![];
    }
    let mut e_raw = end_line.saturating_sub(1);
    if e_raw >= len0 {
        e_raw = len0 - 1;
    }
    if s_raw > e_raw {
        return vec![];
    }
    let region_lines = &all_lines[s_raw..=e_raw];

    let mut segments: Vec<BodySegment> = Vec::new();
    let mut native_start: Option<usize> = None;

    // String state tracking
    let mut in_squote = false; // '
    let mut in_dquote = false; // "
    let mut in_tsquote = false; // '''
    let mut in_tdquote = false; // """

    for (i, line) in region_lines.iter().enumerate() {
        let frame_ln = start_line + i;
        let bytes = line.as_bytes();
        let mut j = 0;
        let mut in_line_comment = false;

        // helper to push native segment
        let mut flush_native = |end_line_inclusive: usize| {
            if let Some(start_ln) = native_start {
                if end_line_inclusive >= start_ln {
                    // build text from start_ln..=end_line_inclusive
                    let start0 = (start_ln - start_line) as usize;
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
        };

        // scan characters
        while j < bytes.len() {
            let ch = bytes[j] as char;

            // Line comment starts with '#', but ignore inside strings
            if !(in_squote || in_dquote || in_tsquote || in_tdquote) {
                if ch == '#' {
                    in_line_comment = true;
                    break;
                }
                // Triple quotes start
                if j + 2 < bytes.len()
                    && bytes[j] == b'\''
                    && bytes[j + 1] == b'\''
                    && bytes[j + 2] == b'\''
                {
                    in_tsquote = true;
                    j += 3;
                    continue;
                }
                if j + 2 < bytes.len()
                    && bytes[j] == b'"'
                    && bytes[j + 1] == b'"'
                    && bytes[j + 2] == b'"'
                {
                    in_tdquote = true;
                    j += 3;
                    continue;
                }
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
            } else {
                // Inside strings
                if in_tsquote {
                    if j + 2 < bytes.len()
                        && bytes[j] == b'\''
                        && bytes[j + 1] == b'\''
                        && bytes[j + 2] == b'\''
                    {
                        in_tsquote = false;
                        j += 3;
                        continue;
                    }
                    j += 1;
                    continue;
                }
                if in_tdquote {
                    if j + 2 < bytes.len()
                        && bytes[j] == b'"'
                        && bytes[j + 1] == b'"'
                        && bytes[j + 2] == b'"'
                    {
                        in_tdquote = false;
                        j += 3;
                        continue;
                    }
                    j += 1;
                    continue;
                }
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
            }

            // Detect Frame statements only if not inside a string/comment and only
            // when tokens appear at the first non-whitespace column.
            if !(in_squote || in_dquote || in_tsquote || in_tdquote) {
                let first_non_ws = line.find(|c: char| !c.is_whitespace());
                if let Some(col0) = first_non_ws {
                    if j == col0 {
                        // Return assignment: system.return = expr
                        if bytes.len() >= col0 + 14 {
                            // Fast check for 'system.return'
                            let head = &line[col0..].as_bytes();
                            let pat = b"system.return";
                            if head.starts_with(pat) {
                                flush_native(frame_ln.saturating_sub(1));
                                native_start = None;
                                segments.push(BodySegment::FrameStmt {
                                    kind: FrameStmtKind::Return,
                                    frame_line: frame_ln,
                                    line_text: (*line).to_string(),
                                });
                                break;
                            }
                        }
                        // Transition: -> $State
                        if ch == '-' && j + 1 < bytes.len() && bytes[j + 1] as char == '>' {
                            flush_native(frame_ln.saturating_sub(1));
                            native_start = None;
                            segments.push(BodySegment::FrameStmt {
                                kind: FrameStmtKind::Transition,
                                frame_line: frame_ln,
                                line_text: (*line).to_string(),
                            });
                            break;
                        }
                        // Parent forward: => $^
                        if ch == '=' && j + 1 < bytes.len() && bytes[j + 1] as char == '>' {
                            // lookahead for $^
                            let mut k = j + 2;
                            while k < bytes.len() && (bytes[k] as char).is_whitespace() {
                                k += 1;
                            }
                            if k + 1 < bytes.len()
                                && bytes[k] as char == '$'
                                && bytes[k + 1] as char == '^'
                            {
                                flush_native(frame_ln.saturating_sub(1));
                                native_start = None;
                                segments.push(BodySegment::FrameStmt {
                                    kind: FrameStmtKind::Forward,
                                    frame_line: frame_ln,
                                    line_text: (*line).to_string(),
                                });
                                break;
                            }
                        }
                        // Stack ops: $$[+]/$$[-]
                        if ch == '$'
                            && j + 4 < bytes.len()
                            && bytes[j + 1] as char == '$'
                            && bytes[j + 2] as char == '['
                        {
                            let sign = bytes[j + 3] as char;
                            if (sign == '+' || sign == '-') && bytes[j + 4] as char == ']' {
                                flush_native(frame_ln.saturating_sub(1));
                                native_start = None;
                                let kind = if sign == '+' {
                                    FrameStmtKind::StackPush
                                } else {
                                    FrameStmtKind::StackPop
                                };
                                segments.push(BodySegment::FrameStmt {
                                    kind,
                                    frame_line: frame_ln,
                                    line_text: (*line).to_string(),
                                });
                                break;
                            }
                        }
                    }
                }
            }

            j += 1;
        }

        if !in_line_comment {
            // If we did not emit a FrameStmt on this line, ensure native run is open
            let line_has_stmt = segments
                .last()
                .map(|s| matches!(s, BodySegment::FrameStmt { frame_line, .. } if *frame_line == frame_ln))
                .unwrap_or(false);
            if !line_has_stmt {
                if native_start.is_none() {
                    native_start = Some(frame_ln);
                }
            }
        }
    }

    if let Some(start) = native_start {
        let end = end_line;
        if end >= start {
            let start0 = (start - start_line) as usize;
            // Clamp end0 to the last valid index of region_lines
            let mut end0 = (end - start_line) as usize;
            let max_idx = region_lines.len().saturating_sub(1);
            if end0 > max_idx { end0 = max_idx; }
            if start0 <= end0 {
                let mut text = String::new();
                for ln in start0..=end0 {
                    text.push_str(region_lines[ln]);
                    if !region_lines[ln].ends_with('\n') {
                        text.push('\n');
                    }
                }
                // Map end_line back to frame lines consistent with clamped end0
                let clamped_end_frame = start + end0;
                segments.push(BodySegment::Native {
                    text,
                    start_line: start,
                    end_line: clamped_end_frame,
                });
            }
        }
    }

    segments
}
