use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::frame_c::ast::{NativeModuleDeclNode, NativeModuleItem};
use crate::frame_c::declaration_importers::{
    get_importer, DeclarationImportContext, DeclarationSourceConfig,
};
use crate::frame_c::utils::{frame_exitcode, RunError};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeclarationImportConfig {
    pub output_dir: PathBuf,
    #[serde(default)]
    pub sources: Vec<RawSourceConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawSourceConfig {
    pub adapter: String,
    pub input: PathBuf,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub module: Option<String>,
    #[serde(default)]
    pub options: Option<serde_json::Value>,
}

impl From<RawSourceConfig> for DeclarationSourceConfig {
    fn from(value: RawSourceConfig) -> Self {
        DeclarationSourceConfig {
            adapter: value.adapter,
            input: value.input,
            target: value.target,
            module: value.module,
            options: value.options,
        }
    }
}

pub fn run_decl_import(
    config_path: &Path,
    force: bool,
    dry_run: bool,
    verbose: bool,
    allow_missing: bool,
) -> Result<(), RunError> {
    let config_contents = fs::read_to_string(config_path).map_err(|err| {
        RunError::new(
            frame_exitcode::CONFIG_ERR,
            &format!(
                "Unable to read declaration config '{}': {}",
                config_path.display(),
                err
            ),
        )
    })?;

    let config: DeclarationImportConfig =
        serde_json::from_str(&config_contents).map_err(|err| {
            RunError::new(
                frame_exitcode::CONFIG_ERR,
                &format!(
                    "Invalid declaration config '{}': {}",
                    config_path.display(),
                    err
                ),
            )
        })?;

    if config.sources.is_empty() {
        return Err(RunError::new(
            frame_exitcode::CONFIG_ERR,
            "Declaration config must include at least one source",
        ));
    }

    let base_dir = config_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));

    let output_dir = if config.output_dir.is_relative() {
        base_dir.join(&config.output_dir)
    } else {
        config.output_dir.clone()
    };

    if !dry_run {
        fs::create_dir_all(&output_dir).map_err(|err| {
            RunError::new(
                frame_exitcode::CONFIG_ERR,
                &format!(
                    "Unable to create output directory '{}': {}",
                    output_dir.display(),
                    err
                ),
            )
        })?;
    }

    let mut generated_files = Vec::new();
    let context = DeclarationImportContext {
        config_dir: base_dir.clone(),
        verbose,
    };

    for raw_source in config.sources {
        let source_config: DeclarationSourceConfig = raw_source.into();
        let adapter_name = source_config.adapter.to_lowercase();
        let importer = match get_importer(&adapter_name) {
            Some(importer) => importer,
            None => {
                return Err(RunError::new(
                    frame_exitcode::CONFIG_ERR,
                    &format!("Unknown declaration importer '{}'.", adapter_name),
                ));
            }
        };

        if verbose {
            println!(
                "[decl import] Using adapter '{}' for {:?}",
                importer.name(),
                source_config.input
            );
        }

        let modules = importer
            .import(&source_config, &context)
            .map_err(|err| RunError::new(frame_exitcode::CONFIG_ERR, &err))?;

        if modules.is_empty() {
            if verbose {
                println!(
                    "[decl import] Adapter '{}' produced no modules for {:?}",
                    importer.name(),
                    source_config.input
                );
            }
            continue;
        }

        validate_coverage(&modules, &source_config, allow_missing, verbose)?;

        for module in modules {
            let file_name = format!("{}.frame_decl", module.path().replace('/', "_"));
            let output_path = output_dir.join(file_name);

            if output_path.exists() && !force {
                return Err(RunError::new(
                    frame_exitcode::CONFIG_ERR,
                    &format!(
                        "Declaration file '{}' already exists. Use --force to overwrite.",
                        output_path.display()
                    ),
                ));
            }

            let stringified = render_native_module(&module);
            if dry_run {
                println!("[decl import] Would write {}", output_path.display());
            } else {
                fs::write(&output_path, stringified).map_err(|err| {
                    RunError::new(
                        frame_exitcode::CONFIG_ERR,
                        &format!(
                            "Failed to write declaration '{}': {}",
                            output_path.display(),
                            err
                        ),
                    )
                })?;
                generated_files.push(output_path);
            }
        }
    }

    if !dry_run {
        if generated_files.is_empty() {
            println!("No declaration files generated.");
        } else {
            println!("Generated {} declaration file(s).", generated_files.len());
        }
    }

    Ok(())
}

fn render_native_module(module: &NativeModuleDeclNode) -> String {
    let mut output = String::new();
    output.push_str("native module ");
    output.push_str(&module.path());
    output.push_str(" {\n");

    for item in &module.items {
        match item {
            NativeModuleItem::Type(type_decl) => {
                output.push_str("    type ");
                output.push_str(&type_decl.name);
                if let Some(alias) = &type_decl.aliased_type {
                    output.push_str(" = ");
                    output.push_str(alias);
                }
                output.push_str("\n");
            }
            NativeModuleItem::Function(func_decl) => {
                if func_decl.is_async {
                    output.push_str("    async ");
                } else {
                    output.push_str("    ");
                }
                output.push_str(&func_decl.name);
                output.push('(');
                let mut first = true;
                for param in &func_decl.parameters {
                    if !first {
                        output.push_str(", ");
                    }
                    output.push_str(&param.name);
                    if let Some(ty) = &param.type_annotation {
                        output.push_str(": ");
                        output.push_str(ty);
                    }
                    first = false;
                }
                output.push(')');
                if let Some(ret) = &func_decl.return_type {
                    output.push_str(" -> ");
                    output.push_str(ret);
                }
                output.push_str("\n");
            }
        }
    }

    output.push_str("}\n");
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn render_empty_module() {
        let module =
            NativeModuleDeclNode::new(vec!["runtime".into(), "socket".into()], 1, 1, vec![]);
        let rendered = render_native_module(&module);
        assert!(rendered.contains("native module runtime/socket"));
    }

    #[test]
    fn decl_import_dry_run_uses_json_cache() {
        let temp = tempdir().expect("create temp dir");
        let config_path = temp.path().join("decl.json");

        let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../framec_tests/fixtures/native_decl_generation/typescript");
        let json_cache = fixtures_dir.join("typedoc_runtime_socket.json");

        let config = json!({
            "outputDir": "out",
            "sources": [
                {
                    "adapter": "typescript",
                    "input": "frame_runtime_ts/index.ts",
                    "module": "runtime/socket",
                    "options": {
                        "jsonCache": json_cache,
                        "include": [
                            "framesocketclient",
                            "frame_socket_client_connect",
                            "frame_socket_client_read_line",
                            "frame_socket_client_write_line",
                            "frame_socket_client_close"
                        ]
                    }
                }
            ]
        });

        fs::write(&config_path, serde_json::to_string_pretty(&config).unwrap())
            .expect("write config");

        run_decl_import(&config_path, false, true, false, false).expect("dry run succeeds");

        assert!(
            !temp.path().join("out").exists(),
            "dry run should not create output directory"
        );
    }
}

fn validate_coverage(
    modules: &[NativeModuleDeclNode],
    source: &DeclarationSourceConfig,
    allow_missing: bool,
    verbose: bool,
) -> Result<(), RunError> {
    let expected = expected_symbols_from_options(source);
    if expected.is_empty() {
        return Ok(());
    }

    let found = collect_symbol_names(modules);

    let missing: Vec<String> = expected
        .into_iter()
        .filter(|name| !found.contains(name))
        .collect();

    if missing.is_empty() {
        return Ok(());
    }

    if allow_missing {
        if verbose {
            eprintln!(
                "[decl import] warning: {} missing symbol(s): {}",
                source.adapter,
                missing.join(", ")
            );
        }
        return Ok(());
    }

    Err(RunError::new(
        frame_exitcode::CONFIG_ERR,
        &format!(
            "Declaration importer '{}' missing expected symbol(s): {}",
            source.adapter,
            missing.join(", ")
        ),
    ))
}

fn expected_symbols_from_options(source: &DeclarationSourceConfig) -> Vec<String> {
    source
        .options
        .as_ref()
        .and_then(|opts| opts.get("include"))
        .and_then(|value| value.as_array().cloned())
        .map(|items| {
            items
                .into_iter()
                .filter_map(|item| item.as_str().map(|s| s.to_lowercase()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn collect_symbol_names(modules: &[NativeModuleDeclNode]) -> HashSet<String> {
    let mut names = HashSet::new();
    for module in modules {
        for item in &module.items {
            match item {
                NativeModuleItem::Type(ty) => {
                    names.insert(ty.name.to_lowercase());
                }
                NativeModuleItem::Function(func) => {
                    names.insert(func.name.to_lowercase());
                }
            }
        }
    }
    names
}
