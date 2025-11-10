use super::native_region_scanner::RegionSpan;
use super::validator::BodyKindV3;
use super::body_closer as closer;
use super::body_closer::BodyCloserV3;
use crate::frame_c::visitors::TargetLanguage;

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
            // Look for IDENT '(' ... ')' '{'
            let mut k = j; while k < n && is_space(bytes[k]) { k += 1; }
            if k < n && bytes[k] == b'(' {
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
                    let owner_id = Some(String::from_utf8_lossy(&bytes[kw_start..j]).to_string());
                    let kind = match section { Section::Actions => BodyKindV3::Action, Section::Operations => BodyKindV3::Operation, _ => BodyKindV3::Handler };
                    items.push(OutlineItemV3{ header_span: RegionSpan{ start: line_start, end: p }, owner_id, kind, open_byte: open, close_byte: close });
                    i = close + 1; continue;
                }
                // found IDENT '(' ... ')' but no '{' -> malformed header
                return Err(OutlineErrorV3{ message: "missing '{' after module artifact header".into() });
            }
            // Otherwise skip to next line
            while i<n && bytes[i]!=b'\n' { i+=1; }
        }
        Ok(items)
    }
}

fn is_space(b: u8) -> bool { b == b' ' || b == b'\t' }
fn is_ident(b: u8) -> bool { b.is_ascii_alphanumeric() || b == b'_' || b == b'$' || b == b'^' }
fn to_lower_ascii(s: &[u8]) -> String { s.iter().map(|b| (*b as char).to_ascii_lowercase()).collect() }
