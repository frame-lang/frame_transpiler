// Frame-to-Python syntax transpiler for V3
// Converts Frame expressions and statements to valid Python code

use std::collections::HashSet;

pub struct PythonTranspilerV3;

impl PythonTranspilerV3 {
    /// Transpile a Frame function body to Python
    pub fn transpile_function_body(&self, frame_body: &str) -> String {
        let mut result = String::new();
        let lines: Vec<&str> = frame_body.lines().collect();
        
        for line in lines {
            let transpiled = self.transpile_line(line);
            result.push_str(&transpiled);
            result.push('\n');
        }
        
        result
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
            return String::new(); // Remove closing braces
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
            return self.transpile_expression(indent, trimmed);
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
            // Remove trailing { if present
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
            // Remove trailing { if present
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
            // Remove trailing { if present
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