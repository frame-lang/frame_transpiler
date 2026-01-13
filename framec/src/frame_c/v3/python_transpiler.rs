// Frame-to-Python syntax transpiler for V3
// Converts Frame expressions and statements to valid Python code

pub struct PythonTranspilerV3;

impl PythonTranspilerV3 {
    /// Transpile a Frame function body to Python without indentation validation
    /// Used when we know the content might have indentation issues from Frame expansion
    pub fn transpile_function_body_unchecked(&self, frame_body: &str) -> String {
        let mut result = String::new();
        let lines: Vec<&str> = frame_body.lines().collect();
        
        // Track brace context - stack of what opened each brace
        let mut brace_stack: Vec<&str> = Vec::new();
        
        for line in lines {
            let trimmed = line.trim();
            
            // Check what opens a brace
            if trimmed.ends_with("{") {
                if trimmed.contains("return") || trimmed.contains("=") || 
                   trimmed.starts_with("{") || trimmed.contains(": {") {
                    // This is a dictionary/object literal
                    brace_stack.push("dict");
                } else if trimmed.starts_with("if ") || trimmed.starts_with("else") || 
                          trimmed.starts_with("for ") || trimmed.starts_with("while ") ||
                          trimmed.starts_with("def ") || trimmed.starts_with("async def ") ||
                          trimmed.starts_with("try") || trimmed.starts_with("except") {
                    // This is a control flow block
                    brace_stack.push("block");
                } else {
                    // Default to block for unknown cases
                    brace_stack.push("block");
                }
            }
            
            // Determine if we should keep closing brace
            let keep_brace = if trimmed == "}" {
                match brace_stack.pop() {
                    Some("dict") => true,
                    Some("block") => false,
                    _ => false, // No matching opening brace
                }
            } else {
                true // Not a closing brace, process normally
            };
            
            let transpiled = if trimmed == "}" && !keep_brace {
                String::new() // Remove block closing braces
            } else {
                self.transpile_line(line)
            };
            
            result.push_str(&transpiled);
            if !transpiled.is_empty() || line.trim().is_empty() {
                result.push('\n');
            }
        }
        
        result
    }
    
    /// Transpile a Frame function body to Python with strict indentation validation
    pub fn transpile_function_body(&self, frame_body: &str) -> Result<String, String> {
        let mut result = String::new();
        let lines: Vec<&str> = frame_body.lines().collect();
        
        // Debug: print what we're validating
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("[PythonTranspilerV3] Validating indentation for {} lines:", lines.len());
            for (i, line) in lines.iter().enumerate() {
                eprintln!("  Line {}: {:?}", i + 1, line);
            }
        }
        
        // Validate indentation consistency
        self.validate_indentation(&lines)?;
        
        // Track brace context - stack of what opened each brace
        let mut brace_stack: Vec<&str> = Vec::new();
        
        for line in lines {
            let trimmed = line.trim();
            
            // Check what opens a brace
            if trimmed.ends_with("{") {
                if trimmed.contains("return") || trimmed.contains("=") || 
                   trimmed.starts_with("{") || trimmed.contains(": {") {
                    // This is a dictionary/object literal
                    brace_stack.push("dict");
                } else if trimmed.starts_with("if ") || trimmed.starts_with("else") || 
                          trimmed.starts_with("for ") || trimmed.starts_with("while ") ||
                          trimmed.starts_with("def ") || trimmed.starts_with("async def ") ||
                          trimmed.starts_with("try") || trimmed.starts_with("except") {
                    // This is a control flow block
                    brace_stack.push("block");
                } else {
                    // Default to block for unknown cases
                    brace_stack.push("block");
                }
            }
            
            // Determine if we should keep closing brace
            let keep_brace = if trimmed == "}" {
                match brace_stack.pop() {
                    Some("dict") => true,
                    Some("block") => false,
                    _ => false, // No matching opening brace
                }
            } else {
                true // Not a closing brace, process normally
            };
            
            let transpiled = if trimmed == "}" && !keep_brace {
                String::new() // Remove block closing braces
            } else {
                self.transpile_line(line)
            };
            
            result.push_str(&transpiled);
            if !transpiled.is_empty() || line.trim().is_empty() {
                result.push('\n');
            }
        }
        
        Ok(result)
    }
    
    /// Validate that indentation is consistent and correct for Python
    fn validate_indentation(&self, lines: &[&str]) -> Result<(), String> {
        let mut indent_stack: Vec<usize> = vec![0];
        let mut prev_indent = 0;
        let mut prev_was_colon = false;
        let mut first_non_empty = true;
        
        for (line_num, line) in lines.iter().enumerate() {
            // Skip empty lines
            let trimmed = line.trim_start();
            if trimmed.is_empty() {
                continue;
            }
            
            // Calculate indentation (spaces only for Python)
            let indent = line.len() - trimmed.len();
            
            // Debug output
            if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                eprintln!("  Checking line {}: indent={}, content={:?}", line_num + 1, indent, trimmed);
            }
            
            // Check for tabs (Python 3 doesn't mix tabs and spaces)
            if line[0..indent].contains('\t') {
                return Err(format!(
                    "Line {}: Mixed tabs and spaces in indentation (Python requires consistent spacing)",
                    line_num + 1
                ));
            }
            
            // Check indentation is multiple of 4 (Python convention)
            if indent % 4 != 0 {
                return Err(format!(
                    "Line {}: Indentation must be a multiple of 4 spaces, found {} spaces",
                    line_num + 1,
                    indent
                ));
            }
            
            // Check for inconsistent indentation changes
            if indent > prev_indent {
                // Indentation increased - should only increase after a colon (except for first line)
                if !prev_was_colon && !first_non_empty {
                    return Err(format!(
                        "Line {}: Unexpected indentation increase (no colon on previous line)",
                        line_num + 1
                    ));
                }
                // Should only increase by 4
                if indent - prev_indent != 4 {
                    return Err(format!(
                        "Line {}: Indentation increased by {} spaces (expected 4)",
                        line_num + 1,
                        indent - prev_indent
                    ));
                }
                indent_stack.push(indent);
            } else if indent < prev_indent {
                // Indentation decreased - must match a previous level
                while !indent_stack.is_empty() && indent_stack[indent_stack.len() - 1] > indent {
                    indent_stack.pop();
                }
                if indent_stack.is_empty() || indent_stack[indent_stack.len() - 1] != indent {
                    return Err(format!(
                        "Line {}: Indentation does not match any previous indentation level",
                        line_num + 1
                    ));
                }
            }
            
            // Check if this line ends with a colon (for next iteration)
            prev_was_colon = trimmed.ends_with(':') || trimmed.ends_with('{');
            prev_indent = indent;
            first_non_empty = false;
        }
        
        Ok(())
    }
    
    /// Transpile a single line of Frame code to Python
    fn transpile_line(&self, line: &str) -> String {
        let trimmed = line.trim_start();
        let indent = &line[0..line.len() - trimmed.len()];
        
        // Skip empty lines and comments
        if trimmed.is_empty() {
            return line.to_string();
        }
        if trimmed.starts_with('#') || trimmed.starts_with("//") {
            // Convert // comments to # comments
            if trimmed.starts_with("//") {
                return format!("{}#{}", indent, &trimmed[2..]);
            }
            return line.to_string();
        }
        
        // Check for various Frame constructs
        if trimmed.starts_with("var ") {
            return self.transpile_var_declaration(indent, trimmed);
        } else if trimmed.starts_with("if ") {
            return self.transpile_if_statement(indent, trimmed);
        } else if trimmed == "}" {
            // Keep the closing brace as-is, will be handled by caller
            return line.to_string();
        } else if trimmed == "} else {" || trimmed == "}else{" {
            return format!("{}else:", indent);
        } else if trimmed.starts_with("} else if ") || trimmed.starts_with("}else if ") {
            let condition = self.extract_elif_condition(trimmed);
            return format!("{}elif {}:", indent, condition);
        } else if trimmed.starts_with("for ") {
            return self.transpile_for_loop(indent, trimmed);
        } else if trimmed.starts_with("while ") {
            return self.transpile_while_loop(indent, trimmed);
        } else if trimmed.starts_with("return ") {
            return self.transpile_return(indent, trimmed);
        } else if trimmed == "return" {
            return format!("{}return", indent);
        } else if trimmed.starts_with("print(") {
            // print statements are already Python-compatible
            return line.to_string();
        } else {
            // For other statements, apply basic transformations
            // But preserve the trimmed content without the indent 
            let transpiled_expr = self.transpile_expression("", trimmed);
            return format!("{}{}", indent, transpiled_expr);
        }
    }
    
    /// Transpile var declaration: "var x = 10" -> "x = 10"
    fn transpile_var_declaration(&self, indent: &str, line: &str) -> String {
        if let Some(rest) = line.strip_prefix("var ") {
            // Handle simple declarations like "var x = 10"
            format!("{}{}", indent, rest)
        } else {
            // Shouldn't happen, but return as-is
            format!("{}{}", indent, line)
        }
    }
    
    /// Transpile if statement: "if condition {" -> "if condition:"
    fn transpile_if_statement(&self, indent: &str, line: &str) -> String {
        if let Some(rest) = line.strip_prefix("if ") {
            // Check if already in Python syntax (ends with :)
            if rest.trim().ends_with(":") {
                // Already has colon, just return as-is with proper indentation
                return format!("{}{}", indent, line.trim());
            }
            
            // Remove trailing { if present (Frame syntax)
            let condition = if let Some(pos) = rest.rfind(" {") {
                &rest[..pos]
            } else if rest.ends_with("{") {
                &rest[..rest.len() - 1]
            } else {
                rest
            }.trim();
            
            // Convert Frame operators to Python
            let condition = self.transpile_condition(condition);
            format!("{}if {}:", indent, condition)
        } else {
            format!("{}{}", indent, line)
        }
    }
    
    /// Extract condition from elif: "} else if condition {" -> "condition"
    fn extract_elif_condition(&self, line: &str) -> String {
        let mut condition = line.trim_start();
        
        // Remove various prefixes
        if condition.starts_with("}else if ") {
            condition = &condition[9..];
        } else if condition.starts_with("} else if ") {
            condition = &condition[10..];
        }
        
        // Remove trailing {
        if let Some(pos) = condition.rfind(" {") {
            condition = &condition[..pos];
        } else if condition.ends_with("{") {
            condition = &condition[..condition.len() - 1];
        }
        
        self.transpile_condition(condition.trim())
    }
    
    /// Transpile for loop: "for item in items {" -> "for item in items:"
    fn transpile_for_loop(&self, indent: &str, line: &str) -> String {
        if let Some(rest) = line.strip_prefix("for ") {
            // Check if already in Python syntax (ends with :)
            if rest.trim().ends_with(":") {
                // Already has colon, just return as-is with proper indentation
                return format!("{}{}", indent, line.trim());
            }
            
            // Remove trailing { if present (Frame syntax)
            let loop_expr = if let Some(pos) = rest.rfind(" {") {
                &rest[..pos]
            } else if rest.ends_with("{") {
                &rest[..rest.len() - 1]
            } else {
                rest
            }.trim();
            
            format!("{}for {}:", indent, loop_expr)
        } else {
            format!("{}{}", indent, line)
        }
    }
    
    /// Transpile while loop: "while condition {" -> "while condition:"
    fn transpile_while_loop(&self, indent: &str, line: &str) -> String {
        if let Some(rest) = line.strip_prefix("while ") {
            // Check if already in Python syntax (ends with :)
            if rest.trim().ends_with(":") {
                // Already has colon, just return as-is with proper indentation
                return format!("{}{}", indent, line.trim());
            }
            
            // Remove trailing { if present (Frame syntax)
            let condition = if let Some(pos) = rest.rfind(" {") {
                &rest[..pos]
            } else if rest.ends_with("{") {
                &rest[..rest.len() - 1]
            } else {
                rest
            }.trim();
            
            let condition = self.transpile_condition(condition);
            format!("{}while {}:", indent, condition)
        } else {
            format!("{}{}", indent, line)
        }
    }
    
    /// Transpile return statement
    fn transpile_return(&self, indent: &str, line: &str) -> String {
        if let Some(rest) = line.strip_prefix("return ") {
            let expr = self.transpile_expression("", rest);
            format!("{}return {}", indent, expr)
        } else {
            format!("{}{}", indent, line)
        }
    }
    
    /// Transpile conditions (used in if/while/elif)
    fn transpile_condition(&self, condition: &str) -> String {
        // Handle logical operators
        let condition = condition
            .replace(" && ", " and ")
            .replace(" || ", " or ")
            .replace("!", "not ");
        
        // For now, return the condition with basic transformations
        // TODO: Add more sophisticated expression parsing if needed
        condition
    }
    
    /// Transpile general expressions and statements
    fn transpile_expression(&self, indent: &str, expr: &str) -> String {
        let mut result = expr.to_string();
        
        // Remove var keyword if present in expressions
        if result.starts_with("var ") {
            result = result[4..].to_string();
        }
        
        // Handle method calls that might need self. prefix
        // This is a simple heuristic - more sophisticated analysis would be needed
        // TODO: Integrate with proper symbol resolution
        
        // Note: Boolean literals should already be in the correct form for the target language
        // Frame V3 uses native code, so Python code should use True/False, not true/false
        
        // Convert logical operators
        result = result
            .replace(" && ", " and ")
            .replace(" || ", " or ");
        
        // Handle negation
        if result.trim().starts_with("!") {
            let rest = result.trim()[1..].trim();
            result = format!("not {}", rest);
        }
        
        format!("{}{}", indent, result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_var_declaration() {
        let transpiler = PythonTranspilerV3;
        assert_eq!(transpiler.transpile_line("    var x = 10"), "    x = 10");
        assert_eq!(transpiler.transpile_line("var result = add(5, 3)"), "result = add(5, 3)");
    }
    
    #[test]
    fn test_indentation_validation() {
        let transpiler = PythonTranspilerV3;
        
        // Valid indentation
        let valid = "var x = 10\nif x > 5 {\n    print(\"big\")\n}";
        assert!(transpiler.transpile_function_body(valid).is_ok());
        
        // Invalid indentation - not multiple of 4
        let invalid = "var x = 10\n  print(\"bad indent\")";
        assert!(transpiler.transpile_function_body(invalid).is_err());
        
        // Mixed tabs and spaces
        let mixed = "var x = 10\n\tprint(\"tab\")";
        assert!(transpiler.transpile_function_body(mixed).is_err());
    }
    
    #[test]
    fn test_if_statement() {
        let transpiler = PythonTranspilerV3;
        assert_eq!(transpiler.transpile_line("if x == 10 {"), "if x == 10:");
        assert_eq!(transpiler.transpile_line("    if result > 0 {"), "    if result > 0:");
    }
    
    #[test]
    fn test_else_clause() {
        let transpiler = PythonTranspilerV3;
        assert_eq!(transpiler.transpile_line("} else {"), "else:");
        assert_eq!(transpiler.transpile_line("    }else{"), "    else:");
    }
    
    #[test]
    fn test_elif_clause() {
        let transpiler = PythonTranspilerV3;
        assert_eq!(transpiler.transpile_line("} else if x == 5 {"), "elif x == 5:");
        assert_eq!(transpiler.transpile_line("}else if y > 0 {"), "elif y > 0:");
    }
    
    #[test]
    fn test_for_loop() {
        let transpiler = PythonTranspilerV3;
        assert_eq!(transpiler.transpile_line("for item in items {"), "for item in items:");
        assert_eq!(transpiler.transpile_line("    for i in range(10) {"), "    for i in range(10):");
    }
    
    #[test]
    fn test_while_loop() {
        let transpiler = PythonTranspilerV3;
        assert_eq!(transpiler.transpile_line("while x < 10 {"), "while x < 10:");
        assert_eq!(transpiler.transpile_line("    while running {"), "    while running:");
    }
    
    #[test]
    fn test_return_statement() {
        let transpiler = PythonTranspilerV3;
        assert_eq!(transpiler.transpile_line("return x + y"), "return x + y");
        assert_eq!(transpiler.transpile_line("    return"), "    return");
    }
    
    #[test]
    fn test_logical_operators() {
        let transpiler = PythonTranspilerV3;
        assert_eq!(transpiler.transpile_line("if x && y {"), "if x and y:");
        assert_eq!(transpiler.transpile_line("if a || b {"), "if a or b:");
        assert_eq!(transpiler.transpile_line("if !valid {"), "if not valid:");
    }
    
    #[test]
    fn test_closing_braces_removed() {
        let transpiler = PythonTranspilerV3;
        assert_eq!(transpiler.transpile_line("}"), "");
        assert_eq!(transpiler.transpile_line("    }"), "");
    }
    
    #[test]
    fn test_comments() {
        let transpiler = PythonTranspilerV3;
        assert_eq!(transpiler.transpile_line("# Python comment"), "# Python comment");
        assert_eq!(transpiler.transpile_line("// C-style comment"), "# C-style comment");
        assert_eq!(transpiler.transpile_line("    // indented comment"), "    # indented comment");
    }
}