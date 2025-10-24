// CLI tool for linting Frame transpiler marker files
// Usage: lint_markers <marked_python_file> [--json]

use crate::frame_c::marker_linter::MarkerLinter;
use serde::{Deserialize, Serialize};
use std::fs;
use std::process;

#[derive(Serialize, Deserialize)]
struct LintReport {
    file: String,
    total_markers: usize,
    total_mappings: usize,
    errors: Vec<String>,
    warnings: Vec<String>,
    status: String,
}

pub fn lint_marker_file(file_path: &str, json_output: bool) -> Result<(), String> {
    // Read the marked Python file
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    // Create and run linter
    let mut linter = MarkerLinter::new();
    linter.parse_marked_file(&content);

    // Also check for blank line mappings
    linter.check_blank_line_mappings(&content);

    // Run lint checks
    let lint_result = linter.lint();

    if json_output {
        // Generate JSON report
        let report = LintReport {
            file: file_path.to_string(),
            total_markers: linter.markers.len(),
            total_mappings: linter.mappings.len(),
            errors: linter.get_errors().iter().map(|e| e.to_string()).collect(),
            warnings: linter.get_warnings().to_vec(),
            status: if lint_result.is_ok() {
                "success".to_string()
            } else {
                "error".to_string()
            },
        };

        let json = serde_json::to_string_pretty(&report)
            .map_err(|e| format!("Failed to generate JSON: {}", e))?;
        println!("{}", json);
    } else {
        // Generate human-readable report
        println!("Frame Marker File Linter");
        println!("========================");
        println!("File: {}", file_path);
        println!();
        println!("{}", linter.generate_report());

        if lint_result.is_err() {
            println!(
                "\n❌ Linting failed with {} errors",
                linter.get_errors().len()
            );
            process::exit(1);
        }
    }

    Ok(())
}

// Integration with transpiler's debug mode
pub fn lint_during_transpilation(
    marked_content: &str,
    source_file: &str,
    frame_ast: Option<&crate::frame_c::ast::FrameModule>,
) -> Result<(), Vec<String>> {
    let mut linter = MarkerLinter::new();
    linter.parse_marked_file(marked_content);

    // If we have AST, we can do more sophisticated checks
    if let Some(ast) = frame_ast {
        // Check that all event handlers have mappings
        validate_ast_mappings(&mut linter, ast)?;
    }

    linter.check_blank_line_mappings(marked_content);

    match linter.lint() {
        Ok(_) => {
            if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                eprintln!("✅ Marker linting passed for {}", source_file);
            }
            Ok(())
        }
        Err(errors) => {
            let error_strings: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
            eprintln!("⚠️  Marker linting issues in {}:", source_file);
            for error in &error_strings {
                eprintln!("   - {}", error);
            }
            Err(error_strings)
        }
    }
}

fn validate_ast_mappings(
    linter: &mut MarkerLinter,
    ast: &crate::frame_c::ast::FrameModule,
) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    // Check all systems
    for system in &ast.systems {
        if let Some(machine) = &system.machine_block_node_opt {
            // Check all states
            for state_ref in &machine.states {
                let state = state_ref.borrow();
                // Event handlers should have mappings
                for handler_ref in &state.evt_handlers_rcref {
                    let handler = handler_ref.borrow();

                    // Skip state declarations - they don't generate executable code
                    // Only event handlers need mappings
                    if let Err(e) = linter.validate_event_handler_mapping(
                        &state.name,
                        "event_handler", // Simplified for now
                        handler.line,
                    ) {
                        errors.push(e);
                    }
                }
            }
        }
    }

    // Check all functions
    for function_ref in &ast.functions {
        let function = function_ref.borrow();
        // Functions should have mappings at their definition line
        if !linter.mappings.contains_key(&function.line) {
            errors.push(format!(
                "Function '{}' at line {} has no mapping",
                function.name, function.line
            ));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lint_valid_file() {
        let content = r#"
# Frame transpiler output
def my_function():
    # __MARKER_001__
    print("Hello")
    return
"#;
        let mut linter = MarkerLinter::new();
        linter.parse_marked_file(content);
        linter.resolve_marker("001", 10, 3);

        assert!(linter.lint().is_ok());
    }

    #[test]
    fn test_lint_with_errors() {
        let content = r#"
# __MARKER_001__
def foo():
    # __MARKER_001__  # Duplicate!
    pass
"#;
        let mut linter = MarkerLinter::new();
        linter.parse_marked_file(content);

        assert!(linter.lint().is_err());
        assert_eq!(linter.get_errors().len(), 2); // Duplicate + unresolved
    }
}
