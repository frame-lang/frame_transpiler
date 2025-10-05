// Validation Rules Module
// All validation rules organized by level and category

pub mod malformed_handler;
pub mod unmatched_braces;
pub mod interface_completeness;
pub mod state_reachability;
pub mod event_flow_analysis;
pub mod return_value_consistency;

// Re-export all rules for easy access
pub use malformed_handler::MalformedHandlerRule;
pub use unmatched_braces::UnmatchedBracesRule;
pub use interface_completeness::InterfaceCompletenessRule;
pub use state_reachability::StateReachabilityRule;
pub use event_flow_analysis::EventFlowAnalysisRule;
pub use return_value_consistency::ReturnValueConsistencyRule;

use super::*;