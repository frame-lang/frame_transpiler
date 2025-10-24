// Source mapping system using markers for accurate line tracking
// v0.73 - Fixed state/event handler source mappings

use regex::Regex;
use std::collections::HashMap;

/// Types of AST nodes we track for source mapping
#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    EventHandler,
    Statement,
    Expression,
    StateTransition,
    FunctionDef,
    InterfaceMethod,
    Action,
    Operation,
    Variable,
    State,
}

/// Information about a source location in the Frame code
#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub frame_line: usize,
    pub frame_file: String,
    pub node_type: NodeType,
    pub description: String,
}

/// Registry that tracks mappings between markers and source locations
pub struct SourceMappingRegistry {
    mappings: HashMap<String, SourceLocation>,
    next_id: usize,
}

impl SourceMappingRegistry {
    pub fn new() -> Self {
        SourceMappingRegistry {
            mappings: HashMap::new(),
            next_id: 1,
        }
    }

    /// Generate a unique marker ID
    pub fn next_marker(&mut self) -> String {
        let marker = format!("##FRAME_MAP_{}##", self.next_id);
        self.next_id += 1;
        marker
    }

    /// Record a mapping between a marker and source location
    pub fn record(&mut self, marker: String, location: SourceLocation) {
        self.mappings.insert(marker, location);
    }

    /// Create and record a marker in one step
    pub fn create_marker(
        &mut self,
        frame_line: usize,
        node_type: NodeType,
        description: String,
    ) -> String {
        let marker = self.next_marker();
        self.record(
            marker.clone(),
            SourceLocation {
                frame_line,
                frame_file: String::new(), // Will be set by visitor
                node_type,
                description,
            },
        );
        marker
    }

    /// Get the source location for a marker
    pub fn get_location(&self, marker: &str) -> Option<&SourceLocation> {
        self.mappings.get(marker)
    }
}

/// Process marked code to produce clean code and source map
pub fn process_marked_code(
    marked_code: &str,
    registry: &SourceMappingRegistry,
) -> (String, Vec<(usize, usize)>) {
    let mut clean_code = String::new();
    let mut source_mappings = Vec::new();
    let mut current_output_line = 1;

    // Regex to find markers
    let marker_regex = Regex::new(r"##FRAME_MAP_\d+##").unwrap();

    for (input_line_num, line) in marked_code.lines().enumerate() {
        // Check if this line contains a marker
        if let Some(marker_match) = marker_regex.find(line) {
            let marker = marker_match.as_str();

            // Check if marker is alone on the line
            if line.trim() == marker {
                // Marker is alone - look up mapping and record it
                if let Some(source_loc) = registry.get_location(marker) {
                    // v0.73: Account for the blank line before the marker
                    // The def appears on the line AFTER current_output_line due to the blank line
                    source_mappings.push((source_loc.frame_line, current_output_line + 1));

                    if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                        eprintln!("DEBUG v0.71: Found standalone marker {} at input line {} (1-indexed: {}), current_output_line={}, mapping Frame line {} to Python line {} (adjusted +1)", 
                                 marker, input_line_num, input_line_num + 1, current_output_line, source_loc.frame_line, current_output_line + 1);
                    }
                }
                // Skip this line entirely - don't add to output, and don't increment line counter
                continue;
            } else {
                // Marker is inline with code - shouldn't happen with our current approach
                // but handle it just in case
                if let Some(source_loc) = registry.get_location(marker) {
                    source_mappings.push((source_loc.frame_line, current_output_line));
                }
                // Remove marker and output the rest
                let clean_line = line.replace(marker, "");
                clean_code.push_str(&clean_line);
                clean_code.push('\n');
                current_output_line += 1;
            }
        } else {
            // No marker, output line as-is
            clean_code.push_str(line);
            clean_code.push('\n');
            current_output_line += 1;
        }
    }

    (clean_code, source_mappings)
}

/// Helper to strip all markers from code (for intermediate testing)
pub fn strip_markers(marked_code: &str) -> String {
    let marker_regex = Regex::new(r"##FRAME_MAP_\d+##").unwrap();
    marker_regex.replace_all(marked_code, "").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker_generation() {
        let mut registry = SourceMappingRegistry::new();

        let marker1 = registry.next_marker();
        let marker2 = registry.next_marker();

        assert_eq!(marker1, "##FRAME_MAP_1##");
        assert_eq!(marker2, "##FRAME_MAP_2##");
        assert_ne!(marker1, marker2);
    }

    #[test]
    fn test_process_marked_code() {
        let mut registry = SourceMappingRegistry::new();

        // Create some markers
        let marker1 =
            registry.create_marker(10, NodeType::FunctionDef, "main function".to_string());
        let marker2 =
            registry.create_marker(11, NodeType::Statement, "print statement".to_string());

        // Create marked code
        let marked_code = format!("{}def main():\n    {}print('hello')\n", marker1, marker2);

        // Process it
        let (clean_code, mappings) = process_marked_code(&marked_code, &registry);

        // Check results
        assert_eq!(clean_code, "def main():\n    print('hello')\n");
        assert_eq!(mappings.len(), 2);
        assert_eq!(mappings[0], (10, 1)); // Frame line 10 -> Python line 1
        assert_eq!(mappings[1], (11, 2)); // Frame line 11 -> Python line 2
    }
}
