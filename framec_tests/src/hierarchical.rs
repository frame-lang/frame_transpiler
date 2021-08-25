//! Test hierarchical event handling and state transitions.

type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "hierarchical.rs"));

impl Hierarchical {
    pub fn enter(&mut self, msg: String) {
        self.enters.push(msg);
    }
    pub fn exit(&mut self, msg: String) {
        self.exits.push(msg);
    }
    pub fn log(&mut self, msg: String) {
        self.tape.push(msg);
    }
    pub fn transition_hook(&mut self, _current: HierarchicalState, _next: HierarchicalState) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test that a continue (`:>`) in a child enter handler calls the parent
    /// enter handler.
    fn enter_continue() {
        let mut sm = Hierarchical::new();
        sm.enters.clear();
        sm.a();
        assert_eq!(sm.enters, vec!["S0", "S"]);
        sm.enters.clear();
        sm.c();
        assert_eq!(sm.enters, vec!["S2", "S0", "S"]);
    }

    #[test]
    /// Test that a continue (`:>`) in a child exit handler calls the parent
    /// exit handler.
    fn exit_continue() {
        let mut sm = Hierarchical::new();
        sm.a();
        sm.exits.clear();
        sm.c();
        assert_eq!(sm.exits, vec!["S0", "S"]);
        sm.exits.clear();
        sm.a();
        assert_eq!(sm.exits, vec!["S2", "S0", "S"]);
    }

    #[test]
    /// Test that a return (`^`) in a child enter handler *does not* call the
    /// parent enter handler.
    fn enter_return() {
        let mut sm = Hierarchical::new();
        sm.enters.clear();
        sm.b();
        assert_eq!(sm.enters, vec!["S1"]);
        sm = Hierarchical::new();
        sm.a();
        sm.a();
        assert_eq!(sm.state, HierarchicalState::T);
        sm.enters.clear();
        sm.c();
        assert_eq!(sm.enters, vec!["S3", "S1"]);
    }

    #[test]
    /// Test that a return (`^`) in a child exit handler *does not* call the
    /// parent exit handler.
    fn exit_return() {
        let mut sm = Hierarchical::new();
        sm.b();
        assert_eq!(sm.state, HierarchicalState::S1);
        sm.exits.clear();
        sm.a();
        assert_eq!(sm.exits, vec!["S1"]);
        sm = Hierarchical::new();
        sm.a();
        sm.a();
        sm.c();
        assert_eq!(sm.state, HierarchicalState::S3);
        sm.exits.clear();
        sm.b();
        assert_eq!(sm.exits, vec!["S3", "S1"]);
    }

    #[test]
    /// Test that location in a hierarchical state is represented correctly.
    /// In this test, all state transitions are performed by the immediately
    /// matching handler.
    fn current_state_simple() {
        let mut sm = Hierarchical::new();
        assert_eq!(sm.state, HierarchicalState::S);
        sm.a();
        assert_eq!(sm.state, HierarchicalState::S0);
        sm.a();
        assert_eq!(sm.state, HierarchicalState::T);
        sm.c();
        assert_eq!(sm.state, HierarchicalState::S3);
        sm.b();
        assert_eq!(sm.state, HierarchicalState::S2);
    }

    #[test]
    /// Test that location in a hierarchical state is represented correctly.
    /// In this test, several state transitions propagate message handling to
    /// parents, either by implicit fall-through or explicit continues.
    fn current_state_with_propagation() {
        let mut sm = Hierarchical::new();
        assert_eq!(sm.state, HierarchicalState::S);
        sm.a();
        assert_eq!(sm.state, HierarchicalState::S0);
        sm.b();
        assert_eq!(sm.state, HierarchicalState::S1);
        sm.b();
        assert_eq!(sm.state, HierarchicalState::S1);
        sm.c();
        assert_eq!(sm.state, HierarchicalState::S1);
        sm.a();
        assert_eq!(sm.state, HierarchicalState::S0);
        sm.c();
        assert_eq!(sm.state, HierarchicalState::S2);
        sm.b();
        assert_eq!(sm.state, HierarchicalState::S1);
    }

    #[test]
    /// Test that a handler in a child overrides the parent handler if the
    /// child handler ends with a return.
    fn override_parent_handler() {
        let mut sm = Hierarchical::new();
        sm.a();
        sm.tape.clear();
        sm.a();
        assert_eq!(sm.state, HierarchicalState::T);
        assert_eq!(sm.tape, vec!["S0.A"]);
        sm.c();
        sm.tape.clear();
        sm.b();
        assert_eq!(sm.state, HierarchicalState::S2);
        assert_eq!(sm.tape, vec!["S3.B"]);
    }

    #[test]
    /// Test that a handler in a child propagates control to the parent
    /// handler if the child handler ends with a continue.
    fn before_parent_handler() {
        let mut sm = Hierarchical::new();
        sm.a();
        sm.tape.clear();
        sm.b();
        assert_eq!(sm.state, HierarchicalState::S1);
        assert_eq!(sm.tape, vec!["S0.B", "S.B"]);
        sm.tape.clear();
        sm.exits.clear();
        sm.enters.clear();
        sm.b();
        assert_eq!(sm.state, HierarchicalState::S1);
        assert_eq!(sm.tape, vec!["S1.B", "S.B"]);
        assert_eq!(sm.exits, vec!["S1"]);
        assert_eq!(sm.enters, vec!["S1"]);
        sm = Hierarchical::new();
        sm.a();
        sm.c();
        assert_eq!(sm.state, HierarchicalState::S2);
        sm.tape.clear();
        sm.exits.clear();
        sm.enters.clear();
        sm.b();
        assert_eq!(sm.state, HierarchicalState::S1);
        assert_eq!(sm.tape, vec!["S2.B", "S0.B", "S.B"]);
        assert_eq!(sm.exits, vec!["S2", "S0", "S"]);
        assert_eq!(sm.enters, vec!["S1"]);
    }

    #[test]
    /// Test that missing event handlers in children automatically propagate
    /// to parents.
    fn defer_to_parent_handler() {
        let mut sm = Hierarchical::new();
        sm.b();
        assert_eq!(sm.state, HierarchicalState::S1);
        sm.tape.clear();
        sm.a();
        assert_eq!(sm.state, HierarchicalState::S0);
        assert_eq!(sm.tape, vec!["S.A"]);
        sm.a();
        sm.c();
        assert_eq!(sm.state, HierarchicalState::S3);
        sm.tape.clear();
        sm.a();
        assert_eq!(sm.state, HierarchicalState::S0);
        assert_eq!(sm.tape, vec!["S.A"]);
    }

    #[test]
    /// Test that propagating control to a parent handler that doesn't handle
    /// the current message is a no-op.
    fn before_missing_handler() {
        let mut sm = Hierarchical::new();
        sm.b();
        assert_eq!(sm.state, HierarchicalState::S1);
        sm.tape.clear();
        sm.exits.clear();
        sm.enters.clear();
        sm.c();
        assert_eq!(sm.state, HierarchicalState::S1);
        assert_eq!(sm.tape, vec!["S1.C"]);
        assert!(sm.exits.is_empty());
        assert!(sm.enters.is_empty());
    }

    #[test]
    /// Test that a continue after a transition statement is ignored.
    fn continue_after_transition_ignored() {
        let mut sm = Hierarchical::new();
        sm.a();
        sm.c();
        assert_eq!(sm.state, HierarchicalState::S2);
        sm.enters.clear();
        sm.tape.clear();
        sm.c();
        assert_eq!(sm.state, HierarchicalState::T); // not S2
        assert_eq!(sm.enters, vec!["T"]);
        assert_eq!(sm.tape, vec!["S2.C"]);
    }
}
