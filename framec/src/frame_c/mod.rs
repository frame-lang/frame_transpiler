mod ast;
pub mod ast_serialize; // AST serialization for debugging (v0.60)
pub mod cli;
pub mod code_builder; // Robust line-aware code generation (v0.75)
pub mod compiler;
pub mod config; // Project configuration system (v0.58)
pub mod declaration_importers;
mod desugar; // Single-file desugaring pass (pseudo-symbol translations)
pub mod fid_cache;
pub mod llvm;
pub mod marker_linter; // Marker file linting for validation (v0.74)
mod module_partitioner; // ModulePartitioner: partitions module into Blocks + Regions
pub mod modules; // Multi-file module system (v0.57)
mod native_region_segmenter; // NativeRegionSegmenter: segments native regions into Native/Directive slices
mod region_scanner; // DFA/DPDA region scanners + queue for specialized native parsing
mod parser;
pub mod runtime_assets; // Shared runtime helpers (v0.87)
mod scanner; // Scanner (tokens) + NativeRegion (alias)
pub mod semantic_analyzer; // Semantic analysis for call resolution (v0.62)
pub mod source_map; // Source map generation for debugging (v0.59)
pub mod source_mapping; // Marker-based source mapping system (v0.73)
mod symbol_table;
mod target_discovery;
mod target_parsers;
pub mod tools; // CLI tools and utilities
pub mod utils;
pub mod validation; // Comprehensive validation system (v0.80.5)
mod visitors;
