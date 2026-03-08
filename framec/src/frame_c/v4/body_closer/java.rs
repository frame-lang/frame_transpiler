use super::{BodyCloser, CloseError, CloseErrorKind};

pub struct BodyCloserJava;

impl BodyCloser for BodyCloserJava {
    fn close_byte(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<usize, CloseError> {
        let mut i = open_brace_index + 1;
        let mut depth: i32 = 1;
        let n = bytes.len();
        while i < n {
            match bytes[i] {
                b'\n' => { i+=1; }
                b'/' if i+1<n && bytes[i+1]==b'/' => { i+=2; while i<n && bytes[i]!=b'\n' { i+=1; } }
                b'/' if i+1<n && bytes[i+1]==b'*' => { i+=2; while i+1<n { if bytes[i]==b'*' && bytes[i+1]==b'/' { i+=2; break; } i+=1; } if i+1>=n { return Err(CloseError{ kind: CloseErrorKind::UnterminatedComment, message:"unterminated comment".into() }); } }
                b'\'' => { i+=1; while i<n { if bytes[i]==b'\\' { i+=2; continue; } if bytes[i]==b'\'' { i+=1; break; } i+=1; } if i>=n { return Err(CloseError{ kind: CloseErrorKind::UnterminatedString, message:"unterminated char".into() }); } }
                b'"' => { i+=1; while i<n { if bytes[i]==b'\\' { i+=2; continue; } if bytes[i]==b'"' { i+=1; break; } i+=1; } if i>=n { return Err(CloseError{ kind: CloseErrorKind::UnterminatedString, message:"unterminated string".into() }); } }
                b'{' => { depth+=1; i+=1; }
                b'}' => { depth-=1; i+=1; if depth==0 { return Ok(i-1); } }
                _ => { i+=1; }
            }
        }
        Err(CloseError{ kind: CloseErrorKind::UnmatchedBraces, message: "body not closed".into() })
    }
}

// Tests moved to Docker environment: framepiler_test_env/common/test-frames/closers/
