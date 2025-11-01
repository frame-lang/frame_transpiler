use crate::frame_c::ast::NativeModuleDeclNode;

use super::{DeclarationImportContext, DeclarationImporter, DeclarationSourceConfig};

#[derive(Debug)]
pub struct PythonStubImporter;

impl DeclarationImporter for PythonStubImporter {
    fn name(&self) -> &'static str {
        "python-stub"
    }

    fn import(
        &self,
        source: &DeclarationSourceConfig,
        context: &DeclarationImportContext,
    ) -> Result<Vec<NativeModuleDeclNode>, String> {
        if context.verbose {
            eprintln!(
                "[decl import] Python importer stub invoked for {:?}",
                source.input
            );
        }
        Ok(Vec::new())
    }
}
