use crate::frame_c::visitors::TargetLanguage;
use super::body_closer as closer;
use super::body_closer::BodyCloserV3;
use super::validator::BodyKindV3;
use super::native_region_scanner::RegionSpan;
use super::prolog_scanner::PrologScannerV3;
use super::import_scanner::{ImportScannerV3};
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
    pub header_span: Option<RegionSpan>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModulePartitionsV3 {
    pub prolog: Option<RegionSpan>,
    pub imports: Vec<RegionSpan>,
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
        match lang {
            crate::frame_c::visitors::TargetLanguage::Python3 => {
                let scanner = ImportScannerPyV3;
                imports = scanner.scan(bytes, i);
                if let Some(last) = imports.last() { i = last.end; }
            }
            crate::frame_c::visitors::TargetLanguage::TypeScript => {
                let scanner = ImportScannerTsV3;
                imports = scanner.scan(bytes, i);
                if let Some(last) = imports.last() { i = last.end; }
            }
            crate::frame_c::visitors::TargetLanguage::CSharp => {
                let scanner = ImportScannerCsV3;
                imports = scanner.scan(bytes, i);
                if let Some(last) = imports.last() { i = last.end; }
            }
            crate::frame_c::visitors::TargetLanguage::C => {
                let scanner = ImportScannerCV3;
                imports = scanner.scan(bytes, i);
                if let Some(last) = imports.last() { i = last.end; }
            }
            crate::frame_c::visitors::TargetLanguage::Cpp => {
                let scanner = ImportScannerCppV3;
                imports = scanner.scan(bytes, i);
                if let Some(last) = imports.last() { i = last.end; }
            }
            crate::frame_c::visitors::TargetLanguage::Java => {
                let scanner = ImportScannerJavaV3;
                imports = scanner.scan(bytes, i);
                if let Some(last) = imports.last() { i = last.end; }
            }
            crate::frame_c::visitors::TargetLanguage::Rust => {
                let scanner = ImportScannerRustV3;
                imports = scanner.scan(bytes, i);
                if let Some(last) = imports.last() { i = last.end; }
            }
            _ => {}
        }
        let mut bodies = Vec::new();
        while i < n {
            if bytes[i] == b'{' {
                // Determine header span (line before '{') and parse for kind/name
                let (header_span, kind, owner) = parse_header(bytes, i);
                let close = match lang {
                    TargetLanguage::Python3 => closer::python::BodyCloserPyV3.close_byte(&bytes[i..], 0).map(|c| i + c),
                    TargetLanguage::TypeScript => closer::typescript::BodyCloserTsV3.close_byte(&bytes[i..], 0).map(|c| i + c),
                    TargetLanguage::CSharp => closer::csharp::BodyCloserCsV3.close_byte(&bytes[i..], 0).map(|c| i + c),
                    TargetLanguage::C => closer::c::BodyCloserCV3.close_byte(&bytes[i..], 0).map(|c| i + c),
                    TargetLanguage::Cpp => closer::cpp::BodyCloserCppV3.close_byte(&bytes[i..], 0).map(|c| i + c),
                    TargetLanguage::Java => closer::java::BodyCloserJavaV3.close_byte(&bytes[i..], 0).map(|c| i + c),
                    TargetLanguage::Rust => closer::rust::BodyCloserRustV3.close_byte(&bytes[i..], 0).map(|c| i + c),
                    _ => Err(closer::CloseErrorV3{ kind: closer::CloseErrorV3Kind::Unimplemented, message: "lang not supported".to_string() }),
                }.map_err(|e| ModulePartitionErrorV3(format!("close error: {:?}", e)))?;
                bodies.push(BodyPartitionV3{ open_byte: i, close_byte: close, kind, owner_id: owner, header_span });
                i = close + 1;
                continue;
            }
            i += 1;
        }
        Ok(ModulePartitionsV3 { prolog, imports, bodies })
    }
}

fn parse_header(bytes: &[u8], open_idx: usize) -> (Option<RegionSpan>, BodyKindV3, Option<String>) {
    // Deterministic SOL header parser (no heuristics):
    // Recognizes: handler NAME ... { | action NAME ... { | operation/op NAME ... {
    // Returns header span, kind, and NAME (owner_id candidate).
    let mut start = open_idx;
    while start > 0 && bytes[start-1] != b'\n' { start -= 1; }
    let header_span = if start < open_idx { Some(RegionSpan { start, end: open_idx }) } else { None };
    let hdr = &bytes[start..open_idx];
    let mut i = 0usize;
    // skip leading whitespace
    while i < hdr.len() && (hdr[i] == b' ' || hdr[i] == b'\t') { i += 1; }
    // read keyword
    let kw_start = i;
    while i < hdr.len() && is_ident_char(hdr[i]) { i += 1; }
    let keyword = &hdr[kw_start..i];
    let kw = to_lower_ascii(keyword);
    // require at least one space before name
    while i < hdr.len() && (hdr[i] == b' ' || hdr[i] == b'\t') { i += 1; }
    // read name identifier (optional)
    let name_start = i;
    while i < hdr.len() && is_ident_char(hdr[i]) { i += 1; }
    let name = if i > name_start { Some(String::from_utf8_lossy(&hdr[name_start..i]).to_string()) } else { None };
    let kind = match kw.as_str() {
        "handler" | "on" => BodyKindV3::Handler,
        "action" => BodyKindV3::Action,
        "operation" | "op" => BodyKindV3::Operation,
        _ => BodyKindV3::Unknown,
    };
    (header_span, kind, name)
}

fn is_ident_char(b: u8) -> bool {
    (b'A'..=b'Z').contains(&b) || (b'a'..=b'z').contains(&b) || (b'0'..=b'9').contains(&b) || b == b'_' || b == b'$' || b == b'^'
}

fn to_lower_ascii(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len());
    for &b in bytes {
        s.push(((b as char).to_ascii_lowercase()) as char);
    }
    s
}
