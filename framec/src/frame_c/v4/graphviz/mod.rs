/// GraphViz DOT backend for Frame V4.
///
/// Bypasses CodegenNode IR (designed for imperative languages) and builds
/// a graph-specific IR (SystemGraph) directly from SystemAst + Arcanum.
///
/// Architecture: SystemAst + Arcanum → SystemGraph → DOT text

pub mod ir;
pub mod builder;
pub mod dot_emitter;

pub use builder::build_system_graph;
pub use dot_emitter::{emit_dot, emit_multi_system};
