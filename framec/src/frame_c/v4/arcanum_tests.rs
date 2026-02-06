#[cfg(test)]
mod tests {
    use super::super::arcanum::*;
    use super::super::ast::*;
    use std::collections::{HashMap, HashSet};

    fn create_test_arcanum() -> Arcanum {
        let mut arc = Arcanum::new();
        
        // Create a test system entry
        let mut sys_entry = SystemEntry::default();
        
        // Add interface methods
        sys_entry.interface_methods.insert("timer".to_string());
        sys_entry.interface_methods.insert("getColor".to_string());
        
        // Add actions
        sys_entry.actions.insert("updateDisplay".to_string());
        sys_entry.actions.insert("logTransition".to_string());
        
        // Add operations
        sys_entry.operations.insert("calculate".to_string());
        
        // Add domain variables
        sys_entry.domain_vars.insert("color".to_string(), VarType::Unknown);
        sys_entry.domain_vars.insert("count".to_string(), VarType::Unknown);
        
        // Add machine with states
        let mut machine_entry = MachineEntry::default();
        machine_entry.states.insert("Red".to_string(), StateDecl {
            name: "Red".to_string(),
            parent: None,
            params: vec![],
            span: Span { start: 0, end: 10 },
        });
        machine_entry.states.insert("Green".to_string(), StateDecl {
            name: "Green".to_string(),
            parent: None,
            params: vec![],
            span: Span { start: 20, end: 30 },
        });
        machine_entry.states.insert("ConfigState".to_string(), StateDecl {
            name: "ConfigState".to_string(),
            parent: None,
            params: vec!["timeout".to_string(), "mode".to_string()],
            span: Span { start: 40, end: 50 },
        });
        
        sys_entry.machines.insert("machine".to_string(), machine_entry);
        arc.systems.insert("TrafficLight".to_string(), sys_entry);
        
        arc
    }

    #[test]
    fn test_interface_method_validation() {
        let arc = create_test_arcanum();
        
        // Interface methods should be accessible
        assert!(arc.is_interface_method("TrafficLight", "timer"));
        assert!(arc.is_interface_method("TrafficLight", "getColor"));
        
        // Non-interface methods should not be accessible
        assert!(!arc.is_interface_method("TrafficLight", "updateDisplay"));
        assert!(!arc.is_interface_method("TrafficLight", "nonExistent"));
        
        // Unknown system should return false
        assert!(!arc.is_interface_method("UnknownSystem", "timer"));
    }

    #[test]
    fn test_action_validation() {
        let arc = create_test_arcanum();
        
        // Actions should exist
        assert!(arc.has_action("TrafficLight", "updateDisplay"));
        assert!(arc.has_action("TrafficLight", "logTransition"));
        
        // Non-actions should not exist
        assert!(!arc.has_action("TrafficLight", "timer"));
        assert!(!arc.has_action("TrafficLight", "nonExistent"));
    }

    #[test]
    fn test_operation_validation() {
        let arc = create_test_arcanum();
        
        // Operations should exist
        assert!(arc.has_operation("TrafficLight", "calculate"));
        
        // Non-operations should not exist
        assert!(!arc.has_operation("TrafficLight", "timer"));
        assert!(!arc.has_operation("TrafficLight", "nonExistent"));
    }

    #[test]
    fn test_state_transition_validation() {
        let arc = create_test_arcanum();
        
        // Valid transitions
        assert!(arc.validate_transition("TrafficLight", "Red").is_ok());
        assert!(arc.validate_transition("TrafficLight", "Green").is_ok());
        
        // Invalid transition
        let err = arc.validate_transition("TrafficLight", "Purple");
        assert!(err.is_err());
        let err_msg = err.unwrap_err();
        assert!(err_msg.contains("Unknown state 'Purple'"));
        assert!(err_msg.contains("$Red"));
        assert!(err_msg.contains("$Green"));
    }

    #[test]
    fn test_state_parameter_arity() {
        let arc = create_test_arcanum();
        
        // States without parameters
        assert_eq!(arc.get_state_param_count("TrafficLight", "Red"), Some(0));
        assert_eq!(arc.get_state_param_count("TrafficLight", "Green"), Some(0));
        
        // State with parameters
        assert_eq!(arc.get_state_param_count("TrafficLight", "ConfigState"), Some(2));
        
        // Non-existent state
        assert_eq!(arc.get_state_param_count("TrafficLight", "NonExistent"), None);
    }

    #[test]
    fn test_get_system_states() {
        let arc = create_test_arcanum();
        
        let states = arc.get_system_states("TrafficLight");
        assert_eq!(states.len(), 3);
        assert!(states.contains(&"$ConfigState".to_string()));
        assert!(states.contains(&"$Green".to_string()));
        assert!(states.contains(&"$Red".to_string()));
        
        // Unknown system
        let states = arc.get_system_states("UnknownSystem");
        assert!(states.is_empty());
    }

    #[test]
    fn test_collect_methods_from_bytes() {
        let source = b"
            interface:
                timer()
                getColor(): str
                setState(newState: str, timeout: int): bool
        ";
        
        let span = Span { start: 0, end: source.len() };
        let methods = super::super::arcanum::collect_methods_in_section(source, &span);
        
        assert_eq!(methods.len(), 3);
        assert!(methods.contains("timer"));
        assert!(methods.contains("getColor"));
        assert!(methods.contains("setState"));
    }

    #[test]
    fn test_collect_domain_vars_from_bytes() {
        let source = b"
                var color = \"red\"
                count: int = 0
                lastUpdate = now()
                isActive: bool
        ";

        let span = Span { start: 0, end: source.len() };
        let vars = super::super::arcanum::collect_domain_vars(source, &span);

        assert_eq!(vars.len(), 4);
        assert!(vars.contains_key("color"));
        assert!(vars.contains_key("count"));
        assert!(vars.contains_key("lastUpdate"));
        assert!(vars.contains_key("isActive"));
    }

    #[test]
    fn test_build_arcanum_from_frame_ast() {
        use super::super::frame_ast::{
            FrameAst, SystemAst as FrameSystemAst, MachineAst, StateAst,
            InterfaceMethod, ActionAst, OperationAst, DomainVar,
            Span as FrameSpan, Type, ActionBody, OperationBody,
        };

        // Create a Frame AST manually
        let system = FrameSystemAst::new("TestSystem".to_string(), FrameSpan::new(0, 100));
        let mut system = system;

        // Add interface methods
        system.interface.push(InterfaceMethod {
            name: "start".to_string(),
            params: vec![],
            return_type: None,
            span: FrameSpan::new(10, 20),
        });
        system.interface.push(InterfaceMethod {
            name: "stop".to_string(),
            params: vec![],
            return_type: None,
            span: FrameSpan::new(20, 30),
        });

        // Add actions (body now just has span - content is extracted from source by codegen)
        system.actions.push(ActionAst {
            name: "doSomething".to_string(),
            params: vec![],
            body: ActionBody {
                span: FrameSpan::new(0, 10),
            },
            span: FrameSpan::new(30, 40),
        });

        // Add operations (body now just has span - content is extracted from source by codegen)
        system.operations.push(OperationAst {
            name: "getValue".to_string(),
            params: vec![],
            return_type: Type::Int,
            body: OperationBody {
                span: FrameSpan::new(0, 10),
            },
            span: FrameSpan::new(40, 50),
        });

        // Add machine with states
        system.machine = Some(MachineAst {
            states: vec![
                StateAst::new("Idle".to_string(), FrameSpan::new(50, 60)),
                StateAst::new("Running".to_string(), FrameSpan::new(60, 70)),
            ],
            span: FrameSpan::new(50, 70),
        });

        // Add domain var
        system.domain.push(DomainVar {
            name: "counter".to_string(),
            var_type: Type::Int,
            initializer: None,
            is_frame: true,
            span: FrameSpan::new(70, 80),
        });

        // Build arcanum from Frame AST
        let ast = FrameAst::System(system);
        let arc = super::super::arcanum::build_arcanum_from_frame_ast(&ast);

        // Verify the arcanum was built correctly
        assert!(arc.systems.contains_key("TestSystem"));

        let sys_entry = arc.systems.get("TestSystem").unwrap();
        assert!(sys_entry.interface_methods.contains("start"));
        assert!(sys_entry.interface_methods.contains("stop"));
        assert!(sys_entry.actions.contains("doSomething"));
        assert!(sys_entry.operations.contains("getValue"));
        assert!(sys_entry.domain_vars.contains_key("counter"));

        // Check states
        assert!(arc.resolve_state("TestSystem", "Idle").is_some());
        assert!(arc.resolve_state("TestSystem", "Running").is_some());
        assert!(arc.resolve_state("TestSystem", "NonExistent").is_none());

        // Check validation methods work
        assert!(arc.validate_transition("TestSystem", "Idle").is_ok());
        assert!(arc.validate_transition("TestSystem", "Running").is_ok());
        assert!(arc.validate_transition("TestSystem", "NonExistent").is_err());
    }

    // ========================================================================
    // Phase 7: Enhanced Arcanum Tests
    // ========================================================================

    #[test]
    fn test_enhanced_state_with_handlers() {
        use super::super::frame_ast::{
            FrameAst, SystemAst as FrameSystemAst, MachineAst, StateAst,
            HandlerAst, HandlerBody, EventParam,
            Span as FrameSpan, Type,
        };

        // Create system with state that has handlers
        let mut system = FrameSystemAst::new("TestSystem".to_string(), FrameSpan::new(0, 200));

        // Create state with handlers
        let mut state = StateAst::new("Active".to_string(), FrameSpan::new(50, 150));

        // Add handler with parameters
        state.handlers.push(HandlerAst {
            event: "tick".to_string(),
            params: vec![
                EventParam {
                    name: "delta".to_string(),
                    param_type: Type::Int,
                    span: FrameSpan::new(60, 70),
                },
            ],
            body: HandlerBody {
                statements: vec![],
                span: FrameSpan::new(70, 100),
            },
            span: FrameSpan::new(55, 100),
        });

        // Add another handler
        state.handlers.push(HandlerAst {
            event: "reset".to_string(),
            params: vec![],
            body: HandlerBody {
                statements: vec![],
                span: FrameSpan::new(110, 140),
            },
            span: FrameSpan::new(105, 140),
        });

        system.machine = Some(MachineAst {
            states: vec![state],
            span: FrameSpan::new(50, 150),
        });

        // Build arcanum
        let ast = FrameAst::System(system);
        let arc = super::super::arcanum::build_arcanum_from_frame_ast(&ast);

        // Test get_state_handlers
        let handlers = arc.get_state_handlers("TestSystem", "Active");
        assert_eq!(handlers.len(), 2);

        // Find tick handler and verify params
        let tick_handler = handlers.iter().find(|h| h.event == "tick");
        assert!(tick_handler.is_some());
        let tick_handler = tick_handler.unwrap();
        assert_eq!(tick_handler.params.len(), 1);
        assert_eq!(tick_handler.params[0].name, "delta");
        assert_eq!(tick_handler.params[0].kind, FrameSymbolKind::HandlerParam);

        // Find reset handler
        let reset_handler = handlers.iter().find(|h| h.event == "reset");
        assert!(reset_handler.is_some());
        assert_eq!(reset_handler.unwrap().params.len(), 0);
    }

    #[test]
    fn test_scope_resolution_handler_params() {
        use super::super::frame_ast::{
            FrameAst, SystemAst as FrameSystemAst, MachineAst, StateAst,
            HandlerAst, HandlerBody, EventParam, DomainVar, StateParam,
            Span as FrameSpan, Type,
        };

        // Create system with domain var, state param, and handler param
        let mut system = FrameSystemAst::new("ScopeTest".to_string(), FrameSpan::new(0, 300));

        // Add domain variable
        system.domain.push(DomainVar {
            name: "counter".to_string(),
            var_type: Type::Int,
            initializer: None,
            is_frame: true,
            span: FrameSpan::new(10, 30),
        });

        // Create state with parameter
        let mut state = StateAst::new("Running".to_string(), FrameSpan::new(50, 200));
        state.params.push(StateParam {
            name: "timeout".to_string(),
            param_type: Type::Int,
            span: FrameSpan::new(55, 65),
        });

        // Add handler with parameter
        state.handlers.push(HandlerAst {
            event: "process".to_string(),
            params: vec![
                EventParam {
                    name: "data".to_string(),
                    param_type: Type::String,
                    span: FrameSpan::new(80, 90),
                },
            ],
            body: HandlerBody {
                statements: vec![],
                span: FrameSpan::new(90, 150),
            },
            span: FrameSpan::new(70, 150),
        });

        system.machine = Some(MachineAst {
            states: vec![state],
            span: FrameSpan::new(50, 200),
        });

        let ast = FrameAst::System(system);
        let arc = super::super::arcanum::build_arcanum_from_frame_ast(&ast);

        // Test: Handler param is accessible in handler scope
        assert!(arc.is_frame_symbol("ScopeTest", Some("Running"), Some("process"), "data"));

        // Test: State param is accessible in handler scope
        assert!(arc.is_frame_symbol("ScopeTest", Some("Running"), Some("process"), "timeout"));

        // Test: Domain var is accessible in handler scope
        assert!(arc.is_frame_symbol("ScopeTest", Some("Running"), Some("process"), "counter"));

        // Test: State param is accessible in state scope (no handler)
        assert!(arc.is_frame_symbol("ScopeTest", Some("Running"), None, "timeout"));

        // Test: Domain var is accessible in system scope
        assert!(arc.is_frame_symbol("ScopeTest", None, None, "counter"));

        // Test: Unknown name returns false (might be native)
        assert!(!arc.is_frame_symbol("ScopeTest", Some("Running"), Some("process"), "unknown_var"));
    }

    #[test]
    fn test_scope_resolution_shadowing() {
        use super::super::frame_ast::{
            FrameAst, SystemAst as FrameSystemAst, MachineAst, StateAst,
            HandlerAst, HandlerBody, EventParam, StateParam,
            Span as FrameSpan, Type,
        };

        // Create system where handler param shadows state param with same name
        let mut system = FrameSystemAst::new("ShadowTest".to_string(), FrameSpan::new(0, 300));

        let mut state = StateAst::new("S".to_string(), FrameSpan::new(50, 200));

        // State param named "x" with type Int
        state.params.push(StateParam {
            name: "x".to_string(),
            param_type: Type::Int,
            span: FrameSpan::new(55, 60),
        });

        // Handler param also named "x" with type String
        state.handlers.push(HandlerAst {
            event: "event".to_string(),
            params: vec![
                EventParam {
                    name: "x".to_string(),
                    param_type: Type::String,
                    span: FrameSpan::new(80, 85),
                },
            ],
            body: HandlerBody {
                statements: vec![],
                span: FrameSpan::new(90, 150),
            },
            span: FrameSpan::new(70, 150),
        });

        system.machine = Some(MachineAst {
            states: vec![state],
            span: FrameSpan::new(50, 200),
        });

        let ast = FrameAst::System(system);
        let arc = super::super::arcanum::build_arcanum_from_frame_ast(&ast);

        // Test: In handler scope, "x" resolves to handler param (shadowing)
        let sym = arc.resolve_frame_symbol("ShadowTest", Some("S"), Some("event"), "x");
        assert!(sym.is_some());
        let sym = sym.unwrap();
        assert_eq!(sym.kind, FrameSymbolKind::HandlerParam);

        // Test: In state scope (no handler), "x" resolves to state param
        let sym = arc.resolve_frame_symbol("ShadowTest", Some("S"), None, "x");
        assert!(sym.is_some());
        let sym = sym.unwrap();
        assert_eq!(sym.kind, FrameSymbolKind::StateParam);
    }

    #[test]
    fn test_get_enhanced_states() {
        use super::super::frame_ast::{
            FrameAst, SystemAst as FrameSystemAst, MachineAst, StateAst,
            Span as FrameSpan,
        };

        let mut system = FrameSystemAst::new("MultiState".to_string(), FrameSpan::new(0, 300));

        system.machine = Some(MachineAst {
            states: vec![
                StateAst::new("Idle".to_string(), FrameSpan::new(50, 80)),
                StateAst::new("Running".to_string(), FrameSpan::new(80, 120)),
                StateAst::new("Stopped".to_string(), FrameSpan::new(120, 150)),
            ],
            span: FrameSpan::new(50, 150),
        });

        let ast = FrameAst::System(system);
        let arc = super::super::arcanum::build_arcanum_from_frame_ast(&ast);

        // Test get_enhanced_states returns all states
        let states = arc.get_enhanced_states("MultiState");
        assert_eq!(states.len(), 3);

        // Test get_enhanced_state for specific state
        let idle = arc.get_enhanced_state("MultiState", "Idle");
        assert!(idle.is_some());
        assert_eq!(idle.unwrap().name, "Idle");

        // Test non-existent state
        let none = arc.get_enhanced_state("MultiState", "NonExistent");
        assert!(none.is_none());

        // Test non-existent system
        let empty = arc.get_enhanced_states("NonExistent");
        assert!(empty.is_empty());
    }

    #[test]
    fn test_enter_exit_handlers() {
        use super::super::frame_ast::{
            FrameAst, SystemAst as FrameSystemAst, MachineAst, StateAst,
            EnterHandler, ExitHandler, HandlerBody, EventParam,
            Span as FrameSpan, Type,
        };

        let mut system = FrameSystemAst::new("LifecycleTest".to_string(), FrameSpan::new(0, 300));

        let mut state = StateAst::new("Active".to_string(), FrameSpan::new(50, 200));

        // Add enter handler with param
        state.enter = Some(EnterHandler {
            params: vec![
                EventParam {
                    name: "config".to_string(),
                    param_type: Type::String,
                    span: FrameSpan::new(60, 70),
                },
            ],
            body: HandlerBody {
                statements: vec![],
                span: FrameSpan::new(75, 100),
            },
            span: FrameSpan::new(55, 100),
        });

        // Add exit handler
        state.exit = Some(ExitHandler {
            body: HandlerBody {
                statements: vec![],
                span: FrameSpan::new(110, 140),
            },
            span: FrameSpan::new(105, 140),
        });

        system.machine = Some(MachineAst {
            states: vec![state],
            span: FrameSpan::new(50, 200),
        });

        let ast = FrameAst::System(system);
        let arc = super::super::arcanum::build_arcanum_from_frame_ast(&ast);

        // Test: Enter handler is tracked
        let handlers = arc.get_state_handlers("LifecycleTest", "Active");
        let enter = handlers.iter().find(|h| h.event == "$>");
        assert!(enter.is_some());
        let enter = enter.unwrap();
        assert!(enter.is_enter);
        assert!(!enter.is_exit);
        assert_eq!(enter.params.len(), 1);
        assert_eq!(enter.params[0].name, "config");

        // Test: Exit handler is tracked
        let exit = handlers.iter().find(|h| h.event == "$<");
        assert!(exit.is_some());
        let exit = exit.unwrap();
        assert!(!exit.is_enter);
        assert!(exit.is_exit);
        assert_eq!(exit.params.len(), 0);  // Exit handlers don't have params

        // Test: Enter param is accessible via scope resolution
        assert!(arc.is_frame_symbol("LifecycleTest", Some("Active"), Some("$>"), "config"));
    }

    #[test]
    fn test_state_parent_tracking() {
        use super::super::frame_ast::{
            FrameAst, SystemAst as FrameSystemAst, MachineAst, StateAst,
            Span as FrameSpan,
        };

        let mut system = FrameSystemAst::new("HSMTest".to_string(), FrameSpan::new(0, 300));

        let parent_state = StateAst::new("Base".to_string(), FrameSpan::new(50, 80));

        let mut child_state = StateAst::new("Child".to_string(), FrameSpan::new(80, 120));
        child_state.parent = Some("Base".to_string());

        system.machine = Some(MachineAst {
            states: vec![parent_state, child_state],
            span: FrameSpan::new(50, 120),
        });

        let ast = FrameAst::System(system);
        let arc = super::super::arcanum::build_arcanum_from_frame_ast(&ast);

        // Test: Parent is tracked in enhanced state
        let child = arc.get_enhanced_state("HSMTest", "Child");
        assert!(child.is_some());
        assert_eq!(child.unwrap().parent, Some("Base".to_string()));

        // Test: Base has no parent
        let base = arc.get_enhanced_state("HSMTest", "Base");
        assert!(base.is_some());
        assert_eq!(base.unwrap().parent, None);

        // Test: Original has_parent API still works
        assert!(arc.has_parent("HSMTest", "Child"));
        assert!(!arc.has_parent("HSMTest", "Base"));
    }
}