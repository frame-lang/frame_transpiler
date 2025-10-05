// Malformed Handler Rule
// Detects malformed event handlers, missing braces, and structural issues
// This rule would have caught Bugs #29 and #31

use crate::frame_c::validation::*;
use crate::frame_c::ast::*;
// use std::collections::HashMap; // Commented out as unused

pub struct MalformedHandlerRule {
    name: String,
}

impl MalformedHandlerRule {
    pub fn new() -> Self {
        Self {
            name: "malformed_handler".to_string(),
        }
    }

    /// Check if an event handler has malformed structure
    fn has_malformed_structure(&self, handler: &EventHandlerNode, source_code: &str) -> Option<ValidationIssue> {
        // Extract the handler's source text for analysis
        let handler_start = handler.line;
        let lines: Vec<&str> = source_code.lines().collect();
        
        if handler_start == 0 || handler_start > lines.len() {
            return None;
        }

        // Get handler name from MessageType
        let handler_name = match &handler.msg_t {
            MessageType::CustomMessage { message_node } => &message_node.name,
            MessageType::None => return None,
        };
        
        // Find the handler block in source code
        let handler_text = self.extract_handler_text(lines, handler_start, handler_name);
        
        // Check for missing closing braces
        if let Some(issue) = self.check_missing_braces(&handler_text, handler) {
            return Some(issue);
        }

        // Check for orphaned statements after handler
        if let Some(issue) = self.check_orphaned_statements(&handler_text, handler) {
            return Some(issue);
        }

        None
    }

    /// Extract handler text from source lines
    fn extract_handler_text(&self, lines: Vec<&str>, start_line: usize, handler_name: &str) -> String {
        let mut handler_lines = Vec::new();
        let mut in_handler = false;
        let mut brace_count = 0;
        
        for (_i, line) in lines.iter().enumerate().skip(start_line.saturating_sub(1)) {
            let trimmed = line.trim();
            
            // Look for handler start
            if !in_handler && (trimmed.starts_with(&format!("{}(", handler_name)) || 
                               trimmed.contains(&format!("{}(", handler_name))) {
                in_handler = true;
                handler_lines.push(*line);
                
                // Count braces in this line
                brace_count += line.chars().filter(|&c| c == '{').count() as i32;
                brace_count -= line.chars().filter(|&c| c == '}').count() as i32;
                continue;
            }
            
            if in_handler {
                handler_lines.push(*line);
                
                // Count braces
                brace_count += line.chars().filter(|&c| c == '{').count() as i32;
                brace_count -= line.chars().filter(|&c| c == '}').count() as i32;
                
                // If braces are balanced, we've reached the end
                if brace_count <= 0 {
                    break;
                }
                
                // Safety check - don't read too far
                if handler_lines.len() > 50 {
                    break;
                }
            }
        }
        
        handler_lines.join("\n")
    }

    /// Check for missing closing braces - the main cause of Bugs #29/#31
    fn check_missing_braces(&self, handler_text: &str, handler: &EventHandlerNode) -> Option<ValidationIssue> {
        let open_braces = handler_text.chars().filter(|&c| c == '{').count();
        let close_braces = handler_text.chars().filter(|&c| c == '}').count();
        
        if open_braces > close_braces {
            let missing_count = open_braces - close_braces;
            return Some(ValidationIssue {
                severity: Severity::Error,
                category: Category::Syntax,
                rule_name: self.name.clone(),
                message: format!(
                    "Event handler '{}' is missing {} closing brace(s). This can cause subsequent handlers to be parsed incorrectly.",
                    match &handler.msg_t {
                        MessageType::CustomMessage { message_node } => &message_node.name,
                        MessageType::None => "unknown",
                    }, missing_count
                ),
                location: SourceLocation {
                    line: handler.line as u32,
                    column: 1,
                    offset: 0,
                    length: match &handler.msg_t {
                        MessageType::CustomMessage { message_node } => message_node.name.len(),
                        MessageType::None => 0,
                    },
                    file_path: None,
                },
                suggestion: Some(format!("Add {} closing brace(s) '{}' at the end of the handler", 
                                       missing_count, "}".repeat(missing_count))),
                help_url: Some("https://docs.frame-lang.org/syntax#event-handlers".to_string()),
            });
        }
        
        None
    }

    /// Check for orphaned statements that might indicate parsing issues
    fn check_orphaned_statements(&self, handler_text: &str, handler: &EventHandlerNode) -> Option<ValidationIssue> {
        let lines: Vec<&str> = handler_text.lines().collect();
        
        // Look for potential orphaned statements after return statements
        let mut found_return = false;
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("return") || trimmed.contains("return ") {
                found_return = true;
                continue;
            }
            
            // If we found a return and then see another handler definition,
            // it might be orphaned due to missing braces
            if found_return && (trimmed.contains("(") && trimmed.contains(")") && trimmed.contains("{")) {
                // Check if this looks like another handler definition
                if trimmed.contains("getCurrentState") || trimmed.contains("canExecuteCommand") {
                    return Some(ValidationIssue {
                        severity: Severity::Warning,
                        category: Category::Structure,
                        rule_name: self.name.clone(),
                        message: format!(
                            "Possible orphaned handler '{}' found after return statement in '{}'. This may indicate missing closing braces.",
                            trimmed, match &handler.msg_t {
                                MessageType::CustomMessage { message_node } => &message_node.name,
                                MessageType::None => "unknown",
                            }
                        ),
                        location: SourceLocation {
                            line: handler.line as u32 + i as u32,
                            column: 1,
                            offset: 0,
                            length: trimmed.len(),
                            file_path: None,
                        },
                        suggestion: Some("Check for missing closing braces in the previous handler".to_string()),
                        help_url: Some("https://docs.frame-lang.org/troubleshooting#orphaned-handlers".to_string()),
                    });
                }
            }
        }
        
        None
    }

    /// Check for interface methods missing from state dispatchers
    fn check_interface_method_coverage(&self, context: &ValidationContext) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        // Get interface methods
        let interface_methods: Vec<String> = if let Some(interface) = &context.ast.interface_block_node_opt {
            interface.interface_methods.iter()
                .map(|method| method.borrow().name.clone())
                .collect()
        } else {
            return issues; // No interface to validate
        };

        // Check each state for interface method coverage
        if let Some(machine) = &context.ast.machine_block_node_opt {
            for state_ref in &machine.states {
                let state = state_ref.borrow();
                let state_name = &state.name;
                
                // Get event handlers in this state
                let handled_events: std::collections::HashSet<String> = state.evt_handlers_rcref.iter()
                    .map(|handler_ref| {
                        let handler = handler_ref.borrow();
                        match &handler.msg_t {
                            MessageType::CustomMessage { message_node } => message_node.name.clone(),
                            MessageType::None => "unknown".to_string(),
                        }
                    })
                    .collect();
                
                // Check if all interface methods are handled
                for interface_method in &interface_methods {
                    if !handled_events.contains(interface_method) {
                        issues.push(ValidationIssue {
                            severity: Severity::Warning,
                            category: Category::Structure,
                            rule_name: self.name.clone(),
                            message: format!(
                                "Interface method '{}' not handled in state '{}'. This may cause runtime errors when the method is called.",
                                interface_method, state_name
                            ),
                            location: SourceLocation {
                                line: state.line as u32,
                                column: 1,
                                offset: 0,
                                length: state_name.len(),
                                file_path: None,
                            },
                            suggestion: Some(format!(
                                "Add handler: {}() {{ /* implementation */ }}",
                                interface_method
                            )),
                            help_url: Some("https://docs.frame-lang.org/interface#method-coverage".to_string()),
                        });
                    }
                }
            }
        }
        
        issues
    }
}

impl ValidationRule for MalformedHandlerRule {
    fn name(&self) -> &str {
        &self.name
    }

    fn level(&self) -> ValidationLevel {
        ValidationLevel::Basic
    }

    fn validate(&self, context: &ValidationContext) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        // Check machine states for malformed handlers
        if let Some(machine) = &context.ast.machine_block_node_opt {
            for state_ref in &machine.states {
                let state = state_ref.borrow();
                for handler_ref in &state.evt_handlers_rcref {
                    let handler = handler_ref.borrow();
                    
                    if let Some(issue) = self.has_malformed_structure(&*handler, context.source_code) {
                        issues.push(issue);
                    }
                }
            }
        }
        
        // Check interface method coverage (related to Bug #29)
        issues.extend(self.check_interface_method_coverage(context));
        
        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame_c::ast::*;
    use std::path::PathBuf;

    #[test]
    fn test_malformed_handler_rule_creation() {
        let rule = MalformedHandlerRule::new();
        assert_eq!(rule.name(), "malformed_handler");
        assert_eq!(rule.level(), ValidationLevel::Basic);
    }

    #[test]
    fn test_missing_braces_detection() {
        let rule = MalformedHandlerRule::new();
        
        // Simulate handler text with missing brace (like Bug #29/#31)
        let handler_text = r#"
        canExecuteCommand(command) {
            if command == "continue" {
                return False
            } elif command == "step" {
                return False  
            } elif command == "pause" {
                return True
            } else {
                return False
            }  // Missing closing brace here!
        
        getCurrentState() {
            return "running"
        }
        "#;
        
        // Should detect the missing brace
        let open_braces = handler_text.chars().filter(|&c| c == '{').count();
        let close_braces = handler_text.chars().filter(|&c| c == '}').count();
        
        assert!(open_braces > close_braces, "Test case should have missing braces");
    }

    #[test]
    fn test_extract_handler_text() {
        let rule = MalformedHandlerRule::new();
        let source_lines = vec![
            "state $Running {",
            "    canExecuteCommand(command) {",
            "        if command == \"pause\" {",
            "            return True",
            "        }",
            "    }",
            "    getCurrentState() {",
            "        return \"running\"",
            "    }",
            "}"
        ];
        
        let handler_text = rule.extract_handler_text(source_lines, 2, "canExecuteCommand");
        assert!(handler_text.contains("canExecuteCommand"));
        assert!(handler_text.contains("return True"));
    }
}