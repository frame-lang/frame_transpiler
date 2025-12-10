// Unit tests for test metadata parsing and filtering

#[cfg(test)]
mod tests {
    use super::super::test_harness_rs::{parse_fixture_metadata, TestMetadata};
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    // Helper to create a test fixture file with given content
    fn create_test_fixture(content: &str) -> (TempDir, std::path::PathBuf) {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("test.frm");
        fs::write(&path, content).unwrap();
        (dir, path)
    }

    #[test]
    fn test_parse_expect_codes() {
        let content = r#"
# Test fixture
# @expect: E301 E302
# @expect: E400

system Test {
    machine:
        $Start { }
}
"#;
        let (_dir, path) = create_test_fixture(content);
        let metadata = parse_fixture_metadata(&path);
        
        assert_eq!(metadata.expect.len(), 3);
        assert!(metadata.expect.contains(&"E301".to_string()));
        assert!(metadata.expect.contains(&"E302".to_string()));
        assert!(metadata.expect.contains(&"E400".to_string()));
    }

    #[test]
    fn test_parse_meta_tags() {
        let content = r#"
# @meta: important test
# @meta: regression
// @meta: v3 feature

system Test { }
"#;
        let (_dir, path) = create_test_fixture(content);
        let metadata = parse_fixture_metadata(&path);
        
        assert_eq!(metadata.meta.len(), 3);
        assert!(metadata.meta.contains(&"important test".to_string()));
        assert!(metadata.meta.contains(&"regression".to_string()));
        assert!(metadata.meta.contains(&"v3 feature".to_string()));
    }

    #[test]
    fn test_parse_boolean_flags() {
        let content = r#"
# @exec-ok
# @flaky
# @core
# @noncore
# @py-compile
# @tsc-compile
# @rs-compile

system Test { }
"#;
        let (_dir, path) = create_test_fixture(content);
        let metadata = parse_fixture_metadata(&path);
        
        assert!(metadata.exec_ok);
        assert!(metadata.flaky);
        assert!(metadata.core);
        assert!(metadata.noncore);
        assert!(metadata.py_compile);
        assert!(metadata.tsc_compile);
        assert!(metadata.rs_compile);
    }

    #[test]
    fn test_parse_skip_conditions() {
        let content = r#"
# @skip-if: windows
# @skip-if: no-docker
// @skip-if: slow-test

system Test { }
"#;
        let (_dir, path) = create_test_fixture(content);
        let metadata = parse_fixture_metadata(&path);
        
        assert_eq!(metadata.skip_if.len(), 3);
        assert!(metadata.skip_if.contains(&"windows".to_string()));
        assert!(metadata.skip_if.contains(&"no-docker".to_string()));
        assert!(metadata.skip_if.contains(&"slow-test".to_string()));
    }

    #[test]
    fn test_parse_run_expectations() {
        let content = r#"
# @run-expect: SUCCESS
# @run-expect: State changed
# @run-exact: 42
// @compile-expect: warning

system Test { }
"#;
        let (_dir, path) = create_test_fixture(content);
        let metadata = parse_fixture_metadata(&path);
        
        assert_eq!(metadata.run_expect.len(), 2);
        assert!(metadata.run_expect.contains(&"SUCCESS".to_string()));
        assert!(metadata.run_expect.contains(&"State changed".to_string()));
        
        assert_eq!(metadata.run_exact.len(), 1);
        assert!(metadata.run_exact.contains(&"42".to_string()));
        
        assert_eq!(metadata.compile_expect.len(), 1);
        assert!(metadata.compile_expect.contains(&"warning".to_string()));
    }

    #[test]
    fn test_parse_working_directory() {
        let content = r#"
# @cwd: /tmp/test

system Test { }
"#;
        let (_dir, path) = create_test_fixture(content);
        let metadata = parse_fixture_metadata(&path);
        
        assert_eq!(metadata.cwd, Some("/tmp/test".to_string()));
    }

    #[test]
    fn test_parse_golden_files() {
        let content = r#"
# @frame-map-golden: test.frame.map
# @visitor-map-golden: test.visitor.map

system Test { }
"#;
        let (_dir, path) = create_test_fixture(content);
        let metadata = parse_fixture_metadata(&path);
        
        assert_eq!(metadata.frame_map_golden, Some("test.frame.map".to_string()));
        assert_eq!(metadata.visitor_map_golden, Some("test.visitor.map".to_string()));
    }

    #[test]
    fn test_parse_expect_mode() {
        let content = r#"
# @expect-mode: superset
# @expect: E100

system Test { }
"#;
        let (_dir, path) = create_test_fixture(content);
        let metadata = parse_fixture_metadata(&path);
        
        assert_eq!(metadata.expect_mode, Some("superset".to_string()));
    }

    #[test]
    fn test_parse_import_calls() {
        let content = r#"
# @import-call: math.sqrt
# @import-call: os.path.join

system Test { }
"#;
        let (_dir, path) = create_test_fixture(content);
        let metadata = parse_fixture_metadata(&path);
        
        assert_eq!(metadata.import_calls.len(), 2);
        assert!(metadata.import_calls.contains(&"math.sqrt".to_string()));
        assert!(metadata.import_calls.contains(&"os.path.join".to_string()));
    }

    #[test]
    fn test_only_first_20_lines() {
        let mut lines = vec!["# Test file".to_string()];
        for i in 0..25 {
            if i == 15 {
                lines.push("# @core".to_string());
            } else if i == 22 {
                lines.push("# @flaky".to_string());  // This should NOT be parsed (beyond line 20)
            } else {
                lines.push(format!("# Line {}", i));
            }
        }
        lines.push("system Test { }".to_string());
        
        let content = lines.join("\n");
        let (_dir, path) = create_test_fixture(&content);
        let metadata = parse_fixture_metadata(&path);
        
        assert!(metadata.core);  // Line 16 - should be parsed
        assert!(!metadata.flaky); // Line 23 - should NOT be parsed
    }

    #[test]
    fn test_mixed_comment_styles() {
        let content = r#"
# @core
// @flaky
# @meta: test1
// @meta: test2
  # @exec-ok
  // @noncore

system Test { }
"#;
        let (_dir, path) = create_test_fixture(content);
        let metadata = parse_fixture_metadata(&path);
        
        assert!(metadata.core);
        assert!(metadata.flaky);
        assert!(metadata.exec_ok);
        assert!(metadata.noncore);
        assert_eq!(metadata.meta.len(), 2);
    }
}

// Tests for the filter matching logic
#[cfg(test)]
mod filter_tests {
    use super::super::test_harness_rs::{TestMetadata, should_run_test, matches_filter};
    
    fn create_metadata_with_flags(core: bool, flaky: bool, exec_ok: bool) -> TestMetadata {
        TestMetadata {
            core,
            flaky,
            exec_ok,
            ..Default::default()
        }
    }
    
    fn create_metadata_with_meta(meta_tags: Vec<&str>) -> TestMetadata {
        TestMetadata {
            meta: meta_tags.iter().map(|s| s.to_string()).collect(),
            ..Default::default()
        }
    }
    
    fn create_metadata_with_skip(skip_conditions: Vec<&str>) -> TestMetadata {
        TestMetadata {
            skip_if: skip_conditions.iter().map(|s| s.to_string()).collect(),
            ..Default::default()
        }
    }
    
    #[test]
    fn test_filter_core() {
        let metadata = create_metadata_with_flags(true, false, false);
        assert!(should_run_test(&metadata, "@core"));
        assert!(!should_run_test(&metadata, "@noncore"));
    }
    
    #[test]
    fn test_filter_negation() {
        let metadata = create_metadata_with_flags(false, true, false);
        assert!(!should_run_test(&metadata, "!flaky"));
        assert!(should_run_test(&metadata, "!core"));
    }
    
    #[test]
    fn test_filter_meta_tag() {
        let metadata = create_metadata_with_meta(vec!["regression", "important"]);
        assert!(should_run_test(&metadata, "meta:regression"));
        assert!(should_run_test(&metadata, "meta:important"));
        assert!(!should_run_test(&metadata, "meta:performance"));
    }
    
    #[test]
    fn test_filter_skip_condition() {
        let metadata = create_metadata_with_skip(vec!["windows", "slow"]);
        assert!(should_run_test(&metadata, "skip:windows"));
        assert!(should_run_test(&metadata, "skip:slow"));
        assert!(!should_run_test(&metadata, "skip:linux"));
    }
}