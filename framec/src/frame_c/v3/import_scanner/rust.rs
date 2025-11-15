use super::super::native_region_scanner::RegionSpan;
use super::{ImportScannerV3, ImportScanResultV3};
use super::super::validator::ValidationIssueV3;

pub struct ImportScannerRustV3;

impl ImportScannerV3 for ImportScannerRustV3 {
    fn scan(&self, bytes: &[u8], start: usize) -> ImportScanResultV3 {
        let mut spans: Vec<RegionSpan> = Vec::new();
        let mut issues: Vec<ValidationIssueV3> = Vec::new();
        let n = bytes.len();
        let mut i = start;
        let mut at_sol = true;
        while i<n {
            if at_sol {
                if bytes[i]==b'\n' || bytes[i]==b'\r' { i+=1; continue; }
                let line_start=i; let mut j=i; while j<n && (bytes[j]==b' '||bytes[j]==b'\t'){ j+=1; }
                // Stop scanning once we hit a V3 module/system header; Rust module imports must precede Frame sections.
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
                if j<n && (starts_kw(bytes,j,b"use") || starts_kw(bytes,j,b"extern") ) {
                    let stmt_start = line_start; let mut k=j; let mut in_s=false; let mut raw_hashes=0usize; let mut block=false; let mut esc=false; let mut found_semicolon=false;
                    while k<n {
                        if !in_s && !block && k+1<n && bytes[k]==b'/' && bytes[k+1]==b'*' { block=true; k+=2; continue; }
                        if block { if k+1<n && bytes[k]==b'*' && bytes[k+1]==b'/' { block=false; k+=2; continue; } k+=1; continue; }
                        if !in_s && k+1<n && bytes[k]==b'/' && bytes[k+1]==b'/' { k+=2; while k<n && bytes[k]!=b'\n' { k+=1; } continue; }
                        // string handling including raw r#".."#
                        if !in_s && bytes[k]==b'r' {
                            // lookahead for r#*"
                            let mut p=k+1; let mut hashes=0usize; if p<n && bytes[p]==b'#' { while p<n && bytes[p]==b'#' { hashes+=1; p+=1; } }
                            if p<n && bytes[p]==b'"' { in_s=true; raw_hashes=hashes; k=p+1; continue; }
                        }
                        if in_s {
                            if raw_hashes>0 {
                                if bytes[k]==b'"' { let mut p=k+1; let mut cnt=0usize; while p<n && cnt<raw_hashes && bytes[p]==b'#' { cnt+=1; p+=1; }
                                    if cnt==raw_hashes { in_s=false; k=p; continue; } }
                                k+=1; continue;
                            } else {
                                if esc { esc=false; k+=1; continue; }
                                if bytes[k]==b'\\' { esc=true; k+=1; continue; }
                                if bytes[k]==b'"' { in_s=false; k+=1; continue; }
                                k+=1; continue;
                            }
                        }
                        if bytes[k]==b';' { spans.push(RegionSpan{ start: stmt_start, end: k }); found_semicolon=true; k+=1; i=k; break; }
                        if bytes[k]==b'\n' { k+=1; } else { k+=1; }
                    }
                    if i==line_start { if !found_semicolon || in_s || block { issues.push(ValidationIssueV3{ message: "E110: unterminated use/extern statement".into() }); } spans.push(RegionSpan{ start: stmt_start, end: n }); i=n; }
                    continue;
                }
                break;
            } else { if bytes[i]==b'\n' { at_sol=true; i+=1; } else { i+=1; } }
        }
        ImportScanResultV3 { spans, issues }
    }
}

fn starts_kw(bytes:&[u8], i:usize, kw:&[u8])->bool{ let n=bytes.len(); if i+kw.len()>n{return false;} if &bytes[i..i+kw.len()]!=kw {return false;} let j=i+kw.len(); if j<n && (bytes[j].is_ascii_alphanumeric()||bytes[j]==b'_'){return false;} true }
