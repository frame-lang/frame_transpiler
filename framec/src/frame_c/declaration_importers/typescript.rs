use crate::frame_c::ast::NativeModuleDeclNode;

use super::{DeclarationImportContext, DeclarationImporter, DeclarationSourceConfig};

#[derive(Debug)]
pub struct TypeScriptTypedocImporter;

impl DeclarationImporter for TypeScriptTypedocImporter {
    fn name(&self) -> &'static str {
        "typescript-typedoc"
    }

    fn import(
        &self,
        source: &DeclarationSourceConfig,
        context: &DeclarationImportContext,
    ) -> Result<Vec<NativeModuleDeclNode>, String> {
        if context.verbose {
            eprintln!(
                "[decl import] TypeScript importer stub invoked for {:?}",
                source.input
            );
        }
        Ok(Vec::new())
    }
}
