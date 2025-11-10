use super::super::native_region_scanner::RegionSpan;
use super::{ImportScannerV3, ImportScanResultV3};
use super::super::validator::ValidationIssueV3;

pub struct ImportScannerCppV3;

impl ImportScannerV3 for ImportScannerCppV3 {
    fn scan(&self, bytes: &[u8], start: usize) -> ImportScanResultV3 {
        let mut spans: Vec<RegionSpan> = Vec::new();
        let mut issues: Vec<ValidationIssueV3> = Vec::new();
        let n = bytes.len();
        let mut i = start;
        let mut at_sol = true;
        while i < n {
            if at_sol {
                if bytes[i]==b'\n' || bytes[i]==b'\r' { i+=1; continue; }
                let line_start = i;
                let mut j=i; while j<n && (bytes[j]==b' '||bytes[j]==b'\t') { j+=1; }
                if j<n && bytes[j]==b'#' {
                    // #include logical line
                    let mut k=j+1; while k<n && (bytes[k]==b' '||bytes[k]==b'\t') { k+=1; }
                    let ks=k; while k<n && bytes[k].is_ascii_alphabetic() { k+=1; }
                    if &bytes[ks..k]==b"include" {
                        let mut p=k; loop { while p<n && bytes[p]!=b'\n' { p+=1; }
                            let mut q=if p>0 { p-1 } else { 0 }; while q>line_start && (bytes[q]==b' '||bytes[q]==b'\t'||bytes[q]==b'\r'){ q-=1; }
                            if bytes.get(q).copied()==Some(b'\\') { if p<n { p+=1; continue; } }
                            break; }
                        // basic closure check for <...> or "..."
                        let mut has_closure=false;
                        let mut s=k; while s<p && (bytes[s]==b' '||bytes[s]==b'\t'){ s+=1; }
                        if s<p {
                            if bytes[s]==b'<' { let mut t=s+1; let mut closed=false; while t<p { if bytes[t]==b'>' { closed=true; break; } t+=1; } if closed { has_closure=true; } }
                            else if bytes[s]==b'"' { let mut t=s+1; let mut esc=false; let mut closed=false; while t<p { let b=bytes[t]; if esc { esc=false; t+=1; continue; } if b==b'\\' { esc=true; t+=1; continue; } if b==b'"' { closed=true; break; } t+=1; } if closed { has_closure=true; } }
                        }
                        if !has_closure { issues.push(ValidationIssueV3{ message: "unterminated C++ #include".into() }); }
                        spans.push(RegionSpan{ start: line_start, end: p.min(n)});
                        i=p.min(n); continue;
                    } else { break; }
                }
                // using/import statements ended by ';'
                if j<n && (starts_kw(bytes,j,b"using") || starts_kw(bytes,j,b"import")) {
                    let stmt_start = line_start; let mut k=j;
                    let mut in_s:u8=0; let mut in_char:bool=false; let mut raw:bool=false; let mut raw_delim: Vec<u8> = Vec::new();
                    let mut found_semicolon=false;
                    while k<n {
                        if !raw && in_s==0 && !in_char && k+1<n && bytes[k]==b'/' && bytes[k+1]==b'*' { k+=2; while k+1<n { if bytes[k]==b'*'&&bytes[k+1]==b'/'{k+=2;break;} k+=1;} continue; }
                        if !raw && in_s==0 && !in_char && k+1<n && bytes[k]==b'/' && bytes[k+1]==b'/' { k+=2; while k<n && bytes[k]!=b'\n' { k+=1; } continue; }
                        if !raw && in_s==0 && !in_char && k+1<n && bytes[k]==b'R' && bytes[k+1]==b'"' {
                            // parse raw string delimiter until '(' then seek )delim"
                            k+=2; raw=true; raw_delim.clear();
                            while k<n && bytes[k]!=b'(' { raw_delim.push(bytes[k]); k+=1; }
                            if k<n { k+=1; } continue;
                        }
                        if raw {
                            // search for )delim"
                            if bytes[k]==b')' {
                                let mut p=k+1; let mut ok=true; for d in &raw_delim { if p>=n || bytes[p]!=*d { ok=false; break; } p+=1; }
                                if ok && p<n && bytes[p]==b'"' { raw=false; k=p+1; continue; }
                            }
                            k+=1; continue;
                        }
                        match bytes[k] {
                            b'\'' if in_s==0 => { in_char = true; k+=1; }
                            b'\\' if in_char => { k+=2; }
                            b'\'' if in_char => { in_char=false; k+=1; }
                            b'"' if !in_char && in_s==0 => { in_s=1; k+=1; }
                            b'\\' if in_s==1 => { k+=2; }
                            b'"' if in_s==1 => { in_s=0; k+=1; }
                            b';' if !in_char && in_s==0 => { spans.push(RegionSpan{ start: stmt_start, end: k }); found_semicolon=true; k+=1; i=k; break; }
                            b'\n' => { k+=1; }
                            _ => { k+=1; }
                        }
                    }
                    if k>=n { if !found_semicolon || in_s!=0 || in_char || raw { issues.push(ValidationIssueV3{ message: "unterminated C++ using/import".into() }); } spans.push(RegionSpan{ start: stmt_start, end: n }); i=n; }
                    continue;
                }
                break; // imports block done
            } else {
                if bytes[i]==b'\n' { at_sol=true; i+=1; } else { i+=1; }
            }
        }
        ImportScanResultV3 { spans, issues }
    }
}

fn starts_kw(bytes:&[u8], i:usize, kw:&[u8])->bool{ let n=bytes.len(); if i+kw.len()>n {return false;} if &bytes[i..i+kw.len()]!=kw {return false;} let j=i+kw.len(); if j<n && (bytes[j].is_ascii_alphanumeric()||bytes[j]==b'_'){return false;} true }
