use std::env;
use std::path::{Path, PathBuf};

use framec::frame_c::v3::test_harness_rs::run_validation_for_category;

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
    let summary = match run_validation_for_category(&root, &framec_path, language, category) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    println!(
        "Summary: passed={} failed={}",
        summary.passed, summary.failed
    );
    if summary.failed > 0 {
        std::process::exit(1);
    }
}
