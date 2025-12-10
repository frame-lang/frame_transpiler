use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};
use serde_json;

/// Test report formats supported
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReportFormat {
    Json,
    JUnit,
    Tap,
    Human,
}

impl ReportFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "json" => Some(Self::Json),
            "junit" => Some(Self::JUnit),
            "tap" => Some(Self::Tap),
            "human" => Some(Self::Human),
            _ => None,
        }
    }
}

/// Comprehensive test report matching Python runner output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestReport {
    pub version: String,
    pub timestamp: String,
    pub duration_ms: u64,
    pub summary: TestSummary,
    pub languages: Vec<LanguageReport>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSummary {
    pub total_tests: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub error_rate: f64,
    pub success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageReport {
    pub language: String,
    pub categories: Vec<CategoryReport>,
    pub total_tests: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryReport {
    pub name: String,
    pub tests: Vec<TestResult>,
    pub duration_ms: u64,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub name: String,
    pub file: String,
    pub status: TestStatus,
    pub duration_ms: u64,
    pub error_message: Option<String>,
    pub output: Option<String>,
    pub metadata: TestMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TestStatus {
    Passed,
    Failed,
    Skipped,
    Error,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestMetadata {
    pub is_negative: bool,
    pub is_core: bool,
    pub is_flaky: bool,
    pub expected_errors: Vec<String>,
    pub run_expectations: Vec<String>,
    pub skip_reason: Option<String>,
}

/// Test reporter for generating various output formats
pub struct TestReporter {
    report: TestReport,
    start_time: SystemTime,
}

impl TestReporter {
    pub fn new() -> Self {
        Self {
            report: TestReport {
                version: env!("CARGO_PKG_VERSION").to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                duration_ms: 0,
                summary: TestSummary {
                    total_tests: 0,
                    passed: 0,
                    failed: 0,
                    skipped: 0,
                    error_rate: 0.0,
                    success_rate: 0.0,
                },
                languages: Vec::new(),
                metadata: HashMap::new(),
            },
            start_time: SystemTime::now(),
        }
    }
    
    pub fn add_language(&mut self, language: LanguageReport) {
        self.report.languages.push(language);
        self.update_summary();
    }
    
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.report.metadata.insert(key, value);
    }
    
    fn update_summary(&mut self) {
        let mut total = 0;
        let mut passed = 0;
        let mut failed = 0;
        let mut skipped = 0;
        
        for lang in &self.report.languages {
            total += lang.total_tests;
            passed += lang.passed;
            failed += lang.failed;
            skipped += lang.skipped;
        }
        
        self.report.summary = TestSummary {
            total_tests: total,
            passed,
            failed,
            skipped,
            error_rate: if total > 0 { failed as f64 / total as f64 } else { 0.0 },
            success_rate: if total > 0 { passed as f64 / total as f64 } else { 0.0 },
        };
        
        self.report.duration_ms = self.start_time.elapsed()
            .unwrap_or(Duration::from_secs(0))
            .as_millis() as u64;
    }
    
    pub fn write_report(&self, path: &Path, format: ReportFormat) -> Result<(), String> {
        let content = match format {
            ReportFormat::Json => self.to_json()?,
            ReportFormat::JUnit => self.to_junit()?,
            ReportFormat::Tap => self.to_tap()?,
            ReportFormat::Human => self.to_human()?,
        };
        
        let mut file = File::create(path)
            .map_err(|e| format!("Failed to create report file: {}", e))?;
        file.write_all(content.as_bytes())
            .map_err(|e| format!("Failed to write report: {}", e))?;
        
        Ok(())
    }
    
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(&self.report)
            .map_err(|e| format!("Failed to serialize JSON: {}", e))
    }
    
    pub fn to_junit(&self) -> Result<String, String> {
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str(&format!(
            "<testsuites tests=\"{}\" failures=\"{}\" errors=\"0\" time=\"{}\">\n",
            self.report.summary.total_tests,
            self.report.summary.failed,
            self.report.duration_ms as f64 / 1000.0
        ));
        
        for lang in &self.report.languages {
            for cat in &lang.categories {
                xml.push_str(&format!(
                    "  <testsuite name=\"{}.{}\" tests=\"{}\" failures=\"{}\" time=\"{}\">\n",
                    lang.language,
                    cat.name,
                    cat.tests.len(),
                    cat.failed,
                    cat.duration_ms as f64 / 1000.0
                ));
                
                for test in &cat.tests {
                    let classname = format!("{}.{}", lang.language, cat.name);
                    xml.push_str(&format!(
                        "    <testcase classname=\"{}\" name=\"{}\" time=\"{}\"",
                        classname,
                        test.name,
                        test.duration_ms as f64 / 1000.0
                    ));
                    
                    match test.status {
                        TestStatus::Passed => xml.push_str("/>\n"),
                        TestStatus::Failed => {
                            xml.push_str(">\n");
                            if let Some(ref msg) = test.error_message {
                                xml.push_str(&format!(
                                    "      <failure message=\"{}\" />\n",
                                    escape_xml(msg)
                                ));
                            }
                            xml.push_str("    </testcase>\n");
                        },
                        TestStatus::Skipped => {
                            xml.push_str(">\n");
                            xml.push_str(&format!(
                                "      <skipped message=\"{}\" />\n",
                                test.metadata.skip_reason.as_ref().unwrap_or(&"".to_string())
                            ));
                            xml.push_str("    </testcase>\n");
                        },
                        TestStatus::Error => {
                            xml.push_str(">\n");
                            if let Some(ref msg) = test.error_message {
                                xml.push_str(&format!(
                                    "      <error message=\"{}\" />\n",
                                    escape_xml(msg)
                                ));
                            }
                            xml.push_str("    </testcase>\n");
                        },
                    }
                }
                
                xml.push_str("  </testsuite>\n");
            }
        }
        
        xml.push_str("</testsuites>\n");
        Ok(xml)
    }
    
    pub fn to_tap(&self) -> Result<String, String> {
        let mut tap = format!("TAP version 13\n1..{}\n", self.report.summary.total_tests);
        let mut test_num = 1;
        
        for lang in &self.report.languages {
            for cat in &lang.categories {
                for test in &cat.tests {
                    let status = match test.status {
                        TestStatus::Passed => "ok",
                        TestStatus::Failed => "not ok",
                        TestStatus::Skipped => "ok",
                        TestStatus::Error => "not ok",
                    };
                    
                    let skip = if matches!(test.status, TestStatus::Skipped) {
                        format!(" # SKIP {}", test.metadata.skip_reason.as_ref().unwrap_or(&"".to_string()))
                    } else {
                        String::new()
                    };
                    
                    tap.push_str(&format!(
                        "{} {} - {}.{}.{}{}\n",
                        status,
                        test_num,
                        lang.language,
                        cat.name,
                        test.name,
                        skip
                    ));
                    
                    if let Some(ref msg) = test.error_message {
                        for line in msg.lines() {
                            tap.push_str(&format!("# {}\n", line));
                        }
                    }
                    
                    test_num += 1;
                }
            }
        }
        
        Ok(tap)
    }
    
    pub fn to_human(&self) -> Result<String, String> {
        let mut output = String::new();
        
        // Header
        output.push_str(&format!(
            "\n╔══════════════════════════════════════════════════════════════╗\n"
        ));
        output.push_str(&format!(
            "║                    Frame Test Results                        ║\n"
        ));
        output.push_str(&format!(
            "╚══════════════════════════════════════════════════════════════╝\n\n"
        ));
        
        // Summary
        output.push_str(&format!(
            "Summary:\n"
        ));
        output.push_str(&format!(
            "  Total:   {}\n",
            self.report.summary.total_tests
        ));
        output.push_str(&format!(
            "  Passed:  {} ({:.1}%)\n",
            self.report.summary.passed,
            self.report.summary.success_rate * 100.0
        ));
        output.push_str(&format!(
            "  Failed:  {} ({:.1}%)\n",
            self.report.summary.failed,
            self.report.summary.error_rate * 100.0
        ));
        output.push_str(&format!(
            "  Skipped: {}\n",
            self.report.summary.skipped
        ));
        output.push_str(&format!(
            "  Duration: {:.2}s\n\n",
            self.report.duration_ms as f64 / 1000.0
        ));
        
        // Per-language results
        for lang in &self.report.languages {
            output.push_str(&format!(
                "Language: {}\n",
                lang.language.to_uppercase()
            ));
            output.push_str(&format!(
                "  Tests: {} | Passed: {} | Failed: {} | Skipped: {}\n\n",
                lang.total_tests,
                lang.passed,
                lang.failed,
                lang.skipped
            ));
            
            // Show failures
            for cat in &lang.categories {
                if cat.failed > 0 {
                    output.push_str(&format!("  Category: {}\n", cat.name));
                    for test in &cat.tests {
                        if matches!(test.status, TestStatus::Failed | TestStatus::Error) {
                            output.push_str(&format!(
                                "    ✗ {}: {}\n",
                                test.name,
                                test.error_message.as_ref().unwrap_or(&"Unknown error".to_string())
                            ));
                        }
                    }
                }
            }
        }
        
        Ok(output)
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

// Note: Add chrono to Cargo.toml for timestamp formatting
// chrono = "0.4"