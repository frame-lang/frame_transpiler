//! Validation pass trait for Frame V4
//!
//! Each validation pass implements this trait to perform a specific category
//! of validation checks.

use super::types::{ValidationConfig, ValidationIssue};
use crate::frame_c::v4::arcanum::Arcanum;
use crate::frame_c::v4::frame_ast::FrameAst;

/// Context passed to validation passes
pub struct ValidationContext<'a> {
    /// Current system name being validated
    pub system_name: Option<String>,
    /// Validation configuration
    pub config: &'a ValidationConfig,
    /// Accumulated issues from previous passes
    pub issues: Vec<ValidationIssue>,
}

impl<'a> ValidationContext<'a> {
    /// Create a new validation context
    pub fn new(config: &'a ValidationConfig) -> Self {
        Self {
            system_name: None,
            config,
            issues: Vec::new(),
        }
    }

    /// Set the current system name
    pub fn with_system(mut self, name: &str) -> Self {
        self.system_name = Some(name.to_string());
        self
    }

    /// Add an issue to the context
    pub fn add_issue(&mut self, issue: ValidationIssue) {
        // Check if suppressed
        if self.config.is_suppressed(&issue.code) {
            return;
        }

        // Adjust severity based on config
        let mut issue = issue;
        issue.severity = self.config.adjust_severity(issue.severity);

        self.issues.push(issue);
    }

    /// Check if we should stop validation (too many errors)
    pub fn should_stop(&self) -> bool {
        if self.config.max_errors == 0 {
            return false;
        }
        self.issues.iter().filter(|i| i.is_error()).count() >= self.config.max_errors
    }

    /// Get current error count
    pub fn error_count(&self) -> usize {
        self.issues.iter().filter(|i| i.is_error()).count()
    }

    /// Get current warning count
    pub fn warning_count(&self) -> usize {
        self.issues.iter().filter(|i| i.is_warning()).count()
    }
}

/// A validation pass that performs a specific category of checks
pub trait ValidationPass: Send + Sync {
    /// Name of this pass (for logging/debugging)
    fn name(&self) -> &'static str;

    /// Run validation on the AST
    ///
    /// Returns a list of validation issues found by this pass.
    /// The pass should NOT filter issues based on config - that's done by the runner.
    fn run(
        &self,
        ast: &FrameAst,
        arcanum: &Arcanum,
        ctx: &mut ValidationContext,
    ) -> Vec<ValidationIssue>;

    /// Check if this pass should run
    ///
    /// Override this to skip passes that aren't relevant for certain ASTs.
    /// Default implementation always returns true.
    fn should_run(&self, _ast: &FrameAst) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame_c::v4::frame_ast::Span;

    struct TestPass;

    impl ValidationPass for TestPass {
        fn name(&self) -> &'static str {
            "test"
        }

        fn run(
            &self,
            _ast: &FrameAst,
            _arcanum: &Arcanum,
            _ctx: &mut ValidationContext,
        ) -> Vec<ValidationIssue> {
            vec![
                ValidationIssue::error("E999", "Test error").with_span(Span::new(0, 10)),
                ValidationIssue::warning("W999", "Test warning"),
            ]
        }
    }

    #[test]
    fn test_context_add_issue() {
        let config = ValidationConfig::default();
        let mut ctx = ValidationContext::new(&config);

        ctx.add_issue(ValidationIssue::error("E402", "Test"));
        ctx.add_issue(ValidationIssue::warning("W410", "Test"));

        assert_eq!(ctx.error_count(), 1);
        assert_eq!(ctx.warning_count(), 1);
    }

    #[test]
    fn test_context_suppress() {
        let config = ValidationConfig {
            suppress: vec!["E402".to_string()],
            ..Default::default()
        };
        let mut ctx = ValidationContext::new(&config);

        ctx.add_issue(ValidationIssue::error("E402", "Suppressed"));
        ctx.add_issue(ValidationIssue::error("E403", "Not suppressed"));

        assert_eq!(ctx.error_count(), 1);
        assert_eq!(ctx.issues[0].code, "E403");
    }

    #[test]
    fn test_context_should_stop() {
        let config = ValidationConfig {
            max_errors: 2,
            ..Default::default()
        };
        let mut ctx = ValidationContext::new(&config);

        ctx.add_issue(ValidationIssue::error("E001", "First"));
        assert!(!ctx.should_stop());

        ctx.add_issue(ValidationIssue::error("E002", "Second"));
        assert!(ctx.should_stop());
    }
}
