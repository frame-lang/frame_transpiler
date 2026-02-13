//! Pipeline Configuration
//!
//! V4 pure preprocessor configuration.

use crate::frame_c::visitors::TargetLanguage;

/// Compilation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompileMode {
    /// Normal code generation (default)
    #[default]
    Production,
    /// Generate validation stubs/facades
    Facade,
    /// Generate executable wrapper for testing
    Exec,
    /// Just emit spliced handler bodies
    BodyOnly,
    /// Validation only, no code generation
    ValidationOnly,
}

/// Trailer configuration (runtime code to append)
#[derive(Debug, Clone, Default)]
pub struct TrailerConfig {
    /// Whether to emit exec trailer (main function)
    pub emit_exec: bool,
    /// Whether to emit body-only mode
    pub body_only: bool,
    /// Custom trailer code to append
    pub custom_trailer: Option<String>,
}

/// Validation configuration
#[derive(Debug, Clone, Default)]
pub struct ValidationConfig {
    /// Validation level (structural, semantic, full)
    pub level: ValidationLevel,
    /// Output format for validation errors
    pub format: ValidationFormat,
    /// Whether to continue on validation errors
    pub continue_on_error: bool,
}

/// Validation level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ValidationLevel {
    /// Skip validation
    None,
    /// Basic structural validation only
    Structural,
    /// Full semantic validation (default)
    #[default]
    Semantic,
}

/// Validation output format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ValidationFormat {
    /// Human-readable format (default)
    #[default]
    Human,
    /// JSON format
    Json,
    /// IDE-friendly format (file:line:col)
    Ide,
}

/// Complete pipeline configuration
#[derive(Debug, Clone, Default)]
pub struct PipelineConfig {
    /// Target language for code generation
    pub target: TargetLanguage,
    /// Compilation mode
    pub mode: CompileMode,
    /// Trailer configuration
    pub trailers: TrailerConfig,
    /// Validation configuration
    pub validation: ValidationConfig,
    /// Whether to emit debug output
    pub debug: bool,
    /// System name override (for single-system modules)
    pub system_name: Option<String>,
}

impl PipelineConfig {
    /// Create a new production configuration for a target language
    pub fn production(target: TargetLanguage) -> Self {
        Self {
            target,
            mode: CompileMode::Production,
            ..Default::default()
        }
    }

    /// Create a validation-only configuration
    pub fn validation_only(target: TargetLanguage) -> Self {
        Self {
            target,
            mode: CompileMode::ValidationOnly,
            ..Default::default()
        }
    }

    /// Create a facade (validation stub) configuration
    pub fn facade(target: TargetLanguage) -> Self {
        Self {
            target,
            mode: CompileMode::Facade,
            ..Default::default()
        }
    }

    /// Create an exec (executable test) configuration
    pub fn exec(target: TargetLanguage) -> Self {
        Self {
            target,
            mode: CompileMode::Exec,
            trailers: TrailerConfig {
                emit_exec: true,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Load configuration from environment variables
    pub fn from_env(target: TargetLanguage) -> Self {
        let mut config = Self::production(target);

        // Check env flags
        if std::env::var("FRAME_EMIT_EXEC").ok().as_deref() == Some("1") {
            config.mode = CompileMode::Exec;
            config.trailers.emit_exec = true;
        }

        if std::env::var("FRAME_EMIT_BODY_ONLY").ok().as_deref() == Some("1") {
            config.mode = CompileMode::BodyOnly;
            config.trailers.body_only = true;
        }

        if std::env::var("FRAME_TRANSPILER_DEBUG").ok().as_deref() == Some("1") {
            config.debug = true;
        }

        config
    }
}

impl Default for TargetLanguage {
    fn default() -> Self {
        TargetLanguage::Python3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_production_config() {
        let config = PipelineConfig::production(TargetLanguage::Python3);
        assert_eq!(config.mode, CompileMode::Production);
        assert_eq!(config.target, TargetLanguage::Python3);
    }

    #[test]
    fn test_validation_only_config() {
        let config = PipelineConfig::validation_only(TargetLanguage::TypeScript);
        assert_eq!(config.mode, CompileMode::ValidationOnly);
    }

    #[test]
    fn test_exec_config() {
        let config = PipelineConfig::exec(TargetLanguage::Rust);
        assert_eq!(config.mode, CompileMode::Exec);
        assert!(config.trailers.emit_exec);
    }

    #[test]
    fn test_default_config() {
        let config = PipelineConfig::default();
        assert_eq!(config.mode, CompileMode::Production);
    }
}
