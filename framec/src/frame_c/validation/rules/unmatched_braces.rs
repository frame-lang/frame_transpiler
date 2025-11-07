// Unmatched Braces Rule
// Detects unmatched braces throughout the Frame source

use crate::frame_c::validation::*;
use std::collections::HashSet;

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
                                suggestion: Some(
                                    "Remove this brace or add a matching opening brace".to_string(),
                                ),
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

    /// Python-aware brace scan: supports '#', single/double quotes, and triple quotes.
    fn analyze_brace_matching_python(&self, source_code: &str) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        let mut brace_stack: Vec<(usize, usize, char)> = Vec::new();
        let mut in_squote = false;
        let mut in_dquote = false;
        let mut in_tsquote = false; // '''
        let mut in_tdquote = false; // """

        for (line_idx, line) in source_code.lines().enumerate() {
            let bytes = line.as_bytes();
            let mut j = 0usize;
            // Treat '#' as comment start when not in a string
            while j < bytes.len() {
                let ch = bytes[j] as char;
                if !(in_squote || in_dquote || in_tsquote || in_tdquote) {
                    // Triple quotes first
                    if j + 2 < bytes.len() && bytes[j] == b'\'' && bytes[j + 1] == b'\'' && bytes[j + 2] == b'\'' {
                        in_tsquote = true; j += 3; continue;
                    }
                    if j + 2 < bytes.len() && bytes[j] == b'"' && bytes[j + 1] == b'"' && bytes[j + 2] == b'"' {
                        in_tdquote = true; j += 3; continue;
                    }
                    if ch == '\'' { in_squote = true; j += 1; continue; }
                    if ch == '"' { in_dquote = true; j += 1; continue; }
                    if ch == '#' { break; } // rest of line is a comment

                    match ch {
                        '{' => { brace_stack.push((line_idx + 1, j + 1, ch)); j += 1; continue; }
                        '}' => {
                            if let Some((_, _, open_ch)) = brace_stack.pop() {
                                if open_ch != '{' {
                                    issues.push(ValidationIssue {
                                        severity: Severity::Error,
                                        category: Category::Syntax,
                                        rule_name: self.name.clone(),
                                        message: format!("Mismatched brace: found '{}'", ch),
                                        location: SourceLocation { line: line_idx as u32 + 1, column: j as u32 + 1, offset: 0, length: 1, file_path: None },
                                        suggestion: Some("Check surrounding braces".to_string()),
                                        help_url: None,
                                    });
                                }
                            } else {
                                issues.push(ValidationIssue {
                                    severity: Severity::Error,
                                    category: Category::Syntax,
                                    rule_name: self.name.clone(),
                                    message: "Unexpected closing brace '}'".to_string(),
                                    location: SourceLocation { line: line_idx as u32 + 1, column: j as u32 + 1, offset: 0, length: 1, file_path: None },
                                    suggestion: Some("Remove this brace or add a matching opening brace".to_string()),
                                    help_url: None,
                                });
                            }
                            j += 1; continue;
                        }
                        _ => { j += 1; continue; }
                    }
                } else {
                    if in_tsquote {
                        if j + 2 < bytes.len() && bytes[j] == b'\'' && bytes[j + 1] == b'\'' && bytes[j + 2] == b'\'' { in_tsquote = false; j += 3; continue; }
                        j += 1; continue;
                    }
                    if in_tdquote {
                        if j + 2 < bytes.len() && bytes[j] == b'"' && bytes[j + 1] == b'"' && bytes[j + 2] == b'"' { in_tdquote = false; j += 3; continue; }
                        j += 1; continue;
                    }
                    if in_squote {
                        if bytes[j] == b'\\' { j += 2; continue; }
                        if ch == '\'' { in_squote = false; j += 1; continue; }
                        j += 1; continue;
                    }
                    if in_dquote {
                        if bytes[j] == b'\\' { j += 2; continue; }
                        if ch == '"' { in_dquote = false; j += 1; continue; }
                        j += 1; continue;
                    }
                }
            }
        }

        for (line, col, brace_char) in brace_stack {
            issues.push(ValidationIssue {
                severity: Severity::Error,
                category: Category::Syntax,
                rule_name: self.name.clone(),
                message: format!("Unclosed brace '{}'", brace_char),
                location: SourceLocation { line: line as u32, column: col as u32, offset: 0, length: 1, file_path: None },
                suggestion: Some("Add a matching closing brace".to_string()),
                help_url: None,
            });
        }

        issues
    }
}

impl UnmatchedBracesRule {
    /// Analyze brace matching while skipping masked line numbers (1-based)
    fn analyze_brace_matching_with_mask(
        &self,
        source_code: &str,
        mask_lines: &HashSet<u32>,
    ) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        let mut brace_stack = Vec::new();
        let mut in_string = false;
        let mut in_comment = false;
        let mut escape_next = false;

        for (line_idx, line) in source_code.lines().enumerate() {
            let line_num = (line_idx as u32) + 1;
            if mask_lines.contains(&line_num) {
                // Skip native/mixed body lines; Frame braces on these lines are not authoritative
                continue;
            }

            for (col_num, ch) in line.chars().enumerate() {
                if escape_next {
                    escape_next = false;
                    continue;
                }
                if ch == '\\' {
                    escape_next = true;
                    continue;
                }
                if ch == '"' && !in_comment {
                    in_string = !in_string;
                    continue;
                }
                if in_string {
                    continue;
                }
                if ch == '/' && line.chars().nth(col_num + 1) == Some('/') {
                    in_comment = true;
                    continue;
                }
                if in_comment {
                    continue;
                }
                match ch {
                    '{' => brace_stack.push((line_num, col_num as u32 + 1, ch)),
                    '}' => {
                        if let Some((open_line, open_col, open_char)) = brace_stack.pop() {
                            if open_char != '{' {
                                issues.push(ValidationIssue {
                                    severity: Severity::Error,
                                    category: Category::Syntax,
                                    rule_name: self.name.clone(),
                                    message: format!(
                                        "Mismatched brace: found '{}' but expected closing for '{}'",
                                        ch, open_char
                                    ),
                                    location: SourceLocation {
                                        line: line_num,
                                        column: col_num as u32 + 1,
                                        offset: 0,
                                        length: 1,
                                        file_path: None,
                                    },
                                    suggestion: Some(format!(
                                        "Check brace at line {}:{}",
                                        open_line, open_col
                                    )),
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
                                    line: line_num,
                                    column: col_num as u32 + 1,
                                    offset: 0,
                                    length: 1,
                                    file_path: None,
                                },
                                suggestion: Some(
                                    "Remove this brace or add a matching opening brace".to_string(),
                                ),
                                help_url: None,
                            });
                        }
                    }
                    _ => {}
                }
            }
            in_comment = false;
        }
        for (line, col, brace_char) in brace_stack {
            issues.push(ValidationIssue {
                severity: Severity::Error,
                category: Category::Syntax,
                rule_name: self.name.clone(),
                message: format!("Unclosed brace '{}'", brace_char),
                location: SourceLocation {
                    line,
                    column: col,
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
        match context.target_language {
            Some(TargetLanguage::Python) => {
                // If AST successfully built (structural braces balanced), skip textual pass
                if context.ast.machine_block_node_opt.is_some()
                    || context.ast.actions_block_node_opt.is_some()
                    || context.ast.interface_block_node_opt.is_some()
                    || context.ast.operations_block_node_opt.is_some()
                {
                    Vec::new()
                } else {
                    self.analyze_brace_matching_python(context.source_code)
                }
            }
            Some(TargetLanguage::TypeScript) => {
                // If we successfully built an AST (i.e., parser balanced structural braces),
                // skip textual unmatched-braces checks to avoid false positives from native spans.
                if context.ast.machine_block_node_opt.is_some()
                    || context.ast.actions_block_node_opt.is_some()
                    || context.ast.interface_block_node_opt.is_some()
                    || context.ast.operations_block_node_opt.is_some()
                {
                    return Vec::new();
                }
                // Mask native/mixed body lines to avoid counting TS-native braces
                let mut mask = HashSet::<u32>::new();
                if let Some(machine) = &context.ast.machine_block_node_opt {
                    for state_r in &machine.states {
                        let state = state_r.borrow();
                        for eh_r in &state.evt_handlers_rcref {
                            let eh = eh_r.borrow();
                            if let Some(items) = &eh.mixed_body {
                                for item in items {
                                    if let crate::frame_c::ast::MixedBodyItem::NativeText { start_line, end_line, .. } = item {
                                        for ln in *start_line as u32..=*end_line as u32 { mask.insert(ln); }
                                    }
                                }
                            }
                        }
                    }
                }
                if let Some(actions) = &context.ast.actions_block_node_opt {
                    for a_r in &actions.actions {
                        let a = a_r.borrow();
                        if let Some(items) = &a.mixed_body {
                            for item in items {
                                if let crate::frame_c::ast::MixedBodyItem::NativeText { start_line, end_line, .. } = item {
                                    for ln in *start_line as u32..=*end_line as u32 { mask.insert(ln); }
                                }
                            }
                        }
                    }
                }
                if let Some(ops) = &context.ast.operations_block_node_opt {
                    for o_r in &ops.operations {
                        let o = o_r.borrow();
                        if let Some(items) = &o.mixed_body {
                            for item in items {
                                if let crate::frame_c::ast::MixedBodyItem::NativeText { start_line, end_line, .. } = item {
                                    for ln in *start_line as u32..=*end_line as u32 { mask.insert(ln); }
                                }
                            }
                        }
                    }
                }
                self.analyze_brace_matching_with_mask(context.source_code, &mask)
            }
            _ => self.analyze_brace_matching(context.source_code),
        }
    }
}
