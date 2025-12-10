use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use framec::frame_c::v3::test_harness_rs::{
    run_python_curated_exec_for_category,
    run_python_exec_smoke,
    run_rust_curated_exec_for_category,
    run_rust_exec_smoke,
    run_typescript_curated_exec_for_category,
    run_validation_for_category,
};

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
    if args.len() < 3 {
        let bin = args.get(0).map(String::as_str).unwrap_or("v3_rs_test_runner");
        eprintln!("Usage:");
        eprintln!("  {bin} <language> <category> [framec_path]");
        eprintln!("  {bin} compare <language> <category|all_v3> [framec_path]");
        std::process::exit(1);
    }

    if args[1] == "compare" {
        run_compare_mode(&args);
        return;
    }

    if args[1] == "exec-smoke" {
        run_exec_smoke_mode(&args);
        return;
    }

    if args[1] == "exec-curated" {
        run_exec_curated_mode(&args);
        return;
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
    // Support a special `all_v3` category to run validation over all
    // `v3_*` language-specific categories for the given language.
    let categories: Vec<String> = if category.eq_ignore_ascii_case("all_v3") {
        discover_v3_categories(&root, language)
    } else {
        vec![category.to_string()]
    };

    if categories.is_empty() {
        eprintln!(
            "No v3_* categories found for language '{}' under framec_tests/language_specific",
            language
        );
        std::process::exit(1);
    }

    let mut total_passed = 0usize;
    let mut total_failed = 0usize;

    for cat in &categories {
        let summary = match run_validation_for_category(&root, &framec_path, language, cat) {
            Ok(s) => s,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        };
        println!(
            "Category {}: passed={} failed={}",
            cat, summary.passed, summary.failed
        );
        total_passed += summary.passed;
        total_failed += summary.failed;
    }

    println!(
        "Total summary: passed={} failed={}",
        total_passed, total_failed
    );
    if total_failed > 0 {
        std::process::exit(1);
    }
}

/// Discover `v3_*` categories for the given language under
/// `framec_tests/language_specific/<language>/`.
fn discover_v3_categories(root: &Path, language: &str) -> Vec<String> {
    let base = root
        .join("framec_tests")
        .join("language_specific")
        .join(language);
    // For now, restrict `all_v3` to the PRT-focused V3 categories where the
    // Rust harness is known to match the Python runner's semantics.
    let candidates: &[&str] = match language {
        "python" => &["v3_core", "v3_control_flow", "v3_systems", "v3_persistence", "v3_systems_runtime"],
        "typescript" => &["v3_core", "v3_control_flow", "v3_systems", "v3_persistence"],
        "rust" => &["v3_core", "v3_control_flow", "v3_systems", "v3_async"],
        _ => &[],
    };

    let mut out: Vec<String> = Vec::new();
    for cat in candidates {
        let p = base.join(cat);
        if p.is_dir() {
            out.push(cat.to_string());
        }
    }
    out.sort();
    out
}

/// Execute Rust V3 exec-smoke fixtures using the Rust harness.
fn run_exec_smoke_mode(args: &[String]) {
    if args.len() < 4 || args.len() > 5 {
        let bin = args.get(0).map(String::as_str).unwrap_or("v3_rs_test_runner");
        eprintln!(
            "Usage: {bin} exec-smoke <language> v3_exec_smoke [framec_path]"
        );
        std::process::exit(1);
    }

    let language = &args[2];
    let category = &args[3];
    if category != "v3_exec_smoke" {
        eprintln!("exec-smoke mode currently expects category v3_exec_smoke");
        std::process::exit(1);
    }

    let framec_path = if args.len() == 5 {
        PathBuf::from(&args[4])
    } else {
        PathBuf::from("target/debug/framec")
    };

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();

    let summary_result = match language.as_str() {
        "rust" => run_rust_exec_smoke(&root, &framec_path, category),
        "python" => run_python_exec_smoke(&root, &framec_path, category),
        "typescript" => framec::frame_c::v3::test_harness_rs::run_typescript_exec_smoke(&root, &framec_path, category),
        _ => {
            eprintln!(
                "exec-smoke mode currently supports languages: python, typescript, rust (category v3_exec_smoke)"
            );
            std::process::exit(1);
        }
    };

    let summary = summary_result.unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    println!(
        "Exec-smoke summary: passed={} failed={}",
        summary.passed, summary.failed
    );
    if summary.failed > 0 {
        std::process::exit(1);
    }
}

/// Execute curated V3 exec fixtures using the Rust harness and FRAME_EMIT_EXEC
/// wrappers. Supported languages:
///   - rust: v3_core, v3_control_flow, v3_systems
///   - python/typescript: v3_core, v3_control_flow, v3_systems, v3_persistence
fn run_exec_curated_mode(args: &[String]) {
    if args.len() < 4 || args.len() > 5 {
        let bin = args.get(0).map(String::as_str).unwrap_or("v3_rs_test_runner");
        eprintln!(
            "Usage: {bin} exec-curated <language> <category|all_curated> [framec_path]"
        );
        std::process::exit(1);
    }

    let language = &args[2];
    let category = &args[3];

    let framec_path = if args.len() == 5 {
        PathBuf::from(&args[4])
    } else {
        PathBuf::from("target/debug/framec")
    };

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();

    let categories: Vec<&str> = match (language.as_str(), category.as_str()) {
        ("rust", "all_curated") => vec!["v3_core", "v3_control_flow", "v3_systems", "v3_async"],
        ("rust", "v3_core") | ("rust", "v3_control_flow") | ("rust", "v3_systems") | ("rust", "v3_async") => vec![category],
        ("rust", other) => {
            eprintln!(
                "exec-curated rust currently supports categories: v3_core, v3_control_flow, v3_systems, v3_async (got: {})",
                other
            );
            std::process::exit(1);
        }
        ("python", "all_curated") | ("typescript", "all_curated") => {
            vec!["v3_core", "v3_control_flow", "v3_systems", "v3_persistence"]
        }
        ("python", cat) | ("typescript", cat) => vec![cat],
        _ => {
            eprintln!(
                "exec-curated mode currently supports languages: rust, python, typescript"
            );
            std::process::exit(1);
        }
    };

    let mut total_passed = 0usize;
    let mut total_failed = 0usize;

    for cat in categories {
        let summary = match language.as_str() {
            "rust" => run_rust_curated_exec_for_category(&root, &framec_path, cat),
            "python" => run_python_curated_exec_for_category(&root, &framec_path, cat),
            "typescript" => run_typescript_curated_exec_for_category(&root, &framec_path, cat),
            _ => unreachable!(),
        }
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });
        println!(
            "Curated exec category {}: passed={} failed={}",
            cat, summary.passed, summary.failed
        );
        total_passed += summary.passed;
        total_failed += summary.failed;
    }

    println!(
        "Curated exec summary: passed={} failed={}",
        total_passed, total_failed
    );
    if total_failed > 0 {
        std::process::exit(1);
    }
}

/// Compare the Rust harness against the Python runner for the given
/// language/category pair (or `all_v3` subset).
fn run_compare_mode(args: &[String]) {
    if args.len() < 4 || args.len() > 5 {
        let bin = args.get(0).map(String::as_str).unwrap_or("v3_rs_test_runner");
        eprintln!(
            "Usage: {bin} compare <language> <category|all_v3> [framec_path]"
        );
        std::process::exit(1);
    }

    let language = &args[2];
    let category = &args[3];
    let framec_path = if args.len() == 5 {
        PathBuf::from(&args[4])
    } else {
        PathBuf::from("target/debug/framec")
    };

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();

    let categories: Vec<String> = if category.eq_ignore_ascii_case("all_v3") {
        discover_v3_categories(&root, language)
    } else {
        vec![category.to_string()]
    };

    if categories.is_empty() {
        eprintln!(
            "No v3_* categories found for language '{}' under framec_tests/language_specific (compare mode)",
            language
        );
        std::process::exit(1);
    }

    let mut mismatches = 0usize;

    for cat in &categories {
        println!("== compare: language={} category={} ==", language, cat);

        // 1) Run Rust harness (validation-only).
        let rust_summary =
            match run_validation_for_category(&root, &framec_path, language, cat) {
                Ok(s) => s,
                Err(err) => {
                    eprintln!("  Rust harness error: {}", err);
                    mismatches += 1;
                    continue;
                }
            };

        // 2) Run Python test runner in transpile-only mode for the same slice.
        let py_output = Command::new("python3")
            .arg("framec_tests/runner/frame_test_runner.py")
            .arg("--languages")
            .arg(language)
            .arg("--categories")
            .arg(cat)
            .arg("--framec")
            .arg(&framec_path)
            .arg("--transpile-only")
            .arg("--no-run")
            .output();

        let (py_ok, py_text) = match py_output {
            Ok(out) => {
                let ok = out.status.success();
                let mut text = String::new();
                text.push_str(&String::from_utf8_lossy(&out.stdout));
                text.push_str(&String::from_utf8_lossy(&out.stderr));
                (ok, text)
            }
            Err(e) => {
                eprintln!("  Python runner spawn error: {}", e);
                mismatches += 1;
                continue;
            }
        };

        let rust_ok = rust_summary.failed == 0;

        println!(
            "  Rust harness: passed={} failed={}",
            rust_summary.passed, rust_summary.failed
        );
        println!("  Python runner exit: {}", if py_ok { "ok" } else { "non-zero" });

        if !py_ok || !rust_ok {
            mismatches += 1;
            if !py_ok {
                eprintln!("  Python runner reported failure for this slice:");
                for line in py_text.lines() {
                    eprintln!("    {}", line);
                }
            }
        }
    }

    if mismatches > 0 {
        eprintln!("compare: {} mismatched slice(s)", mismatches);
        std::process::exit(1);
    } else {
        println!("compare: all slices matched (Python runner and Rust harness both succeeded)");
    }
}
