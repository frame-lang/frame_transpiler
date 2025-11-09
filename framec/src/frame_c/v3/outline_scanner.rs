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
    // SOL-anchored scan for module artifacts: handler|action|operation|op|on NAME { ... }
    pub fn scan(&self, bytes: &[u8], start: usize, lang: TargetLanguage) -> Result<Vec<OutlineItemV3>, OutlineErrorV3> {
        let mut items: Vec<OutlineItemV3> = Vec::new();
        let n = bytes.len();
        let mut i = start;
        while i < n {
            // find SOL
            // skip whitespace/newlines
            while i < n && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r' || bytes[i] == b'\n') { i += 1; }
            if i >= n { break; }
            let line_start = i;
            // read keyword
            let mut j = i;
            while j < n && is_space(bytes[j]) { j += 1; }
            let kw_start = j; while j < n && is_ident(bytes[j]) { j += 1; }
            if kw_start == j { // not an identifier at SOL: skip to next line
                while i < n && bytes[i] != b'\n' { i += 1; }
                continue;
            }
            let kw = to_lower_ascii(&bytes[kw_start..j]);
            let kind = match kw.as_str() { "handler" | "on" => BodyKindV3::Handler, "action" => BodyKindV3::Action, "operation" | "op" => BodyKindV3::Operation, _ => { // not a module artifact
                // skip to next line
                while i < n && bytes[i] != b'\n' { i += 1; }
                continue
            }};
            // spaces then name (optional)
            while j < n && is_space(bytes[j]) { j += 1; }
            let name_start = j; while j < n && is_ident(bytes[j]) { j += 1; }
            let owner_id = if j > name_start { Some(String::from_utf8_lossy(&bytes[name_start..j]).to_string()) } else { None };
            // scan forward to first '{' skipping whitespace/newlines
            let mut k = j; while k < n && (bytes[k] == b' ' || bytes[k] == b'\t' || bytes[k] == b'\r' || bytes[k] == b'\n') { k += 1; }
            if k < n && bytes[k] == b'{' {
                let open = k;
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
                items.push(OutlineItemV3{ header_span: RegionSpan{ start: line_start, end: j }, owner_id, kind, open_byte: open, close_byte: close });
                i = close + 1;
                continue;
            } else { /* no body opener follows header; skip line */ }
            // No '{' found; move to next line
            while i < n && bytes[i] != b'\n' { i += 1; }
        }
        Ok(items)
    }
}

fn is_space(b: u8) -> bool { b == b' ' || b == b'\t' }
fn is_ident(b: u8) -> bool { b.is_ascii_alphanumeric() || b == b'_' || b == b'$' || b == b'^' }
fn to_lower_ascii(s: &[u8]) -> String { s.iter().map(|b| (*b as char).to_ascii_lowercase()).collect() }
