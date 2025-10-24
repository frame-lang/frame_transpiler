// Interface Completeness Rule
// Validates that all interface methods have proper implementations

use crate::frame_c::validation::*;

pub struct InterfaceCompletenessRule {
    name: String,
}

impl InterfaceCompletenessRule {
    pub fn new() -> Self {
        Self {
            name: "interface_completeness".to_string(),
        }
    }
}

impl ValidationRule for InterfaceCompletenessRule {
    fn name(&self) -> &str {
        &self.name
    }

    fn level(&self) -> ValidationLevel {
        ValidationLevel::Structural
    }

    fn validate(&self, _context: &ValidationContext) -> Vec<ValidationIssue> {
        // TODO: Implement interface completeness validation
        // This will be expanded in Phase 1B
        Vec::new()
    }
}
