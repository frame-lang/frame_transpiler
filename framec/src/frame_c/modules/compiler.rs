// Multi-file compiler for Frame v0.57
// Orchestrates compilation of Frame projects with multiple files

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs;
use crate::frame_c::ast::{FrameModule, ImportNode, ImportType};
use crate::frame_c::parser::Parser;
use crate::frame_c::scanner::Scanner;
use crate::frame_c::symbol_table::Arcanum;
use crate::frame_c::config::FrameConfig;
use super::resolver::ModuleResolver;
use super::graph::DependencyGraph;
use super::cache::ModuleCache;
use super::linker::{ModuleLinker, LinkingStrategy};
use super::errors::{ModuleError, ModuleErrorKind, ModuleResult};
use sha2::{Digest, Sha256};

/// Multi-file compiler that handles Frame projects
pub struct MultiFileCompiler {
    /// Module path resolver
    resolver: ModuleResolver,
    
    /// Dependency graph for compilation ordering
    dependency_graph: DependencyGraph,
    
    /// Module cache for incremental compilation
    module_cache: ModuleCache,
    
    /// Module linker for final assembly
    linker: ModuleLinker,
    
    /// Configuration
    _config: FrameConfig,
    
    /// Parsed modules indexed by file path
    parsed_modules: HashMap<PathBuf, CompiledModule>,
}

/// A compiled module with all its metadata
pub struct CompiledModule {
    /// The parsed AST
    pub ast: FrameModule,
    
    /// Symbol table for this module
    pub symbols: Arcanum,
    
    /// File path of the module
    pub file_path: PathBuf,
    
    /// SHA-256 hash of source content
    pub content_hash: String,
    
    /// Exported symbols (functions, classes, systems)
    pub exports: ModuleExports,
    
    /// Dependencies (imported modules)
    pub dependencies: Vec<PathBuf>,
}

/// Exported symbols from a module
#[derive(Debug, Clone)]
pub struct ModuleExports {
    /// Exported functions
    pub functions: HashSet<String>,
    
    /// Exported classes
    pub classes: HashSet<String>,
    
    /// Exported systems
    pub systems: HashSet<String>,
    
    /// Exported enums
    pub enums: HashSet<String>,
    
    /// Exported variables
    pub variables: HashSet<String>,
    
    /// Nested modules
    pub modules: HashSet<String>,
    
    /// Nested module exports (module_name -> exports)
    pub nested_exports: HashMap<String, ModuleExports>,
}

impl ModuleExports {
    fn new() -> Self {
        Self {
            functions: HashSet::new(),
            classes: HashSet::new(),
            systems: HashSet::new(),
            enums: HashSet::new(),
            variables: HashSet::new(),
            modules: HashSet::new(),
            nested_exports: HashMap::new(),
        }
    }
    
    /// Extract exports from a parsed module
    fn from_ast(ast: &FrameModule) -> Self {
        let mut exports = Self::new();
        
        // Extract function names
        for func in &ast.functions {
            exports.functions.insert(func.borrow().name.clone());
        }
        
        // Extract class names
        for class in &ast.classes {
            exports.classes.insert(class.borrow().name.clone());
        }
        
        // Extract system names
        for system in &ast.systems {
            exports.systems.insert(system.name.clone());
        }
        
        // Extract enum names
        for enum_decl in &ast.enums {
            exports.enums.insert(enum_decl.borrow().name.clone());
        }
        
        // Extract module-level variables
        for var in &ast.variables {
            exports.variables.insert(var.borrow().name.clone());
        }
        
        // Extract nested modules and their exports
        for module in &ast.modules {
            let module_ref = module.borrow();
            let module_name = module_ref.name.clone();
            exports.modules.insert(module_name.clone());
            
            // Recursively extract exports from nested module
            let nested = Self::from_module_node(&module_ref);
            exports.nested_exports.insert(module_name, nested);
        }
        
        exports
    }
    
    /// Extract exports from a module node
    fn from_module_node(module_node: &crate::frame_c::ast::ModuleNode) -> Self {
        let mut exports = Self::new();
        
        // Extract functions from the module
        for func in &module_node.functions {
            exports.functions.insert(func.borrow().name.clone());
        }
        
        // Extract variables from the module
        for var in &module_node.variables {
            exports.variables.insert(var.borrow().name.clone());
        }
        
        // Extract nested modules recursively
        for nested_module in &module_node.modules {
            let nested_ref = nested_module.borrow();
            let nested_name = nested_ref.name.clone();
            exports.modules.insert(nested_name.clone());
            
            let nested_exports = Self::from_module_node(&nested_ref);
            exports.nested_exports.insert(nested_name, nested_exports);
        }
        
        exports
    }
    
    /// Check if a symbol is exported
    pub fn has_symbol(&self, name: &str) -> bool {
        self.functions.contains(name) ||
        self.classes.contains(name) ||
        self.systems.contains(name) ||
        self.enums.contains(name) ||
        self.variables.contains(name) ||
        self.modules.contains(name)
    }
}

impl MultiFileCompiler {
    /// Create a new multi-file compiler
    pub fn new(config: FrameConfig) -> ModuleResult<Self> {
        let resolver = ModuleResolver::new(&config)?;
        let dependency_graph = DependencyGraph::new();
        let cache_dir = config.build.output_dir.join(".cache");
        let module_cache = ModuleCache::new(cache_dir);
        let linker = ModuleLinker::new(LinkingStrategy::Concatenation);
        
        Ok(Self {
            resolver,
            dependency_graph,
            module_cache,
            linker,
            _config: config,
            parsed_modules: HashMap::new(),
        })
    }
    
    /// Create a multi-file compiler for a specific entry file
    pub fn new_for_entry(config: FrameConfig, entry_file: &Path) -> ModuleResult<Self> {
        let resolver = ModuleResolver::new_for_entry(&config, entry_file)?;
        let dependency_graph = DependencyGraph::new();
        let cache_dir = config.build.output_dir.join(".cache");
        let module_cache = ModuleCache::new(cache_dir);
        let linker = ModuleLinker::new(LinkingStrategy::Concatenation);
        
        Ok(Self {
            resolver,
            dependency_graph,
            module_cache,
            linker,
            _config: config,
            parsed_modules: HashMap::new(),
        })
    }
    
    /// Set the output directory for separate file generation
    pub fn set_output_dir(&mut self, output_dir: PathBuf) {
        self.linker = ModuleLinker::new(LinkingStrategy::SeparateFiles {
            output_dir,
            create_package: true,
        });
    }
    
    /// Compile a Frame project starting from an entry point
    pub fn compile(&mut self, entry_file: &Path) -> ModuleResult<String> {
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG: Starting multi-file compilation from {:?}", entry_file);
        }
        
        // Phase 1: Discover all modules
        self.discover_modules(entry_file)?;
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG: Discovered {} modules", self.parsed_modules.len());
        }
        
        // Phase 2: Build dependency graph
        self.build_dependency_graph()?;
        
        // Phase 3: Compile modules in dependency order
        let compilation_order = self.dependency_graph.get_build_order()?;
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG: Compilation order has {} modules", compilation_order.len());
        }
        for module_path in compilation_order {
            self.compile_module(&module_path)?;
        }
        
        // Phase 4: Validate imports
        self.validate_imports()?;
        
        // Phase 5: Link modules into final output
        let output = self.link_modules()?;
        
        Ok(output)
    }
    
    /// Discover all modules by recursively parsing imports
    fn discover_modules(&mut self, entry_file: &Path) -> ModuleResult<()> {
        let mut to_process = vec![entry_file.to_path_buf()];
        let mut processed = HashSet::new();
        
        while let Some(file_path) = to_process.pop() {
            if processed.contains(&file_path) {
                continue;
            }
            processed.insert(file_path.clone());
            
            // Parse the module to discover its imports
            let module = self.parse_module(&file_path)?;
            
            // Process imports to find more modules
            for import in &module.ast.imports {
                if let Some(import_path) = self.extract_frame_import_path(import) {
                    let resolved = self.resolver.resolve(&import_path, &file_path)?;
                    if !processed.contains(&resolved.fs_path) {
                        to_process.push(resolved.fs_path);
                    }
                }
            }
            
            // Store the parsed module
            self.parsed_modules.insert(file_path, module);
        }
        
        Ok(())
    }
    
    /// Parse a single Frame module file
    fn parse_module(&mut self, file_path: &Path) -> ModuleResult<CompiledModule> {
        // Check cache first
        let content = fs::read_to_string(file_path).map_err(|e| {
            ModuleError::new(
                ModuleErrorKind::IOError {
                    path: file_path.to_path_buf(),
                    error: e.to_string(),
                },
                file_path.display().to_string(),
            )
        })?;
        
        // Calculate content hash
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let content_hash = format!("{:x}", hasher.finalize());
        
        // Check if cached version is up to date
        if let Some(cached) = self.module_cache.get(file_path, &content_hash)? {
            return Ok(cached);
        }
        
        // Parse the module
        let scanner = Scanner::new(content);
        let (has_errors, errors, tokens) = scanner.scan_tokens();
        
        if has_errors {
            return Err(ModuleError::new(
                ModuleErrorKind::ParseError {
                    error: errors,
                },
                file_path.display().to_string(),
            ));
        }
        
        // Two-pass parsing for symbol table construction
        let mut arcanum = Arcanum::new();
        let mut comments = Vec::new();
        
        // First pass: build symbol table
        {
            let mut syntactic_parser = Parser::new(&tokens, &mut comments, true, arcanum);
            match syntactic_parser.parse() {
                Ok(_) => {
                    if syntactic_parser.had_error() {
                        return Err(ModuleError::new(
                            ModuleErrorKind::ParseError {
                                error: syntactic_parser.get_errors(),
                            },
                            file_path.display().to_string(),
                        ));
                    }
                    arcanum = syntactic_parser.get_arcanum();
                }
                Err(e) => {
                    return Err(ModuleError::new(
                        ModuleErrorKind::ParseError {
                            error: e.error,
                        },
                        file_path.display().to_string(),
                    ));
                }
            }
        }
        
        // Create a new arcanum for the second pass (preserving the symbol tables)
        let arcanum_for_semantic = arcanum;
        
        // Second pass: semantic analysis
        let mut comments2 = comments.clone();
        let mut semantic_parser = Parser::new(&tokens, &mut comments2, false, arcanum_for_semantic);
        
        let ast = match semantic_parser.parse() {
            Ok(module) => module,
            Err(e) => {
                return Err(ModuleError::new(
                    ModuleErrorKind::ParseError {
                        error: e.error,
                    },
                    file_path.display().to_string(),
                ));
            }
        };
        
        if semantic_parser.had_error() {
            return Err(ModuleError::new(
                ModuleErrorKind::ParseError {
                    error: semantic_parser.get_errors(),
                },
                file_path.display().to_string(),
            ));
        }
        
        // Get the final arcanum from the semantic parser
        let final_arcanum = semantic_parser.get_arcanum();
        
        // Extract exports
        let exports = ModuleExports::from_ast(&ast);
        
        // Extract dependencies
        let mut dependencies = Vec::new();
        for import in &ast.imports {
            if let Some(import_path) = self.extract_frame_import_path(import) {
                let resolved = self.resolver.resolve(&import_path, file_path)?;
                dependencies.push(resolved.fs_path);
            }
        }
        
        let compiled = CompiledModule {
            ast,
            symbols: final_arcanum,
            file_path: file_path.to_path_buf(),
            content_hash,
            exports,
            dependencies,
        };
        
        // Cache the compiled module
        self.module_cache.put(file_path, &compiled)?;
        
        Ok(compiled)
    }
    
    /// Extract Frame import path from an import node
    fn extract_frame_import_path(&self, import: &ImportNode) -> Option<String> {
        match &import.import_type {
            ImportType::FrameModule { file_path, .. } |
            ImportType::FrameModuleAliased { file_path, .. } |
            ImportType::FrameSelective { file_path, .. } => Some(file_path.clone()),
            _ => None,
        }
    }
    
    /// Build the dependency graph from parsed modules
    fn build_dependency_graph(&mut self) -> ModuleResult<()> {
        // First, add all modules to the graph
        for (path, module) in &self.parsed_modules {
            // Add the module itself to the graph with its imports
            let imports = module.dependencies.iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect();
            self.dependency_graph.add_module(
                path.to_string_lossy().to_string(),
                path.clone(),
                imports
            )?;
        }
        
        // Then add the dependency relationships
        for (path, module) in &self.parsed_modules {
            for dep_path in &module.dependencies {
                self.dependency_graph.add_dependency(path.clone(), dep_path.clone());
            }
        }
        
        // Check for cycles
        self.dependency_graph.check_cycles()?;
        
        Ok(())
    }
    
    /// Compile a single module (transform AST if needed)
    fn compile_module(&mut self, _module_path: &Path) -> ModuleResult<()> {
        // In this phase, we would perform any module-specific transformations
        // For now, the modules are already parsed and ready
        Ok(())
    }
    
    /// Validate that all imports can be resolved
    fn validate_imports(&mut self) -> ModuleResult<()> {
        for (path, module) in &self.parsed_modules {
            for import in &module.ast.imports {
                match &import.import_type {
                    ImportType::FrameModule { module_name, file_path } => {
                        let resolved = self.resolver.resolve(file_path, path)?;
                        let target_module = self.parsed_modules.get(&resolved.fs_path)
                            .ok_or_else(|| ModuleError::new(
                                ModuleErrorKind::ModuleNotFound {
                                    module: file_path.clone(),
                                    searched_paths: vec![path.clone()],
                                },
                                format!("Module file not found: {}", file_path),
                            ))?;
                        
                        // Validate the imported module/system/class exists
                        if !target_module.exports.has_symbol(module_name) {
                            // Check if any module/system/class exists in the target file
                            let available = self.get_available_symbols(&target_module.exports);
                            return Err(ModuleError::new(
                                ModuleErrorKind::ImportError {
                                    import: module_name.clone(),
                                    reason: format!(
                                        "Symbol '{}' not found in '{}'. Available symbols: {}",
                                        module_name, file_path, available
                                    ),
                                },
                                format!("In file: {}", path.display()),
                            ));
                        }
                    }
                    ImportType::FrameModuleAliased { module_name, file_path, .. } => {
                        let resolved = self.resolver.resolve(file_path, path)?;
                        let target_module = self.parsed_modules.get(&resolved.fs_path)
                            .ok_or_else(|| ModuleError::new(
                                ModuleErrorKind::ModuleNotFound {
                                    module: file_path.clone(),
                                    searched_paths: vec![path.clone()],
                                },
                                format!("Module file not found: {}", file_path),
                            ))?;
                        
                        // Validate the imported module/system/class exists
                        if !target_module.exports.has_symbol(module_name) {
                            let available = self.get_available_symbols(&target_module.exports);
                            return Err(ModuleError::new(
                                ModuleErrorKind::ImportError {
                                    import: module_name.clone(),
                                    reason: format!(
                                        "Symbol '{}' not found in '{}'. Available symbols: {}",
                                        module_name, file_path, available
                                    ),
                                },
                                format!("In file: {}", path.display()),
                            ));
                        }
                    }
                    ImportType::FrameSelective { items, file_path } => {
                        let resolved = self.resolver.resolve(file_path, path)?;
                        let target_module = self.parsed_modules.get(&resolved.fs_path)
                            .ok_or_else(|| ModuleError::new(
                                ModuleErrorKind::ModuleNotFound {
                                    module: file_path.clone(),
                                    searched_paths: vec![path.clone()],
                                },
                                file_path.clone(),
                            ))?;
                        
                        // Check each imported item exists
                        for item in items {
                            if !target_module.exports.has_symbol(item) {
                                let available = self.get_available_symbols(&target_module.exports);
                                return Err(ModuleError::new(
                                    ModuleErrorKind::ImportError {
                                        import: item.clone(),
                                        reason: format!(
                                            "Symbol '{}' not found in '{}'. Available symbols: {}",
                                            item, file_path, available
                                        ),
                                    },
                                    format!("In file: {}", path.display()),
                                ));
                            }
                        }
                    }
                    _ => {
                        // Python imports - no validation needed here
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Get a formatted list of available symbols for error messages
    fn get_available_symbols(&self, exports: &ModuleExports) -> String {
        let mut symbols = Vec::new();
        
        for func in &exports.functions {
            symbols.push(format!("function '{}'", func));
        }
        for class in &exports.classes {
            symbols.push(format!("class '{}'", class));
        }
        for system in &exports.systems {
            symbols.push(format!("system '{}'", system));
        }
        for module in &exports.modules {
            symbols.push(format!("module '{}'", module));
        }
        for enum_name in &exports.enums {
            symbols.push(format!("enum '{}'", enum_name));
        }
        for var in &exports.variables {
            symbols.push(format!("variable '{}'", var));
        }
        
        if symbols.is_empty() {
            "none".to_string()
        } else {
            symbols.join(", ")
        }
    }
    
    /// Link all modules into final output
    fn link_modules(&mut self) -> ModuleResult<String> {
        let compilation_order = self.dependency_graph.get_build_order()?;
        
        // Move modules from parsed_modules to avoid clone
        let mut modules_to_link = Vec::new();
        for path in compilation_order {
            if let Some(module) = self.parsed_modules.remove(&path) {
                modules_to_link.push(module);
            }
        }
        
        // Link modules together
        let output = self.linker.link(modules_to_link)?;
        
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;
    
    #[test]
    fn test_module_exports_extraction() {
        // This would require creating a test FrameModule
        // For now, we'll just test the structure
        let exports = ModuleExports::new();
        assert!(exports.functions.is_empty());
        assert!(exports.classes.is_empty());
        assert!(!exports.has_symbol("nonexistent"));
    }
    
    #[test]
    fn test_multi_file_compiler_creation() {
        let config = FrameConfig::default();
        let compiler = MultiFileCompiler::new(config);
        assert!(compiler.is_ok());
    }
}