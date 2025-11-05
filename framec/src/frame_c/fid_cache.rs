use std::cell::RefCell;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use crate::frame_c::ast::NativeModuleDeclNode;
use crate::frame_c::parser::Parser;
use crate::frame_c::scanner::Scanner;
use crate::frame_c::symbol_table::{Arcanum, NativeModuleSymbol};
use crate::frame_c::visitors::TargetLanguage;

#[derive(Debug, Default, Clone, Copy)]
pub struct FidLoadSummary {
    pub directories_scanned: usize,
    pub files_scanned: usize,
    pub modules_loaded: usize,
}

/// Load cached `.fid` native module declarations for the given target language.
///
/// Directories are discovered in the following order (duplicates are ignored):
/// 1. Each ancestor of `spec_dir` (inclusive) joined with `.frame/cache/fid/<target>`
/// 2. The current working directory joined with `.frame/cache/fid/<target>`
/// 3. Directories listed in `FRAME_FID_PATH` (PATH-separator delimited). Entries may
///    either point directly at a target directory or at the cache root; when the
///    latter is detected the `<target>` segment is appended automatically.
///
/// Returns a [`FidLoadSummary`] describing the work performed.
pub fn load_fid_modules(
    arcanum: &mut Arcanum,
    target: TargetLanguage,
    spec_dir: Option<&Path>,
) -> Result<FidLoadSummary, String> {
    let search_dirs = collect_search_dirs(spec_dir, target);
    let mut summary = FidLoadSummary {
        directories_scanned: search_dirs.len(),
        ..Default::default()
    };

    let mut registered_paths: HashSet<String> = arcanum.native_modules.keys().cloned().collect();

    for dir in search_dirs {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };
            let path = entry.path();
            if path.extension().map(|ext| ext == "fid").unwrap_or(false) {
                summary.files_scanned += 1;
                let modules = parse_fid_file(&path)?;
                for module in modules {
                    let module_ref = module.borrow();
                    let module_path = module_ref.qualified_name.join("::");
                    if registered_paths.contains(&module_path) {
                        continue;
                    }
                    let symbol = NativeModuleSymbol::new(
                        module_ref.qualified_name.clone(),
                        Rc::clone(&module),
                    );
                    arcanum.register_native_module(Rc::new(RefCell::new(symbol)));
                    registered_paths.insert(module_path);
                    summary.modules_loaded += 1;
                }
            }
        }
    }

    Ok(summary)
}

fn parse_fid_file(path: &Path) -> Result<Vec<Rc<RefCell<NativeModuleDeclNode>>>, String> {
    let contents = fs::read_to_string(path)
        .map_err(|err| format!("Failed to read fid file '{}': {}", path.display(), err))?;

    let source_lines = Arc::new(
        contents
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>(),
    );
    let scanner = Scanner::new(contents.clone());
    let (has_errors, errors, tokens, target_regions) = scanner.scan_tokens();
    if has_errors {
        return Err(format!(
            "Failed to scan fid file '{}': {}",
            path.display(),
            errors
        ));
    }

    let target_regions = Arc::new(target_regions);
    let mut comments = Vec::new();
    let mut parser = Parser::new(
        &tokens,
        &mut comments,
        true,
        Arcanum::new(),
        Arc::clone(&target_regions),
        Arc::clone(&source_lines),
    );
    let frame_module = parser.parse().map_err(|err| {
        format!(
            "Failed to parse fid file '{}': {}",
            path.display(),
            err.to_display_string()
        )
    })?;

    Ok(frame_module.native_modules)
}

fn collect_search_dirs(spec_dir: Option<&Path>, target: TargetLanguage) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    let mut seen = HashSet::new();

    let target_segment = fid_target_segment(target);

    if let Some(mut current) = spec_dir.map(|p| p.to_path_buf()) {
        loop {
            let candidate = current.join(".frame/cache/fid").join(target_segment);
            push_unique_dir(&mut dirs, &mut seen, candidate);
            match current.parent() {
                Some(parent) => {
                    if parent == current {
                        break;
                    }
                    current = parent.to_path_buf();
                }
                None => break,
            }
        }
    }

    if let Ok(cwd) = env::current_dir() {
        push_unique_dir(
            &mut dirs,
            &mut seen,
            cwd.join(".frame/cache/fid").join(target_segment),
        );
    }

    if let Ok(paths_var) = env::var("FRAME_FID_PATH") {
        for entry in env::split_paths(&paths_var) {
            let entry_str = entry.to_string_lossy();
            let trimmed = entry_str.trim();
            if trimmed.is_empty() {
                continue;
            }

            if trimmed.contains("{target}") {
                let substituted = trimmed.replace("{target}", target_segment);
                push_unique_dir(&mut dirs, &mut seen, PathBuf::from(substituted));
            } else {
                push_unique_dir(&mut dirs, &mut seen, entry.clone());
                push_unique_dir(&mut dirs, &mut seen, entry.join(target_segment));
            }
        }
    }

    dirs
}

fn push_unique_dir(dirs: &mut Vec<PathBuf>, seen: &mut HashSet<PathBuf>, candidate: PathBuf) {
    if !candidate.is_dir() {
        return;
    }
    let key = match fs::canonicalize(&candidate) {
        Ok(canon) => canon,
        Err(_) => candidate.clone(),
    };
    if seen.insert(key) {
        dirs.push(candidate);
    }
}

fn fid_target_segment(target: TargetLanguage) -> &'static str {
    match target {
        TargetLanguage::Python3 => "python",
        TargetLanguage::TypeScript => "typescript",
        TargetLanguage::Graphviz => "graphviz",
        TargetLanguage::LLVM => "llvm",
        TargetLanguage::C => "c",
        TargetLanguage::Cpp => "cpp",
        TargetLanguage::Java => "java",
        TargetLanguage::CSharp => "csharp",
        TargetLanguage::Rust => "rust",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn loads_modules_from_ancestor_cache() {
        let temp = tempdir().expect("create temp dir");
        let project_root = temp.path();
        let cache_dir = project_root.join(".frame/cache/fid/typescript");
        fs::create_dir_all(&cache_dir).expect("create cache dir");

        let fid_content = "native module runtime::socket {\n    connect(): None\n}\n";
        fs::write(cache_dir.join("runtime_socket.fid"), fid_content).expect("write fid");

        let mut arcanum = Arcanum::new();
        let summary =
            load_fid_modules(&mut arcanum, TargetLanguage::TypeScript, Some(project_root))
                .expect("load fid modules");

        assert!(summary.modules_loaded >= 1);
        assert!(arcanum.lookup_native_module("runtime::socket").is_some());
    }

    #[test]
    fn loads_modules_from_env_path() {
        let temp = tempdir().expect("create temp dir");
        let cache_dir = temp.path().join("custom/cache/fid/typescript");
        fs::create_dir_all(&cache_dir).expect("create cache dir");
        let fid_content = "native module runtime::env {\n    open(): None\n}\n";
        fs::write(cache_dir.join("runtime_env.fid"), fid_content).expect("write fid");

        env::set_var(
            "FRAME_FID_PATH",
            temp.path()
                .join("custom/cache/fid")
                .to_string_lossy()
                .to_string(),
        );

        let mut arcanum = Arcanum::new();
        let summary =
            load_fid_modules(&mut arcanum, TargetLanguage::TypeScript, None).expect("load via env");
        env::remove_var("FRAME_FID_PATH");

        assert!(summary.modules_loaded >= 1);
        assert!(arcanum.lookup_native_module("runtime::env").is_some());
    }
}
