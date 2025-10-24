// JUnit Reporter
// Formats validation results as JUnit XML for CI/CD integration

use crate::frame_c::validation::*;
use std::fmt::Write;

pub struct JunitReporter;

impl JunitReporter {
    pub fn new() -> Self {
        Self
    }
}

impl ValidationReporter for JunitReporter {
    fn format(&self) -> OutputFormat {
        OutputFormat::Junit
    }

    fn report(&self, result: &ValidationResult) -> String {
        let mut output = String::new();

        // XML header
        writeln!(output, r#"<?xml version="1.0" encoding="UTF-8"?>"#).unwrap();

        // Test suite
        let test_count = result.metrics.rules_executed;
        let failure_count = result.metrics.errors;
        let time = result.duration_ms as f64 / 1000.0;

        writeln!(output, r#"<testsuites>"#).unwrap();
        writeln!(output, r#"  <testsuite name="frame-validation" tests="{}" failures="{}" errors="0" time="{:.3}">"#,
                test_count, failure_count, time).unwrap();

        // Individual test cases for each rule
        for issue in &result.issues {
            let test_name = format!("{}.{}", issue.rule_name, result.file_path);
            writeln!(
                output,
                r#"    <testcase name="{}" classname="validation">"#,
                test_name
            )
            .unwrap();

            if issue.severity == Severity::Error {
                writeln!(
                    output,
                    r#"      <failure message="{}">"#,
                    html_escape(&issue.message)
                )
                .unwrap();
                writeln!(
                    output,
                    "Location: {}:{}",
                    issue.location.line, issue.location.column
                )
                .unwrap();
                if let Some(suggestion) = &issue.suggestion {
                    writeln!(output, "Suggestion: {}", html_escape(suggestion)).unwrap();
                }
                writeln!(output, r#"      </failure>"#).unwrap();
            }

            writeln!(output, r#"    </testcase>"#).unwrap();
        }

        writeln!(output, r#"  </testsuite>"#).unwrap();
        writeln!(output, r#"</testsuites>"#).unwrap();

        output
    }
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
