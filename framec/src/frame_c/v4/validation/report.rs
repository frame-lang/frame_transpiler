//! Validation report for Frame V4
//!
//! Collects validation issues and provides various output formats.

use super::types::{ValidationConfig, ValidationIssue};

/// Summary statistics for a validation report
#[derive(Debug, Clone, Default)]
pub struct ValidationSummary {
    pub errors: usize,
    pub warnings: usize,
    pub info: usize,
}

impl ValidationSummary {
    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        self.errors > 0
    }

    /// Total issue count
    pub fn total(&self) -> usize {
        self.errors + self.warnings + self.info
    }
}

/// Validation report containing all issues from validation passes
#[derive(Debug, Clone)]
pub struct ValidationReport {
    /// All validation issues
    issues: Vec<ValidationIssue>,
    /// Source file name (for error reporting)
    source_file: Option<String>,
    /// Source content (for context in error messages)
    source: Option<String>,
}

impl ValidationReport {
    /// Create a new empty report
    pub fn new() -> Self {
        Self {
            issues: Vec::new(),
            source_file: None,
            source: None,
        }
    }

    /// Create report from issues
    pub fn from_issues(issues: Vec<ValidationIssue>) -> Self {
        Self {
            issues,
            source_file: None,
            source: None,
        }
    }

    /// Set source file name
    pub fn with_source_file(mut self, file: impl Into<String>) -> Self {
        self.source_file = Some(file.into());
        self
    }

    /// Set source content
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Add issues from a pass
    pub fn add_issues(&mut self, issues: impl IntoIterator<Item = ValidationIssue>) {
        self.issues.extend(issues);
    }

    /// Get all issues
    pub fn issues(&self) -> &[ValidationIssue] {
        &self.issues
    }

    /// Get errors only
    pub fn errors(&self) -> impl Iterator<Item = &ValidationIssue> {
        self.issues.iter().filter(|i| i.is_error())
    }

    /// Get warnings only
    pub fn warnings(&self) -> impl Iterator<Item = &ValidationIssue> {
        self.issues.iter().filter(|i| i.is_warning())
    }

    /// Get info only
    pub fn info(&self) -> impl Iterator<Item = &ValidationIssue> {
        self.issues.iter().filter(|i| i.is_info())
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|i| i.is_error())
    }

    /// Get summary statistics
    pub fn summary(&self) -> ValidationSummary {
        ValidationSummary {
            errors: self.errors().count(),
            warnings: self.warnings().count(),
            info: self.info().count(),
        }
    }

    /// Filter issues based on config
    pub fn filter_by_config(&mut self, config: &ValidationConfig) {
        self.issues.retain(|issue| {
            // Remove suppressed issues
            if config.is_suppressed(&issue.code) {
                return false;
            }
            // Remove info unless verbose
            if issue.is_info() && !config.verbose {
                return false;
            }
            true
        });

        // Adjust severities
        for issue in &mut self.issues {
            issue.severity = config.adjust_severity(issue.severity);
        }
    }

    /// Format as human-readable output
    pub fn to_human_readable(&self) -> String {
        let mut output = String::new();

        for issue in &self.issues {
            // Header: error[E402]: message
            output.push_str(&format!(
                "{}[{}]: {}\n",
                issue.severity,
                issue.code,
                issue.message
            ));

            // Location
            if let Some(span) = &issue.span {
                let file = self.source_file.as_deref().unwrap_or("<input>");
                if let Some(source) = &self.source {
                    let (line, col) = span_to_line_col(source, span.start);
                    output.push_str(&format!("  --> {}:{}:{}\n", file, line, col));

                    // Show source context
                    if let Some(line_text) = get_line(source, line) {
                        output.push_str(&format!("   |\n"));
                        output.push_str(&format!("{:>3} | {}\n", line, line_text));
                        output.push_str(&format!("   | {}{}\n",
                            " ".repeat(col.saturating_sub(1)),
                            "^".repeat((span.end - span.start).min(line_text.len()))
                        ));
                    }
                } else {
                    output.push_str(&format!("  --> {}:{}:{}\n", file, span.start, span.end));
                }
            }

            // Notes
            for note in &issue.notes {
                output.push_str(&format!("   = note: {}\n", note));
            }

            // Fix hint
            if let Some(hint) = &issue.fix_hint {
                output.push_str(&format!("   = hint: {}\n", hint));
            }

            output.push('\n');
        }

        // Summary
        let summary = self.summary();
        if summary.total() > 0 {
            output.push_str(&format!(
                "{}: {} error(s), {} warning(s)\n",
                if summary.has_errors() { "aborting" } else { "finished" },
                summary.errors,
                summary.warnings
            ));
        }

        output
    }

    /// Format as JSON for IDE integration
    pub fn to_json(&self) -> String {
        let issues: Vec<serde_json::Value> = self.issues.iter().map(|issue| {
            let mut obj = serde_json::json!({
                "code": issue.code,
                "severity": issue.severity.as_str(),
                "message": issue.message,
            });

            if let Some(span) = &issue.span {
                if let Some(source) = &self.source {
                    let (line, col) = span_to_line_col(source, span.start);
                    let (end_line, end_col) = span_to_line_col(source, span.end);
                    obj["line"] = serde_json::json!(line);
                    obj["column"] = serde_json::json!(col);
                    obj["end_line"] = serde_json::json!(end_line);
                    obj["end_column"] = serde_json::json!(end_col);
                } else {
                    obj["start"] = serde_json::json!(span.start);
                    obj["end"] = serde_json::json!(span.end);
                }
            }

            if let Some(file) = &self.source_file {
                obj["file"] = serde_json::json!(file);
            }

            if !issue.notes.is_empty() {
                obj["notes"] = serde_json::json!(issue.notes);
            }

            if let Some(hint) = &issue.fix_hint {
                obj["fix_hint"] = serde_json::json!(hint);
            }

            obj
        }).collect();

        let summary = self.summary();
        let output = serde_json::json!({
            "issues": issues,
            "summary": {
                "errors": summary.errors,
                "warnings": summary.warnings,
                "info": summary.info
            }
        });

        serde_json::to_string_pretty(&output).unwrap_or_else(|_| "{}".to_string())
    }

    /// Format for IDE (file:line:col format)
    pub fn to_ide_format(&self) -> String {
        let mut output = String::new();
        let file = self.source_file.as_deref().unwrap_or("<input>");

        for issue in &self.issues {
            if let Some(span) = &issue.span {
                if let Some(source) = &self.source {
                    let (line, col) = span_to_line_col(source, span.start);
                    output.push_str(&format!(
                        "{}:{}:{}: {}: [{}] {}\n",
                        file,
                        line,
                        col,
                        issue.severity,
                        issue.code,
                        issue.message
                    ));
                } else {
                    output.push_str(&format!(
                        "{}:{}:{}: {}: [{}] {}\n",
                        file,
                        span.start,
                        span.end,
                        issue.severity,
                        issue.code,
                        issue.message
                    ));
                }
            } else {
                output.push_str(&format!(
                    "{}: {}: [{}] {}\n",
                    file,
                    issue.severity,
                    issue.code,
                    issue.message
                ));
            }
        }

        output
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert byte offset to line and column
fn span_to_line_col(source: &str, offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;

    for (i, ch) in source.chars().enumerate() {
        if i >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }

    (line, col)
}

/// Get a specific line from source
fn get_line(source: &str, line_num: usize) -> Option<&str> {
    source.lines().nth(line_num - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame_c::v4::frame_ast::Span;

    #[test]
    fn test_report_summary() {
        let mut report = ValidationReport::new();
        report.add_issues(vec![
            ValidationIssue::error("E402", "Error 1"),
            ValidationIssue::error("E403", "Error 2"),
            ValidationIssue::warning("W410", "Warning 1"),
            ValidationIssue::info("I001", "Info 1"),
        ]);

        let summary = report.summary();
        assert_eq!(summary.errors, 2);
        assert_eq!(summary.warnings, 1);
        assert_eq!(summary.info, 1);
        assert!(summary.has_errors());
    }

    #[test]
    fn test_span_to_line_col() {
        let source = "line1\nline2\nline3";
        assert_eq!(span_to_line_col(source, 0), (1, 1));
        assert_eq!(span_to_line_col(source, 5), (1, 6)); // newline
        assert_eq!(span_to_line_col(source, 6), (2, 1)); // start of line2
        assert_eq!(span_to_line_col(source, 8), (2, 3)); // 'n' in line2
    }

    #[test]
    fn test_human_readable_format() {
        let mut report = ValidationReport::new()
            .with_source_file("test.frm")
            .with_source("@@system Test {\n  machine:\n    $Idle { }\n}");

        report.add_issues(vec![
            ValidationIssue::error("E402", "Unknown state 'Foo'")
                .with_span(Span::new(30, 35))
                .with_note("State must be defined before use")
                .with_fix("Add state $Foo { } to the machine"),
        ]);

        let output = report.to_human_readable();
        assert!(output.contains("error[E402]"));
        assert!(output.contains("Unknown state 'Foo'"));
        assert!(output.contains("test.frm"));
    }

    #[test]
    fn test_json_format() {
        let mut report = ValidationReport::new()
            .with_source_file("test.frm")
            .with_source("@@system Test { }");

        report.add_issues(vec![
            ValidationIssue::error("E402", "Test error").with_span(Span::new(0, 10)),
        ]);

        let json = report.to_json();
        assert!(json.contains("\"code\": \"E402\""));
        assert!(json.contains("\"severity\": \"error\""));
    }

    #[test]
    fn test_filter_by_config() {
        let config = ValidationConfig {
            suppress: vec!["E402".to_string()],
            warnings_as_errors: true,
            verbose: false,
            ..Default::default()
        };

        let mut report = ValidationReport::new();
        report.add_issues(vec![
            ValidationIssue::error("E402", "Suppressed"),
            ValidationIssue::error("E403", "Not suppressed"),
            ValidationIssue::warning("W410", "Becomes error"),
            ValidationIssue::info("I001", "Hidden"),
        ]);

        report.filter_by_config(&config);

        assert_eq!(report.issues().len(), 2);
        assert!(report.issues().iter().all(|i| i.is_error()));
    }
}
