use super::super::native_region_scanner::RegionSpan;
use super::{ImportScannerV3, ImportScanResultV3};
use super::super::validator::ValidationIssueV3;

pub struct ImportScannerTsV3;

impl ImportScannerV3 for ImportScannerTsV3 {
    fn scan(&self, bytes: &[u8], start: usize) -> ImportScanResultV3 {
        let mut spans: Vec<RegionSpan> = Vec::new();
        let mut issues: Vec<ValidationIssueV3> = Vec::new();
        let n = bytes.len();
        let mut i = start;
        let mut at_sol = true;
        while i < n {
            if at_sol {
                // record logical line start
                let line_start = i;
                // skip leading spaces/tabs
                let mut j = i;
                while j < n && (bytes[j] == b' ' || bytes[j] == b'\t') { j += 1; }
                // Stop scanning imports once we hit a V3 section or system header.
                if j < n && (
                    starts_kw(bytes, j, b"system") ||
                    starts_kw(bytes, j, b"machine") ||
                    starts_kw(bytes, j, b"interface") ||
                    starts_kw(bytes, j, b"actions") ||
                    starts_kw(bytes, j, b"operations") ||
                    starts_kw(bytes, j, b"domain")
                ) {
                    break;
                }
                if j < n && (starts_kw(bytes, j, b"import") || starts_kw(bytes, j, b"export")) {
                    // consume a full import/export statement until semicolon at depth zero
                    let stmt_start = line_start;
                    let mut k = j;
                    let mut depth_paren: i32 = 0;
                    let mut depth_brace: i32 = 0;
                    let mut tmpl_depth: i32 = 0; // `${` nesting inside backticks
                    let mut in_s: u8 = 0; // 1 ', 2 ", 3 backtick
                    let mut found_semicolon = false;
                    while k < n {
                        let b = bytes[k];
                        // handle comments outside strings/templates
                        if in_s == 0 {
                            if b == b'/' && k+1 < n && bytes[k+1] == b'/' {
                                // line comment to end-of-line
                                k += 2; while k < n && bytes[k] != b'\n' { k += 1; }
                                // continue scan to find semicolon only if in multi-line context
                                if k < n { k += 1; continue; } else { break; }
                            }
                            if b == b'/' && k+1 < n && bytes[k+1] == b'*' {
                                // block comment
                                k += 2; while k+1 < n { if bytes[k] == b'*' && bytes[k+1] == b'/' { k += 2; break; } k += 1; }
                                continue;
                            }
                        }

                        match b {
                            b'\'' if in_s == 0 => { in_s = 1; k += 1; }
                            b'\"' if in_s == 0 => { in_s = 2; k += 1; }
                            b'`' if in_s == 0 => { in_s = 3; tmpl_depth = 0; k += 1; }
                            b'`' if in_s == 3 && tmpl_depth == 0 => { in_s = 0; k += 1; }
                            b'\\' if in_s == 1 || in_s == 2 => { k += 2; }
                            b'$' if in_s == 3 && k+1 < n && bytes[k+1] == b'{' => { tmpl_depth += 1; k += 2; }
                            b'}' if in_s == 3 && tmpl_depth > 0 => { tmpl_depth -= 1; k += 1; }
                            b'\'' if in_s == 1 => { in_s = 0; k += 1; }
                            b'\"' if in_s == 2 => { in_s = 0; k += 1; }
                            b'(' if in_s == 0 => { depth_paren += 1; k += 1; }
                            b')' if in_s == 0 => { depth_paren -= 1; k += 1; }
                            b'{' if in_s == 0 && in_s != 3 => { depth_brace += 1; k += 1; }
                            b'}' if in_s == 0 && in_s != 3 => { depth_brace -= 1; k += 1; }
                            b';' if in_s == 0 && depth_paren == 0 && depth_brace == 0 => {
                                spans.push(RegionSpan { start: stmt_start, end: k });
                                found_semicolon = true;
                                k += 1; i = k; at_sol = true; break;
                            }
                            b'\n' => { k += 1; }
                            _ => { k += 1; }
                        }
                    }
                    if k >= n {
                        if !found_semicolon || in_s != 0 || depth_paren != 0 || depth_brace != 0 || (in_s == 3 && tmpl_depth != 0) {
                            issues.push(ValidationIssueV3{ message: "E110: unterminated import/export statement".into() });
                        }
                        spans.push(RegionSpan { start: stmt_start, end: n });
                        i = n; break;
                    }
                    continue;
                }
                // Not import/export at SOL, advance to next line
                while i < n && bytes[i] != b'\n' { i += 1; }
                if i < n { i += 1; }
                at_sol = true;
                continue;
            } else {
                if bytes[i] == b'\n' { at_sol = true; i += 1; } else { i += 1; }
            }
        }
        ImportScanResultV3 { spans, issues }
    }
}

fn starts_kw(bytes: &[u8], i: usize, kw: &[u8]) -> bool {
    let n = bytes.len();
    if i + kw.len() > n { return false; }
    if &bytes[i..i+kw.len()] != kw { return false; }
    // boundary: next must be whitespace or '*' or '{' or identifier break
    let j = i + kw.len();
    if j < n && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') { return false; }
    true
}
