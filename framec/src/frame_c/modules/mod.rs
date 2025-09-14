// Module system public API for Frame v0.57
// Provides multi-file module support with import resolution and dependency management

pub mod resolver;
pub mod graph;
pub mod cache;
pub mod linker;
pub mod errors;
pub mod compiler;

// Re-export core types for easy access
pub use resolver::{ModuleResolver, ResolvedModule, ModuleType};
pub use graph::{DependencyGraph, ModuleNode, CompilationStatus};
pub use cache::{ModuleCache, ModuleInfo, ExportedSymbols, CacheMetadata};
pub use linker::{ModuleLinker, LinkingStrategy};
pub use errors::{ModuleError, ModuleErrorKind, ModuleResult};
pub use compiler::{MultiFileCompiler, CompiledModule, ModuleExports};