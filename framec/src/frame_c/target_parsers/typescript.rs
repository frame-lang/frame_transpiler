use std::any::Any;
use std::sync::Arc;

use crate::frame_c::visitors::TargetLanguage;

use super::{TargetAst, TargetDiagnostic, TargetParseContext, TargetParseError};

/// Minimal placeholder TypeScript target parser.
///
/// Until Phase 2.6 lands with a real SWC-backed implementation, we treat the
/// target block as raw text and surface lightweight diagnostics so the crate
/// continues to compile and emit the captured code verbatim.
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

        let line_count = source.lines().count().max(1);
        let diagnostics = vec![TargetDiagnostic {
            message: format!(
                "TypeScript target block captured ({} line{})",
                line_count,
                if line_count == 1 { "" } else { "s" }
            ),
            target_line: 1,
            column: 1,
        }];

        let elements = vec![TypeScriptTargetElement::Statement(
            TypeScriptTargetStatement {
                code: source.clone(),
                start_line: 1,
                end_line: line_count,
            },
        )];

        Ok(Arc::new(TypeScriptTargetAst {
            source,
            diagnostics,
            elements,
        }))
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
