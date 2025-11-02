#![allow(dead_code)] // Scaffolding module: parsers will be wired in future steps.

use std::any::Any;
use std::sync::Arc;

use crate::frame_c::visitors::TargetLanguage;

use super::{TargetAst, TargetDiagnostic, TargetParseContext, TargetParseError};

/// Generic passthrough parser used as scaffolding for targets that do not yet
/// have a dedicated front-end. It preserves the raw snippet while recording
/// lightweight diagnostics so scanners/visitors can validate boundary handling.
#[derive(Clone, Debug)]
pub(crate) struct PassthroughTargetAst {
    source: String,
    diagnostics: Vec<TargetDiagnostic>,
    elements: Vec<PassthroughTargetElement>,
    language: TargetLanguage,
}

#[derive(Clone, Debug)]
pub(crate) struct PassthroughTargetStatement {
    pub code: String,
    pub start_line: usize,
    pub end_line: usize,
}

#[derive(Clone, Debug)]
pub(crate) enum PassthroughTargetElement {
    Statement(PassthroughTargetStatement),
}

impl PassthroughTargetAst {
    fn empty(language: TargetLanguage) -> Self {
        PassthroughTargetAst {
            source: String::new(),
            diagnostics: Vec::new(),
            elements: Vec::new(),
            language,
        }
    }

    pub(crate) fn elements(&self) -> &[PassthroughTargetElement] {
        &self.elements
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame_c::scanner::{TargetRegion, TargetSourceMap};

    fn build_region(language: TargetLanguage, raw: &str) -> TargetRegion {
        TargetRegion {
            start_position: 0,
            end_position: None,
            raw_content: raw.to_string(),
            target: language,
            source_map: TargetSourceMap {
                frame_start_line: 5,
                target_line_offsets: vec![],
            },
        }
    }

    #[test]
    fn captures_rust_block_lines() {
        let parser = PassthroughParser::new(TargetLanguage::Rust);
        let raw = r#"println!("hello");"#;
        let region = build_region(TargetLanguage::Rust, raw);
        let ast = parser
            .parse(TargetParseContext { region: &region })
            .expect("passthrough parse");
        assert_eq!(ast.target_language(), TargetLanguage::Rust);
        assert_eq!(ast.to_code(), raw);
        let statements = ast
            .as_any()
            .downcast_ref::<PassthroughTargetAst>()
            .unwrap()
            .elements();
        assert_eq!(statements.len(), 1);
    }

    #[test]
    fn handles_blank_lines_between_segments() {
        let parser = PassthroughParser::new(TargetLanguage::Cpp);
        let raw = "std::cout << 1;\n\nstd::cout << 2;\n";
        let region = build_region(TargetLanguage::Cpp, raw);
        let ast = parser
            .parse(TargetParseContext { region: &region })
            .expect("passthrough parse");
        let passthrough = ast.as_any().downcast_ref::<PassthroughTargetAst>().unwrap();
        assert_eq!(passthrough.elements().len(), 2);
        assert!(passthrough.diagnostics().first().is_some());
    }
}

impl TargetAst for PassthroughTargetAst {
    fn target_language(&self) -> TargetLanguage {
        self.language
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

/// Builder object that creates [`PassthroughTargetAst`] instances for a given
/// target. Later phases can replace this with dedicated parsers without
/// touching the scanner or visitor plumbing.
pub(crate) struct PassthroughParser {
    language: TargetLanguage,
}

impl PassthroughParser {
    pub(crate) fn new(language: TargetLanguage) -> Self {
        PassthroughParser { language }
    }

    pub(crate) fn parse(
        &self,
        ctx: TargetParseContext<'_>,
    ) -> Result<Arc<dyn TargetAst>, TargetParseError> {
        let source = ctx.region.raw_content.clone();
        if source.trim().is_empty() {
            return Ok(Arc::new(PassthroughTargetAst::empty(self.language)));
        }

        let lines: Vec<&str> = source.split_inclusive('\n').collect();
        let mut elements = Vec::new();
        let mut current_start: Option<usize> = None;

        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            let is_last_line = idx == lines.len() - 1;
            if trimmed.is_empty() {
                if let Some(start_line) = current_start.take() {
                    elements.push(PassthroughTargetElement::Statement(self.collect_statement(
                        &lines,
                        start_line,
                        idx.saturating_sub(1),
                    )));
                }
                continue;
            }

            if current_start.is_none() {
                current_start = Some(idx);
            }

            if is_last_line {
                if let Some(start_line) = current_start.take() {
                    elements.push(PassthroughTargetElement::Statement(
                        self.collect_statement(&lines, start_line, idx),
                    ));
                }
            }
        }

        if let Some(start_line) = current_start.take() {
            elements.push(PassthroughTargetElement::Statement(self.collect_statement(
                &lines,
                start_line,
                lines.len().saturating_sub(1),
            )));
        }

        if elements.is_empty() {
            elements.push(PassthroughTargetElement::Statement(
                PassthroughTargetStatement {
                    code: source.clone(),
                    start_line: 1,
                    end_line: source.lines().count().max(1),
                },
            ));
        }

        let line_count = source.lines().count().max(1);
        let diagnostics = vec![TargetDiagnostic {
            message: format!(
                "{:?} target block captured ({} line{})",
                self.language,
                line_count,
                if line_count == 1 { "" } else { "s" }
            ),
            target_line: 1,
            column: 1,
        }];

        Ok(Arc::new(PassthroughTargetAst {
            source,
            diagnostics,
            elements,
            language: self.language,
        }))
    }

    fn collect_statement(
        &self,
        lines: &[&str],
        start_line_index: usize,
        end_line_index: usize,
    ) -> PassthroughTargetStatement {
        let mut code = String::new();
        for idx in start_line_index..=end_line_index {
            if let Some(line) = lines.get(idx) {
                code.push_str(line);
            }
        }
        let start_line = start_line_index.saturating_add(1);
        let end_line = end_line_index.saturating_add(1);
        PassthroughTargetStatement {
            code,
            start_line,
            end_line,
        }
    }
}
