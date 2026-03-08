//! Validation types for Frame V4
//!
//! Core types used throughout the validation system.

use crate::frame_c::v4::frame_ast::Span;

/// Severity of a validation issue
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// Informational only - does not affect compilation
    Info,
    /// Warning - reported but compilation continues
    Warning,
    /// Error - blocks compilation
    Error,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Info => "info",
            Severity::Warning => "warning",
            Severity::Error => "error",
        }
    }
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A validation issue (error, warning, or info)
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    /// Error code (e.g., "E402", "W410")
    pub code: String,
    /// Severity level
    pub severity: Severity,
    /// Human-readable message
    pub message: String,
    /// Location in source
    pub span: Option<Span>,
    /// Additional context notes
    pub notes: Vec<String>,
    /// Suggested fix
    pub fix_hint: Option<String>,
}

impl ValidationIssue {
    /// Create a new error
    pub fn error(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: code.to_string(),
            severity: Severity::Error,
            message: message.into(),
            span: None,
            notes: Vec::new(),
            fix_hint: None,
        }
    }

    /// Create a new warning
    pub fn warning(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: code.to_string(),
            severity: Severity::Warning,
            message: message.into(),
            span: None,
            notes: Vec::new(),
            fix_hint: None,
        }
    }

    /// Create a new info
    pub fn info(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: code.to_string(),
            severity: Severity::Info,
            message: message.into(),
            span: None,
            notes: Vec::new(),
            fix_hint: None,
        }
    }

    /// Add source span
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    /// Add a context note
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Add a fix hint
    pub fn with_fix(mut self, hint: impl Into<String>) -> Self {
        self.fix_hint = Some(hint.into());
        self
    }

    /// Check if this is an error
    pub fn is_error(&self) -> bool {
        self.severity == Severity::Error
    }

    /// Check if this is a warning
    pub fn is_warning(&self) -> bool {
        self.severity == Severity::Warning
    }

    /// Check if this is info
    pub fn is_info(&self) -> bool {
        self.severity == Severity::Info
    }
}

/// Configuration for validation
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    /// Treat warnings as errors
    pub warnings_as_errors: bool,
    /// Suppress specific error codes
    pub suppress: Vec<String>,
    /// Enable verbose output (show info-level issues)
    pub verbose: bool,
    /// Maximum errors before stopping (0 = unlimited)
    pub max_errors: usize,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            warnings_as_errors: false,
            suppress: Vec::new(),
            verbose: false,
            max_errors: 0,
        }
    }
}

impl ValidationConfig {
    /// Create config that treats warnings as errors
    pub fn strict() -> Self {
        Self {
            warnings_as_errors: true,
            ..Default::default()
        }
    }

    /// Check if an issue should be suppressed
    pub fn is_suppressed(&self, code: &str) -> bool {
        self.suppress.iter().any(|s| s == code)
    }

    /// Adjust severity based on config
    pub fn adjust_severity(&self, severity: Severity) -> Severity {
        if self.warnings_as_errors && severity == Severity::Warning {
            Severity::Error
        } else {
            severity
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_issue_creation() {
        let issue = ValidationIssue::error("E402", "Unknown state 'Foo'")
            .with_span(Span::new(10, 20))
            .with_note("State 'Foo' is not defined in this system")
            .with_fix("Add state $Foo { } to the machine section");

        assert!(issue.is_error());
        assert_eq!(issue.code, "E402");
        assert!(issue.span.is_some());
        assert_eq!(issue.notes.len(), 1);
        assert!(issue.fix_hint.is_some());
    }

    #[test]
    fn test_severity_order() {
        assert!(Severity::Info < Severity::Warning);
        assert!(Severity::Warning < Severity::Error);
    }

    #[test]
    fn test_config_suppress() {
        let config = ValidationConfig {
            suppress: vec!["E410".to_string(), "E411".to_string()],
            ..Default::default()
        };

        assert!(config.is_suppressed("E410"));
        assert!(config.is_suppressed("E411"));
        assert!(!config.is_suppressed("E402"));
    }

    #[test]
    fn test_config_warnings_as_errors() {
        let config = ValidationConfig::strict();

        assert_eq!(config.adjust_severity(Severity::Warning), Severity::Error);
        assert_eq!(config.adjust_severity(Severity::Error), Severity::Error);
        assert_eq!(config.adjust_severity(Severity::Info), Severity::Info);
    }
}
