use super::{BodyCloserV3, CloseErrorV3, CloseErrorV3Kind};

pub struct BodyCloserTsV3;

impl BodyCloserV3 for BodyCloserTsV3 {
    fn close_byte(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<usize, CloseErrorV3> {
        let mut i = open_brace_index + 1;
        let mut depth: i32 = 1;
        let n = bytes.len();
        while i < n {
            match bytes[i] {
                b'\n' => { i += 1; }
                b'\'' | b'"' => {
                    let q = bytes[i]; i += 1;
                    while i < n {
                        if bytes[i] == b'\\' { i += 2; continue; }
                        if bytes[i] == q { i += 1; break; }
                        i += 1;
                    }
                    if i >= n { return Err(CloseErrorV3{ kind: CloseErrorV3Kind::UnterminatedString, message: "unterminated string".into() }); }
                }
                b'`' => {
                    // template with nested ${}
                    i += 1;
                    let mut brace = 0i32;
                    while i < n {
                        if bytes[i] == b'`' && brace == 0 { i += 1; break; }
                        if bytes[i] == b'\\' { i += 2; continue; }
                        if bytes[i] == b'$' && i+1 < n && bytes[i+1]==b'{' { brace += 1; i += 2; continue; }
                        if bytes[i] == b'}' && brace > 0 { brace -= 1; i += 1; continue; }
                        i += 1;
                    }
                    if i >= n { return Err(CloseErrorV3{ kind: CloseErrorV3Kind::UnterminatedString, message: "unterminated template".into() }); }
                }
                b'/' if i+1<n && bytes[i+1]==b'/' => { // line comment
                    i += 2; while i<n && bytes[i]!=b'\n' { i+=1; }
                }
                b'/' if i+1<n && bytes[i+1]==b'*' => { // block comment
                    i += 2; while i+1<n { if bytes[i]==b'*' && bytes[i+1]==b'/' { i+=2; break; } i+=1; }
                    if i+1>=n { return Err(CloseErrorV3{ kind: CloseErrorV3Kind::UnterminatedComment, message: "unterminated comment".into() }); }
                }
                b'{' => { depth += 1; i += 1; }
                b'}' => { depth -= 1; i += 1; if depth==0 { return Ok(i-1); } }
                _ => { i += 1; }
            }
        }
        Err(CloseErrorV3{ kind: CloseErrorV3Kind::UnmatchedBraces, message: "body not closed".into() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn closes_with_backtick_template() {
        let src = b"{\nconst s = `a${{x:1}}b`;\n}\n";
        let mut c = BodyCloserTsV3;
        let idx = c.close_byte(src, 0).unwrap();
        assert_eq!(idx, src.len()-2);
    }
}
