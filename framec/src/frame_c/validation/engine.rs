// Validation Engine Implementation
// Core orchestration of validation rules and reporting

use super::*;
use std::time::Instant;

impl ValidationEngine {
    pub fn new(config: ValidationConfig) -> Self {
        Self {
            rules: Vec::new(),
            reporters: Vec::new(),
            config,
        }
    }

    /// Add a validation rule to the engine
    pub fn add_rule<R: ValidationRule + 'static>(mut self, rule: R) -> Self {
        self.rules.push(Box::new(rule));
        self
    }

    /// Add a reporter to the engine
    pub fn add_reporter<R: ValidationReporter + 'static>(mut self, reporter: R) -> Self {
        self.reporters.push(Box::new(reporter));
        self
    }

    /// Create engine with default rules for the specified level
    pub fn with_default_rules(config: ValidationConfig) -> Self {
        let mut engine = Self::new(config);

        // Add rules based on validation level
        if engine.config.level >= ValidationLevel::Basic {
            engine =
                engine.add_rule(crate::frame_c::validation::rules::MalformedHandlerRule::new());
            engine = engine.add_rule(crate::frame_c::validation::rules::UnmatchedBracesRule::new());
        }

        if engine.config.level >= ValidationLevel::Structural {
            engine = engine
                .add_rule(crate::frame_c::validation::rules::InterfaceCompletenessRule::new());
            engine =
                engine.add_rule(crate::frame_c::validation::rules::StateReachabilityRule::new());
            // Enforce native-only policy inside Python bodies (actions/ops/handlers)
            engine = engine
                .add_rule(crate::frame_c::validation::rules::PythonNativePolicyRule::new());
        }

        if engine.config.level >= ValidationLevel::Semantic {
            engine =
                engine.add_rule(crate::frame_c::validation::rules::EventFlowAnalysisRule::new());
            engine = engine
                .add_rule(crate::frame_c::validation::rules::ReturnValueConsistencyRule::new());
            engine = engine
                .add_rule(crate::frame_c::validation::rules::async_consistency::AsyncConsistencyRule::new());
        }

        // Add default reporter
        engine = engine.add_reporter(crate::frame_c::validation::reporters::HumanReporter::new());

        engine
    }

    /// Validate a Frame system with the configured rules
    pub fn validate(&self, context: ValidationContext) -> ValidationResult {
        let start_time = Instant::now();
        let mut result = ValidationResult::new(
            context.file_path.to_string_lossy().to_string(),
            self.config.level,
        );

        // Execute enabled validation rules
        for rule in &self.rules {
            if !rule.is_enabled(&self.config) {
                continue;
            }

            let rule_issues = rule.validate(&context);
            result.metrics.rules_executed += 1;

            for issue in rule_issues {
                result.add_issue(issue);

                // Check if we've hit the error limit
                if let Some(max_errors) = self.config.max_errors {
                    if result.metrics.errors >= max_errors {
                        result.add_issue(ValidationIssue {
                            severity: Severity::Warning,
                            category: Category::Syntax,
                            rule_name: "engine".to_string(),
                            message: format!("Validation stopped after {} errors", max_errors),
                            location: SourceLocation {
                                line: 1,
                                column: 1,
                                offset: 0,
                                length: 0,
                                file_path: Some(context.file_path.to_string_lossy().to_string()),
                            },
                            suggestion: Some("Fix errors and re-run validation".to_string()),
                            help_url: None,
                        });
                        break;
                    }
                }
            }
        }

        // Calculate metrics from AST
        if let Some(machine) = &context.ast.machine_block_node_opt {
            result.metrics.states_analyzed = machine.states.len();
            // Count total events across all states
            result.metrics.events_analyzed = machine
                .states
                .iter()
                .map(|state_ref| state_ref.borrow().evt_handlers_rcref.len())
                .sum();
        }

        // Determine final success status
        result.success = result.metrics.errors == 0
            && (!self.config.fail_on_warnings || result.metrics.warnings == 0);

        result.duration_ms = start_time.elapsed().as_millis() as u64;
        result
    }

    /// Format validation results using configured reporters
    pub fn format_results(&self, result: &ValidationResult) -> Vec<String> {
        if self.reporters.is_empty() {
            // Fallback to simple formatting if no reporters
            return vec![format!(
                "Validation completed: {} errors, {} warnings",
                result.metrics.errors, result.metrics.warnings
            )];
        }

        self.reporters
            .iter()
            .filter(|reporter| reporter.format() == self.config.output_format)
            .map(|reporter| reporter.report(result))
            .collect()
    }

    /// Validate and format results in one call
    pub fn validate_and_format(
        &self,
        context: ValidationContext,
    ) -> (ValidationResult, Vec<String>) {
        let result = self.validate(context);
        let formatted = self.format_results(&result);
        (result, formatted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_engine_creation() {
        let config = ValidationConfig::default();
        let engine = ValidationEngine::new(config);
        assert_eq!(engine.rules.len(), 0);
        assert_eq!(engine.reporters.len(), 0);
    }

    #[test]
    fn test_validation_engine_with_default_rules() {
        let config = ValidationConfig {
            level: ValidationLevel::Structural,
            ..Default::default()
        };
        let engine = ValidationEngine::with_default_rules(config);
        assert!(engine.rules.len() > 0);
    }

    #[test]
    fn test_validation_result_creation() {
        let result = ValidationResult::new("test.frm".to_string(), ValidationLevel::Basic);
        assert_eq!(result.file_path, "test.frm");
        assert_eq!(result.level, ValidationLevel::Basic);
        assert!(result.success);
        assert_eq!(result.issues.len(), 0);
    }
}
