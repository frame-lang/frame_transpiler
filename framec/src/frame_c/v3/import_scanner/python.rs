use super::super::native_region_scanner::RegionSpan;
use super::{ImportScannerV3, ImportScanResultV3};
use super::super::validator::ValidationIssueV3;

pub struct ImportScannerPyV3;

impl ImportScannerV3 for ImportScannerPyV3 {
    fn scan(&self, bytes: &[u8], start: usize) -> ImportScanResultV3 {
        let mut spans: Vec<RegionSpan> = Vec::new();
        let mut issues: Vec<ValidationIssueV3> = Vec::new();
        let n = bytes.len();
        let mut i = start;
        let mut at_sol = true;
        // Line-oriented scan; SOL-only detection of 'import' or 'from'
        while i < n {
            if at_sol {
                // Determine start of this line
                let line_start = i;
                // skip spaces/tabs
                let mut j = i;
                while j < n && (bytes[j] == b' ' || bytes[j] == b'\t') { j += 1; }
                if j < n && (starts_kw(bytes, j, b"import") || starts_kw(bytes, j, b"from")) {
                    // Collect full logical import statement (handle parens and backslash continuations)
                    let stmt_start = line_start;
                    let mut k = j;
                    let mut paren: i32 = 0;
                    let mut in_s: u8 = 0; // 0 none, 1 ', 2 "
                    let mut in_triple: u8 = 0; // 1 ''' , 2 """
                    while k < n {
                        let b = bytes[k];
                        if in_triple == 1 {
                            if k+2 < n && bytes[k] == b'\'' && bytes[k+1] == b'\'' && bytes[k+2] == b'\'' { k += 3; in_triple = 0; continue; }
                            k += 1; continue;
                        } else if in_triple == 2 {
                            if k+2 < n && bytes[k] == b'\"' as u8 && bytes[k+1] == b'\"' as u8 && bytes[k+2] == b'\"' as u8 { k += 3; in_triple = 0; continue; }
                            k += 1; continue;
                        }
                        match b {
                            b'\'' if in_s == 0 => {
                                if k+2 < n && bytes[k+1] == b'\'' && bytes[k+2] == b'\'' { in_triple = 1; k += 3; continue; }
                                in_s = 1; k += 1; continue;
                            }
                            b'\"' if in_s == 0 => {
                                if k+2 < n && bytes[k+1] == b'\"' as u8 && bytes[k+2] == b'\"' as u8 { in_triple = 2; k += 3; continue; }
                                in_s = 2; k += 1; continue;
                            }
                            b'\'' if in_s == 1 => { in_s = 0; k += 1; continue; }
                            b'\"' if in_s == 2 => { in_s = 0; k += 1; continue; }
                            b'(' => { if in_s == 0 { paren += 1; } k += 1; continue; }
                            b')' => { if in_s == 0 { paren -= 1; } k += 1; continue; }
                            b'#' => { // comment to end of physical line
                                while k < n && bytes[k] != b'\n' { k += 1; }
                                // fallthrough to newline handling
                            }
                            b'\n' => {
                                // check if line continues: paren>0 or trailing backslash
                                // find last non-space before this newline from line start of current physical line
                                let mut p = k; if p>0 { p -= 1; }
                                while p > stmt_start && (bytes[p] == b' ' || bytes[p] == b'\t' || bytes[p] == b'\r') { if p==0 { break; } p -= 1; }
                                let backslash_cont = bytes.get(p).copied() == Some(b'\\');
                                if paren > 0 || backslash_cont {
                                    k += 1; continue; // continue logical line
                                } else {
                                    let stmt_end = k; // exclude newline
                                    spans.push(RegionSpan { start: stmt_start, end: stmt_end });
                                    k += 1; i = k; at_sol = true; break;
                                }
                            }
                            _ => { k += 1; continue; }
                        }
                    }
                    if k >= n {
                        // EOF while inside an import logical line: report issue if open paren/string
                        if paren > 0 || in_s != 0 || in_triple != 0 {
                            issues.push(ValidationIssueV3{ message: "E110: unterminated import statement".into() });
                        }
                        spans.push(RegionSpan { start: stmt_start, end: n }); i = n; break; }
                    continue;
                }
                // Not an import line; advance to end of line
                while i < n && bytes[i] != b'\n' { i += 1; }
                if i < n { i += 1; }
                at_sol = true;
                continue;
            } else {
                // We always reset to SOL at newlines
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
    // enforce boundary: next must be whitespace
    let j = i + kw.len();
    if j < n && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') { return false; }
    true
}
