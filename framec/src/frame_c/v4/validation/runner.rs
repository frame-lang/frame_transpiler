//! Validation runner for Frame V4
//!
//! Orchestrates validation passes and produces a validation report.

use super::pass::{ValidationContext, ValidationPass};
use super::report::ValidationReport;
use super::types::ValidationConfig;
use crate::frame_c::v4::arcanum::Arcanum;
use crate::frame_c::v4::frame_ast::FrameAst;

/// Runs validation passes on Frame AST
pub struct ValidationRunner {
    /// Registered validation passes
    passes: Vec<Box<dyn ValidationPass>>,
    /// Validation configuration
    config: ValidationConfig,
}

impl ValidationRunner {
    /// Create a new validation runner with default passes
    pub fn new(config: ValidationConfig) -> Self {
        Self {
            passes: Vec::new(),
            config,
        }
    }

    /// Create a runner with default configuration
    pub fn default_runner() -> Self {
        Self::new(ValidationConfig::default())
    }

    /// Register a validation pass
    pub fn register_pass<P: ValidationPass + 'static>(&mut self, pass: P) {
        self.passes.push(Box::new(pass));
    }

    /// Get the number of registered passes
    pub fn pass_count(&self) -> usize {
        self.passes.len()
    }

    /// Run all validation passes
    pub fn run(&self, ast: &FrameAst, arcanum: &Arcanum) -> ValidationReport {
        let mut ctx = ValidationContext::new(&self.config);
        let mut all_issues = Vec::new();

        // Run each pass
        for pass in &self.passes {
            // Skip passes that shouldn't run
            if !pass.should_run(ast) {
                continue;
            }

            // Run the pass
            let issues = pass.run(ast, arcanum, &mut ctx);
            all_issues.extend(issues);

            // Check if we should stop (too many errors)
            if ctx.should_stop() {
                break;
            }
        }

        // Build report
        let mut report = ValidationReport::from_issues(all_issues);
        report.filter_by_config(&self.config);
        report
    }

    /// Run validation and return Result
    pub fn validate(&self, ast: &FrameAst, arcanum: &Arcanum) -> Result<(), ValidationReport> {
        let report = self.run(ast, arcanum);
        if report.has_errors() {
            Err(report)
        } else {
            Ok(())
        }
    }
}

/// Builder for ValidationRunner with fluent API
pub struct ValidationRunnerBuilder {
    config: ValidationConfig,
    passes: Vec<Box<dyn ValidationPass>>,
}

impl ValidationRunnerBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            config: ValidationConfig::default(),
            passes: Vec::new(),
        }
    }

    /// Set warnings as errors
    pub fn warnings_as_errors(mut self, value: bool) -> Self {
        self.config.warnings_as_errors = value;
        self
    }

    /// Suppress specific error codes
    pub fn suppress(mut self, codes: Vec<String>) -> Self {
        self.config.suppress = codes;
        self
    }

    /// Enable verbose output
    pub fn verbose(mut self, value: bool) -> Self {
        self.config.verbose = value;
        self
    }

    /// Set max errors
    pub fn max_errors(mut self, max: usize) -> Self {
        self.config.max_errors = max;
        self
    }

    /// Add a validation pass
    pub fn with_pass<P: ValidationPass + 'static>(mut self, pass: P) -> Self {
        self.passes.push(Box::new(pass));
        self
    }

    /// Build the runner
    pub fn build(self) -> ValidationRunner {
        ValidationRunner {
            passes: self.passes,
            config: self.config,
        }
    }
}

impl Default for ValidationRunnerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame_c::v4::frame_ast::{FrameAst, Span, SystemAst};
    use crate::frame_c::v4::validation::types::ValidationIssue;

    struct AlwaysErrorPass;

    impl ValidationPass for AlwaysErrorPass {
        fn name(&self) -> &'static str {
            "always_error"
        }

        fn run(
            &self,
            _ast: &FrameAst,
            _arcanum: &Arcanum,
            _ctx: &mut ValidationContext,
        ) -> Vec<ValidationIssue> {
            vec![ValidationIssue::error("E999", "Test error")]
        }
    }

    struct AlwaysWarningPass;

    impl ValidationPass for AlwaysWarningPass {
        fn name(&self) -> &'static str {
            "always_warning"
        }

        fn run(
            &self,
            _ast: &FrameAst,
            _arcanum: &Arcanum,
            _ctx: &mut ValidationContext,
        ) -> Vec<ValidationIssue> {
            vec![ValidationIssue::warning("W999", "Test warning")]
        }
    }

    struct NeverRunPass;

    impl ValidationPass for NeverRunPass {
        fn name(&self) -> &'static str {
            "never_run"
        }

        fn should_run(&self, _ast: &FrameAst) -> bool {
            false
        }

        fn run(
            &self,
            _ast: &FrameAst,
            _arcanum: &Arcanum,
            _ctx: &mut ValidationContext,
        ) -> Vec<ValidationIssue> {
            vec![ValidationIssue::error("E000", "Should never appear")]
        }
    }

    fn empty_ast() -> FrameAst {
        FrameAst::System(SystemAst::new("Test".to_string(), Span::new(0, 10)))
    }

    fn empty_arcanum() -> Arcanum {
        Arcanum::new()
    }

    #[test]
    fn test_runner_with_error_pass() {
        let mut runner = ValidationRunner::new(ValidationConfig::default());
        runner.register_pass(AlwaysErrorPass);

        let report = runner.run(&empty_ast(), &empty_arcanum());
        assert!(report.has_errors());
        assert_eq!(report.summary().errors, 1);
    }

    #[test]
    fn test_runner_with_warning_pass() {
        let mut runner = ValidationRunner::new(ValidationConfig::default());
        runner.register_pass(AlwaysWarningPass);

        let report = runner.run(&empty_ast(), &empty_arcanum());
        assert!(!report.has_errors());
        assert_eq!(report.summary().warnings, 1);
    }

    #[test]
    fn test_runner_skips_disabled_pass() {
        let mut runner = ValidationRunner::new(ValidationConfig::default());
        runner.register_pass(NeverRunPass);

        let report = runner.run(&empty_ast(), &empty_arcanum());
        assert!(!report.has_errors());
        assert_eq!(report.summary().total(), 0);
    }

    #[test]
    fn test_runner_warnings_as_errors() {
        let config = ValidationConfig::strict();
        let mut runner = ValidationRunner::new(config);
        runner.register_pass(AlwaysWarningPass);

        let report = runner.run(&empty_ast(), &empty_arcanum());
        assert!(report.has_errors());
        assert_eq!(report.summary().errors, 1);
        assert_eq!(report.summary().warnings, 0);
    }

    #[test]
    fn test_runner_builder() {
        let runner = ValidationRunnerBuilder::new()
            .warnings_as_errors(true)
            .suppress(vec!["E999".to_string()])
            .max_errors(10)
            .with_pass(AlwaysErrorPass)
            .with_pass(AlwaysWarningPass)
            .build();

        assert_eq!(runner.pass_count(), 2);
    }

    #[test]
    fn test_validate_result() {
        let mut runner = ValidationRunner::new(ValidationConfig::default());
        runner.register_pass(AlwaysErrorPass);

        let result = runner.validate(&empty_ast(), &empty_arcanum());
        assert!(result.is_err());
    }
}
