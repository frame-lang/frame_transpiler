use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Minimal Rust-based V3 test harness prototype.
///
/// This binary is an exploratory Stage 18 tool. It runs a narrow subset of
/// the V3 transpile/validation tests directly from Rust, without going
/// through the Python runner. The initial scope is:
///   - language: one of `python`, `typescript`, `rust`
///   - category: a single V3 language-specific category
///   - mode: validation-only (no exec)
///
/// Usage:
///   v3_rs_test_runner <language> <category> [framec_path]
///
/// Example:
///   v3_rs_test_runner python v3_core ./target/debug/framec
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.len() > 4 {
        eprintln!(
            "Usage: {} <language> <category> [framec_path]",
            args.get(0).map(String::as_str).unwrap_or("v3_rs_test_runner")
        );
        std::process::exit(1);
    }

    let language = &args[1];
    let category = &args[2];
    let framec_path = if args.len() == 4 {
        PathBuf::from(&args[3])
    } else {
        PathBuf::from("target/debug/framec")
    };

    if !framec_path.is_file() {
        eprintln!(
            "framec binary not found at {}",
            framec_path.display()
        );
        std::process::exit(1);
    }

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();
    let tests_root = root
        .join("framec_tests")
        .join("language_specific")
        .join(language)
        .join(category);

    if !tests_root.is_dir() {
        eprintln!("Test directory not found: {}", tests_root.display());
        std::process::exit(1);
    }

    let mut files: Vec<PathBuf> = Vec::new();
    collect_frm_files(&tests_root, &mut files);
    if files.is_empty() {
        eprintln!("No .frm files found under {}", tests_root.display());
        std::process::exit(1);
    }

    println!(
        "v3_rs_test_runner: language={} category={} files={}",
        language,
        category,
        files.len()
    );

    let mut passed = 0usize;
    let mut failed = 0usize;

    for path in files {
        let status = Command::new(&framec_path)
            .arg("compile")
            .arg("--language")
            .arg(language)
            .arg("--validate-only")
            .arg(&path)
            .current_dir(&root)
            .status();

        match status {
            Ok(s) if s.success() => {
                passed += 1;
            }
            Ok(s) => {
                failed += 1;
                eprintln!(
                    "  FAIL (exit={}): {}",
                    s.code().unwrap_or(-1),
                    path.display()
                );
            }
            Err(e) => {
                failed += 1;
                eprintln!(
                    "  ERROR (spawn): {} ({})",
                    path.display(),
                    e
                );
            }
        }
    }

    println!(
        "Summary: passed={} failed={}",
        passed, failed
    );
    if failed > 0 {
        std::process::exit(1);
    }
}

fn collect_frm_files(dir: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_frm_files(&path, out);
            } else if path
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s.eq_ignore_ascii_case("frm"))
                .unwrap_or(false)
            {
                out.push(path);
            }
        }
    }
}

