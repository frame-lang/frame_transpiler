use std::str;

/// Minimal domain: scanner for Rust demo modules.
///
/// Scans for a top-level `domain:` block and extracts Rust-style domain
/// declarations of the form:
///   name: Type = Expr
///   name: Type
///   name = Expr
///
/// Returns (name, type_opt, init_opt) triples. This is a best-effort parser
/// intended for demo/runtime scaffolding, not full Rust syntax.
pub(crate) fn scan_rs_domain_fields(bytes: &[u8]) -> Vec<(String, Option<String>, Option<String>)> {
    let n = bytes.len();
    let mut i = 0usize;
    let mut domain_start: Option<usize> = None;
    while i < n {
        // skip leading whitespace
        while i < n && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r' || bytes[i] == b'\n') {
            i += 1;
        }
        if i >= n { break; }
        let line_start = i;
        // skip comments
        if bytes[i] == b'#' {
            while i < n && bytes[i] != b'\n' { i += 1; }
            continue;
        }
        if bytes[i] == b'/' && i + 1 < n && bytes[i + 1] == b'/' {
            while i < n && bytes[i] != b'\n' { i += 1; }
            continue;
        }
        // read identifier at SOL
        let mut j = i;
        while j < n && (bytes[j] == b' ' || bytes[j] == b'\t') { j += 1; }
        let kw_start = j;
        while j < n && (bytes[j] as char).is_ascii_alphanumeric() { j += 1; }
        if kw_start < j && j < n && bytes[j] == b':' {
            let kw = String::from_utf8_lossy(&bytes[kw_start..j]).to_ascii_lowercase();
            if kw.as_str() == "domain" {
                // domain: header detected; domain block begins after this line
                let mut k = line_start;
                while k < n && bytes[k] != b'\n' { k += 1; }
                domain_start = Some(if k < n { k + 1 } else { n });
                break;
            }
        }
        while i < n && bytes[i] != b'\n' { i += 1; }
        if i < n { i += 1; }
    }
    let mut out: Vec<(String, Option<String>, Option<String>)> = Vec::new();
    let mut p = match domain_start { Some(s) => s, None => return out };
    while p < n {
        let line_start = p;
        while p < n && bytes[p] != b'\n' { p += 1; }
        let line_end = p;
        if p < n { p += 1; }
        let line = &bytes[line_start..line_end];
        // Compute indent before trimming so we can detect when the domain
        // block has ended and top-level code (e.g., `fn main`) begins.
        let mut indent = 0usize;
        while indent < line.len() && (line[indent] == b' ' || line[indent] == b'\t') {
            indent += 1;
        }
        // trim
        let s = indent; let mut e = line.len();
        while e > s && (line[e-1] == b' ' || line[e-1] == b'\t' || line[e-1] == b'\r') { e -= 1; }
        if s >= e { continue; }
        let slice = &line[s..e];
        // comments
        if slice[0] == b'#' { continue; }
        if slice.len() >= 2 && slice[0] == b'/' && slice[1] == b'/' { continue; }
        // Heuristic: domain variables live inside the `domain:` block at an
        // indented level. Once we see a non-empty, non-comment line at
        // indent == 0, we treat that as the end of the domain block and stop
        // scanning to avoid mis-parsing `fn main` / other top-level Rust.
        if indent == 0 {
            break;
        }
        let line_str = String::from_utf8_lossy(slice).to_string();
        // helper to trim trailing ';'
        let trim_semicolon = |s: &str| {
            let t = s.trim_end();
            if t.ends_with(';') { t[..t.len()-1].trim().to_string() } else { t.to_string() }
        };
        // Parse "name[: Type] (= Expr)?"
        let mut chars = line_str.chars().peekable();
        let mut name = String::new();
        while let Some(&ch) = chars.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                name.push(ch);
                chars.next();
            } else {
                break;
            }
        }
        if name.is_empty() { continue; }
        let mut remainder: String = chars.collect();
        let mut ty: Option<String> = None;
        if let Some(colon_idx) = remainder.find(':') {
            let after_colon = remainder[colon_idx+1..].trim_start();
            let mut ty_end = 0usize;
            for (idx, ch) in after_colon.char_indices() {
                if ch == '=' || ch == ';' { break; }
                ty_end = idx + ch.len_utf8();
            }
            if ty_end > 0 {
                let ty_str = &after_colon[..ty_end];
                let trimmed = ty_str.trim();
                if !trimmed.is_empty() { ty = Some(trimmed.to_string()); }
            }
            remainder = String::from(after_colon);
        }
        let mut init: Option<String> = None;
        if let Some(eq_pos) = remainder.find('=') {
            let val = &remainder[eq_pos+1..];
            let v = trim_semicolon(val);
            if !v.is_empty() { init = Some(v); }
        }
        out.push((name, ty, init));
    }
    out
}
