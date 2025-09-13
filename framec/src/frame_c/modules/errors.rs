// Module system error types for Frame v0.57
// Provides comprehensive error handling for multi-file compilation

use std::fmt;
use std::path::PathBuf;
use std::error::Error;

/// Top-level module system error
#[derive(Debug)]
pub struct ModuleError {
    pub kind: ModuleErrorKind,
    pub module_path: String,
    pub source_location: Option<SourceLocation>,
    pub import_chain: Vec<String>,
    pub suggestions: Vec<String>,
}

/// Specific types of module errors
#[derive(Debug)]
pub enum ModuleErrorKind {
    /// Module file not found during resolution
    NotFound {
        path: String,
        searched_paths: Vec<PathBuf>,
    },
    /// Circular dependency detected
    CircularDependency {
        cycle: Vec<String>,
    },
    /// Symbol conflict between modules
    SymbolConflict {
        symbol: String,
        conflicting_modules: Vec<String>,
    },
    /// Invalid import path
    InvalidPath {
        path: String,
        reason: String,
    },
    /// Security violation (path traversal attempt)
    SecurityViolation {
        path: String,
        reason: String,
    },
    /// Module version incompatibility
    IncompatibleVersion {
        required: String,
        found: String,
    },
    /// Cache corruption or invalid format
    CacheError {
        reason: String,
    },
    /// I/O error during module operations
    IoError {
        path: PathBuf,
        error: String,
    },
}

/// Source code location for error reporting
#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
}

impl ModuleError {
    /// Create a new module error with suggestions
    pub fn new(kind: ModuleErrorKind, module_path: String) -> Self {
        Self {
            kind,
            module_path,
            source_location: None,
            import_chain: Vec::new(),
            suggestions: Vec::new(),
        }
    }
    
    /// Add source location context
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.source_location = Some(location);
        self
    }
    
    /// Add import chain context
    pub fn with_import_chain(mut self, chain: Vec<String>) -> Self {
        self.import_chain = chain;
        self
    }
    
    /// Add helpful suggestions
    pub fn with_suggestions(mut self, suggestions: Vec<String>) -> Self {
        self.suggestions = suggestions;
        self
    }
    
    /// Create a "not found" error with search paths
    pub fn not_found(path: String, searched_paths: Vec<PathBuf>) -> Self {
        Self::new(
            ModuleErrorKind::NotFound { path: path.clone(), searched_paths },
            path,
        )
    }
    
    /// Create a circular dependency error
    pub fn circular_dependency(cycle: Vec<String>) -> Self {
        let path = cycle.first().cloned().unwrap_or_default();
        Self::new(
            ModuleErrorKind::CircularDependency { cycle },
            path,
        )
    }
    
    /// Create a security violation error
    pub fn security_violation(path: String, reason: String) -> Self {
        Self::new(
            ModuleErrorKind::SecurityViolation { path: path.clone(), reason },
            path,
        )
    }
}

impl fmt::Display for ModuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use colored::*;
        
        // Main error message
        writeln!(f, "{}: {}", "Error".red().bold(), self.kind)?;
        
        // Source location if available
        if let Some(ref loc) = self.source_location {
            writeln!(f, "  {} {}:{}:{}", 
                "--&gt;".blue(), 
                loc.file.display(), 
                loc.line, 
                loc.column)?;
        }
        
        // Import chain if relevant
        if !self.import_chain.is_empty() {
            writeln!(f, "\n{}", "Import chain:".yellow())?;
            for (i, module) in self.import_chain.iter().enumerate() {
                writeln!(f, "  {} {}", 
                    format!("{}.", i + 1).dimmed(), 
                    module)?;
            }
        }
        
        // Helpful suggestions
        if !self.suggestions.is_empty() {
            writeln!(f, "\n{}", "Suggestions:".green())?;
            for suggestion in &self.suggestions {
                writeln!(f, "  • {}", suggestion)?;
            }
        }
        
        Ok(())
    }
}

impl fmt::Display for ModuleErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ModuleErrorKind::NotFound { path, searched_paths } => {
                write!(f, "Module not found: '{}'", path)?;
                if !searched_paths.is_empty() {
                    write!(f, "\n  Searched in:")?;
                    for search_path in searched_paths {
                        write!(f, "\n    {}", search_path.display())?;
                    }
                }
                Ok(())
            },
            ModuleErrorKind::CircularDependency { cycle } => {
                write!(f, "Circular dependency detected: ")?;
                for (i, module) in cycle.iter().enumerate() {
                    if i > 0 { write!(f, " → ")?; }
                    write!(f, "{}", module)?;
                }
                write!(f, " → {}", cycle.first().unwrap_or(&String::new()))?;
                Ok(())
            },
            ModuleErrorKind::SymbolConflict { symbol, conflicting_modules } => {
                write!(f, "Symbol '{}' conflicts between modules: {}", 
                    symbol, conflicting_modules.join(", "))
            },
            ModuleErrorKind::InvalidPath { path, reason } => {
                write!(f, "Invalid import path '{}': {}", path, reason)
            },
            ModuleErrorKind::SecurityViolation { path, reason } => {
                write!(f, "Security violation in path '{}': {}", path, reason)
            },
            ModuleErrorKind::IncompatibleVersion { required, found } => {
                write!(f, "Version mismatch: required {}, found {}", required, found)
            },
            ModuleErrorKind::CacheError { reason } => {
                write!(f, "Cache error: {}", reason)
            },
            ModuleErrorKind::IoError { path, error } => {
                write!(f, "I/O error for '{}': {}", path.display(), error)
            },
        }
    }
}

impl Error for ModuleError {}

// Conversion from standard I/O errors
impl From<std::io::Error> for ModuleError {
    fn from(error: std::io::Error) -> Self {
        Self::new(
            ModuleErrorKind::IoError {
                path: PathBuf::new(),
                error: error.to_string(),
            },
            String::new(),
        )
    }
}

/// Result type for module operations
pub type ModuleResult<T> = Result<T, ModuleError>;