use std::path::{Path, PathBuf};

use crate::frame_c::ast::NativeModuleDeclNode;
use crate::frame_c::visitors::TargetLanguage;

pub mod python;
pub mod typescript;

#[derive(Debug, Clone)]
pub struct DeclarationImportContext {
    pub config_dir: PathBuf,
    pub verbose: bool,
    pub native_imports: Vec<NativeImportRequest>,
}

#[derive(Debug, Clone)]
pub struct DeclarationSourceConfig {
    pub adapter: String,
    pub input: PathBuf,
    pub target: Option<String>,
    pub module: Option<String>,
    pub options: Option<serde_json::Value>,
}

impl DeclarationSourceConfig {
    pub fn input_path(&self, base_dir: &Path) -> PathBuf {
        if self.input.is_relative() {
            base_dir.join(&self.input)
        } else {
            self.input.clone()
        }
    }
}

#[derive(Debug, Clone)]
pub struct NativeImportRequest {
    pub spec_path: PathBuf,
    pub target: TargetLanguage,
    pub code: String,
}

pub trait DeclarationImporter {
    fn name(&self) -> &'static str;
    fn import(
        &self,
        source: &DeclarationSourceConfig,
        context: &DeclarationImportContext,
    ) -> Result<Vec<NativeModuleDeclNode>, String>;
}

pub fn get_importer(adapter: &str) -> Option<Box<dyn DeclarationImporter>> {
    match adapter {
        "typescript" | "typescript-typedoc" => {
            Some(Box::new(typescript::TypeScriptTypedocImporter))
        }
        "python" | "python-runtime" | "python-stub" => {
            Some(Box::new(python::PythonRuntimeImporter))
        }
        _ => None,
    }
}
