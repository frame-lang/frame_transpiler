type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "basic.rs"));

impl<'a> Basic<'a> {
    pub fn entered(&mut self, state: String) {
        self.entry_log.push(state);
    }
    pub fn left(&mut self, state: String) {
        self.exit_log.push(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that the enter event is sent for entering the initial state on startup.
    #[test]
    fn intial_enter_event() {
        let sm = Basic::new();
        assert_eq!(sm.entry_log, vec!["S0"]);
    }

    /// Test that enter events are sent to the new state on transition.
    #[test]
    fn transition_enter_events() {
        let mut sm = Basic::new();
        sm.entry_log.clear();
        sm.a();
        sm.b();
        assert_eq!(sm.entry_log, vec!["S1", "S0"]);
    }

    /// Test that exit events are sent to the old state on transition.
    #[test]
    fn transition_exit_events() {
        let mut sm = Basic::new();
        sm.a();
        sm.b();
        assert_eq!(sm.exit_log, vec!["S0", "S1"]);
    }

    /// Test that the state of the machine is updated correctly.
    #[test]
    fn current_state() {
        let mut sm = Basic::new();
        assert_eq!(sm.state, BasicState::S0);
        sm.a();
        assert_eq!(sm.state, BasicState::S1);
        sm.b();
        assert_eq!(sm.state, BasicState::S0);
    }

    /// Test that the machine name from the runtime interface is correct.
    #[test]
    fn machine_name() {
        let info = Basic::machine_info();
        assert_eq!(info.name(), "Basic");
    }

    /// Test that the state names from the runtime interface are correct.
    #[test]
    fn state_names() {
        let info = Basic::machine_info();
        let states = info.states();
        assert_eq!(states.len(), 2);
        assert!(states.iter().any(|s| s.name() == "S0"));
        assert!(states.iter().any(|s| s.name() == "S1"));
    }

    /// Test that the interface names from the runtime interface are correct.
    #[test]
    fn interface_names() {
        let info = Basic::machine_info();
        let methods = info.interface();
        assert_eq!(methods.len(), 2);
        assert!(methods.iter().any(|m| m.name == "A"));
        assert!(methods.iter().any(|m| m.name == "B"));
    }

    /// Test that the event names from the runtime interface are correct.
    #[test]
    fn event_names() {
        let info = Basic::machine_info();
        let methods = info.events();
        assert_eq!(methods.len(), 6);
        assert!(methods.iter().any(|m| m.name == "A"));
        assert!(methods.iter().any(|m| m.name == "B"));
        assert!(methods.iter().any(|m| m.name == "S0:>"));
        assert!(methods.iter().any(|m| m.name == "S0:<"));
        assert!(methods.iter().any(|m| m.name == "S1:>"));
        assert!(methods.iter().any(|m| m.name == "S1:<"));
        assert!(!methods.iter().any(|m| m.name == ">"));
        assert!(!methods.iter().any(|m| m.name == "<"));
    }

    /// Test that the action names from the runtime interface are correct.
    #[test]
    fn action_names() {
        let info = Basic::machine_info();
        let methods = info.actions();
        assert_eq!(methods.len(), 2);
        assert!(methods.iter().any(|m| m.name == "entered"));
        assert!(methods.iter().any(|m| m.name == "left"));
    }

    /// Test that the action signatures from the runtime interface are correct.
    #[test]
    fn action_signatures() {
        let info = Basic::machine_info();
        let entered = info.get_action("entered").unwrap();
        let left = info.get_action("left").unwrap();

        assert_eq!(entered.name, "entered");
        assert_eq!(entered.parameters.len(), 1);
        assert_eq!(entered.parameters[0].name, "msg");
        assert_eq!(entered.parameters[0].vtype, "&String");
        assert!(entered.return_type.is_none());

        assert_eq!(left.name, "left");
        assert_eq!(left.parameters.len(), 1);
        assert_eq!(left.parameters[0].name, "msg");
        assert_eq!(left.parameters[0].vtype, "&String");
        assert!(left.return_type.is_none());
    }

    /// Test that the domain variable names from the runtime interface are correct.
    #[test]
    fn domain_variable_names() {
        let info = Basic::machine_info();
        let names = info.variables();
        assert_eq!(names.len(), 2);
        assert!(names.iter().any(|n| n.name == "entry_log"));
        assert!(names.iter().any(|n| n.name == "exit_log"));
    }

    /// Test that the domain variable types from the runtime interface are correct.
    #[test]
    fn domain_variable_types() {
        let info = Basic::machine_info();
        let entry_log = info.get_variable("entry_log").unwrap();
        let exit_log = info.get_variable("exit_log").unwrap();
        assert_eq!(entry_log.vtype, "Log");
        assert_eq!(exit_log.vtype, "Log");
    }

    /// Test that transitions obtained from the runtime interface are correct.
    #[test]
    fn state_transitions() {
        let info = Basic::machine_info();
        let s0 = info.get_state("S0").unwrap();
        let s1 = info.get_state("S1").unwrap();

        assert_eq!(s0.incoming_transitions().len(), 1);
        assert_eq!(s0.outgoing_transitions().len(), 1);
        assert_eq!(s1.incoming_transitions().len(), 1);
        assert_eq!(s1.outgoing_transitions().len(), 1);

        let s0_in = &s0.incoming_transitions()[0];
        let s0_out = &s0.outgoing_transitions()[0];
        let s1_in = &s1.incoming_transitions()[0];
        let s1_out = &s1.outgoing_transitions()[0];

        assert!(s0_in.is_transition());
        assert_eq!(s0_in.label, "aah");
        assert_eq!(s0_in.source.name(), "S1");
        assert_eq!(s0_in.target.name(), "S0");
        assert_eq!(s0_in.event.name, "B");

        assert!(s0_out.is_transition());
        assert_eq!(s0_out.label, "ooh");
        assert_eq!(s0_out.source.name(), "S0");
        assert_eq!(s0_out.target.name(), "S1");
        assert_eq!(s0_out.event.name, "A");

        assert!(s1_in.is_transition());
        assert_eq!(s1_in.label, "ooh");
        assert_eq!(s1_in.source.name(), "S0");
        assert_eq!(s1_in.target.name(), "S1");
        assert_eq!(s1_in.event.name, "A");

        assert!(s1_out.is_transition());
        assert_eq!(s1_out.label, "aah");
        assert_eq!(s1_out.source.name(), "S1");
        assert_eq!(s1_out.target.name(), "S0");
        assert_eq!(s1_out.event.name, "B");
    }

    /// Test that we can access the current state via the runtime interface.
    #[test]
    fn runtime_current_state() {
        let mut sm = Basic::new();
        assert_eq!(sm.state().info().name(), "S0");
        sm.a();
        assert_eq!(sm.state().info().name(), "S1");
        sm.b();
        assert_eq!(sm.state().info().name(), "S0");
    }

    /// Test that we can access the values of the domain variables via the runtime interface.
    #[test]
    fn runtime_domain_variables() {
        let mut sm = Basic::new();
        sm.a();
        sm.b();
        let entry_log: &Log = sm
            .variables()
            .lookup("entry_log")
            .unwrap()
            .downcast_ref()
            .unwrap();
        let exit_log: &Log = sm
            .variables()
            .lookup("exit_log")
            .unwrap()
            .downcast_ref()
            .unwrap();
        assert_eq!(*entry_log, vec!["S0", "S1", "S0"]);
        assert_eq!(*exit_log, vec!["S0", "S1"]);
    }
}
