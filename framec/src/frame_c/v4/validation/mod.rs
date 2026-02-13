//! Frame V4 Validation Module
//!
//! This module provides a modular validation system for Frame ASTs.
//!
//! # Architecture
//!
//! The validation system uses a pass-based architecture:
//!
//! ```text
//! FrameAst ──→ ValidationRunner ──→ ValidationReport
//!                    │
//!                    ├── StructuralPass (E1xx, E41x)
//!                    ├── EventPass (E42x)
//!                    ├── TransitionPass (E43x)
//!                    ├── HsmPass (E44x)
//!                    ├── StackPass (E45x)
//!                    └── DomainPass (E46x)
//! ```
//!
//! # Usage
//!
//! ```rust,ignore
//! use crate::frame_c::v4::validation::{
//!     ValidationRunner, ValidationRunnerBuilder, ValidationConfig,
//! };
//!
//! // Create runner with default passes
//! let runner = ValidationRunnerBuilder::new()
//!     .warnings_as_errors(false)
//!     .build();
//!
//! // Run validation
//! let report = runner.run(&ast, &arcanum);
//!
//! // Check results
//! if report.has_errors() {
//!     eprintln!("{}", report.to_human_readable());
//! }
//! ```
//!
//! # Error Code Categories
//!
//! | Range | Category |
//! |-------|----------|
//! | E1xx | Structural/syntax errors |
//! | E40x | Basic semantic errors |
//! | E41x | Reachability/completeness |
//! | E42x | Event handling |
//! | E43x | Transition logic |
//! | E44x | HSM (hierarchical state machines) |
//! | E45x | Stack operations |
//! | E46x | Domain variables |

pub mod pass;
pub mod report;
pub mod runner;
pub mod types;

// Re-export main types
pub use pass::{ValidationContext, ValidationPass};
pub use report::{ValidationReport, ValidationSummary};
pub use runner::{ValidationRunner, ValidationRunnerBuilder};
pub use types::{Severity, ValidationConfig, ValidationIssue};

// Validation passes
pub mod passes;

// Re-export passes for convenience
pub use passes::{default_runner, SemanticPass, StructuralPass};
