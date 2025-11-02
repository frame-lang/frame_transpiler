use std::any::Any;
use std::sync::Arc;

use swc_common::sync::Lrc;
use swc_common::{FileName, SourceMap, Spanned};
use swc_ecma_ast::{Decl, EsVersion, ModuleItem, Stmt};
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_parser::{Parser, StringInput, Syntax, TsSyntax};

use crate::frame_c::visitors::TargetLanguage;

use super::{TargetAst, TargetDiagnostic, TargetParseContext, TargetParseError};

#[derive(Default)]
pub(crate) struct TypeScriptTargetParser;

impl TypeScriptTargetParser {
    pub(crate) fn parse(
        &self,
        ctx: TargetParseContext<'_>,
    ) -> Result<Arc<dyn TargetAst>, TargetParseError> {
        let source = ctx.region.raw_content.clone();
        if source.trim().is_empty() {
            return Ok(Arc::new(TypeScriptTargetAst::empty()));
        }

        let wrapped = format!("async function __frame_target__() {{\n{}\n}}\n", source);

        let cm: Lrc<SourceMap> = Default::default();
        let file = cm.new_source_file(FileName::Custom("<frame-target-ts>".into()).into(), wrapped);

        let lexer = Lexer::new(
            Syntax::Typescript(TsSyntax {
                tsx: false,
                decorators: true,
                dts: false,
                no_early_errors: false,
                disallow_ambiguous_jsx_like: false,
            }),
            EsVersion::Es2022,
            StringInput::from(file.as_ref()),
            None,
        );
        let mut parser = Parser::new_from(lexer);

        let module = parser.parse_module().map_err(|err| {
            let span = err.span();
            let message = err.kind().msg().into_owned();
            let (target_line, column) = Self::span_position(&cm, span);
            TargetParseError::Parse {
                target_language: TargetLanguage::TypeScript,
                message,
                target_line,
                column,
            }
        })?;

        if let Some(err) = parser.take_errors().into_iter().next() {
            let span = err.span();
            let message = err.kind().msg().into_owned();
            let (target_line, column) = Self::span_position(&cm, span);
            return Err(TargetParseError::Parse {
                target_language: TargetLanguage::TypeScript,
                message,
                target_line,
                column,
            });
        }

        let snippet_lines: Vec<&str> = source.split_inclusive('\n').collect();
        let mut elements = Vec::new();
        let mut diagnostics = Vec::new();

        for item in module.body {
            if let ModuleItem::Stmt(Stmt::Decl(Decl::Fn(fn_decl))) = item {
                if fn_decl.ident.sym.as_ref() != "__frame_target__" {
                    continue;
                }
                if let Some(body) = &fn_decl.function.body {
                    for stmt in &body.stmts {
                        let span = stmt.span();
                        let start = cm.lookup_char_pos(span.lo());
                        let end = cm.lookup_char_pos(span.hi());

                        if start.line <= 1 {
                            continue;
                        }

                        let mut start_line = start.line.saturating_sub(1);
                        let mut end_line = end.line.saturating_sub(1);

                        if start_line == 0 {
                            start_line = 1;
                        }
                        if end_line < start_line {
                            end_line = start_line;
                        }

                        let code = Self::slice_lines(&snippet_lines, start_line, end_line);
                        if code.trim().is_empty() {
                            continue;
                        }

                        elements.push(TypeScriptTargetElement::Statement(
                            TypeScriptTargetStatement {
                                code,
                                start_line,
                                end_line,
                            },
                        ));
                    }
                }
            }
        }

        if elements.is_empty() {
            let line_count = source.lines().count().max(1);
            diagnostics.push(TargetDiagnostic {
                message: format!(
                    "TypeScript target block captured ({} line{})",
                    line_count,
                    if line_count == 1 { "" } else { "s" }
                ),
                target_line: 1,
                column: 1,
            });
            elements.push(TypeScriptTargetElement::Statement(
                TypeScriptTargetStatement {
                    code: source.clone(),
                    start_line: 1,
                    end_line: line_count,
                },
            ));
        }

        Ok(Arc::new(TypeScriptTargetAst {
            source,
            diagnostics,
            elements,
        }))
    }

    fn span_position(cm: &Lrc<SourceMap>, span: swc_common::Span) -> (usize, usize) {
        let loc = cm.lookup_char_pos(span.lo());
        let mut target_line = loc.line.saturating_sub(1);
        if target_line == 0 {
            target_line = 1;
        }
        let column = loc.col_display.saturating_add(1);
        (target_line, column)
    }

    fn slice_lines(lines: &[&str], start_line: usize, end_line: usize) -> String {
        if lines.is_empty() {
            return String::new();
        }
        let start_index = start_line
            .saturating_sub(1)
            .min(lines.len().saturating_sub(1));
        let end_index = end_line
            .saturating_sub(1)
            .min(lines.len().saturating_sub(1));

        let mut result = String::new();
        for idx in start_index..=end_index {
            result.push_str(lines[idx]);
        }
        if !result.ends_with('\n')
            && end_index + 1 < lines.len()
            && lines[end_index].ends_with('\n')
        {
            result.push('\n');
        }
        result
    }
}

#[derive(Clone, Debug)]
pub(crate) struct TypeScriptTargetStatement {
    pub code: String,
    pub start_line: usize,
    pub end_line: usize,
}

#[derive(Clone, Debug)]
pub(crate) enum TypeScriptTargetElement {
    Statement(TypeScriptTargetStatement),
    #[allow(dead_code)]
    RawSegment(TypeScriptTargetStatement),
}

#[derive(Clone, Debug)]
pub(crate) struct TypeScriptTargetAst {
    source: String,
    diagnostics: Vec<TargetDiagnostic>,
    elements: Vec<TypeScriptTargetElement>,
}

impl TypeScriptTargetAst {
    fn empty() -> Self {
        TypeScriptTargetAst {
            source: String::new(),
            diagnostics: Vec::new(),
            elements: Vec::new(),
        }
    }

    pub(crate) fn elements(&self) -> &[TypeScriptTargetElement] {
        &self.elements
    }
}

impl TargetAst for TypeScriptTargetAst {
    fn target_language(&self) -> TargetLanguage {
        TargetLanguage::TypeScript
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
    fn parses_statements_with_runtime_sockets() {
        let source = r#"if (!this.socket) {
    throw new Error("Socket not connected");
}
const raw = await runtime/socket.frame_socket_client_read_line(this.socket);
return raw;
"#;

        let region = TargetRegion {
            start_position: 0,
            end_position: None,
            raw_content: source.to_string(),
            target: TargetLanguage::TypeScript,
            source_map: TargetSourceMap {
                frame_start_line: 10,
                target_line_offsets: vec![],
            },
        };

        let parser = TypeScriptTargetParser::default();
        let ast = parser
            .parse(TargetParseContext { region: &region })
            .expect("TypeScript target parse succeeds");

        let ts_ast = ast
            .as_any()
            .downcast_ref::<TypeScriptTargetAst>()
            .expect("TypeScript AST");
        let elements = ts_ast.elements();
        assert!(elements.len() >= 3);
        let mut found_socket = false;
        for element in elements {
            if let TypeScriptTargetElement::Statement(stmt) = element {
                if stmt.code.contains("frame_socket_client_read_line") {
                    found_socket = true;
                }
            }
        }
        assert!(found_socket, "expected rewritten socket call segment");
    }
}
