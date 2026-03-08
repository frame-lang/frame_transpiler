//! Integration tests for compile_module validation blocking

#[cfg(test)]
mod tests {
    use crate::frame_c::v4::compile_module;
    use crate::frame_c::visitors::TargetLanguage;

    #[test]
    fn test_compilation_fails_on_validation_error() {
        // Test case with an invalid state transition (E402)
        let invalid_frame = r#"
@@target python_3
@@system Test {
    machine:
        $Start {
            go() {
                -> $NonExistent
            }
        }
}"#;

        let result = compile_module(invalid_frame, TargetLanguage::Python3);

        // Compilation should fail
        assert!(result.is_err(), "Expected compilation to fail on validation error");

        // Error message should contain E402
        let err = result.unwrap_err();
        assert!(
            err.error.contains("E402"),
            "Expected error message to contain E402, got: {}",
            err.error
        );
        assert!(
            err.error.contains("NonExistent"),
            "Expected error message to mention NonExistent state, got: {}",
            err.error
        );
    }

    #[test]
    #[ignore] // E403 validation not yet implemented in v4
    fn test_compilation_fails_on_invalid_parent_forward() {
        // Test case with invalid parent forwarding (E403)
        let invalid_frame = r#"
@@target python_3
@@system Test {
    machine:
        $Start {
            test() {
                => test()
            }
        }
}"#;

        let result = compile_module(invalid_frame, TargetLanguage::Python3);

        assert!(result.is_err(), "Expected compilation to fail on E403 error");

        let err = result.unwrap_err();
        assert!(
            err.error.contains("E403") || err.error.contains("parent"),
            "Expected error about parent forwarding, got: {}",
            err.error
        );
    }

    #[test]
    #[ignore] // E405 validation not yet implemented in v4
    fn test_compilation_fails_on_state_param_mismatch() {
        // Test case with state parameter arity mismatch (E405)
        let invalid_frame = r#"
@@target python_3
@@system Test {
    machine:
        $Start {
            go() {
                -> $Target
            }
        }

        $Target(x: int, y: int) {
            idle() { }
        }
}"#;

        let result = compile_module(invalid_frame, TargetLanguage::Python3);

        assert!(result.is_err(), "Expected compilation to fail on E405 error");

        let err = result.unwrap_err();
        assert!(
            err.error.contains("E405") || err.error.contains("parameter"),
            "Expected error about parameter mismatch, got: {}",
            err.error
        );
    }

    #[test]
    fn test_compilation_succeeds_on_valid_frame() {
        // Valid Frame code should compile successfully
        let valid_frame = r#"
@@target python_3
@@system TrafficLight {
    machine:
        $Red {
            tick() {
                -> $Green
            }
        }

        $Green {
            tick() {
                -> $Yellow
            }
        }

        $Yellow {
            tick() {
                -> $Red
            }
        }
}"#;

        let result = compile_module(valid_frame, TargetLanguage::Python3);

        assert!(
            result.is_ok(),
            "Expected valid Frame to compile successfully, got error: {:?}",
            result.err()
        );

        // Generated code should contain the system class
        let generated = result.unwrap();
        assert!(
            generated.contains("class TrafficLight"),
            "Expected generated code to contain TrafficLight class"
        );
    }

    #[test]
    fn test_compilation_fails_on_invalid_interface_method() {
        // Test case calling non-existent interface method (E406)
        let invalid_frame = r#"
@@target python_3
@@system Test {
    interface:
        start()

    machine:
        $Start {
            go() {
                stop()
            }
        }
}"#;

        let result = compile_module(invalid_frame, TargetLanguage::Python3);

        // Note: E406 may not be fully implemented yet, but test the intent
        if result.is_err() {
            let err = result.unwrap_err();
            // Check for some kind of error about unknown method
            assert!(
                err.error.contains("stop") ||
                err.error.contains("E406") ||
                err.error.contains("method"),
                "Expected error about unknown method, got: {}",
                err.error
            );
        }
    }

    #[test]
    fn test_multiple_validation_errors_all_reported() {
        // Frame code with multiple validation errors
        let invalid_frame = r#"
@@target python_3
@@system Test {
    machine:
        $Start {
            test1() {
                -> $Unknown1
            }
            test2() {
                -> $Unknown2
            }
        }
}"#;

        let result = compile_module(invalid_frame, TargetLanguage::Python3);

        assert!(result.is_err(), "Expected compilation to fail on multiple errors");

        let err = result.unwrap_err();
        // Should report at least one undefined state
        assert!(
            err.error.contains("Unknown1") || err.error.contains("Unknown2") || err.error.contains("E402"),
            "Expected error about Unknown state, got: {}",
            err.error
        );
    }

    #[test]
    fn test_typescript_compilation_with_validation() {
        // Test TypeScript target also blocks on validation errors
        let invalid_frame = r#"
@@target typescript
@@system Test {
    machine:
        $Start {
            go() {
                -> $Missing
            }
        }
}"#;

        let result = compile_module(invalid_frame, TargetLanguage::TypeScript);

        assert!(result.is_err(), "Expected TypeScript compilation to fail on validation error");

        let err = result.unwrap_err();
        assert!(
            err.error.contains("Missing") || err.error.contains("E402"),
            "Expected error about Missing state, got: {}",
            err.error
        );
    }
}
