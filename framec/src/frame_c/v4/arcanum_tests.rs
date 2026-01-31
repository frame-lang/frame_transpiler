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
}