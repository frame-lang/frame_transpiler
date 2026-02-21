#[derive(Debug)]
pub enum CloseErrorKind {
    Unimplemented,
    UnterminatedString,
    UnterminatedComment,
    UnterminatedRawString,
    UnmatchedBraces,
}

#[derive(Debug)]
pub struct CloseError {
    pub kind: CloseErrorKind,
    pub message: String,
}

impl CloseError {
    pub fn unimplemented() -> Self {
        CloseError { kind: CloseErrorKind::Unimplemented, message: "Body closer not yet implemented".to_string() }
    }
}

pub trait BodyCloser {
    fn close_byte(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<usize, CloseError>;
}

pub mod python;
pub mod typescript;
pub mod csharp;
pub mod c;
pub mod cpp;
pub mod java;
pub mod rust;
