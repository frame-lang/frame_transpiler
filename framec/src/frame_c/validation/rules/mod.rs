// Validation Rules Module
// All validation rules organized by level and category

pub mod event_flow_analysis;
pub mod interface_completeness;
pub mod malformed_handler;
pub mod return_value_consistency;
pub mod state_reachability;
pub mod unmatched_braces;

// Re-export all rules for easy access
pub use event_flow_analysis::EventFlowAnalysisRule;
pub use interface_completeness::InterfaceCompletenessRule;
pub use malformed_handler::MalformedHandlerRule;
pub use return_value_consistency::ReturnValueConsistencyRule;
pub use state_reachability::StateReachabilityRule;
pub use unmatched_braces::UnmatchedBracesRule;

// Module for validation rules
