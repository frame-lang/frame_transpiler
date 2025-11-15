use crate::frame_c::visitors::TargetLanguage;
use super::validator::BodyKindV3;
use super::native_region_scanner::RegionSpan;
use super::prolog_scanner::PrologScannerV3;
use super::outline_scanner::OutlineScannerV3;
use super::outline_scanner::OutlineItemV3;
use super::import_scanner::{ImportScannerV3, ImportScanResultV3};
use super::import_scanner::python::ImportScannerPyV3;
use super::import_scanner::typescript::ImportScannerTsV3;
use super::import_scanner::csharp::ImportScannerCsV3;
use super::import_scanner::c::ImportScannerCV3;
use super::import_scanner::cpp::ImportScannerCppV3;
use super::import_scanner::java::ImportScannerJavaV3;
use super::import_scanner::rust::ImportScannerRustV3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BodyPartitionV3 {
    pub open_byte: usize,
    pub close_byte: usize, // index of '}'
    pub kind: BodyKindV3,
    pub owner_id: Option<String>,
    pub state_id: Option<String>,
    pub header_span: Option<RegionSpan>,
}

#[derive(Debug, Clone)]
pub struct ModulePartitionsV3 {
    pub prolog: Option<RegionSpan>,
    pub imports: Vec<RegionSpan>,
    pub import_issues: Vec<super::validator::ValidationIssueV3>,
    pub bodies: Vec<BodyPartitionV3>,
}

#[derive(Debug)]
pub struct ModulePartitionErrorV3(pub String);

pub struct ModulePartitionerV3;

impl ModulePartitionerV3 {
    pub fn partition(bytes: &[u8], lang: TargetLanguage) -> Result<ModulePartitionsV3, ModulePartitionErrorV3> {
        let n = bytes.len();
        let mut i = 0usize;
        // Required prolog: must be first non-whitespace token
        let prolog = match PrologScannerV3.scan(bytes) {
            Ok(span) => Some(span),
            Err(e) => return Err(ModulePartitionErrorV3(format!("prolog error: {:?}", e.kind))),
        };
        i = prolog.as_ref().map(|p| p.end).unwrap_or(0);
        let mut imports: Vec<RegionSpan> = Vec::new();
        // Import scanning (language-specific). For now, Python only in this step.
        let mut import_issues = Vec::new();
        match lang {
            crate::frame_c::visitors::TargetLanguage::Python3 => {
                let scanner = ImportScannerPyV3;
                let res: ImportScanResultV3 = scanner.scan(bytes, i);
                imports = res.spans; import_issues.extend(res.issues);
                if let Some(last) = imports.last() { i = last.end; }
            }
            crate::frame_c::visitors::TargetLanguage::TypeScript => {
                let scanner = ImportScannerTsV3;
                let res = scanner.scan(bytes, i);
                imports = res.spans; import_issues.extend(res.issues);
                if let Some(last) = imports.last() { i = last.end; }
            }
            crate::frame_c::visitors::TargetLanguage::CSharp => {
                let scanner = ImportScannerCsV3;
                let res = scanner.scan(bytes, i);
                imports = res.spans; import_issues.extend(res.issues);
                if let Some(last) = imports.last() { i = last.end; }
            }
            crate::frame_c::visitors::TargetLanguage::C => {
                let scanner = ImportScannerCV3;
                let res = scanner.scan(bytes, i);
                imports = res.spans; import_issues.extend(res.issues);
                if let Some(last) = imports.last() { i = last.end; }
            }
            crate::frame_c::visitors::TargetLanguage::Cpp => {
                let scanner = ImportScannerCppV3;
                let res = scanner.scan(bytes, i);
                imports = res.spans; import_issues.extend(res.issues);
                if let Some(last) = imports.last() { i = last.end; }
            }
            crate::frame_c::visitors::TargetLanguage::Java => {
                let scanner = ImportScannerJavaV3;
                let res = scanner.scan(bytes, i);
                imports = res.spans; import_issues.extend(res.issues);
                if let Some(last) = imports.last() { i = last.end; }
            }
            crate::frame_c::visitors::TargetLanguage::Rust => {
                let scanner = ImportScannerRustV3;
                let res = scanner.scan(bytes, i);
                imports = res.spans; import_issues.extend(res.issues);
                if let Some(last) = imports.last() { i = last.end; }
            }
            _ => {}
        }
        // OutlineScanner authoritative
        let outline_items: Vec<OutlineItemV3> = OutlineScannerV3.scan(bytes, i, lang)
            .map_err(|e| ModulePartitionErrorV3(e.message))?;
        let bodies: Vec<BodyPartitionV3> = outline_items.into_iter().map(|it| BodyPartitionV3{
            open_byte: it.open_byte,
            close_byte: it.close_byte,
            kind: it.kind,
            owner_id: it.owner_id,
            state_id: it.state_id,
            header_span: Some(it.header_span),
        }).collect();
        Ok(ModulePartitionsV3 { prolog, imports, import_issues, bodies })
    }
}

// outline_scanner now provides authoritative header parsing
