use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Summary of a single category run under the Rust V3 test harness.
pub struct TestSummary {
    pub language: String,
    pub category: String,
    pub passed: usize,
    pub failed: usize,
}

/// Run validation-only tests for a single `<language>/<category>` pair.
///
/// This is the library entrypoint used by the `v3_rs_test_runner` binary and
/// is designed to mirror the core behavior of the Python V3 runner for
/// validation-only scenarios:
///   - discovers `.frm` fixtures under
///     `framec_tests/language_specific/<language>/<category>/`
///   - treats fixtures with `@expect: Exxx` metadata as negatives
///   - treats fixtures without `@expect:` as positives
pub fn run_validation_for_category(
    repo_root: &Path,
    framec_path: &Path,
    language: &str,
    category: &str,
) -> Result<TestSummary, String> {
    if !framec_path.is_file() {
        return Err(format!(
            "framec binary not found at {}",
            framec_path.display()
        ));
    }

    let tests_root = repo_root
        .join("framec_tests")
        .join("language_specific")
        .join(language)
        .join(category);

    if !tests_root.is_dir() {
        return Err(format!("Test directory not found: {}", tests_root.display()));
    }

    let mut files: Vec<PathBuf> = Vec::new();
    collect_frm_files(&tests_root, &mut files);
    if files.is_empty() {
        return Err(format!(
            "No .frm files found under {}",
            tests_root.display()
        ));
    }

    println!(
        "v3_rs_test_harness: language={} category={} files={}",
        language,
        category,
        files.len()
    );

    let mut passed = 0usize;
    let mut failed = 0usize;

    for path in files {
        let expected_codes = parse_expected_codes(&path);
        let is_negative = !expected_codes.is_empty();

        let output = Command::new(framec_path)
            .arg("compile")
            .arg("--language")
            .arg(language)
            .arg("--validation-only")
            .arg(&path)
            .current_dir(repo_root)
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

    Ok(TestSummary {
        language: language.to_string(),
        category: category.to_string(),
        passed,
        failed,
    })
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

