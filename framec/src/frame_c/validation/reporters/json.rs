// JSON Reporter
// Formats validation results as structured JSON

use crate::frame_c::validation::*;
use serde::{Serialize, Deserialize};

pub struct JsonReporter;

impl JsonReporter {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Serialize, Deserialize)]
struct JsonValidationResult {
    file_path: String,
    level: String,
    success: bool,
    duration_ms: u64,
    metrics: JsonMetrics,
    issues: Vec<JsonIssue>,
}

#[derive(Serialize, Deserialize)]
struct JsonMetrics {
    rules_executed: usize,
    errors: usize,
    warnings: usize,
    hints: usize,
    states_analyzed: usize,
    events_analyzed: usize,
}

#[derive(Serialize, Deserialize)]
struct JsonIssue {
    severity: String,
    category: String,
    rule_name: String,
    message: String,
    location: JsonLocation,
    suggestion: Option<String>,
    help_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct JsonLocation {
    line: u32,
    column: u32,
    file_path: Option<String>,
}

impl ValidationReporter for JsonReporter {
    fn format(&self) -> OutputFormat {
        OutputFormat::Json
    }

    fn report(&self, result: &ValidationResult) -> String {
        let json_result = JsonValidationResult {
            file_path: result.file_path.clone(),
            level: format!("{:?}", result.level),
            success: result.success,
            duration_ms: result.duration_ms,
            metrics: JsonMetrics {
                rules_executed: result.metrics.rules_executed,
                errors: result.metrics.errors,
                warnings: result.metrics.warnings,
                hints: result.metrics.hints,
                states_analyzed: result.metrics.states_analyzed,
                events_analyzed: result.metrics.events_analyzed,
            },
            issues: result.issues.iter().map(|issue| JsonIssue {
                severity: format!("{:?}", issue.severity),
                category: format!("{:?}", issue.category),
                rule_name: issue.rule_name.clone(),
                message: issue.message.clone(),
                location: JsonLocation {
                    line: issue.location.line,
                    column: issue.location.column,
                    file_path: issue.location.file_path.clone(),
                },
                suggestion: issue.suggestion.clone(),
                help_url: issue.help_url.clone(),
            }).collect(),
        };

        serde_json::to_string_pretty(&json_result).unwrap_or_else(|_| {
            r#"{"error": "Failed to serialize validation result"}"#.to_string()
        })
    }
}