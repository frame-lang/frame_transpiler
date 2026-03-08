//! Validation passes for Frame V4
//!
//! Each pass performs a specific category of validation:
//!
//! - `StructuralPass` - E1xx errors: section order, duplicates
//! - `SemanticPass` - E4xx errors: states, transitions, forwards
//!
//! Future passes (per VALIDATION_EXPANSION_PLAN.md):
//! - `ReachabilityPass` - E41x: unreachable states, dead ends
//! - `EventPass` - E42x: unhandled events, dead handlers
//! - `TransitionPass` - E43x: transition logic errors
//! - `HsmPass` - E44x: hierarchical state machine errors
//! - `StackPass` - E45x: push/pop errors
//! - `DomainPass` - E46x: domain variable errors

pub mod semantic;
pub mod structural;

pub use semantic::SemanticPass;
pub use structural::StructuralPass;

use super::runner::ValidationRunner;
use super::types::ValidationConfig;

/// Create a ValidationRunner with all default passes
pub fn default_runner(config: ValidationConfig) -> ValidationRunner {
    let mut runner = ValidationRunner::new(config);
    runner.register_pass(StructuralPass);
    runner.register_pass(SemanticPass);
    runner
}

/// Create a ValidationRunner with structural passes only
pub fn structural_only(config: ValidationConfig) -> ValidationRunner {
    let mut runner = ValidationRunner::new(config);
    runner.register_pass(StructuralPass);
    runner
}

/// Create a ValidationRunner with semantic passes only
pub fn semantic_only(config: ValidationConfig) -> ValidationRunner {
    let mut runner = ValidationRunner::new(config);
    runner.register_pass(SemanticPass);
    runner
}
