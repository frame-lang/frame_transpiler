// Simplified configuration system for Frame v0.57+
// Focuses on build configuration and Python-specific options

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The simplified Frame configuration structure (v0.57+)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FrameConfig {
    #[serde(default)]
    pub project: ProjectConfig,
    
    #[serde(default)]
    pub build: BuildConfig,
    
    #[serde(default)]
    pub python: PythonConfig,
}

impl Default for FrameConfig {
    fn default() -> Self {
        FrameConfig {
            project: ProjectConfig::default(),
            build: BuildConfig::default(),
            python: PythonConfig::default(),
        }
    }
}

/// Project metadata configuration
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Project name
    pub name: Option<String>,
    
    /// Project version
    pub version: Option<String>,
    
    /// Entry point for multi-file compilation
    pub entry: Option<PathBuf>,
    
    /// Project authors
    #[serde(default)]
    pub authors: Vec<String>,
    
    /// Project description
    pub description: Option<String>,
}

/// Build configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Output directory for generated code
    #[serde(default = "default_output_dir")]
    pub output_dir: PathBuf,
    
    /// Source directories to search for modules
    #[serde(default = "default_source_dirs")]
    pub source_dirs: Vec<PathBuf>,
    
    /// Enable optimizations
    #[serde(default)]
    pub optimize: bool,
    
    /// Enable debug output
    #[serde(default)]
    pub debug: bool,
    
    /// Enable incremental compilation (future)
    #[serde(default = "default_incremental")]
    pub incremental: bool,
}

impl Default for BuildConfig {
    fn default() -> Self {
        BuildConfig {
            output_dir: default_output_dir(),
            source_dirs: default_source_dirs(),
            optimize: false,
            debug: false,
            incremental: default_incremental(),
        }
    }
}

fn default_output_dir() -> PathBuf {
    PathBuf::from("dist")
}

fn default_source_dirs() -> Vec<PathBuf> {
    vec![PathBuf::from("src")]
}

fn default_incremental() -> bool {
    true
}

/// Python-specific configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PythonConfig {
    /// Generate event handlers as individual functions (v0.36 feature)
    #[serde(default = "default_event_handlers")]
    pub event_handlers_as_functions: bool,
    
    /// Python runtime to target
    #[serde(default)]
    pub runtime: PythonRuntime,
    
    /// Minimum Python version required
    pub min_version: Option<String>,
    
    /// Generate public state info (rarely used)
    #[serde(default)]
    pub public_state_info: bool,
    
    /// Generate public compartment (rarely used)
    #[serde(default)]
    pub public_compartment: bool,
}

impl Default for PythonConfig {
    fn default() -> Self {
        PythonConfig {
            event_handlers_as_functions: default_event_handlers(),
            runtime: PythonRuntime::default(),
            min_version: None,
            public_state_info: false,
            public_compartment: false,
        }
    }
}

fn default_event_handlers() -> bool {
    true  // v0.36 default
}

/// Python runtime options
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum PythonRuntime {
    /// Standard Python runtime
    #[default]
    Standard,
    
    /// AsyncIO runtime
    AsyncIO,
    
    /// Trio runtime (future)
    Trio,
}

impl FrameConfig {
    /// Load configuration - for now just returns defaults
    /// In the future, this could load from frame.toml or other config files
    pub fn load(_config_path: &Option<PathBuf>) -> Result<FrameConfig, String> {
        // For now, just return defaults
        // No legacy config support - start fresh
        Ok(FrameConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = FrameConfig::default();
        assert_eq!(config.build.output_dir, PathBuf::from("dist"));
        assert_eq!(config.build.source_dirs, vec![PathBuf::from("src")]);
        assert!(config.python.event_handlers_as_functions);
    }
    
}