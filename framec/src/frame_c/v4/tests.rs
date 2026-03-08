// Integration tests for Frame v4 compiler

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_basic_scanner() {
        let source = r#"@@target python
@@system Test {
    machine:
        $Start {
            go() {
                -> $End()
            }
        }
        $End {}
}"#;

        let tokens = scanner::scan(source, "test.fpy").unwrap();
        
        // Verify we get the expected tokens
        assert!(tokens.len() > 0);
        
        // Check for key tokens
        let has_target = tokens.iter().any(|t| matches!(t.token_type, scanner::TokenType::FrameAnnotation));
        let has_system = tokens.iter().any(|t| t.lexeme == "@@system");
        let has_state = tokens.iter().any(|t| matches!(t.token_type, scanner::TokenType::State));
        
        assert!(has_target, "Should have @@target annotation");
        assert!(has_system, "Should have @@system keyword");
        assert!(has_state, "Should have state token");
    }

    #[test]
    fn test_basic_parser() {
        let source = r#"@@target python

@@system TrafficLight {
    interface:
        timer()
        getColor(): str
    
    machine:
        $Red {
            timer() {
                print("Red light")
                -> $Green()
            }
            
            getColor() {
                return "red"
            }
        }
        
        $Green {
            timer() {
                -> $Red()
            }
            
            getColor() {
                return "green"
            }
        }
}"#;

        let tokens = scanner::scan(source, "test.fpy").unwrap();
        let ast = parser::parse(tokens, source).unwrap();
        
        // Verify AST structure
        assert_eq!(ast.name, "TrafficLight");
        assert_eq!(ast.target, "python");
        
        // Check interface
        assert!(ast.interface.is_some());
        let interface = ast.interface.as_ref().unwrap();
        assert_eq!(interface.methods.len(), 2);
        assert_eq!(interface.methods[0].name, "timer");
        assert_eq!(interface.methods[1].name, "getColor");
        
        // Check machine
        assert!(ast.machine.is_some());
        let machine = ast.machine.as_ref().unwrap();
        assert_eq!(machine.states.len(), 2);
        assert_eq!(machine.states[0].name, "Red");
        assert_eq!(machine.states[1].name, "Green");
        
        // Check handlers
        assert_eq!(machine.states[0].handlers.len(), 2);
        assert_eq!(machine.states[0].handlers[0].name, Some("timer".to_string()));
        assert_eq!(machine.states[0].handlers[1].name, Some("getColor".to_string()));
    }

    #[test]
    fn test_system_params() {
        let source = r#"@@target python

@@system Robot ($(x, y), $>(battery), name) {
    domain:
        name = ""
    
    machine:
        $Start(x, y) {
            $>(battery) {
                print(f"Robot at {x},{y} with {battery}% battery")
            }
        }
}"#;

        let tokens = scanner::scan(source, "test.fpy").unwrap();
        let ast = parser::parse(tokens, source).unwrap();
        
        // Check system parameters
        assert_eq!(ast.params.start_state_params.len(), 2);
        assert_eq!(ast.params.start_state_params[0].name, "x");
        assert_eq!(ast.params.start_state_params[1].name, "y");
        
        assert_eq!(ast.params.enter_params.len(), 1);
        assert_eq!(ast.params.enter_params[0].name, "battery");
        
        assert_eq!(ast.params.domain_params.len(), 1);
        assert_eq!(ast.params.domain_params[0].name, "name");
        
        // Validate should pass
        let validation_result = validator::validate(&ast);
        assert!(validation_result.is_ok(), "Validation should pass for matching parameters");
    }

    #[test]
    fn test_validation_errors() {
        // Test mismatched start state params
        let source = r#"@@target python

@@system Test ($(x, y)) {
    machine:
        $Start {  // Missing (x, y) params - should error
            init() {}
        }
}"#;

        let tokens = scanner::scan(source, "test.fpy").unwrap();
        let ast = parser::parse(tokens, source).unwrap();
        
        let validation_result = validator::validate(&ast);
        assert!(validation_result.is_err(), "Should fail validation for missing state params");
    }

    #[test]
    fn test_python_codegen() {
        let source = r#"@@target python

@@system Simple {
    interface:
        go()
    
    machine:
        $Start {
            go() {
                print("Going!")
                -> $End()
            }
        }
        
        $End {
            go() {
                print("Already at end")
            }
        }
}"#;

        let tokens = scanner::scan(source, "test.fpy").unwrap();
        let ast = parser::parse(tokens, source).unwrap();
        validator::validate(&ast).unwrap();
        
        let code = codegen::generate(&ast, TargetLanguage::Python).unwrap();
        
        // Verify generated code contains key elements
        assert!(code.contains("class Simple:"));
        assert!(code.contains("def __init__(self"));
        assert!(code.contains("def go(self)"));
        assert!(code.contains("self._state = "));
        assert!(code.contains("def _transition_to_Start("));
        assert!(code.contains("def _transition_to_End("));
        assert!(code.contains("def _handle_Start_go("));
        assert!(code.contains("def _handle_End_go("));
    }

    #[test]
    fn test_frame_statements() {
        let source = r#"@@target python

@@system Test {
    machine:
        $A {
            test() {
                x = 1
                -> $B(x)
                unreachable_code()
            }
            
            stack_test() {
                $$[+]
                -> $B()
            }
        }
        
        $B(val) {
            $>(val) {
                print(val)
            }
            
            pop_test() {
                $$[-]
            }
        }
}"#;

        let tokens = scanner::scan(source, "test.fpy").unwrap();
        let ast = parser::parse(tokens, source).unwrap();
        
        let machine = ast.machine.as_ref().unwrap();
        let state_a = &machine.states[0];
        
        // Check frame statements were extracted
        let test_handler = state_a.handlers.iter()
            .find(|h| h.name == Some("test".to_string()))
            .unwrap();
        
        assert_eq!(test_handler.frame_statements.len(), 1);
        match &test_handler.frame_statements[0] {
            ast::FrameStatement::Transition { target, args, .. } => {
                assert_eq!(target, "B");
                assert_eq!(args.len(), 1);
            }
            _ => panic!("Expected transition statement"),
        }
        
        // Check stack operations
        let stack_handler = state_a.handlers.iter()
            .find(|h| h.name == Some("stack_test".to_string()))
            .unwrap();
        
        assert_eq!(stack_handler.frame_statements.len(), 2);
        assert!(matches!(&stack_handler.frame_statements[0], 
                        ast::FrameStatement::StackPush { .. }));
        assert!(matches!(&stack_handler.frame_statements[1], 
                        ast::FrameStatement::Transition { .. }));
    }

    #[test]
    fn test_native_code_preservation() {
        let source = r#"@@target python

@dataclass
class Config:
    name: str
    value: int

@@system Test {
    actions:
        helper(x: int) {
            # This is native Python code
            result = x * 2
            if result > 10:
                print(f"Large: {result}")
            else:
                print(f"Small: {result}")
            return result
        }
}"#;

        let tokens = scanner::scan(source, "test.fpy").unwrap();
        let ast = parser::parse(tokens, source).unwrap();
        
        // Check native annotation preserved
        assert_eq!(ast.native_imports.len(), 0); // Annotations aren't imports
        assert_eq!(ast.annotations.len(), 1);
        match &ast.annotations[0] {
            ast::Annotation::Native { content } => {
                assert!(content.contains("@dataclass"));
            }
            _ => panic!("Expected native annotation"),
        }
        
        // Check native code in method preserved
        let actions = ast.actions.as_ref().unwrap();
        assert_eq!(actions.methods.len(), 1);
        let helper = &actions.methods[0];
        assert!(helper.native_code.contains("result = x * 2"));
        assert!(helper.native_code.contains("if result > 10:"));
    }
}