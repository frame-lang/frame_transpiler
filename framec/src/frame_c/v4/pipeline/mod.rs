//! Frame V4 Pipeline Infrastructure
//!
//! This module provides the core compilation pipeline infrastructure:
//! - `traits.rs`: RegionScanner trait and scanner factory
//! - `config.rs`: PipelineConfig and CompileMode enum
//! - `compiler.rs`: Main compilation logic extracted from mod.rs
//!
//! This replaces the duplicated scanner dispatch code that was spread across mod.rs.

pub mod traits;
pub mod config;
pub mod compiler;

pub use traits::{get_region_scanner, RegionScannerTrait};
pub use config::{
    PipelineConfig, CompileMode, TrailerConfig, ValidationConfig,
    CodegenBackend, UsageStats,
    record_v3_compile, record_v4_compile,
    get_usage_stats, reset_usage_stats, print_usage_report,
};
pub use compiler::{compile_module, compile_ast_based, CompileResult, CompileError};
