mod ast;
pub mod ast_serialize; // AST serialization for debugging (v0.60)
pub mod cli;
pub mod compiler;
pub mod config;     // Project configuration system (v0.58)
pub mod modules;     // Multi-file module system (v0.57)
mod parser;
mod scanner;
pub mod semantic_analyzer; // Semantic analysis for call resolution (v0.62)
pub mod source_map; // Source map generation for debugging (v0.59)
mod symbol_table;
pub mod utils;
mod visitors;
