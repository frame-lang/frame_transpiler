use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::frame_c::ast::{ImportType, NativeModuleDeclNode, NativeModuleItem};
use crate::frame_c::declaration_importers::{
    get_importer, DeclarationImportContext, DeclarationSourceConfig, NativeImportRequest,
};
use crate::frame_c::parser::Parser;
use crate::frame_c::scanner::Scanner;
use crate::frame_c::symbol_table::Arcanum;
use crate::frame_c::utils::{frame_exitcode, RunError};

// New FID manifest structures (v0.90+)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FidManifest {
    pub sources: Vec<FidSource>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FidSource {
    #[serde(rename = "@target")]
    pub target: String,
    pub resources: Vec<FidResource>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
enum FidResource {
    File { file: FidFileResource },
    Module { module: FidModuleResource },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FidFileResource {
    pub uri: String,
    pub modules: Vec<FidModuleMap>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FidModuleMap {
    pub module: String,
    #[serde(rename = "import")]
    pub import_selectors: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FidModuleResource {
    pub name: String,
    #[serde(rename = "import")]
    pub import_selectors: Vec<String>,
}

pub fn run_fid_import(
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
    let manifest: FidManifest = serde_json::from_str(&config_contents).map_err(|err| {
        RunError::new(
            frame_exitcode::CONFIG_ERR,
            &format!(
                "Invalid FID manifest '{}': {}",
                config_path.display(),
                err
            ),
        )
    })?;

    run_fid_manifest(config_path, &manifest, force, dry_run, verbose, allow_missing)
}

fn run_fid_manifest(
    config_path: &Path,
    manifest: &FidManifest,
    force: bool,
    dry_run: bool,
    verbose: bool,
    allow_missing: bool,
) -> Result<(), RunError> {
    if manifest.sources.is_empty() {
        return Err(RunError::new(
            frame_exitcode::CONFIG_ERR,
            "Manifest must include at least one source",
        ));
    }

    let base_dir = config_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));

    let mut generated_files = Vec::new();
    let context = DeclarationImportContext { config_dir: base_dir.clone(), verbose, native_imports: Vec::new() };

    for src in &manifest.sources {
        let (importer_name, target_subdir) = match src.target.as_str() {
            "typescript" => ("typescript-typedoc", "typescript"),
            "python" => ("python-runtime", "python"),
            other => {
                return Err(RunError::new(
                    frame_exitcode::CONFIG_ERR,
                    &format!("Unsupported @target '{}'.", other),
                ));
            }
        };

        let output_dir = base_dir
            .join(".framec")
            .join("cache")
            .join("fid")
            .join(target_subdir);
        if !dry_run {
            fs::create_dir_all(&output_dir).map_err(|err| {
                RunError::new(
                    frame_exitcode::CONFIG_ERR,
                    &format!("Unable to create cache directory '{}': {}", output_dir.display(), err),
                )
            })?;
        }

        for res in &src.resources {
            match res {
                FidResource::File { file } => {
                    for mm in &file.modules {
                        let mut options = serde_json::json!({ "include": mm.import_selectors });
                        if importer_name == "python-runtime" {
                            options["moduleName"] = serde_json::Value::String(mm.module.clone());
                        }
                        let source_config = DeclarationSourceConfig {
                            adapter: importer_name.to_string(),
                            input: PathBuf::from(&file.uri),
                            target: Some(src.target.clone()),
                            module: Some(format!("{}::{}", target_subdir, mm.module)),
                            options: Some(options),
                        };
                        run_single_import(
                            &source_config,
                            &context,
                            &output_dir,
                            force,
                            dry_run,
                            verbose,
                            allow_missing,
                            &mut generated_files,
                        )?;
                    }
                }
                FidResource::Module { module } => {
                    let mut options = serde_json::json!({ "include": module.import_selectors });
                    if importer_name == "python-runtime" {
                        options["moduleName"] = serde_json::Value::String(module.name.clone());
                    }
                    let source_config = DeclarationSourceConfig {
                        adapter: importer_name.to_string(),
                        input: PathBuf::from(&module.name),
                        target: Some(src.target.clone()),
                        module: Some(format!("{}::{}", target_subdir, module.name)),
                        options: Some(options),
                    };
                    run_single_import(
                        &source_config,
                        &context,
                        &output_dir,
                        force,
                        dry_run,
                        verbose,
                        allow_missing,
                        &mut generated_files,
                    )?;
                }
            }
        }
    }

    if !dry_run {
        if generated_files.is_empty() {
            println!("No .fid files generated.");
        } else {
            println!("Generated {} .fid file(s).", generated_files.len());
        }
    }

    Ok(())
}

fn run_single_import(
    source_config: &DeclarationSourceConfig,
    context: &DeclarationImportContext,
    output_dir: &Path,
    force: bool,
    dry_run: bool,
    verbose: bool,
    allow_missing: bool,
    generated_files: &mut Vec<PathBuf>,
) -> Result<(), RunError> {
    let adapter_name = source_config.adapter.to_lowercase();
    let importer = match get_importer(&adapter_name) {
        Some(importer) => importer,
        None => {
            return Err(RunError::new(
                frame_exitcode::CONFIG_ERR,
                &format!("Unknown FID importer '{}'.", adapter_name),
            ));
        }
    };

    if verbose {
        println!(
            "[fid import] Using importer '{}' for {:?}",
            importer.name(), source_config.input
        );
    }

    let modules = importer
        .import(source_config, context)
        .map_err(|err| RunError::new(frame_exitcode::CONFIG_ERR, &err))?;

    if modules.is_empty() {
        if verbose {
            println!(
                "[fid import] Importer '{}' produced no modules for {:?}",
                importer.name(), source_config.input
            );
        }
        return Ok(());
    }

    validate_coverage(&modules, source_config, allow_missing, verbose)?;

    for module in modules {
        let file_stem = module
            .path()
            .replace("::", "_")
            .replace('/', "_")
            .replace('.', "_");
        let file_name = format!("{}.fid", file_stem);
        let output_path = output_dir.join(file_name);

        if output_path.exists() && !force {
            return Err(RunError::new(
                frame_exitcode::CONFIG_ERR,
                &format!("FID file '{}' already exists. Use --force to overwrite.", output_path.display()),
            ));
        }

        let stringified = render_native_module(&module);
        if dry_run {
            println!("[fid import] Would write {}", output_path.display());
        } else {
            fs::write(&output_path, stringified).map_err(|err| {
                RunError::new(
                    frame_exitcode::CONFIG_ERR,
                    &format!("Failed to write .fid '{}': {}", output_path.display(), err),
                )
            })?;
            generated_files.push(output_path);
        }
    }

    Ok(())
}

fn collect_native_imports(spec_paths: &[PathBuf]) -> Result<Vec<NativeImportRequest>, RunError> {
    let mut results = Vec::new();
    let mut seen = HashSet::new();

    for spec_path in spec_paths {
        let source = fs::read_to_string(spec_path).map_err(|err| {
            RunError::new(
                frame_exitcode::CONFIG_ERR,
                &format!(
                    "Unable to read Frame spec '{}': {}",
                    spec_path.display(),
                    err
                ),
            )
        })?;

        let source_lines = Arc::new(
            source
                .lines()
                .map(|line| line.to_string())
                .collect::<Vec<_>>(),
        );
        let scanner = Scanner::new(source.clone());
        let (has_errors, errors, tokens, target_regions_vec) = scanner.scan_tokens();
        if has_errors {
            return Err(RunError::new(
                frame_exitcode::PARSE_ERR,
                &format!(
                    "Failed to scan Frame spec '{}': {}",
                    spec_path.display(),
                    errors
                ),
            ));
        }

        let target_regions = Arc::new(target_regions_vec);
        let mut comments = Vec::new();
        let mut parser = Parser::new(
            &tokens,
            &mut comments,
            true,
            Arcanum::new(),
            Arc::clone(&target_regions),
            Arc::clone(&source_lines),
        );

        let module = parser.parse().map_err(|err| {
            RunError::new(
                frame_exitcode::PARSE_ERR,
                &format!(
                    "Failed to parse Frame spec '{}': {}",
                    spec_path.display(),
                    err.to_display_string()
                ),
            )
        })?;

        if parser.had_error() {
            return Err(RunError::new(
                frame_exitcode::PARSE_ERR,
                &format!(
                    "Parsing errors in Frame spec '{}': {}",
                    spec_path.display(),
                    parser.get_errors()
                ),
            ));
        }

        for import_node in &module.imports {
            if let ImportType::Native { target, code } = &import_node.import_type {
                let key = (*target, code.clone());
                if seen.insert(key.clone()) {
                    results.push(NativeImportRequest {
                        spec_path: spec_path.clone(),
                        target: key.0,
                        code: key.1,
                    });
                }
            }
        }
    }

    Ok(results)
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
                    output.push_str(": ");
                    if ret.eq_ignore_ascii_case("void") {
                        output.push_str("None");
                    } else {
                        output.push_str(ret);
                    }
                }
                output.push_str("\n");
            }
        }
    }

    output.push_str("}\n");
    output
}

// No unit tests in this file: importer integration depends on external tools.

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
                "[fid import] warning: {} missing symbol(s): {}",
                source.adapter,
                missing.join(", ")
            );
        }
        return Ok(());
    }

    Err(RunError::new(
        frame_exitcode::CONFIG_ERR,
        &format!(
            "FID importer '{}' missing expected symbol(s): {}",
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
