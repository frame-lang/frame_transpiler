// Human-Readable Reporter
// Formats validation results for human consumption

use crate::frame_c::validation::*;
use std::fmt::Write;

pub struct HumanReporter {
    show_suggestions: bool,
    show_help_urls: bool,
}

impl HumanReporter {
    pub fn new() -> Self {
        Self {
            show_suggestions: true,
            show_help_urls: true,
        }
    }

    pub fn with_config(show_suggestions: bool, show_help_urls: bool) -> Self {
        Self {
            show_suggestions,
            show_help_urls,
        }
    }

    fn format_severity(&self, severity: Severity) -> (&str, &str) {
        match severity {
            Severity::Error => ("❌", "error"),
            Severity::Warning => ("⚠️", "warning"),
            Severity::Info => ("ℹ️", "info"),
            Severity::Hint => ("💡", "hint"),
        }
    }

    fn format_category(&self, category: Category) -> &str {
        match category {
            Category::Syntax => "syntax",
            Category::Structure => "structure",
            Category::Semantic => "semantic",
            Category::TargetLanguage => "target-language",
            Category::Performance => "performance",
            Category::Style => "style",
        }
    }
}

impl ValidationReporter for HumanReporter {
    fn format(&self) -> OutputFormat {
        OutputFormat::Human
    }

    fn report(&self, result: &ValidationResult) -> String {
        let mut output = String::new();
        
        // Header
        writeln!(output, "🔍 Frame Validation Report").unwrap();
        writeln!(output, "File: {}", result.file_path).unwrap();
        writeln!(output, "Level: {:?}", result.level).unwrap();
        writeln!(output, "Duration: {}ms", result.duration_ms).unwrap();
        writeln!(output).unwrap();

        // Summary
        let status_icon = if result.success { "✅" } else { "❌" };
        writeln!(output, "{} Status: {}", status_icon, if result.success { "PASS" } else { "FAIL" }).unwrap();
        writeln!(output, "   Errors: {}", result.metrics.errors).unwrap();
        writeln!(output, "   Warnings: {}", result.metrics.warnings).unwrap();
        writeln!(output, "   Hints: {}", result.metrics.hints).unwrap();
        writeln!(output).unwrap();

        if result.metrics.states_analyzed > 0 {
            writeln!(output, "📊 Analysis Summary:").unwrap();
            writeln!(output, "   States: {}", result.metrics.states_analyzed).unwrap();
            writeln!(output, "   Events: {}", result.metrics.events_analyzed).unwrap();
            writeln!(output, "   Rules: {}", result.metrics.rules_executed).unwrap();
            writeln!(output).unwrap();
        }

        // Issues
        if !result.issues.is_empty() {
            writeln!(output, "🔍 Issues Found:").unwrap();
            writeln!(output).unwrap();

            for (i, issue) in result.issues.iter().enumerate() {
                let (icon, severity_name) = self.format_severity(issue.severity);
                let category_name = self.format_category(issue.category);
                
                writeln!(output, "{} Issue #{}: {} [{}]", icon, i + 1, issue.message, category_name).unwrap();
                writeln!(output, "   Location: {}:{}", issue.location.line, issue.location.column).unwrap();
                writeln!(output, "   Rule: {}", issue.rule_name).unwrap();
                
                if self.show_suggestions {
                    if let Some(suggestion) = &issue.suggestion {
                        writeln!(output, "   💡 Suggestion: {}", suggestion).unwrap();
                    }
                }
                
                if self.show_help_urls {
                    if let Some(help_url) = &issue.help_url {
                        writeln!(output, "   📖 Help: {}", help_url).unwrap();
                    }
                }
                
                writeln!(output).unwrap();
            }
        } else {
            writeln!(output, "✨ No issues found!").unwrap();
        }

        // Footer
        if !result.success {
            writeln!(output, "💡 Fix the issues above and re-run validation.").unwrap();
        } else {
            writeln!(output, "🎉 All validation checks passed!").unwrap();
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_reporter_creation() {
        let reporter = HumanReporter::new();
        assert_eq!(reporter.format(), OutputFormat::Human);
    }

    #[test]
    fn test_format_severity() {
        let reporter = HumanReporter::new();
        let (icon, name) = reporter.format_severity(Severity::Error);
        assert_eq!(icon, "❌");
        assert_eq!(name, "error");
    }

    #[test]
    fn test_empty_result_report() {
        let reporter = HumanReporter::new();
        let result = ValidationResult::new("test.frm".to_string(), ValidationLevel::Basic);
        let report = reporter.report(&result);
        
        assert!(report.contains("Frame Validation Report"));
        assert!(report.contains("test.frm"));
        assert!(report.contains("PASS"));
        assert!(report.contains("No issues found"));
    }
}