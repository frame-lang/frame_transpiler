// Source map generation for Frame transpiler
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMap {
    pub version: String,
    #[serde(skip_serializing)]
    pub generator: String, // Not required by spec, keep internally
    #[serde(rename = "sourceFile")]
    pub source_file: String,
    #[serde(rename = "targetFile")]
    pub target_file: String,
    pub mappings: Vec<SourceMapping>,
    #[serde(rename = "debugInfo", skip_serializing_if = "Option::is_none")]
    pub debug_info: Option<DebugInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMapping {
    #[serde(rename = "frameLine")]
    pub frame_line: usize,
    #[serde(rename = "pythonLine")]
    pub python_line: usize,
    // Phase 2 optional fields
    #[serde(rename = "frameColumn", skip_serializing_if = "Option::is_none")]
    pub frame_column: Option<usize>,
    #[serde(rename = "pythonColumn", skip_serializing_if = "Option::is_none")]
    pub python_column: Option<usize>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub mapping_type: Option<MappingType>, // Optional per spec
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MappingType {
    FunctionDef,
    VarDecl,
    Assignment,
    MethodCall,
    FunctionCall,
    StateDef,
    StateEnter,
    StateExit,
    EventHandler,
    Transition,
    Print,
    Return,
    If,
    Loop,
    SystemDef,
    InterfaceMethod,
    Statement, // Generic statement type for miscellaneous statements
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugInfo {
    pub systems: Vec<SystemInfo>,
    pub functions: Vec<FunctionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub name: String,
    #[serde(rename = "startLine")]
    pub start_line: usize,
    #[serde(rename = "endLine")]
    pub end_line: usize,
    pub states: Vec<String>,
    pub events: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub name: String,
    #[serde(rename = "startLine")]
    pub start_line: usize,
    #[serde(rename = "endLine")]
    pub end_line: usize,
    pub parameters: Vec<String>,
    pub locals: Vec<String>,
}

pub struct SourceMapBuilder {
    mappings: Vec<SourceMapping>,
    current_python_line: usize,
    source_file: String,
    target_file: String,
    systems: Vec<SystemInfo>,
    functions: Vec<FunctionInfo>,
}

impl SourceMapBuilder {
    pub fn new(source_file: String, target_file: String) -> Self {
        SourceMapBuilder {
            mappings: Vec::new(),
            current_python_line: 1,
            source_file,
            target_file,
            systems: Vec::new(),
            functions: Vec::new(),
        }
    }

    // Simple mapping for Phase 1 - just line numbers
    pub fn add_simple_mapping(&mut self, frame_line: usize, python_line: usize) {
        // Check if this exact mapping already exists to prevent duplicates
        let duplicate_exists = self
            .mappings
            .iter()
            .any(|m| m.frame_line == frame_line && m.python_line == python_line);

        if !duplicate_exists {
            // v0.73: Store as 1-based internally, will convert to 0-based on output
            self.mappings.push(SourceMapping {
                frame_line,
                python_line,
                frame_column: None,
                python_column: None,
                mapping_type: None,
                name: None,
            });
        }
    }

    // Full mapping with type for Phase 2
    pub fn add_mapping(
        &mut self,
        frame_line: usize,
        mapping_type: MappingType,
        name: Option<String>,
    ) {
        self.mappings.push(SourceMapping {
            frame_line,
            python_line: self.current_python_line,
            frame_column: None,
            python_column: None,
            mapping_type: Some(mapping_type),
            name,
        });
    }

    pub fn add_mapping_with_column(
        &mut self,
        frame_line: usize,
        frame_column: usize,
        mapping_type: MappingType,
        name: Option<String>,
    ) {
        self.mappings.push(SourceMapping {
            frame_line,
            frame_column: Some(frame_column),
            python_line: self.current_python_line,
            python_column: None,
            mapping_type: Some(mapping_type),
            name,
        });
    }

    pub fn increment_python_line(&mut self) {
        self.current_python_line += 1;
    }

    pub fn set_python_line(&mut self, line: usize) {
        self.current_python_line = line;
    }

    pub fn get_python_line(&self) -> usize {
        self.current_python_line
    }

    pub fn add_system_info(&mut self, info: SystemInfo) {
        self.systems.push(info);
    }

    pub fn add_function_info(&mut self, info: FunctionInfo) {
        self.functions.push(info);
    }

    /// Clear all existing mappings (v0.71: used for marker-based replacement)
    pub fn clear_mappings(&mut self) {
        self.mappings.clear();
    }

    pub fn build(&self) -> SourceMap {
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG: SourceMapBuilder::build() - raw mappings before conversion:");
            for mapping in &self.mappings {
                eprintln!(
                    "  Frame {} -> Python {}",
                    mapping.frame_line, mapping.python_line
                );
            }
        }

        let debug_info = if !self.systems.is_empty() || !self.functions.is_empty() {
            Some(DebugInfo {
                systems: self.systems.clone(),
                functions: self.functions.clone(),
            })
        } else {
            None
        };

        // v0.74.1: Keep 1-based line numbers for human-readable output
        // The JSON output is primarily for debugging, not for actual debuggers
        // If we need 0-based for real debuggers later, we can add a flag
        let zero_based_mappings: Vec<SourceMapping> = self.mappings.clone();

        SourceMap {
            version: "1.0".to_string(),
            generator: format!("framec_v{}", env!("FRAME_VERSION")),
            source_file: self.source_file.clone(),
            target_file: self.target_file.clone(),
            mappings: zero_based_mappings,
            debug_info,
        }
    }
}

// Debug output structure for JSON response
#[derive(Debug, Serialize, Deserialize)]
pub struct DebugOutput {
    pub python: String,
    #[serde(rename = "sourceMap")]
    pub source_map: SourceMap,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<OutputMetadata>, // Optional extra info beyond spec requirements
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputMetadata {
    #[serde(rename = "frameVersion")]
    pub frame_version: String,
    #[serde(rename = "generatedAt")]
    pub generated_at: String,
    pub checksum: String,
}

impl DebugOutput {
    pub fn new(python: String, source_map: SourceMap, source_content: &str) -> Self {
        use chrono::Utc;
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(source_content.as_bytes());
        let checksum = format!("sha256:{:x}", hasher.finalize());

        DebugOutput {
            python,
            source_map,
            metadata: Some(OutputMetadata {
                frame_version: env!("FRAME_VERSION").to_string(),
                generated_at: Utc::now().to_rfc3339(),
                checksum,
            }),
        }
    }

    // Constructor without metadata for strict spec compliance
    pub fn new_minimal(python: String, source_map: SourceMap) -> Self {
        DebugOutput {
            python,
            source_map,
            metadata: None,
        }
    }
}
