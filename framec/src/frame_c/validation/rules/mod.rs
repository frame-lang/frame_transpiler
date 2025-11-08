// Validation Rules Module
// All validation rules organized by level and category

pub mod event_flow_analysis;
pub mod interface_completeness;
pub mod malformed_handler;
pub mod return_value_consistency;
pub mod state_reachability;
pub mod unmatched_braces;
pub mod async_consistency;
pub mod python_native_policy;
pub mod mixed_body_system_policy;
pub mod negative_patterns;
pub mod transitions_terminal;

// Re-export all rules for easy access
pub use event_flow_analysis::EventFlowAnalysisRule;
pub use interface_completeness::InterfaceCompletenessRule;
pub use malformed_handler::MalformedHandlerRule;
pub use return_value_consistency::ReturnValueConsistencyRule;
pub use state_reachability::StateReachabilityRule;
pub use unmatched_braces::UnmatchedBracesRule;
pub use python_native_policy::PythonNativePolicyRule;
pub use async_consistency::AsyncConsistencyRule;
pub use mixed_body_system_policy::MixedBodySystemPolicyRule;
pub use negative_patterns::NegativePatternsRule;
pub use transitions_terminal::TransitionsTerminalRule;

// Module for validation rules
