// Module path resolver for Frame v0.57
// Resolves import statements to actual file system paths

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs;
use crate::config::FrameConfig;
use super::errors::{ModuleError, ModuleErrorKind, ModuleResult};

/// Module resolver that converts import paths to file system paths
pub struct ModuleResolver {
    /// Search paths in priority order
    search_paths: Vec<PathBuf>,
    
    /// Cache of resolved module paths
    resolution_cache: HashMap<String, ResolvedModule>,
    
    /// Project root directory for security validation
    project_root: PathBuf,
}

/// A resolved module with its metadata
#[derive(Debug, Clone)]
pub struct ResolvedModule {
    /// Original import path from source
    pub import_path: String,
    
    /// Resolved filesystem path
    pub fs_path: PathBuf,
    
    /// Module type classification
    pub module_type: ModuleType,
}

/// Type classification for resolved modules
#[derive(Debug, Clone)]
pub enum ModuleType {
    /// Local .frm file within project
    LocalFile,
    
    /// Future: External package
    Package {
        name: String,
        version: String,
    },
}

impl ModuleResolver {
    /// Create a new module resolver with configuration
    pub fn new(config: &FrameConfig) -> ModuleResult<Self> {
        let mut search_paths = Vec::new();
        
        // Add configured source directories
        for dir in &config.build.source_dirs {
            search_paths.push(dir.clone());
        }
        
        // Add project root as fallback
        search_paths.push(config.project.root.clone());
        
        Ok(Self {
            search_paths,
            resolution_cache: HashMap::new(),
            project_root: config.project.root.clone(),
        })
    }
    
    /// Resolve an import path to a filesystem path
    pub fn resolve(&mut self, import_path: &str, from_file: &Path) -> ModuleResult<ResolvedModule> {
        // Check cache first
        let cache_key = format!("{}:{}", from_file.display(), import_path);
        if let Some(resolved) = self.resolution_cache.get(&cache_key) {
            return Ok(resolved.clone());
        }
        
        // Try resolution strategies in order
        let resolved = self.try_relative_path(import_path, from_file)
            .or_else(|_| self.try_search_paths(import_path))?;
        
        // Validate security constraints
        self.validate_path(&resolved)?;
        
        // Cache the result
        self.resolution_cache.insert(cache_key, resolved.clone());
        
        Ok(resolved)
    }
    
    /// Try resolving as relative path
    fn try_relative_path(&self, import_path: &str, from_file: &Path) -> ModuleResult<ResolvedModule> {
        if !import_path.starts_with("./") && !import_path.starts_with("../") {
            return Err(ModuleError::new(
                ModuleErrorKind::InvalidPath {
                    path: import_path.to_string(),
                    reason: "Not a relative path".to_string(),
                },
                import_path.to_string(),
            ));
        }
        
        let from_dir = from_file.parent().ok_or_else(|| {
            ModuleError::new(
                ModuleErrorKind::InvalidPath {
                    path: from_file.display().to_string(),
                    reason: "No parent directory".to_string(),
                },
                import_path.to_string(),
            )
        })?;
        
        let candidate_path = from_dir.join(import_path);
        
        if candidate_path.exists() {
            Ok(ResolvedModule {
                import_path: import_path.to_string(),
                fs_path: candidate_path,
                module_type: ModuleType::LocalFile,
            })
        } else {
            Err(ModuleError::not_found(
                import_path.to_string(),
                vec![from_dir.to_path_buf()],
            ))
        }
    }
    
    /// Try resolving using configured search paths
    fn try_search_paths(&self, import_path: &str) -> ModuleResult<ResolvedModule> {
        let mut searched_paths = Vec::new();
        
        for search_dir in &self.search_paths {
            let candidate_path = search_dir.join(import_path);
            searched_paths.push(search_dir.clone());
            
            if candidate_path.exists() {
                return Ok(ResolvedModule {
                    import_path: import_path.to_string(),
                    fs_path: candidate_path,
                    module_type: ModuleType::LocalFile,
                });
            }
        }
        
        Err(ModuleError::not_found(import_path.to_string(), searched_paths))
    }
    
    /// Validate that the resolved path is safe and within project bounds
    fn validate_path(&self, resolved: &ResolvedModule) -> ModuleResult<()> {
        // Ensure file has .frm extension
        if let Some(ext) = resolved.fs_path.extension() {
            if ext != "frm" {
                return Err(ModuleError::new(
                    ModuleErrorKind::InvalidPath {
                        path: resolved.import_path.clone(),
                        reason: "Only .frm files can be imported".to_string(),
                    },
                    resolved.import_path.clone(),
                ));
            }
        } else {
            return Err(ModuleError::new(
                ModuleErrorKind::InvalidPath {
                    path: resolved.import_path.clone(),
                    reason: "File must have .frm extension".to_string(),
                },
                resolved.import_path.clone(),
            ));
        }
        
        // Prevent path traversal attacks
        let canonical = resolved.fs_path.canonicalize().map_err(|e| {
            ModuleError::new(
                ModuleErrorKind::InvalidPath {
                    path: resolved.import_path.clone(),
                    reason: format!("Cannot canonicalize path: {}", e),
                },
                resolved.import_path.clone(),
            )
        })?;
        
        // Ensure path is within project root
        if !canonical.starts_with(&self.project_root) {
            return Err(ModuleError::security_violation(
                resolved.import_path.clone(),
                "Path escapes project root directory".to_string(),
            ));
        }
        
        // Check for suspicious symlinks
        if resolved.fs_path.is_symlink() {
            let target = fs::read_link(&resolved.fs_path).map_err(|e| {
                ModuleError::new(
                    ModuleErrorKind::InvalidPath {
                        path: resolved.import_path.clone(),
                        reason: format!("Cannot read symlink: {}", e),
                    },
                    resolved.import_path.clone(),
                )
            })?;
            
            // Don't allow symlinks to system directories
            let target_str = target.to_string_lossy();
            if target_str.starts_with("/etc") || 
               target_str.starts_with("/usr") ||
               target_str.starts_with("/var") ||
               target_str.starts_with("/System") {
                return Err(ModuleError::security_violation(
                    resolved.import_path.clone(),
                    "Symlink points to system directory".to_string(),
                ));
            }
        }
        
        Ok(())
    }
    
    /// Clear the resolution cache (useful for development/testing)
    pub fn clear_cache(&mut self) {
        self.resolution_cache.clear();
    }
    
    /// Get statistics about resolver performance
    pub fn get_stats(&self) -> ResolverStats {
        ResolverStats {
            cache_size: self.resolution_cache.len(),
            search_paths: self.search_paths.clone(),
        }
    }
}

/// Statistics about resolver performance
#[derive(Debug)]
pub struct ResolverStats {
    pub cache_size: usize,
    pub search_paths: Vec<PathBuf>,
}