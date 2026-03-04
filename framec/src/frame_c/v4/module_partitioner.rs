use crate::frame_c::visitors::TargetLanguage;
use super::validator::BodyKind;
use super::native_region_scanner::RegionSpan;
use super::prolog_scanner::PrologScanner;
use super::outline_scanner::OutlineScanner;
use super::outline_scanner::OutlineItem;
use super::import_scanner::{ImportScanner, ImportScanResult};
use super::import_scanner::python::ImportScannerPy;

use super::import_scanner::csharp::ImportScannerCs;
use super::import_scanner::c::ImportScannerC;
use super::import_scanner::cpp::ImportScannerCpp;
use super::import_scanner::java::ImportScannerJava;
use super::import_scanner::rust::ImportScannerRust;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BodyPartition {
    pub open_byte: usize,
    pub close_byte: usize, // index of '}'
    pub kind: BodyKind,
    pub owner_id: Option<String>,
    pub state_id: Option<String>,
    pub header_span: Option<RegionSpan>,
}

#[derive(Debug, Clone)]
pub struct ModulePartitions {
    pub prolog: Option<RegionSpan>,
    pub imports: Vec<RegionSpan>,
    pub import_issues: Vec<super::validator::ValidationIssue>,
    pub bodies: Vec<BodyPartition>,
}

#[derive(Debug)]
pub struct ModulePartitionError(pub String);

pub struct ModulePartitioner;

impl ModulePartitioner {
    pub fn partition(bytes: &[u8], lang: TargetLanguage) -> Result<ModulePartitions, ModulePartitionError> {
        // Required prolog: must be first non-whitespace token
        let prolog = match PrologScanner.scan(bytes) {
            Ok(span) => Some(span),
            Err(e) => return Err(ModulePartitionError(format!("prolog error: {:?}", e.kind))),
        };
        let mut i = prolog.as_ref().map(|p| p.end).unwrap_or(0);
        let mut imports: Vec<RegionSpan> = Vec::new();
        // Import scanning (language-specific). For now, Python only in this step.
        let mut import_issues = Vec::new();
        match lang {
            crate::frame_c::visitors::TargetLanguage::Python3 => {
                let scanner = ImportScannerPy;
                let res: ImportScanResult = scanner.scan(bytes, i);
                imports = res.spans; import_issues.extend(res.issues);
                if let Some(last) = imports.last() { i = last.end; }
            }
            crate::frame_c::visitors::TargetLanguage::TypeScript => {
                // Frame files use Python-style imports regardless of target language
                // So we use the Python scanner to find import regions
                let scanner = ImportScannerPy;
                let res = scanner.scan(bytes, i);
                imports = res.spans; import_issues.extend(res.issues);
                if let Some(last) = imports.last() { i = last.end; }
            }
            crate::frame_c::visitors::TargetLanguage::CSharp => {
                let scanner = ImportScannerCs;
                let res = scanner.scan(bytes, i);
                imports = res.spans; import_issues.extend(res.issues);
                if let Some(last) = imports.last() { i = last.end; }
            }
            crate::frame_c::visitors::TargetLanguage::C => {
                let scanner = ImportScannerC;
                let res = scanner.scan(bytes, i);
                imports = res.spans; import_issues.extend(res.issues);
                if let Some(last) = imports.last() { i = last.end; }
            }
            crate::frame_c::visitors::TargetLanguage::Cpp => {
                let scanner = ImportScannerCpp;
                let res = scanner.scan(bytes, i);
                imports = res.spans; import_issues.extend(res.issues);
                if let Some(last) = imports.last() { i = last.end; }
            }
            crate::frame_c::visitors::TargetLanguage::Java => {
                let scanner = ImportScannerJava;
                let res = scanner.scan(bytes, i);
                imports = res.spans; import_issues.extend(res.issues);
                if let Some(last) = imports.last() { i = last.end; }
            }
            crate::frame_c::visitors::TargetLanguage::Rust => {
                let scanner = ImportScannerRust;
                let res = scanner.scan(bytes, i);
                imports = res.spans; import_issues.extend(res.issues);
                if let Some(last) = imports.last() { i = last.end; }
            }
            _ => {}
        }
        // OutlineScanner authoritative
        let outline_items: Vec<OutlineItem> = OutlineScanner.scan(bytes, i, lang)
            .map_err(|e| ModulePartitionError(e.message))?;
        let bodies: Vec<BodyPartition> = outline_items.into_iter().map(|it| BodyPartition{
            open_byte: it.open_byte,
            close_byte: it.close_byte,
            kind: it.kind,
            owner_id: it.owner_id,
            state_id: it.state_id,
            header_span: Some(it.header_span),
        }).collect();
        Ok(ModulePartitions { prolog, imports, import_issues, bodies })
    }
}

// outline_scanner now provides authoritative header parsing
