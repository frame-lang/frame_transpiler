type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "basic.rs"));

impl Basic {
    pub fn entered(&mut self, state: String) {
        self.entry_log.push(state);
    }
    pub fn left(&mut self, state: String) {
        self.exit_log.push(state);
    }
    pub fn transition_hook(&mut self, _current: BasicState, _next: BasicState) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    // Revisit: currently generated code doesn't send entry event to state machine's initial
    // state on creation
    #[test]
    #[ignore]
    fn intial_state_entry_call() {
        let sm = Basic::new();
        assert_eq!(sm.entry_log, vec!["S1"]);
    }

    #[test]
    fn non_initial_state_entry_calls() {
        let mut sm = Basic::new();
        sm.entry_log.clear();
        sm.A();
        sm.B();
        assert_eq!(sm.entry_log, vec!["S1", "S0"]);
    }
    #[test]
    fn exit_calls() {
        let mut sm = Basic::new();
        sm.A();
        sm.B();
        assert_eq!(sm.exit_log, vec!["S0", "S1"]);
    }
    #[test]
    fn current_state() {
        let mut sm = Basic::new();
        assert_eq!(sm.get_current_state_enum(), BasicState::S0);
        sm.A();
        assert_eq!(sm.get_current_state_enum(), BasicState::S1);
        sm.B();
        assert_eq!(sm.get_current_state_enum(), BasicState::S0);
    }
}
