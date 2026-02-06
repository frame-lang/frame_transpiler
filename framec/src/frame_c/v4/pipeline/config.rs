//! Pipeline Configuration
//!
//! This module replaces the env-flag-based configuration with explicit
//! configuration structures.

use crate::frame_c::visitors::TargetLanguage;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Global counters for tracking V3 vs V4 usage
/// These help us measure progress toward V3 sunset
static V3_COMPILE_COUNT: AtomicUsize = AtomicUsize::new(0);
static V4_COMPILE_COUNT: AtomicUsize = AtomicUsize::new(0);
static V3_FALLBACK_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Codegen backend selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CodegenBackend {
    /// Legacy V3 string-template backend (default for now)
    #[default]
    V3Legacy,
    /// New V4 AST-based backend
    V4Ast,
    /// Try V4 first, fall back to V3 on failure
    V4WithV3Fallback,
}

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
    /// Codegen backend selection (V3 legacy vs V4 AST-based)
    pub backend: CodegenBackend,
    /// Trailer configuration
    pub trailers: TrailerConfig,
    /// Validation configuration
    pub validation: ValidationConfig,
    /// Whether to emit debug output
    pub debug: bool,
    /// System name override (for single-system modules)
    pub system_name: Option<String>,
}

/// Usage statistics for V3/V4 tracking
#[derive(Debug, Clone, Default)]
pub struct UsageStats {
    pub v3_compiles: usize,
    pub v4_compiles: usize,
    pub v3_fallbacks: usize,
}

impl UsageStats {
    /// Calculate percentage of compilations using V4
    pub fn v4_percentage(&self) -> f64 {
        let total = self.v3_compiles + self.v4_compiles;
        if total == 0 {
            0.0
        } else {
            (self.v4_compiles as f64 / total as f64) * 100.0
        }
    }

    /// Check if we're ready to sunset V3 (100% V4 usage)
    pub fn ready_for_v3_sunset(&self) -> bool {
        self.v3_compiles == 0 && self.v3_fallbacks == 0 && self.v4_compiles > 0
    }
}

/// Record a V3 compilation
pub fn record_v3_compile() {
    V3_COMPILE_COUNT.fetch_add(1, Ordering::Relaxed);
}

/// Record a V4 compilation
pub fn record_v4_compile() {
    V4_COMPILE_COUNT.fetch_add(1, Ordering::Relaxed);
}

/// Record a V3 fallback (V4 failed, fell back to V3)
pub fn record_v3_fallback() {
    V3_FALLBACK_COUNT.fetch_add(1, Ordering::Relaxed);
}

/// Get current usage statistics
pub fn get_usage_stats() -> UsageStats {
    UsageStats {
        v3_compiles: V3_COMPILE_COUNT.load(Ordering::Relaxed),
        v4_compiles: V4_COMPILE_COUNT.load(Ordering::Relaxed),
        v3_fallbacks: V3_FALLBACK_COUNT.load(Ordering::Relaxed),
    }
}

/// Reset usage statistics (for testing)
pub fn reset_usage_stats() {
    V3_COMPILE_COUNT.store(0, Ordering::Relaxed);
    V4_COMPILE_COUNT.store(0, Ordering::Relaxed);
    V3_FALLBACK_COUNT.store(0, Ordering::Relaxed);
}

/// Print usage report to stderr
pub fn print_usage_report() {
    let stats = get_usage_stats();
    eprintln!("=== Frame V3/V4 Usage Report ===");
    eprintln!("V3 compilations: {}", stats.v3_compiles);
    eprintln!("V4 compilations: {}", stats.v4_compiles);
    eprintln!("V3 fallbacks:    {}", stats.v3_fallbacks);
    eprintln!("V4 percentage:   {:.1}%", stats.v4_percentage());
    if stats.ready_for_v3_sunset() {
        eprintln!("Status: READY for V3 sunset!");
    } else if stats.v3_compiles > 0 || stats.v3_fallbacks > 0 {
        eprintln!("Status: V3 still in use - {} direct + {} fallbacks",
            stats.v3_compiles, stats.v3_fallbacks);
    }
    eprintln!("================================");
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

    /// Load configuration from environment variables (for backward compatibility)
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

        // V4 backend selection
        // FRAME_USE_V4=1 - Force V4 (fail if not supported)
        // FRAME_USE_V4=fallback - Try V4, fall back to V3
        // FRAME_USE_V4=0 or unset - Use V3 (default during transition)
        match std::env::var("FRAME_USE_V4").ok().as_deref() {
            Some("1") | Some("true") | Some("yes") => {
                config.backend = CodegenBackend::V4Ast;
            }
            Some("fallback") | Some("try") => {
                config.backend = CodegenBackend::V4WithV3Fallback;
            }
            _ => {
                config.backend = CodegenBackend::V3Legacy;
            }
        }

        config
    }

    /// Create a V4-only configuration (for testing new backend)
    pub fn v4_production(target: TargetLanguage) -> Self {
        Self {
            target,
            mode: CompileMode::Production,
            backend: CodegenBackend::V4Ast,
            ..Default::default()
        }
    }

    /// Create a V4-with-fallback configuration (for gradual migration)
    pub fn v4_with_fallback(target: TargetLanguage) -> Self {
        Self {
            target,
            mode: CompileMode::Production,
            backend: CodegenBackend::V4WithV3Fallback,
            ..Default::default()
        }
    }

    /// Check if this config uses V4 backend
    pub fn uses_v4(&self) -> bool {
        matches!(self.backend, CodegenBackend::V4Ast | CodegenBackend::V4WithV3Fallback)
    }

    /// Check if this config allows V3 fallback
    pub fn allows_v3_fallback(&self) -> bool {
        matches!(self.backend, CodegenBackend::V3Legacy | CodegenBackend::V4WithV3Fallback)
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
        assert_eq!(config.backend, CodegenBackend::V3Legacy);
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
        assert_eq!(config.backend, CodegenBackend::V3Legacy);
    }

    #[test]
    fn test_v4_production_config() {
        let config = PipelineConfig::v4_production(TargetLanguage::Python3);
        assert_eq!(config.backend, CodegenBackend::V4Ast);
        assert!(config.uses_v4());
        assert!(!config.allows_v3_fallback());
    }

    #[test]
    fn test_v4_with_fallback_config() {
        let config = PipelineConfig::v4_with_fallback(TargetLanguage::Python3);
        assert_eq!(config.backend, CodegenBackend::V4WithV3Fallback);
        assert!(config.uses_v4());
        assert!(config.allows_v3_fallback());
    }

    #[test]
    fn test_usage_stats() {
        // Reset counters first
        reset_usage_stats();

        // Record some compilations
        record_v3_compile();
        record_v3_compile();
        record_v4_compile();

        let stats = get_usage_stats();
        assert_eq!(stats.v3_compiles, 2);
        assert_eq!(stats.v4_compiles, 1);
        assert_eq!(stats.v3_fallbacks, 0);

        // V4 percentage should be 33.3%
        let pct = stats.v4_percentage();
        assert!(pct > 33.0 && pct < 34.0);

        // Not ready for sunset
        assert!(!stats.ready_for_v3_sunset());

        // Clean up
        reset_usage_stats();
    }

    #[test]
    fn test_ready_for_v3_sunset() {
        reset_usage_stats();

        // Only V4 compilations
        record_v4_compile();
        record_v4_compile();

        let stats = get_usage_stats();
        assert!(stats.ready_for_v3_sunset());
        assert_eq!(stats.v4_percentage(), 100.0);

        reset_usage_stats();
    }

    #[test]
    fn test_fallback_prevents_sunset() {
        reset_usage_stats();

        record_v4_compile();
        record_v3_fallback();

        let stats = get_usage_stats();
        assert!(!stats.ready_for_v3_sunset());

        reset_usage_stats();
    }
}
