use super::super::native_region_scanner::RegionSpan;
use super::{ImportScannerV3, ImportScanResultV3};
use super::super::validator::ValidationIssueV3;

pub struct ImportScannerJavaV3;

impl ImportScannerV3 for ImportScannerJavaV3 {
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
                if j<n && (starts_kw(bytes,j,b"package") || starts_kw(bytes,j,b"import")) {
                    let stmt_start = line_start; let mut k=j; let mut in_s=false; let mut esc=false; let mut block=false; let mut found_semicolon=false;
                    while k<n {
                        if !in_s && !block && k+1<n && bytes[k]==b'/' && bytes[k+1]==b'*' { block=true; k+=2; continue; }
                        if block { if k+1<n && bytes[k]==b'*' && bytes[k+1]==b'/' { block=false; k+=2; continue; } k+=1; continue; }
                        if !in_s && k+1<n && bytes[k]==b'/' && bytes[k+1]==b'/' { k+=2; while k<n && bytes[k]!=b'\n' { k+=1; } continue; }
                        let b=bytes[k];
                        if in_s { if esc { esc=false; k+=1; continue; } if b==b'\\' { esc=true; k+=1; continue; } if b==b'"' { in_s=false; k+=1; continue; } k+=1; continue; }
                        match b { b'"' => { in_s=true; k+=1; }, b';' => { spans.push(RegionSpan{ start: stmt_start, end: k }); found_semicolon=true; k+=1; i=k; break; }, b'\n' => { k+=1; }, _ => { k+=1; } }
                    }
                    if k>=n { if !found_semicolon || in_s || block { issues.push(ValidationIssueV3{ message: "E110: unterminated package/import statement".into() }); } spans.push(RegionSpan{ start: stmt_start, end: n }); i=n; }
                    continue;
                }
                break;
            } else { if bytes[i]==b'\n' { at_sol=true; i+=1; } else { i+=1; } }
        }
        ImportScanResultV3 { spans, issues }
    }
}

fn starts_kw(bytes:&[u8], i:usize, kw:&[u8])->bool{ let n=bytes.len(); if i+kw.len()>n{return false;} if &bytes[i..i+kw.len()]!=kw {return false;} let j=i+kw.len(); if j<n && (bytes[j].is_ascii_alphanumeric()||bytes[j]==b'_'){return false;} true }
