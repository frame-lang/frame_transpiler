use super::native_region_scanner::RegionSpan;
use super::validator::BodyKindV3;
use super::body_closer as closer;
use super::body_closer::BodyCloserV3;
use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v3::validator::ValidationIssueV3;

#[derive(Debug, Clone)]
pub struct OutlineItemV3 {
    pub header_span: RegionSpan,
    pub owner_id: Option<String>,
    pub kind: BodyKindV3,
    pub open_byte: usize,
    pub close_byte: usize,
}

#[derive(Debug)]
pub struct OutlineErrorV3 { pub message: String }

pub struct OutlineScannerV3;

impl OutlineScannerV3 {
    // SOL-anchored scan for module artifacts per V3 grammar: sections + IDENT '('...')' '{'
    pub fn scan(&self, bytes: &[u8], start: usize, lang: TargetLanguage) -> Result<Vec<OutlineItemV3>, OutlineErrorV3> {
        let mut items: Vec<OutlineItemV3> = Vec::new();
        let n = bytes.len();
        let mut i = start;
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Section { None, Actions, Operations, Interface, Machine }
        let mut section = Section::None;
        while i < n {
            // skip to SOL non-space
            while i < n && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r' || bytes[i] == b'\n') { i += 1; }
            if i >= n { break; }
            let line_start = i;
            // first token
            let mut j = i;
            while j < n && is_space(bytes[j]) { j += 1; }
            let kw_start = j; while j < n && is_ident(bytes[j]) { j += 1; }
            if kw_start == j { while i<n && bytes[i]!=b'\n' { i+=1; } continue; }
            let kw = to_lower_ascii(&bytes[kw_start..j]);
            // Section markers
            if j < n && bytes[j] == b':' {
                match kw.as_str() {
                    "actions" => section = Section::Actions,
                    "operations" => section = Section::Operations,
                    "interface" => section = Section::Interface,
                    "machine" => section = Section::Machine,
                    _ => {}
                }
                while i<n && bytes[i]!=b'\n' { i+=1; }
                continue;
            }
            // Skip state lines starting with '$'
            if bytes[kw_start] == b'$' { while i<n && bytes[i]!=b'\n' { i+=1; } continue; }
            // Recognize headers:
            // - In machine: section → IDENT '(' ... ')' '{' (handler), allowing bare names (no 'fn')
            // - Elsewhere → 'fn' or 'async fn' NAME '(' ... ')' '{'
            let mut name_start = kw_start;
            let mut name_end = j;
            let first_tok = to_lower_ascii(&bytes[name_start..name_end]);
            let mut k = j; while k < n && is_space(bytes[k]) { k += 1; }
            let mut is_func_header = false;
            if matches!(section, Section::Machine) {
                // Bare IDENT '(' … ')' is allowed for handler headers
                if k < n && bytes[k] == b'(' {
                    is_func_header = true;
                }
            } else if first_tok == "fn" || first_tok == "async" {
                // If 'async', require 'fn' next; otherwise require IDENT after 'fn'
                let mut next = j; while next < n && is_space(bytes[next]) { next += 1; }
                if first_tok == "async" {
                    let mut w = next; while w < n && is_ident(bytes[w]) { w += 1; }
                    let maybe_fn = to_lower_ascii(&bytes[next..w]);
                    if maybe_fn == "fn" {
                        k = w; while k < n && is_space(bytes[k]) { k += 1; }
                        let mut p = k; while p < n && is_ident(bytes[p]) { p += 1; }
                        if p > k { name_start = k; name_end = p; j = p; k = p; while k < n && is_space(bytes[k]) { k += 1; } is_func_header = true; }
                    }
                } else {
                    // first token is 'fn'
                    let mut p = k; while p < n && is_ident(bytes[p]) { p += 1; }
                    if p > k { name_start = k; name_end = p; j = p; k = p; while k < n && is_space(bytes[k]) { k += 1; } is_func_header = true; }
                }
            }
            if is_func_header && k < n && bytes[k] == b'(' {
                // balance parens
                let mut depth: i32 = 0; let mut p = k;
                while p < n {
                    let c = bytes[p];
                    match c { b'(' => { depth+=1; p+=1; }, b')' => { depth-=1; p+=1; if depth==0 { break; } }, _ => { p+=1; } }
                }
                while p < n && is_space(bytes[p]) { p += 1; }
                if p < n && bytes[p] == b'{' {
                    let open = p;
                    let close = match lang {
                        TargetLanguage::Python3 => closer::python::BodyCloserPyV3.close_byte(&bytes[open..], 0).map(|c| open + c),
                        TargetLanguage::TypeScript => closer::typescript::BodyCloserTsV3.close_byte(&bytes[open..], 0).map(|c| open + c),
                        TargetLanguage::CSharp => closer::csharp::BodyCloserCsV3.close_byte(&bytes[open..], 0).map(|c| open + c),
                        TargetLanguage::C => closer::c::BodyCloserCV3.close_byte(&bytes[open..], 0).map(|c| open + c),
                        TargetLanguage::Cpp => closer::cpp::BodyCloserCppV3.close_byte(&bytes[open..], 0).map(|c| open + c),
                        TargetLanguage::Java => closer::java::BodyCloserJavaV3.close_byte(&bytes[open..], 0).map(|c| open + c),
                        TargetLanguage::Rust => closer::rust::BodyCloserRustV3.close_byte(&bytes[open..], 0).map(|c| open + c),
                        _ => Err(closer::CloseErrorV3{ kind: closer::CloseErrorV3Kind::Unimplemented, message: "unsupported language".into() }),
                    }.map_err(|e| OutlineErrorV3{ message: format!("body close error: {:?}", e) })?;
                    let owner_id = Some(String::from_utf8_lossy(&bytes[name_start..name_end]).to_string());
                    let kind = match section { Section::Actions => BodyKindV3::Action, Section::Operations => BodyKindV3::Operation, _ => BodyKindV3::Handler };
                    items.push(OutlineItemV3{ header_span: RegionSpan{ start: line_start, end: p }, owner_id, kind, open_byte: open, close_byte: close });
                    i = close + 1; continue;
                }
                // found header 'fn name(' ... ')' but no '{' -> malformed header
                return Err(OutlineErrorV3{ message: "missing '{' after module artifact header".into() });
            }
            // Otherwise skip to next line
            while i<n && bytes[i]!=b'\n' { i+=1; }
        }
        Ok(items)
    }

    // Tolerant scan: collect items and outline issues without aborting on first error.
    pub fn scan_collect(&self, bytes: &[u8], start: usize, lang: TargetLanguage) -> (Vec<OutlineItemV3>, Vec<ValidationIssueV3>) {
        let mut items: Vec<OutlineItemV3> = Vec::new();
        let mut issues: Vec<ValidationIssueV3> = Vec::new();
        let n = bytes.len();
        let mut i = start;
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Section { None, Actions, Operations, Interface, Machine }
        let mut section = Section::None;
        while i < n {
            while i < n && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r' || bytes[i] == b'\n') { i += 1; }
            if i >= n { break; }
            let line_start = i;
            let mut j = i; while j < n && is_space(bytes[j]) { j += 1; }
            let kw_start = j; while j < n && is_ident(bytes[j]) { j += 1; }
            if kw_start == j { while i<n && bytes[i]!=b'\n' { i+=1; } continue; }
            let kw = to_lower_ascii(&bytes[kw_start..j]);
            if j < n && bytes[j] == b':' {
                section = match kw.as_str() { "actions" => Section::Actions, "operations" => Section::Operations, "interface" => Section::Interface, "machine" => Section::Machine, _ => Section::None };
                while i<n && bytes[i]!=b'\n' { i+=1; }
                continue;
            }
            if bytes[kw_start] == b'$' { while i<n && bytes[i]!=b'\n' { i+=1; } continue; }
            let mut k = j; while k < n && is_space(bytes[k]) { k += 1; }
            // Recognize handler headers in machine: section without 'fn'; elsewhere require 'fn'/'async fn'
            let first_tok = to_lower_ascii(&bytes[kw_start..j]);
            let mut is_func_header = false;
            let mut name_start = kw_start;
            let mut name_end = j;
            if matches!(section, Section::Machine) {
                if k < n && bytes[k] == b'(' { is_func_header = true; }
            } else if first_tok == "fn" || first_tok == "async" {
                if first_tok == "async" {
                    let mut next = k; while next < n && is_space(bytes[next]) { next += 1; }
                    let mut w = next; while w < n && is_ident(bytes[w]) { w += 1; }
                    let maybe_fn = to_lower_ascii(&bytes[next..w]);
                    if maybe_fn == "fn" {
                        k = w; while k < n && is_space(bytes[k]) { k += 1; }
                        let mut p = k; while p < n && is_ident(bytes[p]) { p += 1; }
                        if p > k { name_start = k; name_end = p; is_func_header = true; k = p; while k < n && is_space(bytes[k]) { k += 1; } }
                    }
                } else {
                    let mut p = k; while p < n && is_ident(bytes[p]) { p += 1; }
                    if p > k { name_start = k; name_end = p; is_func_header = true; k = p; while k < n && is_space(bytes[k]) { k += 1; } }
                }
            }
            if is_func_header && k < n && bytes[k] == b'(' {
                let mut depth: i32 = 0; let mut p = k;
                while p < n { let c = bytes[p]; match c { b'(' => { depth+=1; p+=1; }, b')' => { depth-=1; p+=1; if depth==0 { break; } }, _ => { p+=1; } } }
                while p < n && is_space(bytes[p]) { p += 1; }
                if p < n && bytes[p] == b'{' {
                    let open = p;
                    match lang {
                        TargetLanguage::Python3 => match closer::python::BodyCloserPyV3.close_byte(&bytes[open..], 0) { Ok(c) => {
                            let close = open + c;
                            let owner_id = Some(String::from_utf8_lossy(&bytes[kw_start..j]).to_string());
                            let kind = match section { Section::Actions => BodyKindV3::Action, Section::Operations => BodyKindV3::Operation, _ => BodyKindV3::Handler };
                            items.push(OutlineItemV3{ header_span: RegionSpan{ start: line_start, end: p }, owner_id, kind, open_byte: open, close_byte: close });
                            i = close + 1; continue;
                        }, Err(e) => { issues.push(ValidationIssueV3{ message: format!("body close error: {:?}", e) }); } },
                        TargetLanguage::TypeScript => match closer::typescript::BodyCloserTsV3.close_byte(&bytes[open..], 0) { Ok(c) => {
                            let close = open + c; let owner_id = Some(String::from_utf8_lossy(&bytes[kw_start..j]).to_string()); let kind = match section { Section::Actions => BodyKindV3::Action, Section::Operations => BodyKindV3::Operation, _ => BodyKindV3::Handler }; items.push(OutlineItemV3{ header_span: RegionSpan{ start: line_start, end: p }, owner_id, kind, open_byte: open, close_byte: close }); i = close + 1; continue;
                        }, Err(e) => { issues.push(ValidationIssueV3{ message: format!("body close error: {:?}", e) }); } },
                        TargetLanguage::CSharp => match closer::csharp::BodyCloserCsV3.close_byte(&bytes[open..], 0) { Ok(c) => { let close = open + c; let owner_id = Some(String::from_utf8_lossy(&bytes[kw_start..j]).to_string()); let kind = match section { Section::Actions => BodyKindV3::Action, Section::Operations => BodyKindV3::Operation, _ => BodyKindV3::Handler }; items.push(OutlineItemV3{ header_span: RegionSpan{ start: line_start, end: p }, owner_id, kind, open_byte: open, close_byte: close }); i = close + 1; continue; }, Err(e) => { issues.push(ValidationIssueV3{ message: format!("body close error: {:?}", e) }); } },
                        TargetLanguage::C => match closer::c::BodyCloserCV3.close_byte(&bytes[open..], 0) { Ok(c) => { let close = open + c; let owner_id = Some(String::from_utf8_lossy(&bytes[kw_start..j]).to_string()); let kind = match section { Section::Actions => BodyKindV3::Action, Section::Operations => BodyKindV3::Operation, _ => BodyKindV3::Handler }; items.push(OutlineItemV3{ header_span: RegionSpan{ start: line_start, end: p }, owner_id, kind, open_byte: open, close_byte: close }); i = close + 1; continue; }, Err(e) => { issues.push(ValidationIssueV3{ message: format!("body close error: {:?}", e) }); } },
                        TargetLanguage::Cpp => match closer::cpp::BodyCloserCppV3.close_byte(&bytes[open..], 0) { Ok(c) => { let close = open + c; let owner_id = Some(String::from_utf8_lossy(&bytes[kw_start..j]).to_string()); let kind = match section { Section::Actions => BodyKindV3::Action, Section::Operations => BodyKindV3::Operation, _ => BodyKindV3::Handler }; items.push(OutlineItemV3{ header_span: RegionSpan{ start: line_start, end: p }, owner_id, kind, open_byte: open, close_byte: close }); i = close + 1; continue; }, Err(e) => { issues.push(ValidationIssueV3{ message: format!("body close error: {:?}", e) }); } },
                        TargetLanguage::Java => match closer::java::BodyCloserJavaV3.close_byte(&bytes[open..], 0) { Ok(c) => { let close = open + c; let owner_id = Some(String::from_utf8_lossy(&bytes[kw_start..j]).to_string()); let kind = match section { Section::Actions => BodyKindV3::Action, Section::Operations => BodyKindV3::Operation, _ => BodyKindV3::Handler }; items.push(OutlineItemV3{ header_span: RegionSpan{ start: line_start, end: p }, owner_id, kind, open_byte: open, close_byte: close }); i = close + 1; continue; }, Err(e) => { issues.push(ValidationIssueV3{ message: format!("body close error: {:?}", e) }); } },
                        TargetLanguage::Rust => match closer::rust::BodyCloserRustV3.close_byte(&bytes[open..], 0) { Ok(c) => { let close = open + c; let owner_id = Some(String::from_utf8_lossy(&bytes[kw_start..j]).to_string()); let kind = match section { Section::Actions => BodyKindV3::Action, Section::Operations => BodyKindV3::Operation, _ => BodyKindV3::Handler }; items.push(OutlineItemV3{ header_span: RegionSpan{ start: line_start, end: p }, owner_id, kind, open_byte: open, close_byte: close }); i = close + 1; continue; }, Err(e) => { issues.push(ValidationIssueV3{ message: format!("body close error: {:?}", e) }); } },
                        _ => {}
                    }
                    // recovery: skip to next line after '{'
                    while i<n && bytes[i]!=b'\n' { i+=1; }
                    continue;
                }
                // malformed header: header '(' ... ')' but no '{'
                issues.push(ValidationIssueV3{ message: "missing '{' after module artifact header".into() });
                while i<n && bytes[i]!=b'\n' { i+=1; }
                continue;
            }
            while i<n && bytes[i]!=b'\n' { i+=1; }
        }
        (items, issues)
    }
}

fn is_space(b: u8) -> bool { b == b' ' || b == b'\t' }
fn is_ident(b: u8) -> bool { b.is_ascii_alphanumeric() || b == b'_' || b == b'$' || b == b'^' }
fn to_lower_ascii(s: &[u8]) -> String { s.iter().map(|b| (*b as char).to_ascii_lowercase()).collect() }
