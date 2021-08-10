type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "hierarchical.rs"));

impl Hierarchical {
    pub fn entered(&mut self, state: String) {
        self.entry_log.push(state);
    }
    pub fn left(&mut self, state: String) {
        self.exit_log.push(state);
    }
    pub fn transition_hook(&mut self, _current: HierarchicalState, _next: HierarchicalState) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    // Parent entry event handlers are currently not being called on child state entry
    #[test]
    #[ignore]
    fn hierarchical_entry_calls() {
        let mut sm = Hierarchical::new();
        sm.entry_log.clear();
        sm.A();
        assert_eq!(sm.entry_log, vec!["S", "S0"]);
    }

    // Parent exit event handlers are currently not being called on exit from child state
    #[test]
    #[ignore]
    fn hierarchical_exit_calls() {
        let mut sm = Hierarchical::new();
        sm.A();
        sm.exit_log.clear();
        sm.C();
        assert_eq!(sm.exit_log, vec!["S", "S0"]);
    }

    #[test]
    fn hierarchical_current_state() {
        let mut sm = Hierarchical::new();
        assert_eq!(sm.get_current_state_enum(), HierarchicalState::I);
        sm.A();
        assert_eq!(sm.get_current_state_enum(), HierarchicalState::S0);
        sm.B();
        assert_eq!(sm.get_current_state_enum(), HierarchicalState::S1);
        sm.C();
        assert_eq!(sm.get_current_state_enum(), HierarchicalState::I);
    }
}
