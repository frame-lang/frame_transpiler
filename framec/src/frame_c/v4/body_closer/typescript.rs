use super::{BodyCloser, CloseError, CloseErrorKind};

pub struct BodyCloserTs;

impl BodyCloser for BodyCloserTs {
    fn close_byte(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<usize, CloseError> {
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
                    if i >= n { return Err(CloseError{ kind: CloseErrorKind::UnterminatedString, message: "unterminated string".into() }); }
                }
                b'`' => {
                    // Check for Frame V4 statements: `push$ or `-> pop$
                    // These are NOT template literals - skip to end of line
                    if i + 5 < n && &bytes[i+1..i+6] == b"push$" {
                        // `push$ - skip to newline
                        i += 1;
                        while i < n && bytes[i] != b'\n' { i += 1; }
                        continue;
                    }
                    if i + 7 < n && &bytes[i+1..i+8] == b"-> pop$" {
                        // `-> pop$ - skip to newline
                        i += 1;
                        while i < n && bytes[i] != b'\n' { i += 1; }
                        continue;
                    }
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
                    if i >= n { return Err(CloseError{ kind: CloseErrorKind::UnterminatedString, message: "unterminated template".into() }); }
                }
                b'/' if i+1<n && bytes[i+1]==b'/' => { // line comment
                    i += 2; while i<n && bytes[i]!=b'\n' { i+=1; }
                }
                b'/' if i+1<n && bytes[i+1]==b'*' => { // block comment
                    i += 2; while i+1<n { if bytes[i]==b'*' && bytes[i+1]==b'/' { i+=2; break; } i+=1; }
                    if i+1>=n { return Err(CloseError{ kind: CloseErrorKind::UnterminatedComment, message: "unterminated comment".into() }); }
                }
                b'{' => { depth += 1; i += 1; }
                b'}' => { depth -= 1; i += 1; if depth==0 { return Ok(i-1); } }
                _ => { i += 1; }
            }
        }
        Err(CloseError{ kind: CloseErrorKind::UnmatchedBraces, message: "body not closed".into() })
    }
}

// Tests moved to Docker environment: framepiler_test_env/common/test-frames/closers/
