// Simplified configuration system for Frame v0.57+
// Focuses on build configuration and Python-specific options

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
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

    #[serde(default)]
    pub paths: PathsConfig,

    #[serde(default)]
    pub scripts: HashMap<String, String>,
}

impl Default for FrameConfig {
    fn default() -> Self {
        FrameConfig {
            project: ProjectConfig::default(),
            build: BuildConfig::default(),
            python: PythonConfig::default(),
            paths: PathsConfig::default(),
            scripts: HashMap::new(),
        }
    }
}

/// Project metadata configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Project name
    pub name: Option<String>,

    /// Project version
    pub version: Option<String>,

    /// Entry point for multi-file compilation
    pub entry: Option<PathBuf>,

    /// Project root directory
    #[serde(default = "default_project_root")]
    pub root: PathBuf,

    /// Project authors
    #[serde(default)]
    pub authors: Vec<String>,

    /// Project description
    pub description: Option<String>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: None,
            version: None,
            entry: None,
            root: default_project_root(),
            authors: Vec::new(),
            description: None,
        }
    }
}

/// Build configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Target language (python_3, etc.)
    #[serde(default = "default_target")]
    pub target: String,

    /// Output directory for generated code
    #[serde(default = "default_output_dir")]
    pub output_dir: PathBuf,

    /// Output mode (concatenated or separate_files)
    #[serde(default)]
    pub output_mode: OutputMode,

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

/// Output generation mode
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OutputMode {
    Concatenated,
    SeparateFiles,
}

impl Default for OutputMode {
    fn default() -> Self {
        OutputMode::Concatenated
    }
}

impl Default for BuildConfig {
    fn default() -> Self {
        BuildConfig {
            target: default_target(),
            output_dir: default_output_dir(),
            output_mode: OutputMode::default(),
            source_dirs: default_source_dirs(),
            optimize: false,
            debug: false,
            incremental: default_incremental(),
        }
    }
}

fn default_target() -> String {
    "python_3".to_string()
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

fn default_project_root() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// Paths configuration
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PathsConfig {
    #[serde(default)]
    pub modules: Vec<String>,

    #[serde(default)]
    pub imports: Vec<String>,

    #[serde(default)]
    pub aliases: HashMap<String, String>,
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
    true // v0.36 default
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
    /// Load configuration from frame.toml file
    pub fn load(config_path: &Option<PathBuf>) -> Result<FrameConfig, String> {
        if let Some(path) = config_path {
            Self::load_from_file(path)
        } else {
            // Try to find frame.toml in current directory or parents
            Self::find_and_load()
        }
    }

    /// Load configuration from a specific file
    pub fn load_from_file(path: &PathBuf) -> Result<FrameConfig, String> {
        let contents =
            fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;

        toml::from_str(&contents).map_err(|e| format!("Failed to parse config file: {}", e))
    }

    /// Find and load frame.toml from current directory or parents
    pub fn find_and_load() -> Result<FrameConfig, String> {
        let (_, config) = Self::find_project_config()
            .ok_or_else(|| "No frame.toml found in project hierarchy".to_string())?;
        Ok(config)
    }

    /// Find project configuration by searching up the directory tree
    pub fn find_project_config() -> Option<(PathBuf, FrameConfig)> {
        Self::find_project_config_from(std::env::current_dir().ok()?)
    }

    /// Find project configuration starting from a specific directory
    pub fn find_project_config_from(start_dir: PathBuf) -> Option<(PathBuf, FrameConfig)> {
        let mut current = start_dir;

        loop {
            let config_path = current.join("frame.toml");

            if config_path.exists() {
                if let Ok(config) = Self::load_from_file(&config_path) {
                    return Some((config_path, config));
                }
            }

            // Check for alternative name
            let alt_config = current.join(".framerc.toml");
            if alt_config.exists() {
                if let Ok(config) = Self::load_from_file(&alt_config) {
                    return Some((alt_config, config));
                }
            }

            // Move up to parent directory
            if !current.pop() {
                break;
            }
        }

        None
    }

    /// Create a default frame.toml file
    pub fn create_default(path: &PathBuf, project_name: Option<&str>) -> Result<(), String> {
        let mut config = FrameConfig::default();

        if let Some(name) = project_name {
            config.project.name = Some(name.to_string());
        }

        // Set sensible defaults
        config.project.version = Some("0.1.0".to_string());
        config.project.entry = Some(PathBuf::from("src/main.frm"));
        config.project.description = Some("A Frame language project".to_string());

        // Add common scripts
        config
            .scripts
            .insert("build".to_string(), "framec build".to_string());
        config
            .scripts
            .insert("clean".to_string(), "rm -rf dist/".to_string());
        config
            .scripts
            .insert("dev".to_string(), "framec --watch".to_string());

        // Add common module paths
        config.paths.modules = vec!["src".to_string(), "lib".to_string()];

        let toml_string = toml::to_string_pretty(&config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(path, toml_string).map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(())
    }

    /// Get the entry point file path
    pub fn entry_point(&self) -> PathBuf {
        self.project
            .entry
            .clone()
            .unwrap_or_else(|| PathBuf::from("main.frm"))
    }

    /// Check if we should generate separate files
    pub fn use_separate_files(&self) -> bool {
        self.build.output_mode == OutputMode::SeparateFiles
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
