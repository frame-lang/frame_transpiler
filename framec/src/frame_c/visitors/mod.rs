// Minimal visitors module retained for V3 rebuild.
// Only TargetLanguage remains to support CLI argument parsing.

use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum TargetLanguage {
    Python3,
    TypeScript,
    Graphviz,
    LLVM,
    C,
    Cpp,
    Java,
    CSharp,
    Rust,
}

impl TargetLanguage {
    pub fn file_extension(&self) -> &'static str {
        match self {
            TargetLanguage::Python3 => "py",
            TargetLanguage::TypeScript => "ts",
            TargetLanguage::Graphviz => "graphviz",
            TargetLanguage::LLVM => "ll",
            TargetLanguage::C => "c",
            TargetLanguage::Cpp => "cpp",
            TargetLanguage::Java => "java",
            TargetLanguage::CSharp => "cs",
            TargetLanguage::Rust => "rs",
        }
    }
}

impl TryFrom<&str> for TargetLanguage {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let normalized = value.to_ascii_lowercase();
        if normalized == "python_3" || normalized == "python" {
            Ok(TargetLanguage::Python3)
        } else if normalized == "typescript" || normalized == "ts" {
            Ok(TargetLanguage::TypeScript)
        } else if normalized == "graphviz" {
            Ok(TargetLanguage::Graphviz)
        } else if normalized == "llvm" {
            Ok(TargetLanguage::LLVM)
        } else if normalized == "c" {
            Ok(TargetLanguage::C)
        } else if normalized == "c++" || normalized == "cpp" {
            Ok(TargetLanguage::Cpp)
        } else if normalized == "java" {
            Ok(TargetLanguage::Java)
        } else if normalized == "csharp" || normalized == "c#" || normalized == "cs" {
            Ok(TargetLanguage::CSharp)
        } else if normalized == "rust" || normalized == "rs" {
            Ok(TargetLanguage::Rust)
        } else {
            Err(format!(
                "Unrecognized target language: {}. Supported languages are: python_3 (python), typescript (ts), graphviz, llvm, c, c++, java, csharp, rust",
                normalized
            ))
        }
    }
}

impl TryFrom<String> for TargetLanguage {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

