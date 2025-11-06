use super::BodySegment;

/// Minimal C# segmenter (MVP):
/// For now, treat the entire body slice as a single Native segment.
/// Follow-up work will add string/comment/interpolation awareness and SOL-anchored directives.
#[allow(dead_code)]
pub fn segment_cs_body(source: &str, start_line: usize, end_line: usize) -> Vec<BodySegment> {
    if start_line == 0 || end_line == 0 || end_line < start_line {
        return vec![];
    }
    let all_lines: Vec<&str> = source.lines().collect();
    let s_idx = start_line.saturating_sub(1).min(all_lines.len());
    let e_idx = end_line.saturating_sub(1).min(all_lines.len());
    if s_idx >= all_lines.len() || s_idx > e_idx {
        return vec![];
    }
    let region_lines = &all_lines[s_idx..=e_idx];

    let mut text = String::new();
    for ln in region_lines.iter() {
        text.push_str(ln);
        if !ln.ends_with('\n') {
            text.push('\n');
        }
    }
    vec![BodySegment::Native {
        text,
        start_line,
        end_line,
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segments_entire_cs_body_as_native() {
        let src = r#"@target csharp
system Demo {
    actions:
        run() {
            // C# native body
            var s = $"value: {42}";
        }
}
"#;
        let segs = segment_cs_body(src, 5, 7);
        assert_eq!(segs.len(), 1);
        match &segs[0] {
            BodySegment::Native { text, start_line, end_line } => {
                assert!(text.contains("value:"));
                assert_eq!((*start_line, *end_line), (5, 7));
            }
            _ => panic!("expected Native segment"),
        }
    }
}
