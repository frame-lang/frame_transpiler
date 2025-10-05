// Unmatched Braces Rule
// Detects unmatched braces throughout the Frame source

use crate::frame_c::validation::*;

pub struct UnmatchedBracesRule {
    name: String,
}

impl UnmatchedBracesRule {
    pub fn new() -> Self {
        Self {
            name: "unmatched_braces".to_string(),
        }
    }

    /// Analyze brace matching throughout the source code
    fn analyze_brace_matching(&self, source_code: &str) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        let mut brace_stack = Vec::new();
        let mut in_string = false;
        let mut in_comment = false;
        let mut escape_next = false;
        
        for (line_num, line) in source_code.lines().enumerate() {
            for (col_num, ch) in line.chars().enumerate() {
                // Handle escape sequences
                if escape_next {
                    escape_next = false;
                    continue;
                }
                
                if ch == '\\' {
                    escape_next = true;
                    continue;
                }
                
                // Handle string literals
                if ch == '"' && !in_comment {
                    in_string = !in_string;
                    continue;
                }
                
                if in_string {
                    continue;
                }
                
                // Handle comments
                if ch == '/' && line.chars().nth(col_num + 1) == Some('/') {
                    in_comment = true;
                    continue;
                }
                
                if in_comment {
                    continue;
                }
                
                // Track braces
                match ch {
                    '{' => {
                        brace_stack.push((line_num + 1, col_num + 1, ch));
                    }
                    '}' => {
                        if let Some((open_line, open_col, open_char)) = brace_stack.pop() {
                            if open_char != '{' {
                                issues.push(ValidationIssue {
                                    severity: Severity::Error,
                                    category: Category::Syntax,
                                    rule_name: self.name.clone(),
                                    message: format!("Mismatched brace: found '{}' but expected closing for '{}'", ch, open_char),
                                    location: SourceLocation {
                                        line: line_num as u32 + 1,
                                        column: col_num as u32 + 1,
                                        offset: 0,
                                        length: 1,
                                        file_path: None,
                                    },
                                    suggestion: Some(format!("Check brace at line {}:{}", open_line, open_col)),
                                    help_url: None,
                                });
                            }
                        } else {
                            issues.push(ValidationIssue {
                                severity: Severity::Error,
                                category: Category::Syntax,
                                rule_name: self.name.clone(),
                                message: "Unexpected closing brace '}'".to_string(),
                                location: SourceLocation {
                                    line: line_num as u32 + 1,
                                    column: col_num as u32 + 1,
                                    offset: 0,
                                    length: 1,
                                    file_path: None,
                                },
                                suggestion: Some("Remove this brace or add a matching opening brace".to_string()),
                                help_url: None,
                            });
                        }
                    }
                    _ => {}
                }
            }
            
            // Reset comment state at end of line
            in_comment = false;
        }
        
        // Check for unclosed braces
        for (line, col, brace_char) in brace_stack {
            issues.push(ValidationIssue {
                severity: Severity::Error,
                category: Category::Syntax,
                rule_name: self.name.clone(),
                message: format!("Unclosed brace '{}'", brace_char),
                location: SourceLocation {
                    line: line as u32,
                    column: col as u32,
                    offset: 0,
                    length: 1,
                    file_path: None,
                },
                suggestion: Some("Add a matching closing brace".to_string()),
                help_url: None,
            });
        }
        
        issues
    }
}

impl ValidationRule for UnmatchedBracesRule {
    fn name(&self) -> &str {
        &self.name
    }

    fn level(&self) -> ValidationLevel {
        ValidationLevel::Basic
    }

    fn validate(&self, context: &ValidationContext) -> Vec<ValidationIssue> {
        self.analyze_brace_matching(context.source_code)
    }
}