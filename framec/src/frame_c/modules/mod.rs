// Module system public API for Frame v0.57
// Provides multi-file module support with import resolution and dependency management

pub mod cache;
pub mod compiler;
pub mod errors;
pub mod graph;
pub mod linker;
pub mod resolver;

// Re-export core types for easy access
pub use cache::{CacheMetadata, ExportedSymbols, ModuleCache, ModuleInfo};
pub use compiler::{CompiledModule, ModuleExports, MultiFileCompiler};
pub use errors::{ModuleError, ModuleErrorKind, ModuleResult};
pub use graph::{CompilationStatus, DependencyGraph, ModuleNode};
pub use linker::{LinkingStrategy, ModuleLinker};
pub use resolver::{ModuleResolver, ModuleType, ResolvedModule};
