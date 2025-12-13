// Stub module for test harness - all testing moved to shared environment

use std::path::Path;

#[derive(Debug, Clone)]
pub struct TestConfig {
    pub parallel_workers: usize,
    pub test_timeout: u64,
    pub verbose: bool,
}

#[derive(Debug)]
pub struct TestSummary {
    pub language: String,
    pub category: String,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
}

pub fn run_validation_for_category_with_config(
    _repo_root: &Path,
    _framec_path: &Path,
    _language: &str,
    _category: &str,
    _metadata_filter: Option<&str>,
    _config: &TestConfig,
) -> Result<TestSummary, Box<dyn std::error::Error>> {
    eprintln!("Test infrastructure has been moved to shared environment.");
    eprintln!("Use Docker runner from framepiler_test_env/framepiler/docker/ instead.");
    std::process::exit(exitcode::UNAVAILABLE);
}

pub fn run_python_exec_smoke_with_config(
    _repo_root: &Path,
    _framec_path: &Path,
    _category: &str,
    _metadata_filter: Option<&str>,
    _config: &TestConfig,
) -> Result<TestSummary, Box<dyn std::error::Error>> {
    eprintln!("Test infrastructure has been moved to shared environment.");
    eprintln!("Use Docker runner from framepiler_test_env/framepiler/docker/ instead.");
    std::process::exit(exitcode::UNAVAILABLE);
}

pub fn run_rust_exec_smoke_with_config(
    _repo_root: &Path,
    _framec_path: &Path,
    _category: &str,
    _metadata_filter: Option<&str>,
    _config: &TestConfig,
) -> Result<TestSummary, Box<dyn std::error::Error>> {
    eprintln!("Test infrastructure has been moved to shared environment.");
    eprintln!("Use Docker runner from framepiler_test_env/framepiler/docker/ instead.");
    std::process::exit(exitcode::UNAVAILABLE);
}

pub fn run_typescript_exec_smoke_with_config(
    _repo_root: &Path,
    _framec_path: &Path,
    _category: &str,
    _metadata_filter: Option<&str>,
    _config: &TestConfig,
) -> Result<TestSummary, Box<dyn std::error::Error>> {
    eprintln!("Test infrastructure has been moved to shared environment.");
    eprintln!("Use Docker runner from framepiler_test_env/framepiler/docker/ instead.");
    std::process::exit(exitcode::UNAVAILABLE);
}

pub fn run_python_curated_exec_for_category(
    _repo_root: &Path,
    _framec_path: &Path,
    _category: &str,
) -> Result<TestSummary, Box<dyn std::error::Error>> {
    eprintln!("Test infrastructure has been moved to shared environment.");
    eprintln!("Use Docker runner from framepiler_test_env/framepiler/docker/ instead.");
    std::process::exit(exitcode::UNAVAILABLE);
}

pub fn run_rust_curated_exec_for_category(
    _repo_root: &Path,
    _framec_path: &Path,
    _category: &str,
) -> Result<TestSummary, Box<dyn std::error::Error>> {
    eprintln!("Test infrastructure has been moved to shared environment.");
    eprintln!("Use Docker runner from framepiler_test_env/framepiler/docker/ instead.");
    std::process::exit(exitcode::UNAVAILABLE);
}

pub fn run_typescript_curated_exec_for_category(
    _repo_root: &Path,
    _framec_path: &Path,
    _category: &str,
) -> Result<TestSummary, Box<dyn std::error::Error>> {
    eprintln!("Test infrastructure has been moved to shared environment.");
    eprintln!("Use Docker runner from framepiler_test_env/framepiler/docker/ instead.");
    std::process::exit(exitcode::UNAVAILABLE);
}