//! Tests for the Frame AST parser

#[cfg(test)]
mod tests {
    use crate::frame_c::v4::frame_parser::*;
    use crate::frame_c::v4::frame_ast::*;
    
    #[test]
    fn test_parse_simple_traffic_light() {
        let source = r#"
system TrafficLight {
    machine:
        $Red {
            tick() { 
                -> $Green() 
            }
        }
        $Green {
            tick() { 
                -> $Yellow() 
            }
        }
        $Yellow {
            tick() { 
                -> $Red() 
            }
        }
}"#;
        
        let mut parser = FrameParser::new(source.as_bytes(), TargetLanguage::Python3);
        let result = parser.parse_module();
        
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());
        
        if let Ok(FrameAst::System(system)) = result {
            assert_eq!(system.name, "TrafficLight");
            assert!(system.machine.is_some());
            
            let machine = system.machine.unwrap();
            assert_eq!(machine.states.len(), 3);
            
            // Check Red state
            let red = &machine.states[0];
            assert_eq!(red.name, "Red");
            assert_eq!(red.handlers.len(), 1);
            assert_eq!(red.handlers[0].event, "tick");
            
            // Check handler body
            let body = &red.handlers[0].body;
            assert!(!body.statements.is_empty());
            
            // Should have a transition statement
            match &body.statements[0] {
                Statement::Transition(t) => {
                    assert_eq!(t.target, "Green");
                    assert!(t.args.is_empty());
                }
                _ => panic!("Expected transition statement"),
            }
            
            // Check Green state
            let green = &machine.states[1];
            assert_eq!(green.name, "Green");
            assert_eq!(green.handlers.len(), 1);
            
            // Check Yellow state
            let yellow = &machine.states[2];
            assert_eq!(yellow.name, "Yellow");
            assert_eq!(yellow.handlers.len(), 1);
        } else {
            panic!("Expected System AST");
        }
    }
    
    #[test]
    fn test_parse_state_with_parameters() {
        let source = r#"
system Test {
    machine:
        $Start {
            go() { -> $Target(5, "hello") }
        }
        $Target(x: int, y: string) {
            idle() { }
        }
}"#;
        
        let mut parser = FrameParser::new(source.as_bytes(), TargetLanguage::Python3);
        let result = parser.parse_module();
        
        assert!(result.is_ok());
        
        if let Ok(FrameAst::System(system)) = result {
            let machine = system.machine.unwrap();
            assert_eq!(machine.states.len(), 2);
            
            // Check Start state transition with args
            let start = &machine.states[0];
            let handler = &start.handlers[0];
            match &handler.body.statements[0] {
                Statement::Transition(t) => {
                    assert_eq!(t.target, "Target");
                    assert_eq!(t.args.len(), 2);
                    
                    // Check arguments
                    match &t.args[0] {
                        Expression::Literal(Literal::Int(n)) => assert_eq!(*n, 5),
                        _ => panic!("Expected int literal"),
                    }
                    match &t.args[1] {
                        Expression::Literal(Literal::String(s)) => assert_eq!(s, "hello"),
                        _ => panic!("Expected string literal"),
                    }
                }
                _ => panic!("Expected transition statement"),
            }
            
            // Check Target state parameters
            let target = &machine.states[1];
            assert_eq!(target.name, "Target");
            assert_eq!(target.params.len(), 2);
            assert_eq!(target.params[0].name, "x");
            assert_eq!(target.params[0].param_type, Type::Int);
            assert_eq!(target.params[1].name, "y");
            assert_eq!(target.params[1].param_type, Type::String);
        }
    }
    
    #[test]
    fn test_parse_enter_exit_handlers() {
        let source = r#"
system Test {
    machine:
        $Active {
            $>() {
                // Enter handler
            }
            
            process() {
                // Event handler  
            }
            
            $<() {
                // Exit handler
            }
        }
}"#;
        
        let mut parser = FrameParser::new(source.as_bytes(), TargetLanguage::Python3);
        let result = parser.parse_module();
        
        assert!(result.is_ok());
        
        if let Ok(FrameAst::System(system)) = result {
            let machine = system.machine.unwrap();
            let state = &machine.states[0];
            
            assert!(state.enter.is_some(), "Should have enter handler");
            assert!(state.exit.is_some(), "Should have exit handler");
            assert_eq!(state.handlers.len(), 1);
            assert_eq!(state.handlers[0].event, "process");
        }
    }
    
    #[test]
    fn test_parse_forward_to_parent() {
        let source = r#"
system Test {
    machine:
        $Child => $Parent {
            unhandled() {
                => unhandled()
            }
        }
        $Parent {
            unhandled() { }
        }
}"#;
        
        let mut parser = FrameParser::new(source.as_bytes(), TargetLanguage::Python3);
        let result = parser.parse_module();
        
        assert!(result.is_ok());
        
        if let Ok(FrameAst::System(system)) = result {
            let machine = system.machine.unwrap();
            let child = &machine.states[0];
            
            assert_eq!(child.name, "Child");
            assert_eq!(child.parent, Some("Parent".to_string()));
            
            // Check forward statement
            let handler = &child.handlers[0];
            match &handler.body.statements[0] {
                Statement::Forward(f) => {
                    assert_eq!(f.event, "unhandled");
                }
                _ => panic!("Expected forward statement"),
            }
        }
    }
    
    #[test]
    fn test_parse_stack_operations() {
        let source = r#"
system Test {
    machine:
        $Active {
            push() {
                $$[+]
                -> $Other()
            }
        }
        $Other {
            pop() {
                $$[-]
            }
        }
}"#;
        
        let mut parser = FrameParser::new(source.as_bytes(), TargetLanguage::Python3);
        let result = parser.parse_module();
        
        assert!(result.is_ok());
        
        if let Ok(FrameAst::System(system)) = result {
            let machine = system.machine.unwrap();
            
            // Check stack push
            let active = &machine.states[0];
            let push_handler = &active.handlers[0];
            match &push_handler.body.statements[0] {
                Statement::StackPush(_) => {},
                _ => panic!("Expected stack push"),
            }
            
            // Check stack pop
            let other = &machine.states[1];
            let pop_handler = &other.handlers[0];
            match &pop_handler.body.statements[0] {
                Statement::StackPop(_) => {},
                _ => panic!("Expected stack pop"),
            }
        }
    }
    
    #[test]
    fn test_parse_return_and_continue() {
        let source = r#"
system Test {
    interface:
        getValue(): int
        
    machine:
        $Active {
            getValue() {
                ^ 42
            }
            
            skip() {
                ^>
            }
        }
}"#;
        
        let mut parser = FrameParser::new(source.as_bytes(), TargetLanguage::Python3);
        let result = parser.parse_module();
        
        assert!(result.is_ok());
        
        if let Ok(FrameAst::System(system)) = result {
            // Check interface
            assert_eq!(system.interface.len(), 1);
            assert_eq!(system.interface[0].name, "getValue");
            assert_eq!(system.interface[0].return_type, Some(Type::Int));
            
            let machine = system.machine.unwrap();
            let active = &machine.states[0];
            
            // Check return statement
            let get_value = &active.handlers[0];
            match &get_value.body.statements[0] {
                Statement::Return(r) => {
                    assert!(r.value.is_some());
                    match r.value.as_ref().unwrap() {
                        Expression::Literal(Literal::Int(n)) => assert_eq!(*n, 42),
                        _ => panic!("Expected int literal"),
                    }
                }
                _ => panic!("Expected return statement"),
            }
            
            // Check continue statement
            let skip = &active.handlers[1];
            match &skip.body.statements[0] {
                Statement::Continue(_) => {},
                _ => panic!("Expected continue statement"),
            }
        }
    }
    
    #[test]
    fn test_parse_mixed_native_and_frame() {
        let source = r#"
system Test {
    machine:
        $Active {
            process(data: string) {
                print("Processing:", data)
                -> $Done()
            }
        }
        $Done {
            reset() {
                -> $Active()
            }
        }
}"#;
        
        let mut parser = FrameParser::new(source.as_bytes(), TargetLanguage::Python3);
        let result = parser.parse_module();
        
        assert!(result.is_ok());
        
        if let Ok(FrameAst::System(system)) = result {
            let machine = system.machine.unwrap();
            let active = &machine.states[0];
            let handler = &active.handlers[0];
            
            // Should have native code followed by transition
            assert!(handler.body.statements.len() >= 2);
            
            // First statement should be native code
            match &handler.body.statements[0] {
                Statement::Native(n) => {
                    assert!(n.content.contains("print"));
                }
                _ => panic!("Expected native code block"),
            }
            
            // Second statement should be transition
            match &handler.body.statements[1] {
                Statement::Transition(t) => {
                    assert_eq!(t.target, "Done");
                }
                _ => panic!("Expected transition statement"),
            }
        }
    }
}