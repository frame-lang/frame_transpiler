use std::sync::Arc;

use rustpython_parser::{ast::Suite, Parse};

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
struct PythonTargetAst {
    source: String,
    suite: Suite,
    diagnostics: Vec<TargetDiagnostic>,
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

        PythonTargetAst {
            source,
            suite,
            diagnostics,
        }
    }

    fn empty() -> Self {
        PythonTargetAst {
            source: String::new(),
            suite: Vec::new(),
            diagnostics: Vec::new(),
        }
    }
}

impl TargetAst for PythonTargetAst {
    fn target_language(&self) -> TargetLanguage {
        TargetLanguage::Python3
    }

    fn to_source(&self) -> &str {
        &self.source
    }

    fn diagnostics(&self) -> &[TargetDiagnostic] {
        &self.diagnostics
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
}
