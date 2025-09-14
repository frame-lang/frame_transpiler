// Module caching system for Frame v0.57
// Provides incremental compilation through intelligent caching

use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use super::errors::{ModuleError, ModuleErrorKind, ModuleResult};

/// Module cache manager for incremental compilation
pub struct ModuleCache {
    /// Cache directory path
    cache_dir: PathBuf,
    
    /// In-memory cache of loaded modules
    memory_cache: HashMap<String, CachedModule>,
}

/// A cached module with all metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedModule {
    /// Cache format version
    pub version: String,
    
    /// Frame compiler version that created this cache
    pub frame_version: String,
    
    /// Module information
    pub module_info: ModuleInfo,
    
    /// Exported symbols from this module
    pub exports: ExportedSymbols,
    
    /// Cache metadata for validation
    pub metadata: CacheMetadata,
}

/// Core module information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    /// Module path identifier
    pub module_path: String,
    
    /// Source file path
    pub source_file: PathBuf,
    
    /// Import declarations from this module
    pub imports: Vec<ImportInfo>,
    
    /// Module-level attributes and metadata
    pub attributes: HashMap<String, String>,
}

/// Information about an import statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportInfo {
    /// Import path as written in source
    pub import_path: String,
    
    /// Resolved file path
    pub resolved_path: PathBuf,
    
    /// What was imported (module, specific items, etc.)
    pub import_type: ImportType,
}

/// Type of import statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportType {
    /// Import entire module: `import Utils from "./utils.frm"`
    Module { alias: Option<String> },
    
    /// Import specific items: `import { add, multiply } from "./math.frm"`
    Selective { items: Vec<String> },
    
    /// Import all: `import * from "./helpers.frm"`
    All,
}

/// Symbols exported by a module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedSymbols {
    /// Exported functions
    pub functions: Vec<FunctionSignature>,
    
    /// Exported systems (state machines)
    pub systems: Vec<SystemSignature>,
    
    /// Exported classes
    pub classes: Vec<ClassSignature>,
    
    /// Exported enums
    pub enums: Vec<EnumSignature>,
    
    /// Exported type aliases
    pub type_aliases: Vec<TypeAliasSignature>,
    
    /// Exported module-level variables
    pub variables: Vec<VariableSignature>,
    
    /// Nested modules
    pub modules: Vec<ModuleSignature>,
}

/// Function signature for export tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionSignature {
    pub name: String,
    pub parameters: Vec<String>,
    pub return_type: Option<String>,
    pub is_async: bool,
}

/// System signature for export tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSignature {
    pub name: String,
    pub interface_methods: Vec<String>,
    pub states: Vec<String>,
}

/// Class signature for export tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassSignature {
    pub name: String,
    pub methods: Vec<String>,
    pub static_methods: Vec<String>,
    pub variables: Vec<String>,
}

/// Enum signature for export tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumSignature {
    pub name: String,
    pub variants: Vec<String>,
    pub enum_type: String, // "int", "string", etc.
}

/// Type alias signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAliasSignature {
    pub name: String,
    pub target_type: String,
}

/// Variable signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableSignature {
    pub name: String,
    pub var_type: Option<String>,
}

/// Module signature for nested modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleSignature {
    pub name: String,
    pub exports: ExportedSymbols,
}

/// Cache metadata for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetadata {
    /// SHA-256 hash of source file content
    pub source_hash: String,
    
    /// Last modification time of source file
    pub last_modified: SystemTime,
    
    /// Dependencies with their hashes for change detection
    pub dependencies: Vec<DependencyHash>,
    
    /// When this cache entry was created
    pub cached_at: SystemTime,
}

/// Dependency hash information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyHash {
    pub module_path: String,
    pub source_hash: String,
    pub last_modified: SystemTime,
}

impl ModuleCache {
    /// Create a new module cache
    pub fn new(cache_dir: PathBuf) -> Self {
        // Create cache directory if it doesn't exist (ignore errors for now)
        let _ = fs::create_dir_all(&cache_dir);
        
        Self {
            cache_dir,
            memory_cache: HashMap::new(),
        }
    }
    
    /// Get a cached module if it exists and is valid
    pub fn get(&self, _file_path: &Path, _content_hash: &str) -> ModuleResult<Option<super::compiler::CompiledModule>> {
        // For now, always return None (cache miss)
        // Full implementation will check disk cache
        Ok(None)
    }
    
    /// Store a compiled module in the cache
    pub fn put(&mut self, _file_path: &Path, _module: &super::compiler::CompiledModule) -> ModuleResult<()> {
        // For now, just store in memory cache
        // Full implementation will persist to disk
        Ok(())
    }
    
    /// Load a cached module if it exists and is valid
    pub fn load(&mut self, module_path: &str) -> ModuleResult<Option<CachedModule>> {
        // Check memory cache first
        if let Some(cached) = self.memory_cache.get(module_path) {
            return Ok(Some(cached.clone()));
        }
        
        // Try loading from disk
        let cache_file = self.cache_path(module_path);
        if !cache_file.exists() {
            return Ok(None);
        }
        
        let json = fs::read_to_string(&cache_file).map_err(|e| {
            ModuleError::new(
                ModuleErrorKind::CacheError {
                    reason: format!("Cannot read cache file: {}", e),
                },
                module_path.to_string(),
            )
        })?;
        
        let cached: CachedModule = serde_json::from_str(&json).map_err(|e| {
            ModuleError::new(
                ModuleErrorKind::CacheError {
                    reason: format!("Cannot parse cache file: {}", e),
                },
                module_path.to_string(),
            )
        })?;
        
        // Validate cache
        if !self.is_cache_valid(&cached)? {
            // Remove invalid cache file
            let _ = fs::remove_file(&cache_file);
            return Ok(None);
        }
        
        // Store in memory cache
        self.memory_cache.insert(module_path.to_string(), cached.clone());
        
        Ok(Some(cached))
    }
    
    /// Save a module to cache
    pub fn save(&mut self, module_path: &str, cached_module: &CachedModule) -> ModuleResult<()> {
        // Save to disk
        let cache_file = self.cache_path(module_path);
        
        // Ensure parent directory exists
        if let Some(parent) = cache_file.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                ModuleError::new(
                    ModuleErrorKind::CacheError {
                        reason: format!("Cannot create cache directory: {}", e),
                    },
                    module_path.to_string(),
                )
            })?;
        }
        
        let json = serde_json::to_string_pretty(cached_module).map_err(|e| {
            ModuleError::new(
                ModuleErrorKind::CacheError {
                    reason: format!("Cannot serialize cache: {}", e),
                },
                module_path.to_string(),
            )
        })?;
        
        fs::write(&cache_file, json).map_err(|e| {
            ModuleError::new(
                ModuleErrorKind::CacheError {
                    reason: format!("Cannot write cache file: {}", e),
                },
                module_path.to_string(),
            )
        })?;
        
        // Store in memory cache
        self.memory_cache.insert(module_path.to_string(), cached_module.clone());
        
        Ok(())
    }
    
    /// Check if a cached module is still valid
    fn is_cache_valid(&self, cached: &CachedModule) -> ModuleResult<bool> {
        // Check Frame version compatibility (major.minor must match)
        let current_version = env!("CARGO_PKG_VERSION");
        if !self.is_version_compatible(&cached.frame_version, current_version) {
            return Ok(false);
        }
        
        // Check source file hash
        let current_hash = Self::hash_file(&cached.module_info.source_file)?;
        if current_hash != cached.metadata.source_hash {
            return Ok(false);
        }
        
        // Check dependency hashes
        for dep in &cached.metadata.dependencies {
            let dep_path = PathBuf::from(&dep.module_path);
            if dep_path.exists() {
                let current_dep_hash = Self::hash_file(&dep_path)?;
                if current_dep_hash != dep.source_hash {
                    return Ok(false);
                }
            } else {
                // Dependency no longer exists
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Check version compatibility
    fn is_version_compatible(&self, cached_version: &str, current_version: &str) -> bool {
        // For now, require exact match
        // TODO: Implement semantic version comparison
        cached_version == current_version
    }
    
    /// Generate cache file path for a module
    fn cache_path(&self, module_path: &str) -> PathBuf {
        // Replace path separators and special characters with underscores
        let safe_name = module_path
            .replace("::", "_")
            .replace("/", "_")
            .replace("\\", "_")
            .replace(".", "_");
        
        self.cache_dir.join(format!("{}.frmc", safe_name))
    }
    
    /// Calculate SHA-256 hash of a file
    fn hash_file(path: &Path) -> ModuleResult<String> {
        let contents = fs::read(path).map_err(|e| {
            ModuleError::new(
                ModuleErrorKind::IoError {
                    path: path.to_path_buf(),
                    error: e.to_string(),
                },
                path.display().to_string(),
            )
        })?;
        
        let mut hasher = Sha256::new();
        hasher.update(contents);
        Ok(format!("{:x}", hasher.finalize()))
    }
    
    /// Clear all caches (both memory and disk)
    pub fn clear_all(&mut self) -> ModuleResult<()> {
        // Clear memory cache
        self.memory_cache.clear();
        
        // Remove cache directory
        if self.cache_dir.exists() {
            fs::remove_dir_all(&self.cache_dir).map_err(|e| {
                ModuleError::new(
                    ModuleErrorKind::CacheError {
                        reason: format!("Cannot clear cache directory: {}", e),
                    },
                    String::new(),
                )
            })?;
        }
        
        // Recreate cache directory
        fs::create_dir_all(&self.cache_dir).map_err(|e| {
            ModuleError::new(
                ModuleErrorKind::CacheError {
                    reason: format!("Cannot recreate cache directory: {}", e),
                },
                String::new(),
            )
        })?;
        
        Ok(())
    }
    
    /// Get cache statistics
    pub fn get_stats(&self) -> CacheStats {
        let disk_cache_count = self.cache_dir.read_dir()
            .map(|entries| entries.count())
            .unwrap_or(0);
        
        CacheStats {
            memory_cache_size: self.memory_cache.len(),
            disk_cache_size: disk_cache_count,
            cache_directory: self.cache_dir.clone(),
        }
    }
}

/// Cache statistics
#[derive(Debug)]
pub struct CacheStats {
    pub memory_cache_size: usize,
    pub disk_cache_size: usize,
    pub cache_directory: PathBuf,
}

impl Default for ModuleCache {
    fn default() -> Self {
        Self::new(PathBuf::from(".frame/cache"))
    }
}