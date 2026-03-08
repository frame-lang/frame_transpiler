use super::super::native_region_scanner::RegionSpan;
use super::{ImportScanner, ImportScanResult};
use super::super::validator::ValidationIssue;

pub struct ImportScannerCs;

impl ImportScanner for ImportScannerCs {
    fn scan(&self, bytes: &[u8], start: usize) -> ImportScanResult {
        let mut spans: Vec<RegionSpan> = Vec::new();
        let mut issues: Vec<ValidationIssue> = Vec::new();
        let n = bytes.len();
        let mut i = start;
        let mut at_sol = true;
        'outer: while i < n {
            if at_sol {
                // handle blank line quickly
                if bytes[i] == b'\n' || bytes[i] == b'\r' { i += 1; continue; }
                let line_start = i;
                // skip indentation
                let mut j = i;
                while j < n && (bytes[j] == b' ' || bytes[j] == b'\t') { j += 1; }
                // preprocessor line
                if j < n && bytes[j] == b'#' {
                    let mut k = j; while k < n && bytes[k] != b'\n' { k += 1; }
                    spans.push(RegionSpan{ start: line_start, end: k });
                    i = if k < n { k + 1 } else { n }; at_sol = true; continue;
                }
                // using directive
                if j < n && starts_kw(bytes, j, b"using") {
                    let stmt_start = line_start; let mut k = j + 5; // after 'using'
                    // require non-identifier boundary after keyword
                    // scan to semicolon, being string/comment aware
                    let mut in_s: u8 = 0; // 1 '"', 2 verbatim, 3 interp normal, 4 interp verbatim, 5 raw ("""), braces tracked
                    let mut raw_quotes: usize = 0; let mut interp_brace: i32 = 0;
                    // consume until ';' at top level
                    let found_semicolon = false;
                    while k < n {
                        let b = bytes[k];
                        match b {
                            b'/' if in_s == 0 && k+1<n && bytes[k+1]==b'/' => { k+=2; while k<n && bytes[k]!=b'\n' { k+=1; } }
                            b'/' if in_s == 0 && k+1<n && bytes[k+1]==b'*' => { k+=2; while k+1<n { if bytes[k]==b'*' && bytes[k+1]==b'/' { k+=2; break; } k+=1; } }
                            b'@' if in_s == 0 && k+1<n && bytes[k+1]==b'"' => { in_s = 2; k+=2; continue; }
                            b'$' if in_s == 0 && k+1<n && bytes[k+1]==b'"' => { in_s = 3; k+=2; continue; }
                            b'$' if in_s == 0 && k+2<n && bytes[k+1]==b'@' && bytes[k+2]==b'"' => { in_s = 4; k+=3; continue; }
                            b'"' if in_s == 0 => {
                                // raw string detection: """ ... """
                                let mut q=1usize; let mut p=k+1; while p<n && bytes[p]==b'"' { q+=1; p+=1; }
                                if q>=3 { in_s = 5; raw_quotes = q; k = p; continue; } else { in_s = 1; k+=1; continue; }
                            }
                            b'"' if in_s == 1 => { in_s = 0; k+=1; continue; }
                            b'\\' if in_s == 1 => { k+=2; continue; }
                            b'"' if in_s == 2 => { // verbatim, doubled quotes escape
                                if k+1<n && bytes[k+1]==b'"' { k+=2; continue; } else { in_s = 0; k+=1; continue; }
                            }
                            b'"' if in_s == 3 && interp_brace == 0 => { in_s = 0; k+=1; continue; }
                            b'{' if (in_s == 3 || in_s == 4) => { interp_brace += 1; k+=1; continue; }
                            b'}' if (in_s == 3 || in_s == 4) && interp_brace>0 => { interp_brace -= 1; k+=1; continue; }
                            b'"' if in_s == 4 => { // verbatim interp, doubled quote escape
                                if k+1<n && bytes[k+1]==b'"' { k+=2; continue; } else if interp_brace==0 { in_s = 0; k+=1; continue; } else { k+=1; continue; }
                            }
                            b'"' if in_s == 5 => { // closing raw string requires raw_quotes
                                let mut q=1usize; let mut p=k+1; while p<n && bytes[p]==b'"' { q+=1; p+=1; }
                                if q>=raw_quotes { in_s = 0; k = p; continue; } else { k = p; continue; }
                            }
                            b';' if in_s == 0 => {
                                spans.push(RegionSpan{ start: stmt_start, end: k });
                                k+=1; i = k; at_sol = true; continue 'outer;
                            }
                            b'\n' => { k+=1; }
                            _ => { k+=1; }
                        }
                    }
                    // EOF without semicolon — still record import up to EOF and report issue
                    if !found_semicolon || in_s != 0 || (in_s == 3 || in_s == 4) && interp_brace != 0 || in_s == 5 {
                        issues.push(ValidationIssue{ message: "E110: unterminated using directive".into() });
                    }
                    spans.push(RegionSpan{ start: stmt_start, end: n });
                    break;
                }
                // Not a preprocessor/using at SOL => imports block finished
                break;
            } else {
                if bytes[i] == b'\n' { at_sol = true; i+=1; } else { i+=1; }
            }
        }
        ImportScanResult { spans, issues }
    }
}

fn starts_kw(bytes: &[u8], i: usize, kw: &[u8]) -> bool {
    let n = bytes.len();
    if i + kw.len() > n { return false; }
    if &bytes[i..i+kw.len()] != kw { return false; }
    // require boundary
    let j = i + kw.len();
    if j < n && (bytes[j].is_ascii_alphanumeric() || bytes[j]==b'_') { return false; }
    true
}
