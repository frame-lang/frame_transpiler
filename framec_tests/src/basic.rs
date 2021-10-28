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

    #[test]
    fn intial_state_entry_call() {
        let sm = Basic::new();
        assert_eq!(sm.entry_log, vec!["S0"]);
    }

    #[test]
    fn non_initial_state_entry_calls() {
        let mut sm = Basic::new();
        sm.entry_log.clear();
        sm.a();
        sm.b();
        assert_eq!(sm.entry_log, vec!["S1", "S0"]);
    }

    #[test]
    fn exit_calls() {
        let mut sm = Basic::new();
        sm.a();
        sm.b();
        assert_eq!(sm.exit_log, vec!["S0", "S1"]);
    }

    #[test]
    fn current_state() {
        let mut sm = Basic::new();
        assert_eq!(sm.state, BasicState::S0);
        sm.a();
        assert_eq!(sm.state, BasicState::S1);
        sm.b();
        assert_eq!(sm.state, BasicState::S0);
    }

    #[test]
    /// Test that we can access the current state via the runtime interface.
    fn runtime_current_state() {
        let mut sm = Basic::new();
        assert_eq!(sm.state().info().name(), "S0");
        sm.a();
        assert_eq!(sm.state().info().name(), "S1");
        sm.b();
        assert_eq!(sm.state().info().name(), "S0");
    }

    #[test]
    /// Test that we can access the domain variables via the runtime interface.
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
