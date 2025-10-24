mod ast;
pub mod ast_serialize; // AST serialization for debugging (v0.60)
pub mod cli;
pub mod code_builder; // Robust line-aware code generation (v0.75)
pub mod compiler;
pub mod config; // Project configuration system (v0.58)
pub mod marker_linter; // Marker file linting for validation (v0.74)
pub mod modules; // Multi-file module system (v0.57)
mod parser;
mod scanner;
pub mod semantic_analyzer; // Semantic analysis for call resolution (v0.62)
pub mod source_map; // Source map generation for debugging (v0.59)
pub mod source_mapping; // Marker-based source mapping system (v0.73)
mod symbol_table;
pub mod tools; // CLI tools and utilities
pub mod utils;
pub mod validation; // Comprehensive validation system (v0.80.5)
mod visitors;
