use super::{BodyCloser, CloseError, CloseErrorKind};

pub struct BodyCloserRust;

impl BodyCloser for BodyCloserRust {
    fn close_byte(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<usize, CloseError> {
        let mut i = open_brace_index + 1;
        let mut depth: i32 = 1;
        let n = bytes.len();
        let mut block_comment_nest: i32 = 0;
        while i < n {
            if block_comment_nest > 0 {
                if i+1<n && bytes[i]==b'/' && bytes[i+1]==b'*' { block_comment_nest+=1; i+=2; continue; }
                if i+1<n && bytes[i]==b'*' && bytes[i+1]==b'/' { block_comment_nest-=1; i+=2; continue; }
                i+=1; continue;
            }
            match bytes[i] {
                b'\n' => { i+=1; }
                b'/' if i+1<n && bytes[i+1]==b'/' => { i+=2; while i<n && bytes[i]!=b'\n' { i+=1; } }
                b'/' if i+1<n && bytes[i+1]==b'*' => { block_comment_nest=1; i+=2; }
                b'\'' => { // char or lifetime; disambiguate by next char
                    // treat as char if next is not whitespace or apostrophe alone
                    let _j=i+1; // char literal cases: '\'' or 'a'
                    i+=1; while i<n { if bytes[i]==b'\\' { i+=2; continue; } if bytes[i]==b'\'' { i+=1; break; } i+=1; }
                    if i>=n { return Err(CloseError{ kind: CloseErrorKind::UnterminatedString, message:"unterminated char".into() }); }
                }
                b'"' => { i+=1; while i<n { if bytes[i]==b'\\' { i+=2; continue; } if bytes[i]==b'"' { i+=1; break; } i+=1; } if i>=n { return Err(CloseError{ kind: CloseErrorKind::UnterminatedString, message:"unterminated string".into() }); } }
                b'r' => { // raw string r#"..."# or br#"..."#
                    let mut j=i+1; let mut hashes=0usize; if j<n && bytes[j]==b'#' { while j<n && bytes[j]==b'#' { hashes+=1; j+=1; } }
                    if j<n && bytes[j]==b'"' { // raw string
                        j+=1; // inside
                        loop { if j>=n { return Err(CloseError{ kind: CloseErrorKind::UnterminatedRawString, message:"unterminated raw".into() }); }
                            if bytes[j]==b'"' {
                                let mut k=j+1; let mut m=0usize; while m<hashes && k<n && bytes[k]==b'#' { m+=1; k+=1; }
                                if m==hashes { i = k; break; }
                            }
                            j+=1;
                        }
                    } else { i+=1; continue; }
                }
                b'{' => { depth+=1; i+=1; }
                b'}' => { depth-=1; i+=1; if depth==0 { return Ok(i-1); } }
                _ => { i+=1; }
            }
        }
        Err(CloseError{ kind: CloseErrorKind::UnmatchedBraces, message: "body not closed".into() })
    }
}

// Tests moved to Docker environment: framepiler_test_env/common/test-frames/closers/
