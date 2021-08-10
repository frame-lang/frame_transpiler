type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "hierarchical_guard.rs"));

impl HierarchicalGuard {
    pub fn entered(&mut self, state: String) {
        self.entry_log.push(state);
    }
    pub fn left(&mut self, state: String) {
        self.exit_log.push(state);
    }
    pub fn transition_hook(
        &mut self,
        _current: HierarchicalGuardState,
        _next: HierarchicalGuardState,
    ) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hierarchical_child_transition_handler_with_guard() {
        let mut sm = HierarchicalGuard::new();
        sm.A();
        sm.B(true);
        assert_eq!(sm.get_current_state_enum(), HierarchicalGuardState::S1);
    }

    // Revisit: parent handler for event isn't getting called if child has handler
    // regardless of wether it results on transition or not
    #[test]
    #[ignore]
    fn hierarchical_parent_transition_handler_with_guard() {
        let mut sm = HierarchicalGuard::new();
        sm.A();
        sm.B(false);
        assert_eq!(sm.get_current_state_enum(), HierarchicalGuardState::I);
    }
}
