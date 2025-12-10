use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::frame_c::v3::docker_executor::{DockerTestExecutor, DockerTestResult};

/// Test execution configuration
#[derive(Debug, Clone)]
pub struct TestConfig {
    /// Use Docker for test execution
    pub use_docker: bool,
    /// Docker image override (if not using default for language)
    pub docker_image: Option<String>,
    /// Number of parallel workers for test execution
    pub parallel_workers: usize,
    /// Timeout in seconds for individual tests
    pub test_timeout: u64,
    /// Verbose output
    pub verbose: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            use_docker: false,
            docker_image: None,
            parallel_workers: 1,
            test_timeout: 60,
            verbose: false,
        }
    }
}

/// Comprehensive test metadata matching Python runner capabilities
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TestMetadata {
    /// Expected error codes for negative tests
    pub expect: Vec<String>,
    /// Mode for error code matching: "equal" or "superset"
    pub expect_mode: Option<String>,
    /// Test description/metadata
    pub meta: Vec<String>,
    /// Conditions for skipping test
    pub skip_if: Vec<String>,
    /// Expected patterns in execution output
    pub run_expect: Vec<String>,
    /// Exact execution output expected
    pub run_exact: Vec<String>,
    /// Expected patterns in compilation output
    pub compile_expect: Vec<String>,
    /// Working directory for test execution
    pub cwd: Option<String>,
    /// Marker that execution should succeed
    pub exec_ok: bool,
    /// Test is known to be flaky
    pub flaky: bool,
    /// Test is core functionality
    pub core: bool,
    /// Test is non-core functionality
    pub noncore: bool,
    /// Import call validations
    pub import_calls: Vec<String>,
    /// Golden frame map file
    pub frame_map_golden: Option<String>,
    /// Golden visitor map file
    pub visitor_map_golden: Option<String>,
    /// Python-specific compilation check
    pub py_compile: bool,
    /// TypeScript-specific compilation check
    pub tsc_compile: bool,
    /// Rust-specific compilation check
    pub rs_compile: bool,
}

/// Summary of a single category run under the Rust V3 test harness.
pub struct TestSummary {
    pub language: String,
    pub category: String,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
}

impl TestSummary {
    pub fn total(&self) -> usize {
        self.passed + self.failed + self.skipped
    }
    pub fn junit(&self) -> String {
        format!(
            "<testsuite name=\"{}-{}\" tests=\"{}\" failures=\"{}\" skipped=\"{}\"></testsuite>",
            self.language,
            self.category,
            self.total(),
            self.failed,
            self.skipped
        )
    }
}

/// Ensure a reusable Cargo crate exists for Rust exec fixtures so we can link
/// against tokio/serde without invoking rustc directly.
fn ensure_rust_exec_crate(out_root: &Path, repo_root: &Path, package_name: &str) -> Result<(), String> {
    fs::create_dir_all(out_root.join("src"))
        .map_err(|e| format!("create exec output dir {}: {e}", out_root.display()))?;
    let persistence_path = repo_root.join("runtime").join("persistence_rs");
    let persistence_str = persistence_path
        .to_str()
        .ok_or_else(|| format!("non-UTF8 path for persistence lib: {}", persistence_path.display()))?
        .replace('\\', "\\\\");
    let cargo_toml = format!(
        r#"[package]
name = "{package_name}"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = {{ version = "1", features = ["rt-multi-thread", "macros", "time"] }}
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1"
frame_persistence_rs = {{ path = "{persistence_str}" }}

[workspace]
members = ["."]
resolver = "2"
"#
    );
    fs::write(out_root.join("Cargo.toml"), cargo_toml)
        .map_err(|e| format!("write Cargo.toml in {}: {e}", out_root.display()))
}

/// Write the generated Rust program into the exec crate and run it via Cargo,
/// returning the combined stdout/stderr for downstream checks.
fn cargo_run_rust_program(out_root: &Path, program_src: &str) -> Result<std::process::Output, String> {
    let src_path = out_root.join("src").join("main.rs");
    fs::write(&src_path, program_src).map_err(|e| {
        format!(
            "  ERROR: failed to write Rust source {}: {}",
            src_path.display(),
            e
        )
    })?;
    let mut cmd = Command::new("cargo");
    // Propagate marker env so FRAME_EMIT_EXEC wrappers can print stack/forward markers.
    let markers = std::env::var("FRAME_EXEC_MARKERS").unwrap_or_else(|_| "1".to_string());
    cmd.env("FRAME_EXEC_MARKERS", markers);
    cmd
        .arg("run")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg(out_root.join("Cargo.toml"))
        .current_dir(out_root)
        .output()
        .map_err(|e| format!("  ERROR (spawn cargo run): {} ({})", src_path.display(), e))
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
///   - optionally filters tests by metadata tags
pub fn run_validation_for_category(
    repo_root: &Path,
    framec_path: &Path,
    language: &str,
    category: &str,
) -> Result<TestSummary, String> {
    run_validation_for_category_with_filter(repo_root, framec_path, language, category, None)
}

/// Run validation-only tests with optional metadata filtering
pub fn run_validation_for_category_with_filter(
    repo_root: &Path,
    framec_path: &Path,
    language: &str,
    category: &str,
    metadata_filter: Option<&str>,
) -> Result<TestSummary, String> {
    // Use default test config for backwards compatibility
    let config = TestConfig::default();
    run_validation_for_category_with_config(
        repo_root,
        framec_path,
        language,
        category,
        metadata_filter,
        &config,
    )
}

/// Run validation-only tests with full configuration options
pub fn run_validation_for_category_with_config(
    repo_root: &Path,
    framec_path: &Path,
    language: &str,
    category: &str,
    metadata_filter: Option<&str>,
    config: &TestConfig,
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
    let mut skipped = 0usize;

    for path in files {
        // Apply metadata filtering if requested
        if let Some(filter) = metadata_filter {
            let metadata = parse_fixture_metadata(&path);
            if !should_run_test(&metadata, filter) {
                skipped += 1;
                continue;
            }
        }

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
        skipped,
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

/// Check if a test should run based on metadata filter.
/// Supports multiple filter types:
///   - @core: Run only core tests
///   - @noncore: Run only non-core tests
///   - @flaky: Include flaky tests (normally skipped)
///   - !flaky: Exclude flaky tests
///   - skip:<condition>: Check skip conditions
pub(crate) fn should_run_test(metadata: &TestMetadata, filter: &str) -> bool {
    // Parse filter expression
    let filter_lower = filter.to_lowercase();
    
    // Handle negation
    if let Some(negated) = filter_lower.strip_prefix('!') {
        return !matches_filter(metadata, negated);
    }
    
    matches_filter(metadata, &filter_lower)
}

/// Check if metadata matches a single filter criterion
pub(crate) fn matches_filter(metadata: &TestMetadata, filter: &str) -> bool {
    match filter {
        "core" | "@core" => metadata.core,
        "noncore" | "@noncore" => metadata.noncore,
        "flaky" | "@flaky" => metadata.flaky,
        "exec-ok" | "@exec-ok" => metadata.exec_ok,
        "py-compile" | "@py-compile" => metadata.py_compile,
        "tsc-compile" | "@tsc-compile" => metadata.tsc_compile,
        "rs-compile" | "@rs-compile" => metadata.rs_compile,
        filter if filter.starts_with("skip:") => {
            let condition = &filter[5..];
            metadata.skip_if.iter().any(|s| s.contains(condition))
        }
        filter if filter.starts_with("meta:") => {
            let tag = &filter[5..];
            metadata.meta.iter().any(|m| m.to_lowercase().contains(tag))
        }
        _ => {
            // Check if filter matches any meta tag
            metadata.meta.iter().any(|m| m.to_lowercase().contains(filter))
        }
    }
}

/// Parse comprehensive metadata from fixture header (first 20 lines).
/// Matches Python runner's parse_fixture_meta() functionality.
pub fn parse_fixture_metadata(path: &Path) -> TestMetadata {
    let mut meta = TestMetadata::default();
    
    let content = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return meta,
    };
    
    let lines: Vec<_> = content.lines().take(20).collect();
    
    for line in lines {
        let trimmed = line.trim_start();
        if !trimmed.starts_with('#') && !trimmed.starts_with("//") {
            continue;
        }
        
        // Remove comment markers and trim
        let clean = if trimmed.starts_with('#') {
            trimmed.trim_start_matches('#').trim()
        } else {
            trimmed.trim_start_matches("//").trim()
        };
        
        // Parse various directives
        if let Some(rest) = clean.strip_prefix("@expect:") {
            // Extract error codes
            for tok in rest.split(|c: char| c.is_whitespace() || c == ',' || c == ';') {
                let t = tok.trim();
                if t.len() >= 2 && t.starts_with('E') && t[1..].chars().all(|ch| ch.is_ascii_digit()) {
                    meta.expect.push(t.to_string());
                }
            }
        } else if let Some(mode) = clean.strip_prefix("@expect-mode:") {
            meta.expect_mode = Some(mode.trim().to_string());
        } else if let Some(desc) = clean.strip_prefix("@meta:") {
            meta.meta.push(desc.trim().to_string());
        } else if let Some(cond) = clean.strip_prefix("@skip-if:") {
            meta.skip_if.push(cond.trim().to_string());
        } else if let Some(pat) = clean.strip_prefix("@run-expect:") {
            meta.run_expect.push(pat.trim().to_string());
        } else if let Some(exact) = clean.strip_prefix("@run-exact:") {
            meta.run_exact.push(exact.trim().to_string());
        } else if let Some(pat) = clean.strip_prefix("@compile-expect:") {
            meta.compile_expect.push(pat.trim().to_string());
        } else if let Some(dir) = clean.strip_prefix("@cwd:") {
            meta.cwd = Some(dir.trim().to_string());
        } else if clean.starts_with("@exec-ok") {
            meta.exec_ok = true;
        } else if clean.starts_with("@flaky") {
            meta.flaky = true;
        } else if clean.starts_with("@core") {
            meta.core = true;
        } else if clean.starts_with("@noncore") {
            meta.noncore = true;
        } else if let Some(call) = clean.strip_prefix("@import-call:") {
            meta.import_calls.push(call.trim().to_string());
        } else if let Some(golden) = clean.strip_prefix("@frame-map-golden:") {
            meta.frame_map_golden = Some(golden.trim().to_string());
        } else if let Some(golden) = clean.strip_prefix("@visitor-map-golden:") {
            meta.visitor_map_golden = Some(golden.trim().to_string());
        } else if clean.starts_with("@py-compile") {
            meta.py_compile = true;
        } else if clean.starts_with("@tsc-compile") {
            meta.tsc_compile = true;
        } else if clean.starts_with("@rs-compile") {
            meta.rs_compile = true;
        }
    }
    
    meta
}

/// Parse expected error codes from metadata (`@expect:`) in a .frm file.
///
/// This is a minimal parser intended for V3 fixtures that use lines like:
///   # @expect: E301
///   // @expect: E200 E300
fn parse_expected_codes(path: &Path) -> Vec<String> {
    let meta = parse_fixture_metadata(path);
    meta.expect
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
    run_rust_exec_smoke_with_filter(repo_root, framec_path, category, None)
}

/// Execute Rust V3 exec-smoke fixtures with optional metadata filtering
pub fn run_rust_exec_smoke_with_filter(
    repo_root: &Path,
    framec_path: &Path,
    category: &str,
    metadata_filter: Option<&str>,
) -> Result<TestSummary, String> {
    // Use default test config for backwards compatibility
    let config = TestConfig::default();
    run_rust_exec_smoke_with_config(
        repo_root,
        framec_path,
        category,
        metadata_filter,
        &config,
    )
}

/// Execute Rust V3 exec-smoke fixtures with full configuration
pub fn run_rust_exec_smoke_with_config(
    repo_root: &Path,
    framec_path: &Path,
    category: &str,
    metadata_filter: Option<&str>,
    config: &TestConfig,
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
    let mut skipped = 0usize;

    // Use a fixed temporary directory under the repo for generated Rust sources.
    let out_root = repo_root.join("target").join("v3_rs_exec_smoke").join("rust");
    ensure_rust_exec_crate(&out_root, repo_root, "v3_rs_exec_smoke")
        .map_err(|e| format!("Failed to prepare Rust exec-smoke crate: {e}"))?;

    for frm_path in files {
        // Apply metadata filtering if requested
        if let Some(filter) = metadata_filter {
            let metadata = parse_fixture_metadata(&frm_path);
            if !should_run_test(&metadata, filter) {
                skipped += 1;
                continue;
            }
        }
        let stem = match frm_path.file_stem().and_then(|s| s.to_str()) {
            Some(s) => s.to_string(),
            None => {
                eprintln!("  WARN: skipping fixture with invalid name: {}", frm_path.display());
                continue;
            }
        };

        // 1) Compile Frame → Rust source via framec --emit-debug with FRAME_EMIT_EXEC=1.
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

        let run_output = match cargo_run_rust_program(&out_root, &compile_src) {
            Ok(run) => run,
            Err(e) => {
                failed += 1;
                eprintln!("  {}", e);
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
    }

    Ok(TestSummary {
        language: language.to_string(),
        category: category.to_string(),
        passed,
        failed,
        skipped,
    })
}

/// Execute Python V3 exec-smoke fixtures by:
///   - compiling `.frm` to a standalone Python program via FRAME_EMIT_EXEC=1
///   - running `python3` on the emitted script
///   - applying the same stem-based marker checks as the Python runner
pub fn run_python_exec_smoke(
    repo_root: &Path,
    framec_path: &Path,
    category: &str,
) -> Result<TestSummary, String> {
    run_python_exec_smoke_with_filter(repo_root, framec_path, category, None)
}

/// Execute Python V3 exec-smoke fixtures with optional metadata filtering
pub fn run_python_exec_smoke_with_filter(
    repo_root: &Path,
    framec_path: &Path,
    category: &str,
    metadata_filter: Option<&str>,
) -> Result<TestSummary, String> {
    // Use default test config for backwards compatibility
    let config = TestConfig::default();
    run_python_exec_smoke_with_config(
        repo_root,
        framec_path,
        category,
        metadata_filter,
        &config,
    )
}

/// Execute Python V3 exec-smoke fixtures with full configuration
pub fn run_python_exec_smoke_with_config(
    repo_root: &Path,
    framec_path: &Path,
    category: &str,
    metadata_filter: Option<&str>,
    config: &TestConfig,
) -> Result<TestSummary, String> {
    let language = "python";
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
    let mut skipped = 0usize;

    let out_root = repo_root.join("target").join("v3_rs_exec_smoke").join("python");
    if let Err(e) = fs::create_dir_all(&out_root) {
        return Err(format!(
            "Failed to create Python exec-smoke output dir {}: {e}",
            out_root.display()
        ));
    }

    for frm_path in files {
        // Apply metadata filtering if requested
        if let Some(filter) = metadata_filter {
            let metadata = parse_fixture_metadata(&frm_path);
            if !should_run_test(&metadata, filter) {
                skipped += 1;
                continue;
            }
        }
        let stem = match frm_path.file_stem().and_then(|s| s.to_str()) {
            Some(s) => s.to_string(),
            None => {
                eprintln!("  WARN: skipping fixture with invalid name: {}", frm_path.display());
                continue;
            }
        };

        let py_path = out_root.join(format!("{stem}.fpy"));

        // Compile Frame → Python. For core/control_flow/systems and
        // exec-smoke we use the exec wrapper; for persistence we rely on
        // the module's own `main`.
        let mut cmd = Command::new(framec_path);
        cmd.arg("compile")
            .arg("-l")
            .arg("python_3")
            .arg("--emit-debug")
            .arg(&frm_path)
            .current_dir(repo_root);
        if category == "v3_core"
            || category == "v3_control_flow"
            || category == "v3_systems"
            || category == "v3_exec_smoke"
        {
            cmd.env("FRAME_EMIT_EXEC", "1");
        }
        let compile_output = cmd.output();

        let program_src = match compile_output {
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

        let mut program_src = program_src;
        // For persistence fixtures, ensure `main()` is invoked when the script
        // is executed so that @run-expect patterns based on prints are
        // observable, even if the compiled module does not auto-call main.
        if category == "v3_persistence" && program_src.contains("def main(") {
            if !program_src.contains("if __name__ == '__main__'") {
                program_src.push_str("\nif __name__ == '__main__':\n    main()\n");
            }
        }

        if let Err(e) = fs::write(&py_path, program_src) {
            failed += 1;
            eprintln!(
                "  ERROR: failed to write Python script {}: {}",
                py_path.display(),
                e
            );
            continue;
        }

        // Run the test either in Docker or locally based on configuration
        let (run_success, out_text) = if config.use_docker {
            // Execute in Docker container
            let mut executor = match DockerTestExecutor::for_language("python") {
                Ok(ex) => ex,
                Err(e) => {
                    failed += 1;
                    eprintln!("  ERROR creating Docker executor: {}", e);
                    continue;
                }
            };
            
            // Mount the repo root and output directory
            executor.add_volume(repo_root, Path::new("/repo"));
            executor.add_volume(&out_root, Path::new("/output"));
            
            // Set PYTHONPATH in container
            executor.add_env("PYTHONPATH", "/repo");
            executor.workdir("/output");
            
            // Run the test file
            let container_py_path = format!("/output/{}.fpy", stem);
            match executor.run_test_file("python", &frm_path, Path::new(&container_py_path)) {
                Ok(result) => {
                    let mut text = String::new();
                    text.push_str(&result.stdout);
                    text.push_str(&result.stderr);
                    (result.success, text)
                }
                Err(e) => {
                    failed += 1;
                    eprintln!("  ERROR running Docker test: {}", e);
                    continue;
                }
            }
        } else {
            // Run locally with existing logic
            let mut cmd = Command::new("python3");
            cmd.arg(&py_path).current_dir(&out_root);
            if let Ok(existing) = std::env::var("PYTHONPATH") {
                let mut new_path = repo_root.to_path_buf();
                new_path.push(""); // ensure trailing separator
                let merged = format!(
                    "{}{}{}",
                    repo_root.display(),
                    std::path::MAIN_SEPARATOR,
                    existing
                );
                cmd.env("PYTHONPATH", merged);
            } else {
                cmd.env("PYTHONPATH", repo_root);
            }
            let run_output = match cmd.output() {
                Ok(run) => run,
                Err(e) => {
                    failed += 1;
                    eprintln!(
                        "  ERROR (spawn python3): {} ({})",
                        py_path.display(),
                        e
                    );
                    continue;
                }
            };

            let mut text = String::new();
            text.push_str(&String::from_utf8_lossy(&run_output.stdout));
            text.push_str(&String::from_utf8_lossy(&run_output.stderr));
            (run_output.status.success(), text)
        };

        let mut exec_ok = true;
        let mut reason = String::new();

        if !run_success {
            exec_ok = false;
            reason = "non-zero exit".to_string();
        } else {
            let lower = out_text.to_lowercase();
            if lower.contains("traceback") || lower.contains("error") {
                // Treat uncaught errors as failures for exec smoke.
                exec_ok = false;
                reason = "Python error/traceback detected".to_string();
            }
        }

        // Apply the same stem-based markers as the Python runner uses for
        // v3_exec_smoke (TRANSITION:, FORWARD:PARENT, STACK:PUSH/STACK:POP).
        if exec_ok {
            let name = stem.as_str();
            match name {
                "transition_basic" | "transition_args" => {
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
    }

    Ok(TestSummary {
        language: language.to_string(),
        category: category.to_string(),
        passed,
        failed,
        skipped,
    })
}

/// Execute TypeScript V3 exec-smoke fixtures by:
///   - compiling `.frm` to a standalone TS program via FRAME_EMIT_EXEC=1
///   - compiling TS to JS with `tsc`
///   - running `node` on the emitted JS
///   - applying the same stem-based marker checks as the Python runner
pub fn run_typescript_exec_smoke(
    repo_root: &Path,
    framec_path: &Path,
    category: &str,
) -> Result<TestSummary, String> {
    run_typescript_exec_smoke_with_filter(repo_root, framec_path, category, None)
}

/// Execute TypeScript V3 exec-smoke fixtures with optional metadata filtering
pub fn run_typescript_exec_smoke_with_filter(
    repo_root: &Path,
    framec_path: &Path,
    category: &str,
    metadata_filter: Option<&str>,
) -> Result<TestSummary, String> {
    // Use default test config for backwards compatibility
    let config = TestConfig::default();
    run_typescript_exec_smoke_with_config(
        repo_root,
        framec_path,
        category,
        metadata_filter,
        &config,
    )
}

/// Execute TypeScript V3 exec-smoke fixtures with full configuration
pub fn run_typescript_exec_smoke_with_config(
    repo_root: &Path,
    framec_path: &Path,
    category: &str,
    metadata_filter: Option<&str>,
    config: &TestConfig,
) -> Result<TestSummary, String> {
    let language = "typescript";
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
    let mut skipped = 0usize;

    let out_root = repo_root.join("target").join("v3_rs_exec_smoke").join("typescript");
    if let Err(e) = fs::create_dir_all(&out_root) {
        return Err(format!(
            "Failed to create TypeScript exec-smoke output dir {}: {e}",
            out_root.display()
        ));
    }

    // Resolve TypeScript compiler similarly to the Python runner.
    let tsc_cmd = {
        let project_root = repo_root;
        let local_bin = project_root
            .join("node_modules")
            .join(".bin")
            .join("tsc");
        let local_direct = project_root
            .join("node_modules")
            .join("typescript")
            .join("bin")
            .join("tsc");
        if local_bin.is_file() {
            Some(local_bin)
        } else if local_direct.is_file() {
            Some(local_direct)
        } else {
            // Fallback: rely on PATH and assume `tsc` is resolvable there.
            // We do not introduce an additional crate dependency here.
            Some(PathBuf::from("tsc"))
        }
    };

    let tsc = match tsc_cmd {
        Some(p) => p,
        None => {
            return Err("tsc not found - please install TypeScript (npm install)".to_string());
        }
    };

    for frm_path in files {
        // Apply metadata filtering if requested
        if let Some(filter) = metadata_filter {
            let metadata = parse_fixture_metadata(&frm_path);
            if !should_run_test(&metadata, filter) {
                skipped += 1;
                continue;
            }
        }
        
        let stem = match frm_path.file_stem().and_then(|s| s.to_str()) {
            Some(s) => s.to_string(),
            None => {
                eprintln!("  WARN: skipping fixture with invalid name: {}", frm_path.display());
                continue;
            }
        };

        let ts_path = out_root.join(format!("{stem}.frts"));
        let js_path = out_root.join(format!("{stem}.js"));

        // Compile Frame → TypeScript with FRAME_EMIT_EXEC=1.
        let mut cmd = Command::new(framec_path);
        cmd.arg("compile")
            .arg("-l")
            .arg("typescript")
            .arg("--emit-debug")
            .arg(&frm_path)
            .current_dir(repo_root);
        cmd.env("FRAME_EMIT_EXEC", "1");
        // Provide FRAME_TS_EXEC_IMPORT so the generated harness can import the runtime.
        let runtime_ts = repo_root.join("frame_runtime_ts").join("index");
        cmd.env(
            "FRAME_TS_EXEC_IMPORT",
            runtime_ts.to_str().unwrap_or("frame_runtime_ts/index"),
        );
        let compile_output = cmd.output();

        let program_src = match compile_output {
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

        let mut program_src = program_src;
        // For persistence fixtures, ensure `main()` is invoked when executing
        // the compiled TypeScript module.
        if category == "v3_persistence" && program_src.contains("function main(") {
            if !program_src.contains("main();") {
                program_src.push_str("\nmain();\n");
            }
        }

        if let Err(e) = fs::write(&ts_path, program_src) {
            failed += 1;
            eprintln!(
                "  ERROR: failed to write TS script {}: {}",
                ts_path.display(),
                e
            );
            continue;
        }

        // Compile TS → JS using tsc.
        let tsc_output = Command::new(&tsc)
            .arg("--target")
            .arg("es5")
            .arg("--module")
            .arg("commonjs")
            .arg("--skipLibCheck")
            .arg(&ts_path)
            .current_dir(&out_root)
            .output();

        let run_output = match tsc_output {
            Ok(out) => {
                if !out.status.success() {
                    failed += 1;
                    eprintln!("  FAIL (tsc): {}", ts_path.display());
                    let mut text = String::new();
                    text.push_str(&String::from_utf8_lossy(&out.stdout));
                    text.push_str(&String::from_utf8_lossy(&out.stderr));
                    for line in text.lines() {
                        eprintln!("    {}", line);
                    }
                    continue;
                }
                // Run JS via node.
                let mut node_cmd = Command::new("node");
                node_cmd.arg(&js_path).current_dir(&out_root);
                // Ensure Node can resolve project-local modules such as
                // `frame_runtime_ts` and `frame_persistence_ts` by prepending
                // the repo root to NODE_PATH.
                if let Ok(existing) = std::env::var("NODE_PATH") {
                    let merged = format!("{}{}{}", repo_root.display(), std::path::MAIN_SEPARATOR, existing);
                    node_cmd.env("NODE_PATH", merged);
                } else {
                    node_cmd.env("NODE_PATH", repo_root);
                }
                match node_cmd.output() {
                    Ok(run) => run,
                    Err(e) => {
                        failed += 1;
                        eprintln!(
                            "  ERROR (spawn node): {} ({})",
                            js_path.display(),
                            e
                        );
                        continue;
                    }
                }
            }
            Err(e) => {
                failed += 1;
                eprintln!(
                    "  ERROR (spawn tsc): {} ({})",
                    ts_path.display(),
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
        } else if out_text.contains("FAIL") {
            exec_ok = false;
            reason = "FAIL marker detected".to_string();
        }

        // Apply the same stem-based markers as Python exec-smoke (TRANSITION, FORWARD, STACK).
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
    }

    Ok(TestSummary {
        language: language.to_string(),
        category: category.to_string(),
        passed,
        failed,
        skipped,
    })
}

/// Execute Python curated exec fixtures (e.g., v3_core, v3_control_flow,
/// v3_systems, v3_persistence) using FRAME_EMIT_EXEC and `@run-expect` /
/// `@run-exact` metadata.
pub fn run_python_curated_exec_for_category(
    repo_root: &Path,
    framec_path: &Path,
    category: &str,
) -> Result<TestSummary, String> {
    let language = "python";
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
    let skipped = 0usize;  // TODO: Add metadata filtering support for curated tests

    let out_root = repo_root.join("target").join("v3_rs_exec_curated").join("python");
    if let Err(e) = fs::create_dir_all(&out_root) {
        return Err(format!(
            "Failed to create curated Python exec output dir {}: {e}",
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
        if run_expect.is_empty() && run_exact.is_none() {
            println!(
                "  SKIP (no @run-expect/@run-exact): {}",
                frm_path.display()
            );
            passed += 1;
            continue;
        }

        let py_path = out_root.join(format!("{stem}.fpy"));

        // Compile Frame → Python. For core/control_flow/systems we use the
        // exec wrapper; for persistence we rely on the module's own
        // `main()` function and append a call guard if needed.
        let mut cmd = Command::new(framec_path);
        cmd.arg("compile")
            .arg("-l")
            .arg("python_3")
            .arg("--emit-debug")
            .arg(&frm_path)
            .current_dir(repo_root);
        if category == "v3_core" || category == "v3_control_flow" || category == "v3_systems" {
            cmd.env("FRAME_EMIT_EXEC", "1");
        }
        let compile_output = cmd.output();

        let mut program_src = match compile_output {
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

        // For persistence fixtures, ensure `main()` is invoked when the script
        // runs, so @run-expect patterns based on prints are observable.
        if category == "v3_persistence" && program_src.contains("def main(") {
            if !program_src.contains("if __name__ == '__main__'") {
                program_src.push_str("\nif __name__ == '__main__':\n    main()\n");
            }
        }

        if let Err(e) = fs::write(&py_path, program_src) {
            failed += 1;
            eprintln!(
                "  ERROR: failed to write Python script {}: {}",
                py_path.display(),
                e
            );
            continue;
        }

        // Run python3 on the emitted script with PYTHONPATH including the repo root.
        let mut run_cmd = Command::new("python3");
        run_cmd.arg(&py_path).current_dir(&out_root);
        if let Ok(existing) = std::env::var("PYTHONPATH") {
            let merged = format!("{}{}{}", repo_root.display(), std::path::MAIN_SEPARATOR, existing);
            run_cmd.env("PYTHONPATH", merged);
        } else {
            run_cmd.env("PYTHONPATH", repo_root);
        }
        let run_output = match run_cmd.output() {
            Ok(run) => run,
            Err(e) => {
                failed += 1;
                eprintln!(
                    "  ERROR (spawn python3): {} ({})",
                    py_path.display(),
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
            if lower.contains("traceback") || lower.contains("error") {
                exec_ok = false;
                reason = "Python error/traceback detected".to_string();
            }
        }

        // Apply @run-expect patterns as regexes (fallback to literal).
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
    }

    Ok(TestSummary {
        language: language.to_string(),
        category: category.to_string(),
        passed,
        failed,
        skipped,
    })
}

/// Execute TypeScript curated exec fixtures using FRAME_EMIT_EXEC and
/// `@run-expect` / `@run-exact` metadata.
pub fn run_typescript_curated_exec_for_category(
    repo_root: &Path,
    framec_path: &Path,
    category: &str,
) -> Result<TestSummary, String> {
    let language = "typescript";
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
    let skipped = 0usize;  // TODO: Add metadata filtering support for curated tests

    let out_root = repo_root.join("target").join("v3_rs_exec_curated").join("typescript");
    if let Err(e) = fs::create_dir_all(&out_root) {
        return Err(format!(
            "Failed to create curated TypeScript exec output dir {}: {e}",
            out_root.display()
        ));
    }

    // Resolve TypeScript compiler as for exec-smoke.
    let tsc_cmd = {
        let project_root = repo_root;
        let local_bin = project_root
            .join("node_modules")
            .join(".bin")
            .join("tsc");
        let local_direct = project_root
            .join("node_modules")
            .join("typescript")
            .join("bin")
            .join("tsc");
        if local_bin.is_file() {
            Some(local_bin)
        } else if local_direct.is_file() {
            Some(local_direct)
        } else {
            Some(PathBuf::from("tsc"))
        }
    };

    let tsc = match tsc_cmd {
        Some(p) => p,
        None => {
            return Err("tsc not found - please install TypeScript (npm install)".to_string());
        }
    };

    for frm_path in files {
        let stem = match frm_path.file_stem().and_then(|s| s.to_str()) {
            Some(s) => s.to_string(),
            None => {
                eprintln!("  WARN: skipping fixture with invalid name: {}", frm_path.display());
                continue;
            }
        };

        let (run_expect, run_exact) = parse_run_meta(&frm_path);
        if run_expect.is_empty() && run_exact.is_none() {
            println!(
                "  SKIP (no @run-expect/@run-exact): {}",
                frm_path.display()
            );
            passed += 1;
            continue;
        }

        let ts_path = out_root.join(format!("{stem}.frts"));
        let js_path = out_root.join(format!("{stem}.js"));

        // Compile Frame → TypeScript. For core/control_flow/systems we use
        // exec wrappers; for persistence we rely on module `main` and append
        // a call to `main()` if needed.
        let mut cmd = Command::new(framec_path);
        cmd.arg("compile")
            .arg("-l")
            .arg("typescript")
            .arg("--emit-debug")
            .arg(&frm_path)
            .current_dir(repo_root);
        if category == "v3_core" || category == "v3_control_flow" || category == "v3_systems" {
            cmd.env("FRAME_EMIT_EXEC", "1");
            let runtime_ts = repo_root.join("frame_runtime_ts").join("index");
            cmd.env(
                "FRAME_TS_EXEC_IMPORT",
                runtime_ts.to_str().unwrap_or("frame_runtime_ts/index"),
            );
        }
        let compile_output = cmd.output();

        let mut program_src = match compile_output {
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

        if category == "v3_persistence" && program_src.contains("function main(") {
            if !program_src.contains("main();") {
                program_src.push_str("\nmain();\n");
            }
        }

        if let Err(e) = fs::write(&ts_path, program_src) {
            failed += 1;
            eprintln!(
                "  ERROR: failed to write TS script {}: {}",
                ts_path.display(),
                e
            );
            continue;
        }

        // Compile TS → JS using tsc.
        let tsc_output = Command::new(&tsc)
            .arg("--target")
            .arg("es5")
            .arg("--module")
            .arg("commonjs")
            .arg("--skipLibCheck")
            .arg(&ts_path)
            .current_dir(&out_root)
            .output();

        let run_output = match tsc_output {
            Ok(out) => {
                if !out.status.success() {
                    failed += 1;
                    eprintln!("  FAIL (tsc): {}", ts_path.display());
                    let mut text = String::new();
                    text.push_str(&String::from_utf8_lossy(&out.stdout));
                    text.push_str(&String::from_utf8_lossy(&out.stderr));
                    for line in text.lines() {
                        eprintln!("    {}", line);
                    }
                    continue;
                }
                // Run JS via node. Ensure Node can resolve project-local
                // modules such as `frame_runtime_ts` and
                // `frame_persistence_ts` by prepending the repo root to
                // NODE_PATH, mirroring the exec-smoke harness.
                let mut node_cmd = Command::new("node");
                node_cmd.arg(&js_path).current_dir(&out_root);
                if let Ok(existing) = std::env::var("NODE_PATH") {
                    let merged = format!(
                        "{}{}{}",
                        repo_root.display(),
                        std::path::MAIN_SEPARATOR,
                        existing
                    );
                    node_cmd.env("NODE_PATH", merged);
                } else {
                    node_cmd.env("NODE_PATH", repo_root);
                }
                match node_cmd.output() {
                    Ok(run) => run,
                    Err(e) => {
                        failed += 1;
                        eprintln!(
                            "  ERROR (spawn node): {} ({})",
                            js_path.display(),
                            e
                        );
                        continue;
                    }
                }
            }
            Err(e) => {
                failed += 1;
                eprintln!(
                    "  ERROR (spawn tsc): {} ({})",
                    ts_path.display(),
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
        } else if out_text.contains("FAIL") {
            exec_ok = false;
            reason = "FAIL marker detected".to_string();
        }

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
    }

    Ok(TestSummary {
        language: language.to_string(),
        category: category.to_string(),
        passed,
        failed,
        skipped,
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
    let skipped = 0usize;  // TODO: Add metadata filtering support for curated tests

    let out_root = repo_root.join("target").join("v3_rs_exec_curated").join("rust");
    ensure_rust_exec_crate(&out_root, repo_root, "v3_rs_exec_curated")
        .map_err(|e| format!("Failed to prepare Rust curated exec crate: {e}"))?;

    for frm_path in files {
        let _stem = match frm_path.file_stem().and_then(|s| s.to_str()) {
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

        let run_output = match cargo_run_rust_program(&out_root, &compile_src) {
            Ok(run) => run,
            Err(e) => {
                failed += 1;
                eprintln!("  {}", e);
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
    }

    Ok(TestSummary {
        language: language.to_string(),
        category: category.to_string(),
        passed,
        failed,
        skipped,
    })
}
