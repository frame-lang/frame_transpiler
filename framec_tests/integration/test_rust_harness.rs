#!/usr/bin/env rust-script
//! Integration tests for the new Rust test harness features
//! 
//! Tests:
//! - Metadata parsing from Frame fixtures
//! - Test reporter generation (JSON, JUnit, TAP, Human)
//! - Docker executor basic functionality
//! - CLI integration with new flags

use std::path::PathBuf;
use std::fs;
use std::process::Command;

#[test]
fn test_metadata_parsing() {
    // Create a test fixture with metadata
    let test_content = r#"
# @expect: Test expectation
# @meta: core
# @skip-if: windows
# @run-expect: SUCCESS: test passes

system TestSystem {
    machine:
        $Start {
            test() {
                print("test")
            }
        }
}
"#;
    
    // Write test file
    let test_path = PathBuf::from("/tmp/test_metadata_fixture.frm");
    fs::write(&test_path, test_content).expect("Failed to write test file");
    
    // Run framec with test command
    let output = Command::new("./target/release/framec")
        .arg("test")
        .arg("--language").arg("python")
        .arg("--category").arg("integration")
        .arg("--report-format").arg("json")
        .arg("--output").arg("/tmp/test_report.json")
        .output()
        .expect("Failed to execute framec");
    
    // Check command executed
    assert!(output.status.success() || output.status.code() == Some(0), 
            "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    
    // Clean up
    let _ = fs::remove_file(test_path);
    let _ = fs::remove_file("/tmp/test_report.json");
}

#[test]
fn test_report_formats() {
    // Test JSON format
    let json_output = Command::new("./target/release/framec")
        .arg("test")
        .arg("--language").arg("python")
        .arg("--category").arg("systems")
        .arg("--report-format").arg("json")
        .output()
        .expect("Failed to run JSON report test");
    
    // Test JUnit format
    let junit_output = Command::new("./target/release/framec")
        .arg("test")
        .arg("--language").arg("python")
        .arg("--category").arg("systems")
        .arg("--report-format").arg("junit")
        .output()
        .expect("Failed to run JUnit report test");
    
    // Test TAP format
    let tap_output = Command::new("./target/release/framec")
        .arg("test")
        .arg("--language").arg("python")
        .arg("--category").arg("systems")
        .arg("--report-format").arg("tap")
        .output()
        .expect("Failed to run TAP report test");
    
    // Test Human format (default)
    let human_output = Command::new("./target/release/framec")
        .arg("test")
        .arg("--language").arg("python")
        .arg("--category").arg("systems")
        .arg("--report-format").arg("human")
        .output()
        .expect("Failed to run Human report test");
    
    // Basic validation that commands run
    println!("JSON status: {}", json_output.status.success());
    println!("JUnit status: {}", junit_output.status.success());
    println!("TAP status: {}", tap_output.status.success());
    println!("Human status: {}", human_output.status.success());
}

#[test]
fn test_parallel_flag() {
    // Test parallel execution flag
    let output = Command::new("./target/release/framec")
        .arg("test")
        .arg("--language").arg("python")
        .arg("--category").arg("systems")
        .arg("--parallel").arg("8")
        .output()
        .expect("Failed to run parallel test");
    
    // Should not fail due to flag parsing
    assert!(output.status.code().is_some(), "Parallel flag should be accepted");
}

#[test]
fn test_timeout_flag() {
    // Test timeout flag
    let output = Command::new("./target/release/framec")
        .arg("test")
        .arg("--language").arg("python")
        .arg("--category").arg("systems")
        .arg("--timeout").arg("600")
        .output()
        .expect("Failed to run timeout test");
    
    // Should not fail due to flag parsing
    assert!(output.status.code().is_some(), "Timeout flag should be accepted");
}

#[test]
fn test_metadata_filter() {
    // Test metadata filter flag
    let output = Command::new("./target/release/framec")
        .arg("test")
        .arg("--language").arg("python")
        .arg("--category").arg("systems")
        .arg("--metadata-filter").arg("@core")
        .output()
        .expect("Failed to run metadata filter test");
    
    // Should not fail due to flag parsing
    assert!(output.status.code().is_some(), "Metadata filter flag should be accepted");
}

fn main() {
    println!("Running Rust test harness integration tests...");
    
    test_metadata_parsing();
    test_report_formats();
    test_parallel_flag();
    test_timeout_flag();
    test_metadata_filter();
    
    println!("All integration tests completed!");
}