use super::super::native_region_scanner::RegionSpan;
use super::ImportScannerV3;

pub struct ImportScannerCV3;

impl ImportScannerV3 for ImportScannerCV3 {
    fn scan(&self, bytes: &[u8], start: usize) -> Vec<RegionSpan> {
        let mut spans: Vec<RegionSpan> = Vec::new();
        let n = bytes.len();
        let mut i = start;
        let mut at_sol = true;
        while i < n {
            if at_sol {
                if bytes[i] == b'\n' || bytes[i] == b'\r' { i+=1; continue; }
                let line_start = i;
                let mut j = i;
                while j<n && (bytes[j]==b' '||bytes[j]==b'\t') { j+=1; }
                if j<n && bytes[j]==b'#' {
                    // capture directive logical line if it's include
                    let mut k = j+1; while k<n && (bytes[k]==b' '||bytes[k]==b'\t') { k+=1; }
                    let key_start=k; while k<n && bytes[k].is_ascii_alphabetic() { k+=1; }
                    let kw = &bytes[key_start..k];
                    if kw == b"include" {
                        // logical line: continue if trailing backslash
                        let mut p=k; loop {
                            while p<n && bytes[p]!=b'\n' { p+=1; }
                            // check backslash continuation
                            let mut q = if p>0 { p-1 } else { 0 };
                            while q>line_start && (bytes[q]==b' '||bytes[q]==b'\t'||bytes[q]==b'\r') { q-=1; }
                            let cont = bytes.get(q).copied()==Some(b'\\');
                            if cont { if p<n { p+=1; continue; } else { break; } } else { break; }
                        }
                        spans.push(RegionSpan{ start: line_start, end: p.min(n) });
                        i = p.min(n); at_sol = true; continue;
                    } else {
                        break; // end of import section
                    }
                }
                break; // not a directive -> end imports
            } else {
                if bytes[i]==b'\n' { at_sol=true; i+=1; } else { i+=1; }
            }
        }
        spans
    }
}

