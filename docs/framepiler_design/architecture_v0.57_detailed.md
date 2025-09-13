star# Frame Transpiler v0.57 Implementation Architecture

## Overview

This document provides the detailed technical architecture for Frame v0.57's multi-file module system implementation. It bridges the gap between the design vision and the concrete implementation plan.

## System Architecture

### Component Dependency Graph

```
┌─────────────────────────────────────────┐
│            CLI (cli.rs)                 │
│         framec build/compile             │
└─────────────┬───────────────────────────┘
              │
              ▼
┌─────────────────────────────────────────┐
│         Compiler (compiler.rs)          │
│    Orchestrates compilation pipeline    │
└──┬──────────┬────────────┬──────────────┘
   │          │            │
   ▼          ▼            ▼
┌──────────────────┐ ┌──────────────────┐ ┌──────────────────┐
│ Config Manager   │ │ Module Resolver   │ │ Dependency Graph │
│ (config.rs)      │ │ (resolver.rs)     │ │ (graph.rs)       │
└──────────────────┘ └──────────────────┘ └──────────────────┘
                            │
                            ▼
        ┌───────────────────────────────────────┐
        │         For Each Module               │
        └───────────────────────────────────────┘
                            │
        ┌───────────────────┴───────────────────┐
        ▼                                       ▼
┌──────────────────┐                 ┌──────────────────┐
│ Module Cache     │                 │ Module Parser    │
│ (cache.rs)       │                 │ (parser/mod.rs)  │
└──────────────────┘                 └──────────────────┘
        │                                       │
        └───────────────────┬───────────────────┘
                            ▼
                ┌──────────────────┐
                │ Symbol Resolver  │
                │ (symbol_table.rs)│
                └──────────────────┘
                            │
                            ▼
                ┌──────────────────┐
                │ Module Linker    │
                │ (linker.rs)      │
                └──────────────────┘
                            │
                            ▼
                ┌──────────────────┐
                │ Code Generator   │
                │ (python_visitor) │
                └──────────────────┘
```

## Core Module System Components

### 1. Module Resolver (`framec/src/frame_c/modules/resolver.rs`)

```rust
pub struct ModuleResolver {
    /// Search paths in priority order
    search_paths: Vec<PathBuf>,
    
    /// Cache of resolved module paths
    resolution_cache: HashMap<String, ResolvedModule>,
    
    /// Project root directory
    project_root: PathBuf,
}

pub struct ResolvedModule {
    /// Original import path from source
    import_path: String,
    
    /// Resolved filesystem path
    fs_path: PathBuf,
    
    /// Module type
    module_type: ModuleType,
}

pub enum ModuleType {
    /// Local .frm file
    LocalFile,
    
    /// Future: External package
    Package(String, Version),
}

impl ModuleResolver {
    /// Create resolver with config
    pub fn new(config: &FrameConfig) -> Self {
        let mut search_paths = vec![];
        
        // Add configured source directories
        for dir in &config.build.source_dirs {
            search_paths.push(dir.clone());
        }
        
        // Add project root
        search_paths.push(config.project_root.clone());
        
        Self {
            search_paths,
            resolution_cache: HashMap::new(),
            project_root: config.project_root.clone(),
        }
    }
    
    /// Resolve an import path to a filesystem path
    pub fn resolve(&mut self, import_path: &str, from_file: &Path) 
        -> Result<ResolvedModule, ModuleError> 
    {
        // Check cache first
        if let Some(resolved) = self.resolution_cache.get(import_path) {
            return Ok(resolved.clone());
        }
        
        // Try resolution strategies in order
        let resolved = self.try_relative_path(import_path, from_file)
            .or_else(|_| self.try_search_paths(import_path))?;
        
        // Validate and cache
        self.validate_path(&resolved)?;
        self.resolution_cache.insert(import_path.to_string(), resolved.clone());
        
        Ok(resolved)
    }
    
    fn validate_path(&self, module: &ResolvedModule) -> Result<(), ModuleError> {
        // Prevent path traversal attacks
        let canonical = module.fs_path.canonicalize()
            .map_err(|e| ModuleError::InvalidPath { 
                path: module.import_path.clone(),
                reason: e.to_string(),
            })?;
        
        // Ensure path is within project
        if !canonical.starts_with(&self.project_root) {
            return Err(ModuleError::SecurityViolation {
                path: module.import_path.clone(),
                reason: "Path escapes project root".to_string(),
            });
        }
        
        Ok(())
    }
}
```

### 2. Dependency Graph (`framec/src/frame_c/modules/graph.rs`)

```rust
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::toposort;

pub struct DependencyGraph {
    /// Directed graph of module dependencies
    graph: DiGraph<ModuleNode, ()>,
    
    /// Map from module path to graph node
    module_indices: HashMap<String, NodeIndex>,
}

pub struct ModuleNode {
    /// Module identifier
    path: String,
    
    /// Resolved filesystem path
    fs_path: PathBuf,
    
    /// Direct dependencies
    imports: Vec<String>,
    
    /// Compilation status
    status: CompilationStatus,
}

pub enum CompilationStatus {
    Pending,
    Parsing,
    Parsed,
    Compiled,
    Cached,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            module_indices: HashMap::new(),
        }
    }
    
    /// Add a module and its dependencies
    pub fn add_module(&mut self, path: String, fs_path: PathBuf, imports: Vec<String>) 
        -> Result<NodeIndex, ModuleError> 
    {
        // Get or create node for this module
        let module_idx = self.get_or_create_node(path.clone(), fs_path);
        
        // Add edges for dependencies
        for import in imports {
            let dep_idx = self.get_or_create_placeholder(import.clone());
            self.graph.add_edge(module_idx, dep_idx, ());
        }
        
        Ok(module_idx)
    }
    
    /// Get compilation order (topological sort)
    pub fn compilation_order(&self) -> Result<Vec<String>, ModuleError> {
        match toposort(&self.graph, None) {
            Ok(indices) => {
                let order: Vec<String> = indices.iter()
                    .map(|idx| self.graph[*idx].path.clone())
                    .collect();
                Ok(order)
            }
            Err(_) => {
                // Cycle detected - find and report it
                let cycle = self.find_cycle()?;
                Err(ModuleError::CircularDependency { chain: cycle })
            }
        }
    }
    
    fn find_cycle(&self) -> Result<Vec<String>, ModuleError> {
        // Use DFS to find a cycle
        use petgraph::visit::DfsPostOrder;
        
        // ... cycle detection implementation
        Ok(vec![]) // Placeholder
    }
}
```

### 3. Module Cache (`framec/src/frame_c/modules/cache.rs`)

```rust
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize)]
pub struct ModuleCache {
    /// Cache format version
    version: String,
    
    /// Frame compiler version
    frame_version: String,
    
    /// Module information
    module_info: ModuleInfo,
    
    /// Exported symbols
    exports: ExportedSymbols,
    
    /// Cache metadata
    metadata: CacheMetadata,
}

#[derive(Serialize, Deserialize)]
pub struct ModuleInfo {
    /// Module path (e.g., "Utils::Math")
    module_path: String,
    
    /// Source file path
    source_file: PathBuf,
    
    /// Import declarations
    imports: Vec<ImportInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct ExportedSymbols {
    functions: Vec<FunctionSignature>,
    systems: Vec<SystemSignature>,
    classes: Vec<ClassSignature>,
    enums: Vec<EnumSignature>,
    type_aliases: Vec<TypeAliasSignature>,
    variables: Vec<VariableSignature>,
}

#[derive(Serialize, Deserialize)]
pub struct CacheMetadata {
    /// SHA-256 hash of source file
    source_hash: String,
    
    /// Last modification time
    last_modified: SystemTime,
    
    /// Dependencies with their hashes
    dependencies: Vec<DependencyHash>,
}

impl ModuleCache {
    /// Load cache for a module
    pub fn load(module_path: &str) -> Result<Option<Self>, CacheError> {
        let cache_path = Self::cache_path(module_path);
        
        if !cache_path.exists() {
            return Ok(None);
        }
        
        let json = fs::read_to_string(&cache_path)?;
        let cache: ModuleCache = serde_json::from_str(&json)?;
        
        // Validate cache
        if !cache.is_valid()? {
            return Ok(None);
        }
        
        Ok(Some(cache))
    }
    
    /// Save cache for a module
    pub fn save(&self) -> Result<(), CacheError> {
        let cache_path = Self::cache_path(&self.module_info.module_path);
        
        // Ensure cache directory exists
        if let Some(parent) = cache_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&cache_path, json)?;
        
        Ok(())
    }
    
    /// Check if cache is valid
    pub fn is_valid(&self) -> Result<bool, CacheError> {
        // Check Frame version compatibility
        if !self.is_version_compatible()? {
            return Ok(false);
        }
        
        // Check source file hash
        let current_hash = Self::hash_file(&self.module_info.source_file)?;
        if current_hash != self.metadata.source_hash {
            return Ok(false);
        }
        
        // Check dependency hashes
        for dep in &self.metadata.dependencies {
            if !dep.is_valid()? {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    fn cache_path(module_path: &str) -> PathBuf {
        let safe_name = module_path.replace("::", "_");
        PathBuf::from(".frame/cache").join(format!("{}.frmc", safe_name))
    }
    
    fn hash_file(path: &Path) -> Result<String, CacheError> {
        let contents = fs::read(path)?;
        let mut hasher = Sha256::new();
        hasher.update(contents);
        Ok(format!("{:x}", hasher.finalize()))
    }
}
```

### 4. Module Linker (`framec/src/frame_c/modules/linker.rs`)

```rust
pub struct ModuleLinker {
    /// Modules in dependency order
    modules: Vec<CompiledModule>,
    
    /// Linking strategy
    strategy: LinkingStrategy,
}

pub struct CompiledModule {
    /// Module identifier
    module_path: String,
    
    /// Parsed AST
    ast: FrameModule,
    
    /// Symbol table
    symbols: Rc<RefCell<ModuleSymbol>>,
    
    /// Generated code (if already visited)
    generated_code: Option<String>,
}

pub enum LinkingStrategy {
    /// Simple concatenation in dependency order
    Concatenation,
    
    /// Future: Smart linking with optimization
    Optimized {
        dead_code_elimination: bool,
        inline_small_functions: bool,
    },
}

impl ModuleLinker {
    pub fn new(strategy: LinkingStrategy) -> Self {
        Self {
            modules: Vec::new(),
            strategy,
        }
    }
    
    /// Add a compiled module
    pub fn add_module(&mut self, module: CompiledModule) {
        self.modules.push(module);
    }
    
    /// Link all modules into final output
    pub fn link(&mut self, visitor: &mut dyn Visitor) -> Result<String, LinkError> {
        match self.strategy {
            LinkingStrategy::Concatenation => self.link_concatenation(visitor),
            LinkingStrategy::Optimized { .. } => self.link_optimized(visitor),
        }
    }
    
    fn link_concatenation(&mut self, visitor: &mut dyn Visitor) 
        -> Result<String, LinkError> 
    {
        let mut output = String::new();
        
        // Generate header (imports, runtime)
        output.push_str(&visitor.generate_header());
        
        // Process each module in dependency order
        for module in &mut self.modules {
            if module.generated_code.is_none() {
                // Visit AST to generate code
                visitor.visit_frame_module(&module.ast)?;
                module.generated_code = Some(visitor.get_code());
            }
            
            output.push_str(&module.generated_code.as_ref().unwrap());
            output.push_str("\n\n");
        }
        
        // Generate footer (main execution)
        output.push_str(&visitor.generate_footer());
        
        Ok(output)
    }
}
```

## Parser Modularization

### Directory Structure

```
framec/src/frame_c/parser/
├── mod.rs              # Public API and Parser struct
├── parser.rs           # Core parsing logic
├── expressions.rs      # Expression parsing
├── statements.rs       # Statement parsing
├── types.rs           # Type annotation parsing
├── imports.rs         # Import statement parsing
├── functions.rs       # Function parsing
├── systems.rs         # System parsing
├── classes.rs         # Class parsing
├── patterns.rs        # Pattern matching
└── errors.rs          # Error types and handling
```

### Import Parser (`parser/imports.rs`)

```rust
impl Parser {
    /// Parse import statement
    pub(super) fn parse_import(&mut self) -> Result<ImportNode, ParseError> {
        // import Module::Path from "source"
        // from "source" import item1, item2
        
        if self.match_token(&[TokenType::Import]) {
            self.parse_import_from()
        } else if self.match_token(&[TokenType::From]) {
            self.parse_from_import()
        } else {
            Err(self.error_at_current("Expected import or from"))
        }
    }
    
    fn parse_import_from(&mut self) -> Result<ImportNode, ParseError> {
        // import Module::Path [as Alias] from "source"
        
        let module_path = self.parse_module_path()?;
        
        let alias = if self.match_token(&[TokenType::As]) {
            Some(self.consume_identifier()?)
        } else {
            None
        };
        
        self.consume(TokenType::From, "Expected 'from'")?;
        let source = self.consume_string()?;
        
        Ok(ImportNode {
            module_path,
            source_type: self.determine_source_type(&source),
            imported_items: vec![],
            module_alias: alias,
            line: self.current_line(),
        })
    }
    
    fn parse_module_path(&mut self) -> Result<String, ParseError> {
        // Module::SubModule::Item
        let mut path = self.consume_identifier()?;
        
        while self.match_token(&[TokenType::DoubleColon]) {
            path.push_str("::");
            path.push_str(&self.consume_identifier()?);
        }
        
        Ok(path)
    }
    
    fn determine_source_type(&self, source: &str) -> ImportSource {
        if source.starts_with("./") || source.starts_with("../") {
            ImportSource::File(PathBuf::from(source))
        } else {
            // Future: package imports
            ImportSource::Package(source.to_string(), Version::new(0, 0, 0))
        }
    }
}
```

## Configuration System Refactor

### Simplified Config Structure

```rust
// framec/src/frame_c/config.rs (reduced from 794 to ~200 lines)

#[derive(Serialize, Deserialize)]
pub struct FrameConfig {
    pub project: ProjectConfig,
    pub build: BuildConfig,
    pub python: PythonConfig,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: Option<String>,
    pub version: Option<String>,
    pub entry: Option<PathBuf>,
    pub authors: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BuildConfig {
    pub output_dir: PathBuf,
    pub source_dirs: Vec<PathBuf>,
    pub target: TargetLanguage,
    pub optimize: bool,
    pub debug: bool,
    pub incremental: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PythonConfig {
    pub event_handlers_as_functions: bool,
    pub runtime: PythonRuntime,
    pub min_version: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum PythonRuntime {
    Standard,
    AsyncIO,
    Trio,  // Future
}

impl FrameConfig {
    /// Load configuration with migration support
    pub fn load(project_root: &Path) -> Result<Self, ConfigError> {
        // Try frame.toml first (new format)
        let toml_path = project_root.join("frame.toml");
        if toml_path.exists() {
            return Self::load_toml(&toml_path);
        }
        
        // Try config.yaml (legacy)
        let yaml_path = project_root.join("config.yaml");
        if yaml_path.exists() {
            eprintln!("Warning: config.yaml is deprecated. Run 'framec migrate-config' to update.");
            return Self::load_yaml_and_migrate(&yaml_path);
        }
        
        // Return defaults
        Ok(Self::default())
    }
}
```

## Compiler Orchestration

### Enhanced Compiler (`compiler.rs`)

```rust
impl Compiler {
    /// New multi-file build entry point
    pub fn build(&mut self, config: &FrameConfig) -> Result<(), CompileError> {
        // Determine entry point
        let entry = config.project.entry
            .as_ref()
            .ok_or(CompileError::NoEntryPoint)?;
        
        // Initialize components
        let mut resolver = ModuleResolver::new(config);
        let mut graph = DependencyGraph::new();
        let mut cache = CacheManager::new(&config.build.output_dir);
        let mut linker = ModuleLinker::new(LinkingStrategy::Concatenation);
        
        // Phase 1: Discover all modules
        self.discover_modules(entry, &mut resolver, &mut graph)?;
        
        // Phase 2: Get compilation order
        let compilation_order = graph.compilation_order()?;
        
        // Phase 3: Compile each module
        for module_path in compilation_order {
            let compiled = self.compile_module(
                &module_path,
                &mut resolver,
                &mut cache,
            )?;
            linker.add_module(compiled);
        }
        
        // Phase 4: Link and generate output
        let mut visitor = PythonVisitor::new(config);
        let output = linker.link(&mut visitor)?;
        
        // Phase 5: Write output
        let output_path = config.build.output_dir.join("output.py");
        fs::write(&output_path, output)?;
        
        Ok(())
    }
    
    fn compile_module(&mut self, 
        module_path: &str,
        resolver: &mut ModuleResolver,
        cache: &mut CacheManager,
    ) -> Result<CompiledModule, CompileError> 
    {
        // Check cache first
        if let Some(cached) = cache.get(module_path)? {
            return Ok(cached);
        }
        
        // Resolve and load source
        let resolved = resolver.resolve(module_path, &self.current_file)?;
        let source = fs::read_to_string(&resolved.fs_path)?;
        
        // Parse module
        let mut scanner = Scanner::new(&source);
        let tokens = scanner.scan_tokens()?;
        
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;
        
        // Build symbol table
        let symbols = self.build_symbols(&ast)?;
        
        // Cache the compiled module
        cache.store(module_path, &ast, &symbols)?;
        
        Ok(CompiledModule {
            module_path: module_path.to_string(),
            ast,
            symbols,
            generated_code: None,
        })
    }
}
```

## Error Handling Architecture

### Hierarchical Error System

```rust
// framec/src/frame_c/errors.rs

#[derive(Debug)]
pub enum FrameError {
    Parse(ParseError),
    Module(ModuleError),
    Symbol(SymbolError),
    Cache(CacheError),
    Link(LinkError),
    Config(ConfigError),
    IO(std::io::Error),
}

#[derive(Debug)]
pub struct ModuleError {
    pub kind: ModuleErrorKind,
    pub module_path: String,
    pub source_location: Option<SourceLocation>,
    pub import_chain: Vec<String>,
    pub suggestions: Vec<String>,
}

#[derive(Debug)]
pub enum ModuleErrorKind {
    NotFound,
    CircularDependency,
    SymbolConflict(String),
    InvalidPath(String),
    SecurityViolation(String),
    IncompatibleVersion,
}

impl fmt::Display for ModuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format with colors if terminal supports it
        use colored::*;
        
        writeln!(f, "{}: {}", "Error".red().bold(), self.kind)?;
        
        if let Some(loc) = &self.source_location {
            writeln!(f, "  {} {}:{}:{}", 
                "-->".blue(), 
                loc.file.display(), 
                loc.line, 
                loc.column)?;
        }
        
        if !self.import_chain.is_empty() {
            writeln!(f, "\n{}", "Import chain:".yellow())?;
            for (i, module) in self.import_chain.iter().enumerate() {
                writeln!(f, "  {} {}", 
                    format!("{}.", i + 1).dim(), 
                    module)?;
            }
        }
        
        if !self.suggestions.is_empty() {
            writeln!(f, "\n{}", "Did you mean:".green())?;
            for suggestion in &self.suggestions {
                writeln!(f, "  • {}", suggestion)?;
            }
        }
        
        Ok(())
    }
}
```

## CLI Integration

### Build Command (`cli.rs`)

```rust
#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a single Frame file (legacy)
    Compile {
        #[clap(short, long)]
        language: String,
        
        input: PathBuf,
    },
    
    /// Build a multi-file Frame project
    Build {
        /// Entry point (overrides frame.toml)
        #[clap(long)]
        entry: Option<PathBuf>,
        
        /// Watch for changes
        #[clap(long)]
        watch: bool,
        
        /// Enable experimental modules
        #[clap(long)]
        experimental_modules: bool,
    },
    
    /// Create a new Frame project
    New {
        name: String,
        
        /// Use template
        #[clap(long)]
        template: Option<String>,
    },
    
    /// Run tests
    Test {
        /// Test filter
        filter: Option<String>,
    },
    
    /// Migrate old config to frame.toml
    MigrateConfig,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Build { entry, watch, experimental_modules } => {
            if !experimental_modules {
                eprintln!("Multi-file support is experimental. Use --experimental-modules to enable.");
                std::process::exit(1);
            }
            
            let config = FrameConfig::load(&std::env::current_dir()?)?;
            let mut compiler = Compiler::new();
            
            if watch {
                watch_and_build(&mut compiler, &config)?;
            } else {
                compiler.build(&config)?;
            }
        }
        
        Commands::Compile { language, input } => {
            // Legacy single-file compilation
            let mut compiler = Compiler::new();
            compiler.compile_single_file(&input, &language)?;
        }
        
        // ... other commands
    }
    
    Ok(())
}
```

## Performance Optimization Strategies

### Incremental Compilation
1. **Module-level caching**: Cache parsed AST and symbols
2. **Hash-based invalidation**: Only recompile changed modules
3. **Dependency tracking**: Recompile dependents of changed modules

### Parallel Compilation (Future)
```rust
// Conceptual - not implemented in v0.57
use rayon::prelude::*;

fn compile_parallel(modules: Vec<String>) -> Result<Vec<CompiledModule>, Error> {
    modules.par_iter()
        .map(|module| compile_module(module))
        .collect()
}
```

### Memory Management
- Use `Rc<RefCell<>>` for shared AST nodes
- Lazy loading of symbol tables
- Stream processing for large files

## Testing Architecture

### Multi-File Test Framework

```python
# framec_tests/runner/multi_file_runner.py

class MultiFileTest:
    def __init__(self, project_dir: Path):
        self.project_dir = project_dir
        self.entry_point = self.find_entry_point()
        self.expected_output = self.load_expected()
        
    def run(self) -> TestResult:
        # Compile project
        result = subprocess.run(
            ['framec', 'build', '--experimental-modules'],
            cwd=self.project_dir,
            capture_output=True,
        )
        
        if result.returncode != 0:
            return TestResult.compile_error(result.stderr)
        
        # Run generated code
        output_file = self.project_dir / 'dist' / 'output.py'
        result = subprocess.run(
            ['python3', output_file],
            capture_output=True,
        )
        
        # Validate output
        if result.stdout.decode() == self.expected_output:
            return TestResult.success()
        else:
            return TestResult.output_mismatch(
                expected=self.expected_output,
                actual=result.stdout.decode()
            )
```

## Migration Support

### Config Migration Tool

```rust
// framec/src/frame_c/migrate.rs

pub fn migrate_config(old_path: &Path, new_path: &Path) -> Result<(), Error> {
    // Load old YAML config
    let yaml = fs::read_to_string(old_path)?;
    let old_config: legacy::OldFrameConfig = serde_yaml::from_str(&yaml)?;
    
    // Convert to new format
    let new_config = FrameConfig {
        project: ProjectConfig {
            name: Some(old_path.parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("unnamed")
                .to_string()),
            version: Some("0.1.0".to_string()),
            entry: None,
            authors: vec![],
        },
        build: BuildConfig {
            output_dir: PathBuf::from("dist"),
            source_dirs: vec![PathBuf::from("src")],
            target: TargetLanguage::Python3,
            optimize: false,
            debug: true,
            incremental: true,
        },
        python: PythonConfig {
            event_handlers_as_functions: old_config.codegen.python.code
                .event_handlers_as_functions,
            runtime: PythonRuntime::Standard,
            min_version: None,
        },
    };
    
    // Write new TOML config
    let toml = toml::to_string_pretty(&new_config)?;
    fs::write(new_path, toml)?;
    
    println!("✓ Migrated config.yaml to frame.toml");
    println!("  Review the new configuration and adjust as needed.");
    
    Ok(())
}
```

## Security Considerations

### Path Traversal Prevention
```rust
fn validate_import_path(path: &str, project_root: &Path) -> Result<(), SecurityError> {
    let resolved = Path::new(path).canonicalize()?;
    
    // Must be within project root
    if !resolved.starts_with(project_root) {
        return Err(SecurityError::PathEscapesRoot(path.to_string()));
    }
    
    // No symlinks to sensitive locations
    if resolved.is_symlink() {
        let target = fs::read_link(&resolved)?;
        if target.starts_with("/etc") || 
           target.starts_with("/usr") ||
           target.starts_with("/var") {
            return Err(SecurityError::SuspiciousSymlink(path.to_string()));
        }
    }
    
    Ok(())
}
```

## Backward Compatibility

### Single-File Mode Preservation
```rust
impl Compiler {
    /// Legacy single-file compilation
    pub fn compile_single_file(&mut self, path: &Path, target: &str) 
        -> Result<String, Error> 
    {
        let source = fs::read_to_string(path)?;
        let mut scanner = Scanner::new(&source);
        let tokens = scanner.scan_tokens()?;
        
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;
        
        let mut visitor = match target {
            "python_3" => Box::new(PythonVisitor::new_legacy()),
            _ => return Err(Error::UnsupportedTarget(target.to_string())),
        };
        
        visitor.visit_frame_module(&ast)?;
        Ok(visitor.get_code())
    }
}
```

## Monitoring and Debugging

### Debug Output
```rust
// Enable with FRAME_DEBUG=1
macro_rules! debug_log {
    ($($arg:tt)*) => {
        if std::env::var("FRAME_DEBUG").is_ok() {
            eprintln!("[DEBUG] {}", format!($($arg)*));
        }
    }
}

// Usage in resolver
debug_log!("Resolving module: {} from {}", import_path, from_file.display());
debug_log!("  Found at: {}", resolved_path.display());
```

### Build Profiling
```rust
use std::time::Instant;

struct BuildProfiler {
    phases: Vec<(String, Duration)>,
    current_phase: Option<(String, Instant)>,
}

impl BuildProfiler {
    fn start_phase(&mut self, name: &str) {
        if let Some((prev_name, start)) = self.current_phase.take() {
            self.phases.push((prev_name, start.elapsed()));
        }
        self.current_phase = Some((name.to_string(), Instant::now()));
    }
    
    fn report(&self) {
        println!("\nBuild Profile:");
        for (phase, duration) in &self.phases {
            println!("  {}: {:?}", phase, duration);
        }
    }
}
```

---

## Implementation Checklist

### Phase 1: Foundation (Week 1-2)
- [ ] Config system refactor
- [ ] Parser modularization
- [ ] Basic import parsing
- [ ] Module resolver skeleton

### Phase 2: Core (Week 3-4)
- [ ] Dependency graph implementation
- [ ] Circular dependency detection
- [ ] Module cache (JSON)
- [ ] Symbol table extensions

### Phase 3: Integration (Week 5-6)
- [ ] Module linker
- [ ] Multi-file compilation pipeline
- [ ] Error handling improvements
- [ ] CLI build command

### Phase 4: Testing (Week 7)
- [ ] Multi-file test framework
- [ ] Test suite migration
- [ ] Performance benchmarks
- [ ] Documentation

This architecture provides the complete technical blueprint for implementing Frame v0.57's multi-file module system.