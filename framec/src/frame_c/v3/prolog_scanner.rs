use super::native_region_scanner::RegionSpan;

#[derive(Debug)]
pub struct PrologErrorV3 { pub kind: PrologErrorV3Kind, pub message: String }

#[derive(Debug)]
pub enum PrologErrorV3Kind { Missing, NotFirstNonWhitespace, InvalidHead }

pub struct PrologScannerV3;

impl PrologScannerV3 {
    pub fn scan(&self, bytes: &[u8]) -> Result<RegionSpan, PrologErrorV3> {
        let n = bytes.len();
        let mut i = 0usize;
        // Skip UTF-8 BOM if present
        if n >= 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF { i = 3; }
        // Skip whitespace/newlines
        while i < n && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r' || bytes[i] == b'\n') { i += 1; }
        if i >= n { return Err(PrologErrorV3{ kind: PrologErrorV3Kind::Missing, message: "empty file; expected @target prolog".into() }); }
        if bytes[i] != b'@' {
            return Err(PrologErrorV3{ kind: PrologErrorV3Kind::NotFirstNonWhitespace, message: "first non-whitespace token must be @target".into() });
        }
        let line_start = i;
        // Ensure it starts with @target
        let head = b"@target";
        if i + head.len() > n || &bytes[i..i+head.len()] != head {
            return Err(PrologErrorV3{ kind: PrologErrorV3Kind::InvalidHead, message: "expected @target prolog".into() });
        }
        i += head.len();
        // Require space then language token
        if i >= n || !(bytes[i] == b' ' || bytes[i] == b'\t') {
            return Err(PrologErrorV3{ kind: PrologErrorV3Kind::InvalidHead, message: "expected space after @target".into() });
        }
        while i < n && (bytes[i] == b' ' || bytes[i] == b'\t') { i += 1; }
        if i >= n || bytes[i] == b'\n' || bytes[i] == b'\r' {
            return Err(PrologErrorV3{ kind: PrologErrorV3Kind::InvalidHead, message: "expected language after @target".into() });
        }
        // consume language token to end-of-line
        while i < n && bytes[i] != b'\n' { i += 1; }
        Ok(RegionSpan{ start: line_start, end: i })
    }
}

