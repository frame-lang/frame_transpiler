// Frame Transpiler Validation System
// Comprehensive validation framework for Frame language syntax and semantics

pub mod analysis;
pub mod engine;
pub mod reporters;
pub mod rules;
pub mod targets;

use crate::frame_c::ast::*;
use crate::frame_c::symbol_table::SymbolTable;
use std::path::Path;

/// Core validation engine that orchestrates all validation rules
pub struct ValidationEngine {
    pub rules: Vec<Box<dyn ValidationRule>>,
    pub reporters: Vec<Box<dyn ValidationReporter>>,
    pub config: ValidationConfig,
}

/// Configuration for validation behavior
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    pub level: ValidationLevel,
    pub target_language: Option<TargetLanguage>,
    pub output_format: OutputFormat,
    pub fail_on_warnings: bool,
    pub max_errors: Option<usize>,
}

/// Validation levels - progressive validation depth
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValidationLevel {
    Basic = 1,          // Level 1: Syntax only
    Structural = 2,     // Level 2: + Frame structure
    Semantic = 3,       // Level 3: + Semantic analysis
    TargetLanguage = 4, // Level 4: + Generated code
}

/// Target languages for validation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetLanguage {
    Python,
    TypeScript,
    Java,
    CSharp,
    Rust,
    Cpp,
}

/// Output formats for validation results
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Human, // Human-readable text
    Json,  // Structured JSON
    Junit, // JUnit XML for CI/CD
    Sarif, // SARIF format for security tools
}

/// Context provided to validation rules
pub struct ValidationContext<'a> {
    pub ast: &'a SystemNode,
    pub source_code: &'a str,
    pub file_path: &'a Path,
    pub target_language: Option<TargetLanguage>,
    pub generated_code: Option<&'a str>,
    pub symbol_table: Option<&'a SymbolTable>,
}

/// Individual validation rule trait
pub trait ValidationRule: Send + Sync {
    fn name(&self) -> &str;
    fn level(&self) -> ValidationLevel;
    fn validate(&self, context: &ValidationContext) -> Vec<ValidationIssue>;
    fn is_enabled(&self, config: &ValidationConfig) -> bool {
        config.level >= self.level()
    }
}

/// Validation result reporter trait
pub trait ValidationReporter: Send + Sync {
    fn format(&self) -> OutputFormat;
    fn report(&self, results: &ValidationResult) -> String;
}

/// Complete validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub file_path: String,
    pub level: ValidationLevel,
    pub issues: Vec<ValidationIssue>,
    pub metrics: ValidationMetrics,
    pub success: bool,
    pub duration_ms: u64,
}

/// Individual validation issue
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub severity: Severity,
    pub category: Category,
    pub rule_name: String,
    pub message: String,
    pub location: SourceLocation,
    pub suggestion: Option<String>,
    pub help_url: Option<String>,
}

/// Issue severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Error,   // Validation fails
    Warning, // Potential issue
    Info,    // Informational
    Hint,    // Optimization suggestion
}

/// Issue categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Syntax,         // Basic syntax errors
    Structure,      // Frame structure issues
    Semantic,       // Logic/flow issues
    TargetLanguage, // Generated code issues
    Performance,    // Optimization suggestions
    Style,          // Code style issues
}

/// Validation metrics and statistics
#[derive(Debug, Clone, Default)]
pub struct ValidationMetrics {
    pub rules_executed: usize,
    pub errors: usize,
    pub warnings: usize,
    pub hints: usize,
    pub lines_validated: usize,
    pub states_analyzed: usize,
    pub events_analyzed: usize,
    pub complexity_score: Option<f64>,
}

/// Source location for error reporting
#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub line: u32,
    pub column: u32,
    pub offset: usize,
    pub length: usize,
    pub file_path: Option<String>,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            level: ValidationLevel::Structural,
            target_language: None,
            output_format: OutputFormat::Human,
            fail_on_warnings: false,
            max_errors: Some(100),
        }
    }
}

impl ValidationResult {
    pub fn new(file_path: String, level: ValidationLevel) -> Self {
        Self {
            file_path,
            level,
            issues: Vec::new(),
            metrics: ValidationMetrics::default(),
            success: true,
            duration_ms: 0,
        }
    }

    pub fn add_issue(&mut self, issue: ValidationIssue) {
        match issue.severity {
            Severity::Error => {
                self.metrics.errors += 1;
                self.success = false;
            }
            Severity::Warning => self.metrics.warnings += 1,
            Severity::Info => {}
            Severity::Hint => self.metrics.hints += 1,
        }
        self.issues.push(issue);
    }

    pub fn has_errors(&self) -> bool {
        self.metrics.errors > 0
    }

    pub fn has_warnings(&self) -> bool {
        self.metrics.warnings > 0
    }

    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }
}

// TokenLocation implementation removed - not available in current scanner
