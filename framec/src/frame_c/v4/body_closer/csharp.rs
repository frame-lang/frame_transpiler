use super::{BodyCloser, CloseError, CloseErrorKind};

pub struct BodyCloserCs;

impl BodyCloser for BodyCloserCs {
    fn close_byte(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<usize, CloseError> {
        let mut i = open_brace_index + 1;
        let mut depth: i32 = 1;
        let n = bytes.len();
        while i < n {
            // Preprocessor at SOL is handled by scanning to end of line when `#` at SOL.
            match bytes[i] {
                b'\n' => { i += 1; }
                b'/' if i+1<n && bytes[i+1]==b'/' => { i+=2; while i<n && bytes[i]!=b'\n' { i+=1; } }
                b'/' if i+1<n && bytes[i+1]==b'*' => {
                    i+=2; while i+1<n { if bytes[i]==b'*' && bytes[i+1]==b'/' { i+=2; break; } i+=1; }
                    if i+1>=n { return Err(CloseError{ kind: CloseErrorKind::UnterminatedComment, message: "unterminated comment".into() }); }
                }
                b'#' => { // preprocessor line
                    while i<n && bytes[i]!=b'\n' { i+=1; }
                }
                b'\'' => { // char literal
                    i+=1; while i<n { if bytes[i]==b'\\' { i+=2; continue; } if bytes[i]==b'\'' { i+=1; break; } i+=1; }
                    if i>=n { return Err(CloseError{ kind: CloseErrorKind::UnterminatedString, message: "unterminated char".into() }); }
                }
                b'@' => {
                    // verbatim or @$" or @""
                    if i+1<n && bytes[i+1]==b'"' { i+=2; // verbatim
                        loop { if i>=n { return Err(CloseError{ kind: CloseErrorKind::UnterminatedString, message: "unterminated verbatim string".into() }); }
                            if i+1<n && bytes[i]==b'"' && bytes[i+1]==b'"' { i+=2; continue; }
                            if bytes[i]==b'"' { i+=1; break; }
                            i+=1;
                        }
                    } else if i+2<n && (bytes[i+1]==b'$' && bytes[i+2]==b'"') { // @$"
                        i+=3; // treat as verbatim + interpolation braces ignored
                        loop { if i>=n { return Err(CloseError{ kind: CloseErrorKind::UnterminatedString, message: "unterminated interp verbatim".into() }); }
                            if i+1<n && bytes[i]==b'"' && bytes[i+1]==b'"' { i+=2; continue; }
                            if bytes[i]==b'"' { i+=1; break; }
                            i+=1;
                        }
                    } else { i+=1; }
                }
                b'$' => {
                    // interpolated $"..." or raw $"""...""" with $ count
                    let mut j=i; let mut _dollars=0; while j<n && bytes[j]==b'$' { _dollars+=1; j+=1; }
                    let mut k=j; let mut quotes=0; while k<n && bytes[k]==b'"' { quotes+=1; k+=1; }
                    if quotes>=3 { // raw string
                        i = k; // inside raw
                        loop {
                            if i>=n { return Err(CloseError{ kind: CloseErrorKind::UnterminatedRawString, message: "unterminated raw".into() }); }
                            // close when we see quotes consecutive == quotes
                            if bytes[i]==b'"' {
                                let mut q=0; let mut p=i; while p<n && bytes[p]==b'"' { q+=1; p+=1; }
                                if q>=quotes { i=p; break; }
                                i=p;
                            } else { i+=1; }
                        }
                    } else if j<n && bytes[j]==b'"' { // $"
                        i = j+1; // after opening quote
                        loop { if i>=n { return Err(CloseError{ kind: CloseErrorKind::UnterminatedString, message: "unterminated interp string".into() }); }
                            if bytes[i]==b'\\' { i+=2; continue; }
                            if bytes[i]==b'"' { i+=1; break; }
                            i+=1;
                        }
                    } else { i+=1; }
                }
                b'"' => { // normal string
                    i+=1; while i<n { if bytes[i]==b'\\' { i+=2; continue; } if bytes[i]==b'"' { i+=1; break; } i+=1; }
                    if i>=n { return Err(CloseError{ kind: CloseErrorKind::UnterminatedString, message: "unterminated string".into() }); }
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
