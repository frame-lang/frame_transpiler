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

use crate::frame_c::visitors::TargetLanguage;

/// Single dispatch point for language-specific body closers.
/// Given the full byte slice, the position of the opening `{`, and the target language,
/// returns the absolute position of the matching closing `}`.
pub fn close_body(bytes: &[u8], open: usize, lang: TargetLanguage) -> Result<usize, CloseError> {
    match lang {
        TargetLanguage::Python3 => python::BodyCloserPy.close_byte(&bytes[open..], 0).map(|c| open + c),
        TargetLanguage::TypeScript => typescript::BodyCloserTs.close_byte(&bytes[open..], 0).map(|c| open + c),
        TargetLanguage::CSharp => csharp::BodyCloserCs.close_byte(&bytes[open..], 0).map(|c| open + c),
        TargetLanguage::C => c::BodyCloserC.close_byte(&bytes[open..], 0).map(|c| open + c),
        TargetLanguage::Cpp => cpp::BodyCloserCpp.close_byte(&bytes[open..], 0).map(|c| open + c),
        TargetLanguage::Java => java::BodyCloserJava.close_byte(&bytes[open..], 0).map(|c| open + c),
        TargetLanguage::Rust => rust::BodyCloserRust.close_byte(&bytes[open..], 0).map(|c| open + c),
        _ => Err(CloseError { kind: CloseErrorKind::Unimplemented, message: "unsupported language".into() }),
    }
}
