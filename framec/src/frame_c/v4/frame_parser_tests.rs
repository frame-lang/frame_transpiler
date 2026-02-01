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
    
    #[test]
    fn test_parse_interface_section() {
        let source = r#"
system Calculator {
    interface:
        add(x: int, y: int): int
        subtract(a: float, b: float): float
        getName(): string
        process(data: string)
        
    machine:
        $Idle {
        }
}"#;
        
        let mut parser = FrameParser::new(source.as_bytes(), TargetLanguage::Python3);
        let result = parser.parse_module();
        
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());
        
        if let Ok(FrameAst::System(system)) = result {
            assert_eq!(system.interface.len(), 4);
            
            // Check add method
            let add = &system.interface[0];
            assert_eq!(add.name, "add");
            assert_eq!(add.params.len(), 2);
            assert_eq!(add.params[0].name, "x");
            assert_eq!(add.params[0].param_type, Type::Int);
            assert_eq!(add.params[1].name, "y");
            assert_eq!(add.params[1].param_type, Type::Int);
            assert_eq!(add.return_type, Some(Type::Int));
            
            // Check subtract method
            let subtract = &system.interface[1];
            assert_eq!(subtract.name, "subtract");
            assert_eq!(subtract.params.len(), 2);
            assert_eq!(subtract.return_type, Some(Type::Float));
            
            // Check getName method
            let get_name = &system.interface[2];
            assert_eq!(get_name.name, "getName");
            assert_eq!(get_name.params.len(), 0);
            assert_eq!(get_name.return_type, Some(Type::String));
            
            // Check process method (no return type)
            let process = &system.interface[3];
            assert_eq!(process.name, "process");
            assert_eq!(process.params.len(), 1);
            assert_eq!(process.return_type, None);
        }
    }
    
    #[test]
    fn test_parse_actions_section() {
        let source = r#"
system Robot {
    machine:
        $Idle {
        }
        
    actions:
        moveForward(distance: float) {
            # Python code to move robot
            self.position += distance
            print(f"Moving {distance} units")
        }
        
        turnLeft() {
            self.angle -= 90
        }
        
        log(message: string) {
            print(f"[LOG] {message}")
        }
}"#;
        
        let mut parser = FrameParser::new(source.as_bytes(), TargetLanguage::Python3);
        let result = parser.parse_module();
        
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());
        
        if let Ok(FrameAst::System(system)) = result {
            assert_eq!(system.actions.len(), 3);
            
            // Check moveForward action
            let move_forward = &system.actions[0];
            assert_eq!(move_forward.name, "moveForward");
            assert_eq!(move_forward.params.len(), 1);
            assert_eq!(move_forward.params[0].name, "distance");
            assert_eq!(move_forward.params[0].param_type, Type::Float);
            assert!(move_forward.body.native.content.contains("self.position"));
            
            // Check turnLeft action
            let turn_left = &system.actions[1];
            assert_eq!(turn_left.name, "turnLeft");
            assert_eq!(turn_left.params.len(), 0);
            
            // Check log action
            let log = &system.actions[2];
            assert_eq!(log.name, "log");
            assert_eq!(log.params.len(), 1);
            assert_eq!(log.params[0].param_type, Type::String);
        }
    }
    
    #[test]
    fn test_parse_operations_section() {
        let source = r#"
system MathLib {
    machine:
        $Ready {
        }
        
    operations:
        calculate(x: int, y: int): int {
            return x * y + 10
        }
        
        isValid(value: float): bool {
            return value > 0 and value < 100
        }
        
        format(n: int): string {
            return f"Number: {n}"
        }
}"#;
        
        let mut parser = FrameParser::new(source.as_bytes(), TargetLanguage::Python3);
        let result = parser.parse_module();
        
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());
        
        if let Ok(FrameAst::System(system)) = result {
            assert_eq!(system.operations.len(), 3);
            
            // Check calculate operation
            let calc = &system.operations[0];
            assert_eq!(calc.name, "calculate");
            assert_eq!(calc.params.len(), 2);
            assert_eq!(calc.return_type, Type::Int);
            assert!(calc.body.native.content.contains("return"));
            
            // Check isValid operation
            let is_valid = &system.operations[1];
            assert_eq!(is_valid.name, "isValid");
            assert_eq!(is_valid.return_type, Type::Bool);
            
            // Check format operation
            let format = &system.operations[2];
            assert_eq!(format.name, "format");
            assert_eq!(format.return_type, Type::String);
        }
    }
    
    #[test]
    fn test_parse_domain_section() {
        let source = r#"
system Counter {
    machine:
        $Active {
        }
        
    domain:
        var count = 0
        var name = "Counter"
        var threshold: int = 100
        var rate: float = 1.5
        var active = true
}"#;
        
        let mut parser = FrameParser::new(source.as_bytes(), TargetLanguage::Python3);
        let result = parser.parse_module();
        
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());
        
        if let Ok(FrameAst::System(system)) = result {
            assert_eq!(system.domain.len(), 5);
            
            // Check count variable
            let count = &system.domain[0];
            assert_eq!(count.name, "count");
            assert!(count.initializer.is_some());
            assert!(count.is_frame);
            
            // Check name variable
            let name = &system.domain[1];
            assert_eq!(name.name, "name");
            assert!(name.initializer.is_some());
            
            // Check threshold variable with type
            let threshold = &system.domain[2];
            assert_eq!(threshold.name, "threshold");
            assert_eq!(threshold.var_type, Type::Int);
            
            // Check rate variable
            let rate = &system.domain[3];
            assert_eq!(rate.name, "rate");
            assert_eq!(rate.var_type, Type::Float);
            
            // Check active variable
            let active = &system.domain[4];
            assert_eq!(active.name, "active");
        }
    }
    
    #[test]
    fn test_parse_complete_system() {
        let source = r#"
system CompleteSystem {
    interface:
        start()
        stop()
        getStatus(): string
        
    machine:
        $Idle {
            start() {
                doStartup()
                -> $Running()
            }
        }
        
        $Running {
            stop() {
                doShutdown()
                -> $Idle()
            }
            
            getStatus() {
                ^ "running"
            }
        }
        
    actions:
        doStartup() {
            print("Starting up...")
        }
        
        doShutdown() {
            print("Shutting down...")
        }
        
    operations:
        checkHealth(): bool {
            return true
        }
        
    domain:
        var status = "idle"
        var counter = 0
}"#;
        
        let mut parser = FrameParser::new(source.as_bytes(), TargetLanguage::Python3);
        let result = parser.parse_module();
        
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());
        
        if let Ok(FrameAst::System(system)) = result {
            assert_eq!(system.name, "CompleteSystem");
            
            // Check all sections are parsed
            assert_eq!(system.interface.len(), 3);
            assert!(system.machine.is_some());
            assert_eq!(system.machine.as_ref().unwrap().states.len(), 2);
            assert_eq!(system.actions.len(), 2);
            assert_eq!(system.operations.len(), 1);
            assert_eq!(system.domain.len(), 2);
            
            // Verify integration between sections
            let idle_state = &system.machine.as_ref().unwrap().states[0];
            assert_eq!(idle_state.handlers.len(), 1);
            assert_eq!(idle_state.handlers[0].event, "start");
            
            // Check that handler calls action
            let handler_body = &idle_state.handlers[0].body;
            assert!(handler_body.statements.len() >= 2); // Native call + transition
        }
    }
}