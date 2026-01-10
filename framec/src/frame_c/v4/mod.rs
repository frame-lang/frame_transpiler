// Frame v4 - Pure Preprocessor Implementation
//
// Frame v4 is a complete rewrite that treats Frame as a preprocessing tool
// that generates native code for state machines. No MIR, no MixedBody,
// no runtime libraries - just clean code generation.

pub mod ast;
pub mod scanner;
pub mod parser;
pub mod validator;
pub mod codegen;
pub mod annotations;
pub mod error;
pub mod mir;
pub mod native_scanner;
pub mod compiler_v3_adapter;
pub mod v3_based_compiler;
pub mod v3_direct_compiler;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod test_simple;

use self::error::ErrorsAcc;

/// Result type for v4 compilation
pub enum FrameV4Result {
    Ok(FrameV4Output),
    Err(ErrorsAcc),
}

/// Output from v4 compilation
pub struct FrameV4Output {
    pub code: String,
    pub source_map: Option<String>,
    pub warnings: Vec<String>,
}

/// Main entry point for v4 compilation
pub struct FrameV4Compiler {
    target: TargetLanguage,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TargetLanguage {
    Python,
    TypeScript,
    Rust,
    C,
    Cpp,
    Java,
    CSharp,
    Go,
}

impl From<crate::frame_c::visitors::TargetLanguage> for TargetLanguage {
    fn from(v3: crate::frame_c::visitors::TargetLanguage) -> Self {
        match v3 {
            crate::frame_c::visitors::TargetLanguage::Python3 => TargetLanguage::Python,
            crate::frame_c::visitors::TargetLanguage::TypeScript => TargetLanguage::TypeScript,
            crate::frame_c::visitors::TargetLanguage::Rust => TargetLanguage::Rust,
            crate::frame_c::visitors::TargetLanguage::C => TargetLanguage::C,
            crate::frame_c::visitors::TargetLanguage::Cpp => TargetLanguage::Cpp,
            crate::frame_c::visitors::TargetLanguage::Java => TargetLanguage::Java,
            crate::frame_c::visitors::TargetLanguage::CSharp => TargetLanguage::CSharp,
            _ => TargetLanguage::Python, // Default for unsupported
        }
    }
}

impl FrameV4Compiler {
    pub fn new(target: TargetLanguage) -> Self {
        Self { target }
    }

    /// Compile Frame source to native code
    pub fn compile(&self, source: &str, file_path: &str) -> FrameV4Result {
        // Check which implementation to use
        if std::env::var("USE_V3_DIRECT").is_ok() {
            // Use v3's complete compile_module directly - gives full v3 functionality
            let compiler = v3_direct_compiler::V3DirectCompiler::new(self.target);
            compiler.compile(source, file_path)
        } else if std::env::var("USE_V3_PARSERS").is_ok() {
            // Use v3's specialized parsers (system_parser, machine_parser, etc.) with v4 code generation
            let compiler = v3_based_compiler::V3BasedCompiler::new(self.target);
            compiler.compile(source, file_path)
        } else if std::env::var("USE_V3_ADAPTER").is_ok() {
            // Use v3 adapter with v3's proven parsers but v4 code generation
            let adapter = compiler_v3_adapter::FrameV4CompilerAdapter::new(self.target);
            adapter.compile(source, file_path)
        } else {
            // Default to v3 direct for now since it's the only one that fully works
            let compiler = v3_direct_compiler::V3DirectCompiler::new(self.target);
            compiler.compile(source, file_path)
        }
    }
    
    fn compile_v4_pure(&self, source: &str, file_path: &str) -> FrameV4Result {
        // 1. Scan and tokenize
        let tokens = match scanner::scan(source, file_path) {
            Ok(tokens) => tokens,
            Err(e) => return FrameV4Result::Err(e),
        };

        // 2. Parse to AST with MIR support
        let ast = match parser::parse(tokens, source, self.target) {
            Ok(ast) => ast,
            Err(e) => return FrameV4Result::Err(e),
        };

        // 3. Validate Frame structure
        if let Err(e) = validator::validate(&ast) {
            return FrameV4Result::Err(e);
        }

        // 4. Generate native code
        let code = match codegen::generate(&ast, self.target) {
            Ok(code) => code,
            Err(e) => return FrameV4Result::Err(e),
        };

        // 5. Optional: Generate source map
        let source_map = if std::env::var("FRAME_SOURCE_MAP").is_ok() {
            Some(codegen::generate_source_map(&ast, &code, file_path))
        } else {
            None
        };

        FrameV4Result::Ok(FrameV4Output {
            code,
            source_map,
            warnings: Vec::new(),
        })
    }

    /// Detect target language from pragma or extension
    pub fn detect_target(source: &str, file_path: &str) -> Result<TargetLanguage, String> {
        // First check for @@target pragma
        if let Some(target) = Self::parse_target_pragma(source) {
            return Ok(target);
        }

        // Fall back to extension
        match std::path::Path::new(file_path)
            .extension()
            .and_then(|s| s.to_str())
        {
            Some("fpy") => Ok(TargetLanguage::Python),
            Some("frts") => Ok(TargetLanguage::TypeScript),
            Some("frs") => Ok(TargetLanguage::Rust),
            Some("fc") => Ok(TargetLanguage::C),
            Some("fcpp") => Ok(TargetLanguage::Cpp),
            Some("fjava") => Ok(TargetLanguage::Java),
            Some("frcs") => Ok(TargetLanguage::CSharp),
            Some("fgo") => Ok(TargetLanguage::Go),
            Some("frm") => {
                eprintln!("Warning: .frm extension is deprecated. Use language-specific extensions.");
                Err("No @@target pragma found and .frm extension is ambiguous".to_string())
            }
            _ => Err(format!("Unknown file extension and no @@target pragma: {}", file_path)),
        }
    }

    fn parse_target_pragma(source: &str) -> Option<TargetLanguage> {
        // Look for @@target at start of file
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("@@target ") {
                let target = trimmed.trim_start_matches("@@target ").trim();
                return match target {
                    "python" | "python_3" => Some(TargetLanguage::Python),
                    "typescript" => Some(TargetLanguage::TypeScript),
                    "rust" => Some(TargetLanguage::Rust),
                    "c" => Some(TargetLanguage::C),
                    "cpp" | "c++" => Some(TargetLanguage::Cpp),
                    "java" => Some(TargetLanguage::Java),
                    "csharp" | "c#" => Some(TargetLanguage::CSharp),
                    "go" => Some(TargetLanguage::Go),
                    _ => None,
                };
            }
            // Stop looking after non-empty, non-comment lines
            if !trimmed.is_empty() && !trimmed.starts_with('#') && !trimmed.starts_with("//") {
                break;
            }
        }
        None
    }
}