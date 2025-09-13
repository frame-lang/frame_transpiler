// Module system public API for Frame v0.57
// Provides multi-file module support with import resolution and dependency management

pub mod resolver;
pub mod graph;
pub mod cache;
pub mod linker;
pub mod errors;

// Re-export core types for easy access
pub use resolver::{ModuleResolver, ResolvedModule, ModuleType};
pub use graph::{DependencyGraph, ModuleNode, CompilationStatus};
pub use cache::{ModuleCache, ModuleInfo, ExportedSymbols, CacheMetadata};
pub use linker::{ModuleLinker, CompiledModule, LinkingStrategy};
pub use errors::{ModuleError, ModuleErrorKind};

use std::path::PathBuf;
use crate::config::FrameConfig;

/// Multi-file compilation coordinator
pub struct MultiFileCompiler {
    resolver: ModuleResolver,
    graph: DependencyGraph,
    cache: ModuleCache,
    linker: ModuleLinker,
}

impl MultiFileCompiler {
    /// Create a new multi-file compiler with configuration
    pub fn new(config: &FrameConfig) -> Result<Self, ModuleError> {
        Ok(Self {
            resolver: ModuleResolver::new(config)?,
            graph: DependencyGraph::new(),
            cache: ModuleCache::new()?,
            linker: ModuleLinker::new(LinkingStrategy::Concatenation),
        })
    }
    
    /// Compile a multi-file Frame project starting from entry point
    pub fn compile(&mut self, entry_point: &PathBuf) -> Result<String, ModuleError> {
        // Phase 1: Discover all modules and build dependency graph
        self.discover_modules(entry_point)?;
        
        // Phase 2: Get compilation order (topological sort)
        let compilation_order = self.graph.compilation_order()?;
        
        // Phase 3: Compile each module in dependency order
        let mut compiled_modules = Vec::new();
        for module_path in compilation_order {
            let compiled = self.compile_module(&module_path)?;
            compiled_modules.push(compiled);
        }
        
        // Phase 4: Link all modules into final output
        self.linker.link_modules(compiled_modules)
    }
    
    fn discover_modules(&mut self, _entry_point: &PathBuf) -> Result<(), ModuleError> {
        // Implementation will be added in later phases
        todo!("Module discovery implementation")
    }
    
    fn compile_module(&mut self, _module_path: &str) -> Result<CompiledModule, ModuleError> {
        // Implementation will be added in later phases
        todo!("Module compilation implementation")
    }
}