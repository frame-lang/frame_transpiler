//! Frame supports two different operations for changing the current state of the machine:
//! "change-state" (`->>`) which simply changes to the new state, and "transition" (`->`), which
//! also sends an exit event to the old state and an enter event to the new state.
//!
//! This file tests that these operations work correctly. It also tests that the optional hook
//! methods for each operation are invoked when states are changed, and that transition callbacks
//! registered via the runtime system are invoked.

type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "transition.rs"));

#[allow(dead_code)]
impl<'a> TransitionSm<'a> {
    pub fn enter(&mut self, state: String) {
        self.enters.push(state);
    }
    pub fn exit(&mut self, state: String) {
        self.exits.push(state);
    }
    pub fn clear_all(&mut self) {
        self.enters.clear();
        self.exits.clear();
        self.hooks.clear();
    }
    pub fn transition_hook(&mut self, old_state: TransitionSmState, new_state: TransitionSmState) {
        let s = format!("{:?}->{:?}", old_state, new_state);
        self.hooks.push(s);
    }
    pub fn change_state_hook(
        &mut self,
        old_state: TransitionSmState,
        new_state: TransitionSmState,
    ) {
        let s = format!("{:?}->>{:?}", old_state, new_state);
        self.hooks.push(s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_runtime::unsync::*;
    use std::sync::Mutex;

    /// Test that transition works and triggers enter and exit events.
    #[test]
    fn transition_events() {
        let mut sm = TransitionSm::new();
        sm.clear_all();
        sm.transit();
        assert_eq!(sm.state, TransitionSmState::S1);
        assert_eq!(sm.exits, vec!["S0"]);
        assert_eq!(sm.enters, vec!["S1"]);
    }

    /// Test that change-state works and does not trigger events.
    #[test]
    fn change_state_no_events() {
        let mut sm = TransitionSm::new();
        sm.clear_all();
        sm.change();
        assert_eq!(sm.state, TransitionSmState::S1);
        sm.change();
        assert_eq!(sm.state, TransitionSmState::S2);
        sm.change();
        assert_eq!(sm.state, TransitionSmState::S3);
        sm.change();
        assert_eq!(sm.state, TransitionSmState::S4);
        assert!(sm.exits.is_empty());
        assert!(sm.enters.is_empty());
    }

    /// Test transition that triggers another transition in an enter event handler.
    #[test]
    fn cascading_transition() {
        let mut sm = TransitionSm::new();
        sm.change();
        sm.clear_all();
        assert_eq!(sm.state, TransitionSmState::S1);
        sm.transit();
        assert_eq!(sm.state, TransitionSmState::S3);
        assert_eq!(sm.exits, vec!["S1", "S2"]);
        assert_eq!(sm.enters, vec!["S2", "S3"]);
    }

    /// Test transition that triggers a change-state from an enter event handler.
    #[test]
    fn cascading_change_state() {
        let mut sm = TransitionSm::new();
        sm.change();
        sm.change();
        sm.change();
        sm.clear_all();
        assert_eq!(sm.state, TransitionSmState::S3);
        sm.transit();
        assert_eq!(sm.state, TransitionSmState::S0);
        assert_eq!(sm.exits, vec!["S3"]);
        assert_eq!(sm.enters, vec!["S4"]);
    }

    /// Test that the names of old/new state instances match the names of expected states in the
    /// static transition info.
    #[test]
    fn consistent_transition_event() {
        let mut sm = TransitionSm::new();
        sm.event_monitor_mut()
            .add_transition_callback(Callback::new("test", |t: &Transition| {
                let source_name = t.info.source.name;
                let target_name = t.info.target.name;
                let old_name = t.old_state.info().name;
                let new_name = t.new_state.info().name;
                assert_eq!(source_name, old_name);
                assert_eq!(target_name, new_name);
            }));
        sm.transit();
        sm.transit();
        sm.transit();
        assert_eq!(sm.state, TransitionSmState::S0);
        sm.change();
        sm.change();
        sm.change();
        sm.change();
        assert_eq!(sm.state, TransitionSmState::S4);
    }

    /// Test transition callbacks.
    #[test]
    fn transition_callback() {
        let transits = Mutex::new(Vec::new());
        let mut sm = TransitionSm::new();
        sm.event_monitor_mut()
            .add_transition_callback(Callback::new("test", |t: &Transition| {
                transits.lock().unwrap().push(t.to_string());
            }));
        sm.transit();
        assert_eq!(*transits.lock().unwrap(), vec!["S0->S1"]);
        transits.lock().unwrap().clear();
        sm.transit();
        assert_eq!(*transits.lock().unwrap(), vec!["S1->S2", "S2->S3"]);
    }

    /// Test change-state callbacks.
    #[test]
    fn change_state_callback() {
        let transits = Mutex::new(Vec::new());
        let mut sm = TransitionSm::new();
        sm.event_monitor_mut()
            .add_transition_callback(Callback::new("test", |t: &Transition| {
                transits.lock().unwrap().push(t.to_string());
            }));
        sm.change();
        assert_eq!(*transits.lock().unwrap(), vec!["S0->>S1"]);
        transits.lock().unwrap().clear();
        sm.change();
        assert_eq!(*transits.lock().unwrap(), vec!["S1->>S2"]);
        transits.lock().unwrap().clear();
        sm.change();
        assert_eq!(*transits.lock().unwrap(), vec!["S2->>S3"]);
        transits.lock().unwrap().clear();
        sm.transit();
        assert_eq!(*transits.lock().unwrap(), vec!["S3->S4", "S4->>S0"]);
    }

    /// Test that transition IDs are correct.
    #[test]
    fn transition_ids() {
        let ids = Mutex::new(Vec::new());
        let mut sm = TransitionSm::new();
        sm.event_monitor_mut()
            .add_transition_callback(Callback::new("test", |t: &Transition| {
                ids.lock().unwrap().push(t.info.id);
            }));
        sm.transit();
        sm.transit();
        sm.transit();
        assert_eq!(*ids.lock().unwrap(), vec![0, 2, 4, 7, 9]);
        ids.lock().unwrap().clear();
        sm.change();
        sm.change();
        sm.change();
        sm.change();
        assert_eq!(*ids.lock().unwrap(), vec![1, 3, 6, 8]);
    }

    /// Test transition hook method.
    #[test]
    fn transition_hook() {
        let mut sm = TransitionSm::new();
        sm.transit();
        assert_eq!(sm.hooks, vec!["S0->S1"]);
        sm.clear_all();
        sm.transit();
        assert_eq!(sm.hooks, vec!["S1->S2", "S2->S3"]);
    }

    /// Test change-state hook method.
    #[test]
    fn change_state_hook() {
        let mut sm = TransitionSm::new();
        sm.change();
        assert_eq!(sm.hooks, vec!["S0->>S1"]);
        sm.clear_all();
        sm.change();
        assert_eq!(sm.hooks, vec!["S1->>S2"]);
        sm.clear_all();
        sm.change();
        assert_eq!(sm.hooks, vec!["S2->>S3"]);
        sm.clear_all();
        sm.transit();
        assert_eq!(sm.hooks, vec!["S3->S4", "S4->>S0"]);
    }
}
