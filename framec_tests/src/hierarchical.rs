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
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_runtime::*;

    /// Test that a continue (`:>`) in a child enter handler calls the parent enter handler.
    #[test]
    fn enter_continue() {
        let mut sm = Hierarchical::new();
        sm.enters.clear();
        sm.a();
        assert_eq!(sm.enters, vec!["S0", "S"]);
        sm.enters.clear();
        sm.c();
        assert_eq!(sm.enters, vec!["S2", "S0", "S"]);
    }

    /// Test that a continue (`:>`) in a child exit handler calls the parent exit handler.
    #[test]
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

    /// Test that a return (`^`) in a child enter handler *does not* call the parent enter handler.
    #[test]
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

    /// Test that a return (`^`) in a child exit handler *does not* call the parent exit handler.
    #[test]
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

    /// Test that location in a hierarchical state is represented correctly. In this test, all
    /// state transitions are performed by the immediately matching handler.
    #[test]
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

    /// Test that location in a hierarchical state is represented correctly. In this test, several
    /// state transitions propagate message handling to parents, either by implicit fall-through or
    /// explicit continues.
    #[test]
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

    /// Test that a handler in a child overrides the parent handler if the child handler ends with
    /// a return.
    #[test]
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

    /// Test that a handler in a child propagates control to the parent handler if the child
    /// handler ends with a continue.
    #[test]
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

    /// Test that missing event handlers in children automatically propagate to parents.
    #[test]
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

    /// Test that propagating control to a parent handler that doesn't handle the current message
    /// is a no-op.
    #[test]
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

    /// Test that a continue after a transition statement is ignored.
    #[test]
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

    /// Test that the state names from the runtime interface are correct.
    #[test]
    fn state_names() {
        let sm = Hierarchical::new();
        let states = sm.info().states;
        assert_eq!(states.len(), 7);
        assert!(states.iter().any(|s| s.name == "I"));
        assert!(states.iter().any(|s| s.name == "S"));
        assert!(states.iter().any(|s| s.name == "S0"));
        assert!(states.iter().any(|s| s.name == "S1"));
        assert!(states.iter().any(|s| s.name == "S2"));
        assert!(states.iter().any(|s| s.name == "S3"));
        assert!(states.iter().any(|s| s.name == "T"));
        assert!(!states.iter().any(|s| s.name == "A"));
    }

    /// Test that the initial state from the runtime interface is correct.
    #[test]
    fn initial_state() {
        let info = Hierarchical::machine_info();
        let init = info.initial_state();
        assert!(init.is_some());
        assert_eq!(init.unwrap().name, "I");
    }

    /// Test that the top-level states from the runtime interface is correct.
    #[test]
    fn top_level_states() {
        let info = Hierarchical::machine_info();
        let states = info.top_level_states();
        assert_eq!(states.len(), 3);
        assert!(states.iter().any(|s| s.name == "I"));
        assert!(states.iter().any(|s| s.name == "S"));
        assert!(states.iter().any(|s| s.name == "T"));
    }

    /// Test that states have the right parents via the runtime interface.
    #[test]
    fn state_parents() {
        let info = Hierarchical::machine_info();
        let i = info.get_state("I").unwrap();
        let s = info.get_state("S").unwrap();
        let s0 = info.get_state("S0").unwrap();
        let s1 = info.get_state("S1").unwrap();
        let s2 = info.get_state("S2").unwrap();
        let s3 = info.get_state("S3").unwrap();
        let t = info.get_state("T").unwrap();
        assert!(i.parent.is_none());
        assert!(s.parent.is_none());
        assert!(s0.parent.is_some());
        assert!(s1.parent.is_some());
        assert!(s2.parent.is_some());
        assert!(s3.parent.is_some());
        assert!(t.parent.is_none());
        assert_eq!(s0.parent.unwrap().name, "S");
        assert_eq!(s1.parent.unwrap().name, "S");
        assert_eq!(s2.parent.unwrap().name, "S0");
        assert_eq!(s3.parent.unwrap().name, "S1");
    }

    /// Test that states have the right ancestors via the runtime interface.
    #[test]
    fn state_ancestors() {
        let info = Hierarchical::machine_info();
        let i = info.get_state("I").unwrap();
        let s = info.get_state("S").unwrap();
        let s0 = info.get_state("S0").unwrap();
        let s1 = info.get_state("S1").unwrap();
        let s2 = info.get_state("S2").unwrap();
        let s3 = info.get_state("S3").unwrap();
        let t = info.get_state("T").unwrap();
        assert_eq!(i.ancestors().len(), 0);
        assert_eq!(s.ancestors().len(), 0);
        assert_eq!(s0.ancestors().len(), 1);
        assert_eq!(s1.ancestors().len(), 1);
        assert_eq!(s2.ancestors().len(), 2);
        assert_eq!(s3.ancestors().len(), 2);
        assert_eq!(t.ancestors().len(), 0);
        assert_eq!(
            s0.ancestors().iter().map(|c| c.name).collect::<Vec<&str>>(),
            vec!["S"]
        );
        assert_eq!(
            s1.ancestors().iter().map(|c| c.name).collect::<Vec<&str>>(),
            vec!["S"]
        );
        assert_eq!(
            s2.ancestors().iter().map(|c| c.name).collect::<Vec<&str>>(),
            vec!["S0", "S"]
        );
        assert_eq!(
            s3.ancestors().iter().map(|c| c.name).collect::<Vec<&str>>(),
            vec!["S1", "S"]
        );
    }

    /// Test that states have the right children via the runtime interface.
    #[test]
    fn state_children() {
        let info = Hierarchical::machine_info();
        let i = info.get_state("I").unwrap();
        let s = info.get_state("S").unwrap();
        let s0 = info.get_state("S0").unwrap();
        let s1 = info.get_state("S1").unwrap();
        let s2 = info.get_state("S2").unwrap();
        let s3 = info.get_state("S3").unwrap();
        let t = info.get_state("T").unwrap();
        assert_eq!(i.children().len(), 0);
        assert_eq!(s.children().len(), 2);
        assert_eq!(s0.children().len(), 1);
        assert_eq!(s1.children().len(), 1);
        assert_eq!(s2.children().len(), 0);
        assert_eq!(s3.children().len(), 0);
        assert_eq!(t.children().len(), 0);
        assert_eq!(
            s.children().iter().map(|c| c.name).collect::<Vec<&str>>(),
            vec!["S0", "S1"]
        );
        assert_eq!(
            s0.children().iter().map(|c| c.name).collect::<Vec<&str>>(),
            vec!["S2"]
        );
        assert_eq!(
            s1.children().iter().map(|c| c.name).collect::<Vec<&str>>(),
            vec!["S3"]
        );
    }

    /// Test that statically rendered smcat matches smcat rendered via the runtime system.
    #[test]
    fn smcat_static_dynamic_same() {
        let smcat_file = concat!(env!("OUT_DIR"), "/", "hierarchical.smcat");
        let smcat_static = std::fs::read_to_string(smcat_file).expect("expected smcat file");

        let renderer = smcat::Renderer::new(smcat::CssStyle);
        let smcat_dynamic = renderer.render_static(Hierarchical::machine_info());

        assert_eq!(smcat_static, smcat_dynamic);
    }
}
