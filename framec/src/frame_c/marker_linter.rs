// Frame Transpiler Marker File Linter
// Validates intermediate marked Python files for source mapping correctness

use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct MarkerInfo {
    pub marker_id: String,
    pub python_line: usize,
    pub frame_line: Option<usize>, // Set when marker is resolved
}

#[derive(Debug, Clone)]
pub enum LintError {
    DuplicateMarker {
        marker_id: String,
        first_line: usize,
        second_line: usize,
    },
    UnresolvedMarker {
        marker_id: String,
        python_line: usize,
    },
    OrphanedMarker {
        marker_id: String,
        python_line: usize,
    },
    ConflictingMappings {
        frame_line: usize,
        python_lines: Vec<usize>,
    },
    MissingCriticalMapping {
        description: String,
        frame_line: usize,
    },
    OutOfOrderMapping {
        frame_line1: usize,
        python_line1: usize,
        frame_line2: usize,
        python_line2: usize,
    },
}

impl fmt::Display for LintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LintError::DuplicateMarker {
                marker_id,
                first_line,
                second_line,
            } => {
                write!(
                    f,
                    "Duplicate marker '{}' found at Python lines {} and {}",
                    marker_id, first_line, second_line
                )
            }
            LintError::UnresolvedMarker {
                marker_id,
                python_line,
            } => {
                write!(
                    f,
                    "Unresolved marker '{}' at Python line {} - no Frame line mapping",
                    marker_id, python_line
                )
            }
            LintError::OrphanedMarker {
                marker_id,
                python_line,
            } => {
                write!(
                    f,
                    "Orphaned marker '{}' at Python line {} - marker exists but was never used",
                    marker_id, python_line
                )
            }
            LintError::ConflictingMappings {
                frame_line,
                python_lines,
            } => {
                write!(
                    f,
                    "Frame line {} maps to multiple Python lines: {:?}",
                    frame_line, python_lines
                )
            }
            LintError::MissingCriticalMapping {
                description,
                frame_line,
            } => {
                write!(
                    f,
                    "Missing critical mapping: {} at Frame line {}",
                    description, frame_line
                )
            }
            LintError::OutOfOrderMapping {
                frame_line1,
                python_line1,
                frame_line2,
                python_line2,
            } => {
                write!(f, "Out-of-order mapping: Frame {} -> Python {} comes before Frame {} -> Python {}, but Python lines are reversed",
                       frame_line1, python_line1, frame_line2, python_line2)
            }
        }
    }
}

pub struct MarkerLinter {
    pub markers: HashMap<String, MarkerInfo>,
    pub mappings: HashMap<usize, Vec<usize>>, // frame_line -> vec of python_lines
    errors: Vec<LintError>,
    warnings: Vec<String>,
}

impl MarkerLinter {
    pub fn new() -> Self {
        Self {
            markers: HashMap::new(),
            mappings: HashMap::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Parse a marked Python file and extract all markers
    pub fn parse_marked_file(&mut self, content: &str) {
        for (line_num, line) in content.lines().enumerate() {
            let python_line = line_num + 1; // Convert to 1-based

            // Look for markers in comments: # __MARKER_123__
            const MARKER_PREFIX: &str = "# __MARKER_";
            if let Some(marker_start) = line.find(MARKER_PREFIX) {
                let rest = &line[marker_start + MARKER_PREFIX.len()..];
                if let Some(marker_end) = rest.find("__") {
                    let marker_id = rest[..marker_end].to_string();

                    // Check for duplicate markers
                    if let Some(existing) = self.markers.get(&marker_id) {
                        self.errors.push(LintError::DuplicateMarker {
                            marker_id: marker_id.clone(),
                            first_line: existing.python_line,
                            second_line: python_line,
                        });
                    } else {
                        self.markers.insert(
                            marker_id.clone(),
                            MarkerInfo {
                                marker_id,
                                python_line,
                                frame_line: None,
                            },
                        );
                    }
                }
            }
        }
    }

    /// Add a resolved mapping from Frame line to Python line
    pub fn add_mapping(&mut self, frame_line: usize, python_line: usize) {
        self.mappings
            .entry(frame_line)
            .or_insert_with(Vec::new)
            .push(python_line);
    }

    /// Resolve a marker with its Frame line
    pub fn resolve_marker(&mut self, marker_id: &str, frame_line: usize, python_line: usize) {
        if let Some(marker) = self.markers.get_mut(marker_id) {
            marker.frame_line = Some(frame_line);
            self.add_mapping(frame_line, python_line);
        } else {
            self.errors.push(LintError::OrphanedMarker {
                marker_id: marker_id.to_string(),
                python_line,
            });
        }
    }

    /// Run all lint checks
    pub fn lint(&mut self) -> Result<(), Vec<LintError>> {
        self.check_unresolved_markers();
        self.check_conflicting_mappings();
        self.check_mapping_order();
        self.check_critical_mappings();

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    /// Check for markers that were never resolved
    fn check_unresolved_markers(&mut self) {
        for (marker_id, info) in &self.markers {
            if info.frame_line.is_none() {
                self.errors.push(LintError::UnresolvedMarker {
                    marker_id: marker_id.clone(),
                    python_line: info.python_line,
                });
            }
        }
    }

    /// Check for Frame lines that map to multiple Python lines
    fn check_conflicting_mappings(&mut self) {
        for (frame_line, python_lines) in &self.mappings {
            if python_lines.len() > 1 {
                // Remove exact duplicates first
                let unique_lines: HashSet<_> = python_lines.iter().cloned().collect();
                if unique_lines.len() > 1 {
                    self.errors.push(LintError::ConflictingMappings {
                        frame_line: *frame_line,
                        python_lines: unique_lines.into_iter().collect(),
                    });
                }
            }
        }
    }

    /// Check that mappings maintain order (Frame line N should map to Python line <= Frame line N+1's Python line)
    fn check_mapping_order(&mut self) {
        let mut sorted_mappings: Vec<_> = self
            .mappings
            .iter()
            .filter_map(|(frame_line, python_lines)| {
                python_lines.first().map(|py_line| (*frame_line, *py_line))
            })
            .collect();
        sorted_mappings.sort_by_key(|(frame_line, _)| *frame_line);

        for window in sorted_mappings.windows(2) {
            let (frame1, python1) = window[0];
            let (frame2, python2) = window[1];

            // If Frame lines are in order but Python lines are reversed, that's an error
            if frame1 < frame2 && python1 > python2 {
                self.errors.push(LintError::OutOfOrderMapping {
                    frame_line1: frame1,
                    python_line1: python1,
                    frame_line2: frame2,
                    python_line2: python2,
                });
            }
        }
    }

    /// Check that critical constructs have mappings
    fn check_critical_mappings(&mut self) {
        // This would need AST information to be complete, but we can check patterns
        // For now, this is a placeholder that could be enhanced

        // Check if event handlers have mappings
        // Pattern: def __handle_*_enter should have a mapping
        // Pattern: def __handle_*_exit should have a mapping

        // This would need to be called with AST information about what Frame lines
        // contain event handlers, function definitions, etc.
    }

    /// Get all errors found
    pub fn get_errors(&self) -> &[LintError] {
        &self.errors
    }

    /// Get all warnings
    pub fn get_warnings(&self) -> &[String] {
        &self.warnings
    }

    /// Generate a report of all issues found
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        if self.errors.is_empty() && self.warnings.is_empty() {
            report.push_str("✅ No issues found in marker file\n");
            report.push_str(&format!("   Total markers: {}\n", self.markers.len()));
            report.push_str(&format!("   Total mappings: {}\n", self.mappings.len()));
        } else {
            if !self.errors.is_empty() {
                report.push_str(&format!("❌ {} errors found:\n", self.errors.len()));
                for error in &self.errors {
                    report.push_str(&format!("   - {}\n", error));
                }
                report.push_str("\n");
            }

            if !self.warnings.is_empty() {
                report.push_str(&format!("⚠️  {} warnings:\n", self.warnings.len()));
                for warning in &self.warnings {
                    report.push_str(&format!("   - {}\n", warning));
                }
            }
        }

        report
    }

    /// Validate a specific pattern in the marked file
    pub fn validate_event_handler_mapping(
        &self,
        state_name: &str,
        handler_type: &str,
        frame_line: usize,
    ) -> Result<(), String> {
        // Check if the frame line has a mapping
        if !self.mappings.contains_key(&frame_line) {
            return Err(format!(
                "Event handler {}::{} at Frame line {} has no mapping",
                state_name, handler_type, frame_line
            ));
        }

        // Could add more specific validation here
        Ok(())
    }

    /// Check if a Frame line maps to a blank Python line
    pub fn check_blank_line_mappings(&mut self, python_content: &str) {
        let python_lines: Vec<_> = python_content.lines().collect();

        for (frame_line, python_line_nums) in &self.mappings {
            for python_line in python_line_nums {
                if *python_line > 0 && *python_line <= python_lines.len() {
                    let line_content = python_lines[*python_line - 1].trim();
                    if line_content.is_empty() || line_content.starts_with('#') {
                        self.warnings.push(format!(
                            "Frame line {} maps to blank/comment Python line {}",
                            frame_line, python_line
                        ));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_markers() {
        let mut linter = MarkerLinter::new();
        let content = r#"
def foo():
    # __MARKER_123__
    print("hello")
    # __MARKER_456__
    return
"#;
        linter.parse_marked_file(content);
        assert_eq!(linter.markers.len(), 2);
        assert!(linter.markers.contains_key("123"));
        assert!(linter.markers.contains_key("456"));
    }

    #[test]
    fn test_duplicate_marker_detection() {
        let mut linter = MarkerLinter::new();
        let content = r#"
# __MARKER_123__
def foo():
    # __MARKER_123__
    print("hello")
"#;
        linter.parse_marked_file(content);
        assert_eq!(linter.errors.len(), 1);
        matches!(&linter.errors[0], LintError::DuplicateMarker { .. });
    }

    #[test]
    fn test_conflicting_mappings() {
        let mut linter = MarkerLinter::new();
        linter.add_mapping(10, 20);
        linter.add_mapping(10, 25); // Same Frame line, different Python line
        linter.check_conflicting_mappings();
        assert_eq!(linter.errors.len(), 1);
        matches!(&linter.errors[0], LintError::ConflictingMappings { .. });
    }

    #[test]
    fn test_out_of_order_mappings() {
        let mut linter = MarkerLinter::new();
        linter.add_mapping(10, 30);
        linter.add_mapping(11, 25); // Frame 11 maps to earlier Python line than Frame 10
        linter.check_mapping_order();
        assert_eq!(linter.errors.len(), 1);
        matches!(&linter.errors[0], LintError::OutOfOrderMapping { .. });
    }
}
