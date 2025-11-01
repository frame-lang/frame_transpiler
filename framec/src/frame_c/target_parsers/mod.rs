use std::any::Any;
use std::sync::Arc;

use crate::frame_c::scanner::TargetRegion;
use crate::frame_c::visitors::TargetLanguage;

pub mod python;
pub mod typescript;

#[allow(dead_code)]
pub struct ParsedTargetBlock {
    pub region_index: usize,
    pub frame_start_line: usize,
    pub frame_end_line: usize,
    pub ast: Arc<dyn TargetAst>,
}

#[allow(dead_code)]
pub trait TargetAst: Send + Sync + 'static {
    fn target_language(&self) -> TargetLanguage;
    fn to_source(&self) -> &str;
    fn to_code(&self) -> String;
    fn diagnostics(&self) -> &[TargetDiagnostic];
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct TargetDiagnostic {
    pub message: String,
    pub target_line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub(crate) struct TargetParseContext<'a> {
    pub region: &'a TargetRegion,
}

#[derive(Debug)]
pub(crate) enum TargetParseError {
    Unsupported(TargetLanguage),
    Parse {
        target_language: TargetLanguage,
        message: String,
        target_line: usize,
        column: usize,
    },
}

pub(crate) fn parse_target_region(
    target_language: TargetLanguage,
    region: &TargetRegion,
) -> Result<Arc<dyn TargetAst>, TargetParseError> {
    match target_language {
        TargetLanguage::Python3 => {
            let parser = python::PythonTargetParser::default();
            parser.parse(TargetParseContext { region })
        }
        TargetLanguage::TypeScript => {
            let parser = typescript::TypeScriptTargetParser::default();
            parser.parse(TargetParseContext { region })
        }
        other => Err(TargetParseError::Unsupported(other)),
    }
}
