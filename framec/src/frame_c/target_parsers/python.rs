use std::any::Any;
use std::sync::Arc;

use rustpython_parser::{ast::Ranged, ast::Suite, Parse};

use crate::frame_c::visitors::TargetLanguage;

use super::{TargetAst, TargetDiagnostic, TargetParseContext, TargetParseError};

#[derive(Default)]
pub(crate) struct PythonTargetParser;

impl PythonTargetParser {
    pub(crate) fn parse(
        &self,
        ctx: TargetParseContext<'_>,
    ) -> Result<Arc<dyn TargetAst>, TargetParseError> {
        let dedented = self.dedent_source(&ctx.region.raw_content);
        if dedented.trim().is_empty() {
            return Ok(Arc::new(PythonTargetAst::empty()));
        }

        match Suite::parse(&dedented, "<frame-target-python>") {
            Ok(suite) => Ok(Arc::new(PythonTargetAst::new(dedented, suite))),
            Err(err) => {
                let offset = u32::from(err.offset) as usize;
                let (line, column) = self.offset_to_line_col(&dedented, offset);
                Err(TargetParseError::Parse {
                    target_language: TargetLanguage::Python3,
                    message: err.to_string(),
                    target_line: line,
                    column,
                })
            }
        }
    }

    fn dedent_source(&self, raw: &str) -> String {
        let mut min_indent = usize::MAX;
        for line in raw.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let indent = line.chars().take_while(|c| *c == ' ' || *c == '\t').count();
            if indent < min_indent {
                min_indent = indent;
            }
        }

        if min_indent == usize::MAX {
            // All whitespace - normalize to empty lines so diagnostics stay consistent
            return raw
                .lines()
                .map(|_| String::new())
                .collect::<Vec<String>>()
                .join("\n");
        }

        let has_trailing_newline = raw.ends_with('\n');

        let mut lines: Vec<String> = Vec::new();
        for line in raw.lines() {
            if line.trim().is_empty() {
                lines.push(String::new());
            } else {
                let trimmed = line
                    .chars()
                    .skip(min_indent)
                    .collect::<String>()
                    .trim_end_matches('\r')
                    .to_string();
                lines.push(trimmed);
            }
        }

        let mut result = lines.join("\n");
        if has_trailing_newline {
            result.push('\n');
        }
        result
    }

    fn offset_to_line_col(&self, source: &str, offset: usize) -> (usize, usize) {
        let mut current_offset = 0usize;
        let mut line = 1usize;
        let mut column = 1usize;

        for ch in source.chars() {
            if current_offset >= offset {
                break;
            }

            current_offset += ch.len_utf8();
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        (line, column)
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(crate) struct PythonTargetStatement {
    pub code: String,
    pub start_line: usize,
    pub end_line: usize,
}

#[derive(Clone, Debug)]
pub(crate) enum PythonTargetElement {
    RawSegment(PythonTargetStatement),
    Statement(PythonTargetStatement),
}

#[allow(dead_code)]
pub(crate) struct PythonTargetAst {
    source: String,
    suite: Suite,
    diagnostics: Vec<TargetDiagnostic>,
    elements: Vec<PythonTargetElement>,
}

impl PythonTargetAst {
    fn new(source: String, suite: Suite) -> Self {
        let line_count = source.lines().count().max(1);
        let diagnostics = vec![TargetDiagnostic {
            message: format!(
                "Python target block parsed ({} line{})",
                line_count,
                if line_count == 1 { "" } else { "s" }
            ),
            target_line: 1,
            column: 1,
        }];

        let elements = Self::build_elements(&source, &suite);

        PythonTargetAst {
            source,
            suite,
            diagnostics,
            elements,
        }
    }

    fn empty() -> Self {
        PythonTargetAst {
            source: String::new(),
            suite: Vec::new(),
            diagnostics: Vec::new(),
            elements: Vec::new(),
        }
    }

    fn build_elements(source: &str, suite: &Suite) -> Vec<PythonTargetElement> {
        if source.is_empty() {
            return Vec::new();
        }

        let segments: Vec<&str> = source.split_inclusive('\n').collect();
        let mut elements = Vec::new();
        let mut current_line = 1usize;
        let total_lines = segments.len();
        let line_starts = Self::compute_line_starts(source);

        if suite.is_empty() {
            if let Some(code) = Self::slice_segments(&segments, 1, total_lines) {
                let statement = PythonTargetStatement {
                    code,
                    start_line: 1,
                    end_line: total_lines,
                };
                elements.push(PythonTargetElement::RawSegment(statement));
            }
            return elements;
        }

        for stmt in suite {
            let range = stmt.range();
            let start_offset = range.start().to_usize();
            let end_offset = range.end().to_usize();

            let mut start_line = Self::offset_to_line(start_offset, &line_starts);
            if start_line == 0 {
                start_line = 1;
            }

            let end_lookup = if end_offset == 0 {
                0
            } else {
                end_offset.saturating_sub(1)
            };
            let mut end_line = Self::offset_to_line(end_lookup, &line_starts);
            end_line = end_line.max(start_line);

            if start_line > total_lines {
                continue;
            }

            if start_line > current_line {
                if let Some(code) =
                    Self::slice_segments(&segments, current_line, start_line.saturating_sub(1))
                {
                    let statement = PythonTargetStatement {
                        code,
                        start_line: current_line,
                        end_line: start_line.saturating_sub(1),
                    };
                    elements.push(PythonTargetElement::RawSegment(statement));
                }
            }

            end_line = end_line.min(total_lines);
            if let Some(code) = Self::slice_segments(&segments, start_line, end_line) {
                let statement = PythonTargetStatement {
                    code,
                    start_line,
                    end_line,
                };
                elements.push(PythonTargetElement::Statement(statement));
            }

            current_line = end_line.saturating_add(1);
        }

        if current_line <= total_lines {
            if let Some(code) = Self::slice_segments(&segments, current_line, total_lines) {
                let statement = PythonTargetStatement {
                    code,
                    start_line: current_line,
                    end_line: total_lines,
                };
                elements.push(PythonTargetElement::RawSegment(statement));
            }
        }

        elements
    }

    fn slice_segments(segments: &[&str], start_line: usize, end_line: usize) -> Option<String> {
        if segments.is_empty() || start_line == 0 || start_line > end_line {
            return None;
        }

        let len = segments.len();
        let start_idx = start_line.saturating_sub(1);
        if start_idx >= len {
            return None;
        }

        let end_idx = end_line.min(len);
        if end_idx == 0 || end_idx <= start_idx {
            return None;
        }

        let slice = segments[start_idx..end_idx].join("");
        Some(slice)
    }

    fn compute_line_starts(source: &str) -> Vec<usize> {
        let mut starts = vec![0];
        for (idx, ch) in source.char_indices() {
            if ch == '\n' {
                starts.push(idx + 1);
            }
        }
        if !source.is_empty() && !source.ends_with('\n') {
            starts.push(source.len());
        }
        starts
    }

    fn offset_to_line(offset: usize, line_starts: &[usize]) -> usize {
        if line_starts.is_empty() {
            return 1;
        }

        match line_starts.binary_search(&offset) {
            Ok(idx) => idx + 1,
            Err(idx) => idx.max(1),
        }
    }

    pub(crate) fn elements(&self) -> &[PythonTargetElement] {
        &self.elements
    }

    #[allow(dead_code)]
    pub(crate) fn suite(&self) -> &Suite {
        &self.suite
    }
}

impl TargetAst for PythonTargetAst {
    fn target_language(&self) -> TargetLanguage {
        TargetLanguage::Python3
    }

    fn to_source(&self) -> &str {
        &self.source
    }

    fn to_code(&self) -> String {
        self.source.clone()
    }

    fn diagnostics(&self) -> &[TargetDiagnostic] {
        &self.diagnostics
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame_c::scanner::{TargetRegion, TargetSourceMap};

    #[test]
    fn dedent_preserves_structure() {
        let parser = PythonTargetParser::default();
        let source = "        print(\"hi\")\n            if True:\n                return 1\n";
        let dedented = parser.dedent_source(source);
        assert_eq!(dedented, "print(\"hi\")\n    if True:\n        return 1\n");
    }

    #[test]
    fn parse_simple_block() {
        let parser = PythonTargetParser::default();
        let region = TargetRegion {
            start_position: 0,
            end_position: None,
            raw_content: "        print(\"ok\")\n        return\n".to_string(),
            target: TargetLanguage::Python3,
            source_map: TargetSourceMap {
                frame_start_line: 10,
                target_line_offsets: vec![],
            },
        };

        let ctx = TargetParseContext { region: &region };
        let ast = parser.parse(ctx).expect("parse should succeed");
        assert_eq!(ast.to_source(), "print(\"ok\")\nreturn\n");
        assert_eq!(ast.to_code(), "print(\"ok\")\nreturn\n".to_string());
        assert_eq!(ast.diagnostics().len(), 1);
        assert!(ast.diagnostics()[0]
            .message
            .contains("Python target block parsed"));
    }

    #[test]
    fn parse_invalid_python_reports_error() {
        let parser = PythonTargetParser::default();
        let region = TargetRegion {
            start_position: 0,
            end_position: None,
            raw_content: "        if True\n            print(\"oops\")\n".to_string(),
            target: TargetLanguage::Python3,
            source_map: TargetSourceMap {
                frame_start_line: 5,
                target_line_offsets: vec![],
            },
        };

        let ctx = TargetParseContext { region: &region };
        let result = parser.parse(ctx);
        match result {
            Err(TargetParseError::Parse {
                target_language,
                message,
                target_line,
                column,
            }) => {
                assert_eq!(target_language, TargetLanguage::Python3);
                assert_eq!(target_line, 1);
                assert!(column >= 1);
                assert!(
                    !message.trim().is_empty(),
                    "expected non-empty diagnostic message"
                );
            }
            Err(other) => panic!("unexpected error variant: {:?}", other),
            Ok(_) => panic!("expected parse error, but parsing succeeded"),
        }
    }

    #[test]
    fn segments_include_comments_and_statements() {
        let parser = PythonTargetParser::default();
        let region = TargetRegion {
            start_position: 0,
            end_position: None,
            raw_content: "        # leading comment\n\n        if True:\n            print(\"ok\")\n        else:\n            pass\n".to_string(),
            target: TargetLanguage::Python3,
            source_map: TargetSourceMap {
                frame_start_line: 42,
                target_line_offsets: vec![],
            },
        };

        let ctx = TargetParseContext { region: &region };
        let ast = parser.parse(ctx).expect("parse should succeed");
        let python_ast = ast
            .as_any()
            .downcast_ref::<PythonTargetAst>()
            .expect("expected PythonTargetAst");
        let elements = python_ast.elements();
        assert_eq!(elements.len(), 2, "expected raw segment + statement");
        match &elements[0] {
            PythonTargetElement::RawSegment(stmt) => {
                assert!(stmt.code.contains("# leading comment"));
                assert_eq!(stmt.start_line, 1);
            }
            other => panic!("expected raw segment, got {:?}", other),
        }
        match &elements[1] {
            PythonTargetElement::Statement(stmt) => {
                assert!(stmt.code.contains("if True:"));
                assert!(stmt.code.contains("print(\"ok\")"));
                assert_eq!(stmt.start_line, 3);
                assert_eq!(stmt.end_line, 6);
            }
            other => panic!("expected statement, got {:?}", other),
        }
    }
}
