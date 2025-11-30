use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use regex::Regex;

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

/// Parse `@run-expect:` and `@run-exact:` metadata from the first 20 lines
/// of a fixture, mirroring the Python runner's behavior.
fn parse_run_meta(path: &Path) -> (Vec<String>, Option<String>) {
    let mut run_expect: Vec<String> = Vec::new();
    let mut run_exact: Option<String> = None;
    let content = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return (run_expect, run_exact),
    };
    for (i, line) in content.lines().enumerate() {
        if i > 20 {
            break;
        }
        let trimmed = line.trim_start();
        if !(trimmed.starts_with('#') || trimmed.starts_with("//")) {
            continue;
        }
        if let Some(idx) = trimmed.find("@run-expect:") {
            let pat = trimmed[idx + "@run-expect:".len()..].trim();
            if !pat.is_empty() {
                run_expect.push(pat.to_string());
            }
            continue;
        }
        if let Some(idx) = trimmed.find("@run-exact:") {
            let pat = trimmed[idx + "@run-exact:".len()..].trim_end();
            if !pat.is_empty() {
                run_exact = Some(pat.to_string());
            }
            continue;
        }
    }
    (run_expect, run_exact)
}

/// Execute Rust V3 exec-smoke fixtures by:
///   - compiling `.frm` to Rust source via `framec --emit-debug`
///   - compiling the Rust source with `rustc`
///   - running the resulting binary
///   - applying the same marker checks used by the Python runner
pub fn run_rust_exec_smoke(
    repo_root: &Path,
    framec_path: &Path,
    category: &str,
) -> Result<TestSummary, String> {
    let language = "rust";
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
        "v3_rs_exec_harness: language={} category={} files={}",
        language,
        category,
        files.len()
    );

    let mut passed = 0usize;
    let mut failed = 0usize;

    // Use a fixed temporary directory under the repo for generated Rust sources.
    let out_root = repo_root.join("target").join("v3_rs_exec_smoke").join("rust");
    if let Err(e) = fs::create_dir_all(&out_root) {
        return Err(format!(
            "Failed to create exec-smoke output dir {}: {e}",
            out_root.display()
        ));
    }

    for frm_path in files {
        let stem = match frm_path.file_stem().and_then(|s| s.to_str()) {
            Some(s) => s.to_string(),
            None => {
                eprintln!("  WARN: skipping fixture with invalid name: {}", frm_path.display());
                continue;
            }
        };

        let rs_path = out_root.join(format!("{stem}__v3.rs"));
        let exe_path = out_root.join(format!("{stem}__v3_exec"));

        // 1) Compile Frame → Rust source via framec --emit-debug with FRAME_EMIT_EXEC=1
        let mut cmd = Command::new(framec_path);
        cmd.arg("compile")
            .arg("-l")
            .arg(language)
            .arg("--emit-debug")
            .arg(&frm_path)
            .current_dir(repo_root);
        cmd.env("FRAME_EMIT_EXEC", "1");
        let compile_output = cmd.output();

        let compile_src = match compile_output {
            Ok(out) => {
                if !out.status.success() {
                    failed += 1;
                    eprintln!("  FAIL (compile): {}", frm_path.display());
                    let mut text = String::new();
                    text.push_str(&String::from_utf8_lossy(&out.stdout));
                    text.push_str(&String::from_utf8_lossy(&out.stderr));
                    for line in text.lines() {
                        eprintln!("    {}", line);
                    }
                    continue;
                }
                String::from_utf8_lossy(&out.stdout).to_string()
            }
            Err(e) => {
                failed += 1;
                eprintln!(
                    "  ERROR (spawn framec): {} ({})",
                    frm_path.display(),
                    e
                );
                continue;
            }
        };

        if let Err(e) = fs::write(&rs_path, compile_src) {
            failed += 1;
            eprintln!(
                "  ERROR: failed to write Rust source {}: {}",
                rs_path.display(),
                e
            );
            continue;
        }

        // 2) rustc compile
        let rustc_output = Command::new("rustc")
            .arg(&rs_path)
            .arg("-o")
            .arg(&exe_path)
            .current_dir(&out_root)
            .output();

        let run_output = match rustc_output {
            Ok(out) => {
                if !out.status.success() {
                    failed += 1;
                    eprintln!("  FAIL (rustc): {}", rs_path.display());
                    let mut text = String::new();
                    text.push_str(&String::from_utf8_lossy(&out.stdout));
                    text.push_str(&String::from_utf8_lossy(&out.stderr));
                    for line in text.lines() {
                        eprintln!("    {}", line);
                    }
                    continue;
                }
                // 3) Run binary
                match Command::new(&exe_path)
                    .current_dir(&out_root)
                    .output()
                {
                    Ok(run) => run,
                    Err(e) => {
                        failed += 1;
                        eprintln!(
                            "  ERROR (spawn exec): {} ({})",
                            exe_path.display(),
                            e
                        );
                        continue;
                    }
                }
            }
            Err(e) => {
                failed += 1;
                eprintln!(
                    "  ERROR (spawn rustc): {} ({})",
                    rs_path.display(),
                    e
                );
                continue;
            }
        };

        let mut out_text = String::new();
        out_text.push_str(&String::from_utf8_lossy(&run_output.stdout));
        out_text.push_str(&String::from_utf8_lossy(&run_output.stderr));

        let mut exec_ok = true;
        let mut reason = String::new();

        if !run_output.status.success() {
            exec_ok = false;
            reason = format!("non-zero exit ({})", run_output.status);
        } else {
            let lower = out_text.to_lowercase();
            if lower.contains("panic") || out_text.contains("FAIL:") || out_text.contains("FAILED:") {
                exec_ok = false;
                reason = "panic/FAIL marker detected".to_string();
            }
        }

        // Fixture-specific marker checks (mirror Python runner behavior).
        if exec_ok {
            let name = stem.as_str();
            match name {
                "transition_basic" => {
                    if !out_text.contains("TRANSITION:") {
                        exec_ok = false;
                        reason = "Missing TRANSITION marker".to_string();
                    }
                }
                "forward_parent" => {
                    if !out_text.contains("FORWARD:PARENT") {
                        exec_ok = false;
                        reason = "Missing FORWARD:PARENT marker".to_string();
                    }
                }
                "stack_ops" => {
                    if !(out_text.contains("STACK:PUSH") && out_text.contains("STACK:POP")) {
                        exec_ok = false;
                        reason = "Missing STACK markers".to_string();
                    }
                }
                "mixed_ops" => {
                    if !(out_text.contains("STACK:PUSH") && out_text.contains("TRANSITION:")) {
                        exec_ok = false;
                        reason = "Missing MIXED markers".to_string();
                    }
                }
                "stack_then_transition" | "nested_stack_then_transition" => {
                    if !(out_text.contains("STACK:PUSH")
                        && out_text.contains("STACK:POP")
                        && out_text.contains("TRANSITION:"))
                    {
                        exec_ok = false;
                        reason = "Missing STACK/TRANSITION markers".to_string();
                    }
                }
                "if_forward_else_transition" => {
                    if !(out_text.contains("FORWARD:PARENT") || out_text.contains("TRANSITION:")) {
                        exec_ok = false;
                        reason = "Missing FORWARD or TRANSITION marker".to_string();
                    }
                }
                _ => {}
            }
        }

        if exec_ok {
            passed += 1;
        } else {
            failed += 1;
            eprintln!("  FAIL (exec): {} -- {}", frm_path.display(), reason);
            for line in out_text.lines().take(20) {
                eprintln!("    {}", line);
            }
        }

        // Best-effort cleanup of the executable (ignore errors).
        let _ = fs::remove_file(&exe_path);
    }

    Ok(TestSummary {
        language: language.to_string(),
        category: category.to_string(),
        passed,
        failed,
    })
}

/// Execute curated Rust V3 exec fixtures (v3_core, v3_control_flow, v3_systems)
/// using FRAME_EMIT_EXEC wrappers and `@run-expect` / `@run-exact` metadata.
pub fn run_rust_curated_exec_for_category(
    repo_root: &Path,
    framec_path: &Path,
    category: &str,
) -> Result<TestSummary, String> {
    let language = "rust";
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
        "v3_rs_exec_harness_curated: language={} category={} files={}",
        language,
        category,
        files.len()
    );

    let mut passed = 0usize;
    let mut failed = 0usize;

    let out_root = repo_root.join("target").join("v3_rs_exec_curated").join("rust");
    if let Err(e) = fs::create_dir_all(&out_root) {
        return Err(format!(
            "Failed to create curated exec output dir {}: {e}",
            out_root.display()
        ));
    }

    for frm_path in files {
        let stem = match frm_path.file_stem().and_then(|s| s.to_str()) {
            Some(s) => s.to_string(),
            None => {
                eprintln!("  WARN: skipping fixture with invalid name: {}", frm_path.display());
                continue;
            }
        };

        let (run_expect, run_exact) = parse_run_meta(&frm_path);
        // Mirror the Python runner's gating: only enforce curated exec when
        // run expectations are present; otherwise treat as exec-gated.
        if run_expect.is_empty() && run_exact.is_none() {
            println!(
                "  SKIP (no @run-expect/@run-exact): {}",
                frm_path.display()
            );
            passed += 1;
            continue;
        }

        let rs_path = out_root.join(format!("{stem}__v3.rs"));
        let exe_path = out_root.join(format!("{stem}__v3_exec"));

        // Compile Frame → Rust source with FRAME_EMIT_EXEC=1.
        let mut cmd = Command::new(framec_path);
        cmd.arg("compile")
            .arg("-l")
            .arg(language)
            .arg("--emit-debug")
            .arg(&frm_path)
            .current_dir(repo_root);
        cmd.env("FRAME_EMIT_EXEC", "1");
        let compile_output = cmd.output();

        let compile_src = match compile_output {
            Ok(out) => {
                if !out.status.success() {
                    failed += 1;
                    eprintln!("  FAIL (compile): {}", frm_path.display());
                    let mut text = String::new();
                    text.push_str(&String::from_utf8_lossy(&out.stdout));
                    text.push_str(&String::from_utf8_lossy(&out.stderr));
                    for line in text.lines() {
                        eprintln!("    {}", line);
                    }
                    continue;
                }
                String::from_utf8_lossy(&out.stdout).to_string()
            }
            Err(e) => {
                failed += 1;
                eprintln!(
                    "  ERROR (spawn framec): {} ({})",
                    frm_path.display(),
                    e
                );
                continue;
            }
        };

        if let Err(e) = fs::write(&rs_path, compile_src) {
            failed += 1;
            eprintln!(
                "  ERROR: failed to write Rust source {}: {}",
                rs_path.display(),
                e
            );
            continue;
        }

        // rustc compile
        let rustc_output = Command::new("rustc")
            .arg(&rs_path)
            .arg("-o")
            .arg(&exe_path)
            .current_dir(&out_root)
            .output();

        let run_output = match rustc_output {
            Ok(out) => {
                if !out.status.success() {
                    failed += 1;
                    eprintln!("  FAIL (rustc): {}", rs_path.display());
                    let mut text = String::new();
                    text.push_str(&String::from_utf8_lossy(&out.stdout));
                    text.push_str(&String::from_utf8_lossy(&out.stderr));
                    for line in text.lines() {
                        eprintln!("    {}", line);
                    }
                    continue;
                }
                match Command::new(&exe_path)
                    .current_dir(&out_root)
                    .output()
                {
                    Ok(run) => run,
                    Err(e) => {
                        failed += 1;
                        eprintln!(
                            "  ERROR (spawn exec): {} ({})",
                            exe_path.display(),
                            e
                        );
                        continue;
                    }
                }
            }
            Err(e) => {
                failed += 1;
                eprintln!(
                    "  ERROR (spawn rustc): {} ({})",
                    rs_path.display(),
                    e
                );
                continue;
            }
        };

        let mut out_text = String::new();
        out_text.push_str(&String::from_utf8_lossy(&run_output.stdout));
        out_text.push_str(&String::from_utf8_lossy(&run_output.stderr));

        let mut exec_ok = true;
        let mut reason = String::new();

        if !run_output.status.success() {
            exec_ok = false;
            reason = format!("non-zero exit ({})", run_output.status);
        } else {
            let lower = out_text.to_lowercase();
            if lower.contains("panic") || out_text.contains("FAIL:") || out_text.contains("FAILED:") {
                exec_ok = false;
                reason = "panic/FAIL marker detected".to_string();
            }
        }

        // Apply @run-expect patterns as regexes (fall back to literal match on error).
        if exec_ok && !run_expect.is_empty() {
            for pat in &run_expect {
                let re = Regex::new(pat).unwrap_or_else(|_| Regex::new(&regex::escape(pat)).unwrap());
                if !re.is_match(&out_text) {
                    exec_ok = false;
                    reason = format!("Run output expectation failed: missing pattern {:?}", pat);
                    break;
                }
            }
        }

        // Apply @run-exact if present.
        if exec_ok {
            if let Some(ref exact) = run_exact {
                let want = exact.trim();
                let got = out_text.trim();
                if got != want {
                    exec_ok = false;
                    reason = format!("Run exact mismatch.\nWanted:\n{}\nGot:\n{}", want, got);
                }
            }
        }

        if exec_ok {
            passed += 1;
        } else {
            failed += 1;
            eprintln!("  FAIL (curated exec): {} -- {}", frm_path.display(), reason);
            for line in out_text.lines().take(20) {
                eprintln!("    {}", line);
            }
        }

        let _ = fs::remove_file(&exe_path);
    }

    Ok(TestSummary {
        language: language.to_string(),
        category: category.to_string(),
        passed,
        failed,
    })
}
