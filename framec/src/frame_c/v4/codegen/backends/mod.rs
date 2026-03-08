//! Language-specific backend implementations
//!
//! Each backend implements the LanguageBackend trait to generate code
//! for a specific target language.

pub mod python;
pub mod typescript;
pub mod rust;
pub mod csharp;
pub mod c;
pub mod cpp;
pub mod java;
