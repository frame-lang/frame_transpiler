#[derive(Debug)]
pub enum CloseErrorV3Kind {
    Unimplemented,
    UnterminatedString,
    UnterminatedComment,
    UnterminatedRawString,
    UnmatchedBraces,
}

#[derive(Debug)]
pub struct CloseErrorV3 {
    pub kind: CloseErrorV3Kind,
    pub message: String,
}

impl CloseErrorV3 {
    pub fn unimplemented() -> Self {
        CloseErrorV3 { kind: CloseErrorV3Kind::Unimplemented, message: "Body closer not yet implemented".to_string() }
    }
}

pub trait BodyCloserV3 {
    fn close_byte(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<usize, CloseErrorV3>;
}

pub mod python;
pub mod typescript;
pub mod csharp;
pub mod c;
pub mod cpp;
pub mod java;
pub mod rust;
