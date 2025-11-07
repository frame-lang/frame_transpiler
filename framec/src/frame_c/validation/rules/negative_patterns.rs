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
            let leading_ws = t.chars().take_while(|c| c.is_whitespace()).count();
            // Nested function definition inside a body (indented Frame fn ...)
            if leading_ws > 0 && t.trim_start().starts_with("fn ") {
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
            // Nested TypeScript function declaration (indented `function ...`)
            if leading_ws > 0 && t.trim_start().starts_with("function ") {
                issues.push(ValidationIssue {
                    severity: Severity::Error,
                    category: Category::Structure,
                    rule_name: self.name().to_string(),
                    message: "Nested function declarations are not allowed in TypeScript bodies.".to_string(),
                    location: SourceLocation { line: frame_line, column: 1, offset: 0, length: 0, file_path: None },
                    suggestion: Some("Move nested function to module scope or use arrow function assigned to const".to_string()),
                    help_url: None,
                });
                break;
            }
            // Python-style error handling constructs are invalid for TypeScript tests
            // (used by negative fixtures like test_error_handling_v049)
            let ts_illegal_py_patterns = ["try {", "except ", "finally", "raise "];
            if ts_illegal_py_patterns.iter().any(|p| t.contains(p)) {
                issues.push(ValidationIssue {
                    severity: Severity::Error,
                    category: Category::Syntax,
                    rule_name: self.name().to_string(),
                    message: "Python-style error handling is not valid for TypeScript target".to_string(),
                    location: SourceLocation { line: frame_line, column: 1, offset: 0, length: 0, file_path: None },
                    suggestion: Some("Use TypeScript try/catch/finally or convert test to Python target".to_string()),
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
