use super::{BodyCloserV3, CloseErrorV3, CloseErrorV3Kind};

pub struct BodyCloserCppV3;

impl BodyCloserV3 for BodyCloserCppV3 {
    fn close_byte(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<usize, CloseErrorV3> {
        let mut i = open_brace_index + 1;
        let mut depth: i32 = 1;
        let n = bytes.len();
        while i < n {
            match bytes[i] {
                b'\n' => { i+=1; }
                b'/' if i+1<n && bytes[i+1]==b'/' => { i+=2; while i<n && bytes[i]!=b'\n' { i+=1; } }
                b'/' if i+1<n && bytes[i+1]==b'*' => { i+=2; while i+1<n { if bytes[i]==b'*' && bytes[i+1]==b'/' { i+=2; break; } i+=1; } if i+1>=n { return Err(CloseErrorV3{ kind: CloseErrorV3Kind::UnterminatedComment, message:"unterminated comment".into() }); } }
                b'\'' => { i+=1; while i<n { if bytes[i]==b'\\' { i+=2; continue; } if bytes[i]==b'\'' { i+=1; break; } i+=1; } if i>=n { return Err(CloseErrorV3{ kind: CloseErrorV3Kind::UnterminatedString, message:"unterminated char".into() }); } }
                b'"' => { i+=1; while i<n { if bytes[i]==b'\\' { i+=2; continue; } if bytes[i]==b'"' { i+=1; break; } i+=1; } if i>=n { return Err(CloseErrorV3{ kind: CloseErrorV3Kind::UnterminatedString, message:"unterminated string".into() }); } }
                b'R' if i+1<n && bytes[i+1]==b'"' => { // raw string R"delim( ... )delim"
                    let mut j=i+2; // after R"
                    let mut delim = Vec::new();
                    while j<n && bytes[j]!=b'(' { delim.push(bytes[j]); j+=1; if delim.len()>32 { break; } }
                    if j>=n || bytes[j]!=b'(' { i+=1; continue; } // not a raw string opener
                    j+=1; // inside content
                    // closing )delim"
                    loop {
                        if j>=n { return Err(CloseErrorV3{ kind: CloseErrorV3Kind::UnterminatedRawString, message:"unterminated raw".into() }); }
                        if bytes[j]==b')' {
                            let mut k=j+1; let mut m=0usize; while m<delim.len() && k<n && bytes[k]==delim[m] { k+=1; m+=1; }
                            if m==delim.len() && k<n && bytes[k]==b'"' { i = k+1; break; }
                        }
                        j+=1;
                    }
                }
                b'{' => { depth+=1; i+=1; }
                b'}' => { depth-=1; i+=1; if depth==0 { return Ok(i-1); } }
                _ => { i+=1; }
            }
        }
        Err(CloseErrorV3{ kind: CloseErrorV3Kind::UnmatchedBraces, message: "body not closed".into() })
    }
}

// Tests moved to Docker environment: framepiler_test_env/common/test-frames/v3/closers/
