use super::{BodyCloserV3, CloseErrorV3};

pub struct BodyCloserPyV3;

impl BodyCloserV3 for BodyCloserPyV3 {
    fn close_byte(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<usize, CloseErrorV3> {
        let mut i = open_brace_index + 1;
        let mut depth: i32 = 1;
        let n = bytes.len();
        while i < n {
            match bytes[i] {
                b'\n' => { i += 1; }
                b'\'' | b'"' => {
                    // detect triple quotes
                    let q = bytes[i];
                    let mut triple = false;
                    if i + 2 < n && bytes[i+1] == q && bytes[i+2] == q { triple = true; }
                    i += 1;
                    if triple { i += 2; }
                    // consume until matching quote(s)
                    loop {
                        if i >= n { return Err(CloseErrorV3{ kind: super::CloseErrorV3Kind::UnterminatedString, message: "unterminated string".into() }); }
                        if triple {
                            if bytes[i] == q && i + 2 < n && bytes[i+1]==q && bytes[i+2]==q { i += 3; break; }
                            i += 1;
                        } else {
                            if bytes[i] == b'\\' { i += 2; continue; }
                            if bytes[i] == q { i += 1; break; }
                            i += 1;
                        }
                    }
                }
                b'#' => { // comment to end of line
                    while i < n && bytes[i] != b'\n' { i += 1; }
                }
                b'{' => { depth += 1; i += 1; }
                b'}' => { depth -= 1; i += 1; if depth == 0 { return Ok(i-1); } }
                b'f' | b'F' | b'r' | b'R' | b'b' | b'B' => {
                    // string prefixes like f"..", r'..', etc.
                    if i + 1 < n && (bytes[i+1] == b'\'' || bytes[i+1] == b'"') {
                        let q = bytes[i+1];
                        let mut triple = false;
                        if i + 3 < n && bytes[i+2]==q && bytes[i+3]==q { triple = true; }
                        i += 2; // move to after opening quote
                        if triple { i += 2; }
                        loop {
                            if i >= n { return Err(CloseErrorV3{ kind: super::CloseErrorV3Kind::UnterminatedString, message: "unterminated string".into() }); }
                            if triple {
                                if bytes[i] == q && i + 2 < n && bytes[i+1]==q && bytes[i+2]==q { i += 3; break; }
                                i += 1;
                            } else {
                                if bytes[i] == b'\\' { i += 2; continue; }
                                if bytes[i] == q { i += 1; break; }
                                i += 1;
                            }
                        }
                    } else { i += 1; }
                }
                _ => { i += 1; }
            }
        }
        Err(CloseErrorV3{ kind: super::CloseErrorV3Kind::UnmatchedBraces, message: "body not closed".into() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn closes_with_string_brace() {
        let src = b"{\nprint(\"}\")\n}\n";
        let mut c = BodyCloserPyV3;
        let idx = c.close_byte(src, 0).unwrap();
        assert_eq!(idx, src.len()-2);
    }
}
