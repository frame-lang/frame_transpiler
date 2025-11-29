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
        let expected_codes = parse_expected_codes(&path);
        let is_negative = !expected_codes.is_empty();

        let output = Command::new(&framec_path)
            .arg("compile")
            .arg("--language")
            .arg(language)
            .arg("--validation-only")
            .arg(&path)
            .current_dir(&root)
            .output();

        match output {
            Ok(out) => {
                let success = out.status.success();
                let code = out.status.code().unwrap_or(-1);
                let mut text = String::new();
                text.push_str(&String::from_utf8_lossy(&out.stdout));
                text.push_str(&String::from_utf8_lossy(&out.stderr));

                let (test_passed, reason) = if !is_negative {
                    // Positive fixture (no @expect): expect validation success.
                    if success {
                        (true, None)
                    } else {
                        (false, Some(format!("expected success, exit={code}")))
                    }
                } else {
                    // Negative fixture (has @expect): expect failure and all expected codes present.
                    if success {
                        (
                            false,
                            Some("expected failure, but validation succeeded".to_string()),
                        )
                    } else {
                        let mut missing: Vec<String> = Vec::new();
                        for ec in &expected_codes {
                            if !text.contains(ec) {
                                missing.push(ec.clone());
                            }
                        }
                        if missing.is_empty() {
                            (true, None)
                        } else {
                            (
                                false,
                                Some(format!("missing expected codes: {:?}", missing)),
                            )
                        }
                    }
                };

                if test_passed {
                    passed += 1;
                } else {
                    failed += 1;
                    eprintln!("  FAIL: {}", path.display());
                    if let Some(r) = reason {
                        eprintln!("    {}", r);
                    }
                    if !text.trim().is_empty() {
                        for line in text.lines() {
                            eprintln!("    {}", line);
                        }
                    }
                }
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

/// Parse expected error codes from metadata (`@expect:`) in a .frm file.
///
/// This is a minimal parser intended for V3 fixtures that use lines like:
///   # @expect: E301
///   // @expect: E200 E300
fn parse_expected_codes(path: &Path) -> Vec<String> {
    let mut codes: Vec<String> = Vec::new();
    let content = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return codes,
    };
    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') || trimmed.starts_with("//") {
            if let Some(idx) = trimmed.find("@expect:") {
                let rest = &trimmed[idx + "@expect:".len()..];
                for tok in rest.split(|c: char| c.is_whitespace() || c == ',' || c == ';') {
                    let t = tok.trim();
                    if t.len() >= 2 && t.starts_with('E') && t[1..].chars().all(|ch| ch.is_ascii_digit()) {
                        codes.push(t.to_string());
                    }
                }
            }
        }
    }
    codes
}
