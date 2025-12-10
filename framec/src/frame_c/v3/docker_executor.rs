use std::path::{Path, PathBuf};
use std::process::Command;
use std::collections::HashMap;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Docker-based test executor for containerized testing
pub struct DockerTestExecutor {
    /// Docker image to use
    image: String,
    /// Volumes to mount (host_path -> container_path)
    volumes: Vec<(PathBuf, PathBuf)>,
    /// Environment variables
    env_vars: HashMap<String, String>,
    /// Working directory in container
    workdir: String,
    /// Container name prefix
    container_prefix: String,
    /// Cleanup containers after execution
    auto_cleanup: bool,
}

impl DockerTestExecutor {
    /// Create a new Docker test executor
    pub fn new(image: &str) -> Self {
        Self {
            image: image.to_string(),
            volumes: Vec::new(),
            env_vars: HashMap::new(),
            workdir: "/work".to_string(),
            container_prefix: "frame-transpiler-test".to_string(),  // Use transpiler namespace
            auto_cleanup: true,
        }
    }
    
    /// Create executor for a specific language
    pub fn for_language(language: &str) -> Result<Self, String> {
        let image = match language {
            "python" | "python_3" => "frame-transpiler-python:latest",
            "typescript" => "frame-transpiler-typescript:latest",
            "rust" => "frame-transpiler-rust:latest",
            _ => return Err(format!("Unsupported language: {}", language)),
        };
        Ok(Self::new(image))
    }
    
    /// Add a volume mount
    pub fn add_volume(&mut self, host_path: &Path, container_path: &Path) -> &mut Self {
        self.volumes.push((host_path.to_path_buf(), container_path.to_path_buf()));
        self
    }
    
    /// Add an environment variable
    pub fn add_env(&mut self, key: &str, value: &str) -> &mut Self {
        self.env_vars.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Set working directory
    pub fn workdir(&mut self, dir: &str) -> &mut Self {
        self.workdir = dir.to_string();
        self
    }
    
    /// Set execution timeout
    pub fn set_timeout(&mut self, timeout: Duration) {
        // Store timeout for later use
        self.env_vars.insert("TEST_TIMEOUT".to_string(), timeout.as_secs().to_string());
    }
    
    /// Execute a command in a Docker container
    pub fn execute(&self, command: &[&str]) -> Result<DockerTestResult, String> {
        let container_name = format!("{}-{}", self.container_prefix, Uuid::new_v4());
        
        let mut docker_cmd = Command::new("docker");
        docker_cmd.arg("run");
        
        // Add container name
        docker_cmd.arg("--name").arg(&container_name);
        
        // Add volumes
        for (host, container) in &self.volumes {
            let volume_spec = format!(
                "{}:{}:ro",
                host.display(),
                container.display()
            );
            docker_cmd.arg("-v").arg(volume_spec);
        }
        
        // Add environment variables
        for (key, value) in &self.env_vars {
            docker_cmd.arg("-e").arg(format!("{}={}", key, value));
        }
        
        // Set working directory
        docker_cmd.arg("-w").arg(&self.workdir);
        
        // Remove container after execution if auto_cleanup
        if self.auto_cleanup {
            docker_cmd.arg("--rm");
        }
        
        // Add image
        docker_cmd.arg(&self.image);
        
        // Add command
        for arg in command {
            docker_cmd.arg(arg);
        }
        
        // Execute
        let output = docker_cmd.output()
            .map_err(|e| format!("Failed to execute Docker command: {}", e))?;
        
        Ok(DockerTestResult {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
            container_name,
        })
    }
    
    /// Run a single test file in Docker
    pub fn run_test_file(
        &self,
        language: &str,
        _test_file: &Path,
        generated_file: &Path,
    ) -> Result<DockerTestResult, String> {
        // Build the test command based on the language
        let generated_path = generated_file.to_str()
            .ok_or_else(|| "Invalid generated file path".to_string())?;
        
        // Store format strings to avoid temporary lifetime issues
        let rust_cmd = format!("rustc {} -o /tmp/test && /tmp/test", generated_file.display());
        
        let command: Vec<&str> = match language {
            "python" | "python_3" => {
                vec![
                    "python3",
                    generated_path,
                ]
            }
            "typescript" => {
                vec![
                    "node",
                    generated_path,
                ]
            }
            "rust" => {
                // For Rust, we need to compile and run
                vec![
                    "sh",
                    "-c",
                    &rust_cmd,
                ]
            }
            _ => {
                return Err(format!("Unsupported language: {}", language));
            }
        };
        
        self.execute(&command)
    }
    
    /// Run tests for a specific language and category with parallel workers
    pub fn run_tests(
        &self,
        language: &str,
        category: &str,
        parallel: usize,
    ) -> Result<Vec<DockerTestResult>, String> {
        // Build the test command based on the language
        // Store format strings to avoid temporary lifetime issues
        let fixtures_path = format!("/fixtures/{}", category);
        let cargo_manifest = format!("/fixtures/{}/Cargo.toml", category);
        let threads_str = parallel.to_string();
        
        let command: Vec<&str> = match language {
            "python" | "python_3" => {
                vec![
                    "python3",
                    "-m",
                    "pytest",
                    "-v",
                    "--json-report",
                    "--json-report-file=/tmp/test-results.json",
                    &fixtures_path,
                ]
            }
            "typescript" => {
                vec![
                    "npm",
                    "test",
                    "--",
                    &fixtures_path,
                ]
            }
            "rust" => {
                vec![
                    "cargo",
                    "test",
                    "--manifest-path",
                    &cargo_manifest,
                    "--",
                    "--test-threads",
                    &threads_str,
                ]
            }
            _ => {
                return Err(format!("Unsupported language: {}", language));
            }
        };
        
        let result = self.execute(&command)?;
        
        // TODO: Parse test results from JSON report
        // For now, return a single result
        Ok(vec![result])
    }
}

/// Result from Docker execution
#[derive(Debug, Clone)]
pub struct DockerTestResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub container_name: String,
}

/// Test execution summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestExecutionSummary {
    pub language: String,
    pub category: String,
    pub total_tests: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub duration_ms: u64,
    pub test_results: Vec<TestCaseResult>,
}

/// Individual test case result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCaseResult {
    pub name: String,
    pub file: String,
    pub status: TestStatus,
    pub duration_ms: u64,
    pub error: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TestStatus {
    Passed,
    Failed,
    Skipped,
    Error,
}

/// Parallel Docker executor for running multiple containers
pub struct ParallelDockerExecutor {
    executors: Vec<DockerTestExecutor>,
    max_parallel: usize,
}

impl ParallelDockerExecutor {
    pub fn new(max_parallel: usize) -> Self {
        Self {
            executors: Vec::new(),
            max_parallel,
        }
    }
    
    pub fn add_executor(&mut self, executor: DockerTestExecutor) {
        self.executors.push(executor);
    }
    
    /// Run all executors in parallel with worker limit
    /// NOTE: Currently disabled - will be implemented after base functionality works
    pub fn run_all(&self) -> Vec<Result<DockerTestResult, String>> {
        // Placeholder implementation
        Vec::new()
    }
}

// Note: Add uuid to Cargo.toml dependencies for container naming
// uuid = { version = "1", features = ["v4"] }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_docker_executor_creation() {
        let mut executor = DockerTestExecutor::new("frame-prt-test:latest");
        executor
            .add_volume(Path::new("/host/framec"), Path::new("/framec"))
            .add_env("TEST_LANGUAGE", "python")
            .workdir("/fixtures");
            
        assert_eq!(executor.image, "frame-prt-test:latest");
        assert_eq!(executor.volumes.len(), 1);
        assert_eq!(executor.env_vars.get("TEST_LANGUAGE"), Some(&"python".to_string()));
    }
}