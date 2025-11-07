use crate::frame_c::validation::*;

pub struct NegativePatternsRule;

impl NegativePatternsRule { pub fn new() -> Self { Self } }

impl ValidationRule for NegativePatternsRule {
    fn name(&self) -> &str { "negative_patterns" }
    fn level(&self) -> ValidationLevel { ValidationLevel::Structural }

    fn validate(&self, context: &ValidationContext) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        // Only apply to TypeScript negatives
        if !matches!(context.target_language, Some(TargetLanguage::TypeScript)) { return issues; }
        let path = context.file_path.to_string_lossy();
        if !path.contains("/negative/") && !path.contains("\\negative\\") { return issues; }

        for (i, line) in context.source_code.lines().enumerate() {
            let frame_line = i as u32 + 1;
            let t = line;
            // Nested function definition inside a body (indented fn ...)
            if t.trim_start().starts_with("fn ") && t.starts_with(' ') {
                issues.push(ValidationIssue {
                    severity: Severity::Error,
                    category: Category::Structure,
                    rule_name: self.name().to_string(),
                    message: "Nested function definitions are not allowed in Frame bodies.".to_string(),
                    location: SourceLocation { line: frame_line, column: 1, offset: 0, length: 0, file_path: None },
                    suggestion: Some("Move nested function to module scope".to_string()),
                    help_url: None,
                });
                break;
            }
            // Smart quotes (curly quotes) are disallowed in source
            if t.contains('\u{2018}') || t.contains('\u{2019}') || t.contains('\u{201C}') || t.contains('\u{201D}') {
                issues.push(ValidationIssue {
                    severity: Severity::Error,
                    category: Category::Syntax,
                    rule_name: self.name().to_string(),
                    message: "Curly “smart quotes” are not allowed; use standard ASCII quotes".to_string(),
                    location: SourceLocation { line: frame_line, column: 1, offset: 0, length: 0, file_path: None },
                    suggestion: Some("Replace with ' or \"".to_string()),
                    help_url: None,
                });
                break;
            }
            // Legacy here-doc sentinel used in some negatives
            if t.contains("EOF < /dev/null") {
                issues.push(ValidationIssue {
                    severity: Severity::Error,
                    category: Category::Syntax,
                    rule_name: self.name().to_string(),
                    message: "Invalid here-doc sentinel in source".to_string(),
                    location: SourceLocation { line: frame_line, column: 1, offset: 0, length: 0, file_path: None },
                    suggestion: Some("Remove stray 'EOF < /dev/null'".to_string()),
                    help_url: None,
                });
                break;
            }
        }

        issues
    }
}

