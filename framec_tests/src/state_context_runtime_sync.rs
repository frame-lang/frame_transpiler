//! Tests the interaction of several features (state variables, state parameters, event parameters,
//! event variables, return values) that are implemented via state contexts, with the thread-safe
//! version of the runtime system enabled.
//!
//! This is the same state machine as used in `state_context.rs` and `state_context_runtime.rs` and
//! several tests are redundant among them. However, the generated code is quite different for each
//! combination of features, so these tests are doing work. The runtime "info" tests are not
//! repeated since the generated code should be the same for both.

type Log = Vec<String>;
include!(concat!(
    env!("OUT_DIR"),
    "/",
    "state_context_runtime_sync.rs"
));

impl StateContextSm {
    pub fn log(&mut self, name: String, val: i32) {
        self.tape.push(format!("{}={}", name, val));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_runtime::*;

    fn has_send(_: &impl Send) {}
    fn has_sync(_: &impl Sync) {}

    /// Test that the state machine implements the `Send` and `Sync` traits. Will cause a compile
    /// error if it doesn't.
    #[test]
    fn implements_send_and_sync() {
        let sm = StateContextSm::new();
        has_send(&sm);
        has_sync(&sm);
    }

    #[test]
    fn initial_state() {
        let mut sm = StateContextSm::new();
        let r = sm.inc();
        assert_eq!(r, 4);
        sm.log_state();
        assert_eq!(sm.tape, vec!["w=3", "w=4", "w=4"]);
    }

    #[test]
    fn transition() {
        let mut sm = StateContextSm::new();
        sm.inc();
        sm.inc();
        sm.tape.clear();

        sm.start();
        assert_eq!(sm.tape, vec!["a=3", "b=5", "x=15"]);
        sm.tape.clear();

        sm.inc();
        let r = sm.inc();
        assert_eq!(r, 17);
        assert_eq!(sm.tape, vec!["x=16", "x=17"]);
        sm.tape.clear();

        sm.next(3);
        assert_eq!(sm.tape, vec!["c=10", "x=27", "a=30", "y=17", "z=47"]);
        sm.tape.clear();

        sm.inc();
        sm.inc();
        let r = sm.inc();
        assert_eq!(r, 50);
        assert_eq!(sm.tape, vec!["z=48", "z=49", "z=50"]);
    }

    #[test]
    fn change_state() {
        let mut sm = StateContextSm::new();
        println!("1");
        sm.inc();
        println!("2");
        sm.inc();
        println!("3");
        sm.start();
        println!("4");
        sm.tape.clear();

        sm.inc();
        println!("5");
        assert_eq!(sm.tape, vec!["x=16"]);
        sm.tape.clear();

        sm.change(10);
        println!("6");
        sm.log_state();
        println!("7");
        assert_eq!(sm.tape, vec!["y=26", "z=0"]);
        sm.tape.clear();

        sm.inc();
        sm.change(100);
        sm.log_state();
        assert_eq!(sm.state, StateContextSmState::Init);
        assert_eq!(sm.tape, vec!["z=1", "tmp=127", "w=0"]);
    }

    /// Test that we can access the current state via the runtime interface.
    #[test]
    fn runtime_current_state() {
        let mut sm = StateContextSm::new();
        assert_eq!(sm.state().info().name, "Init");
        sm.start();
        assert_eq!(sm.state().info().name, "Foo");
        sm.next(3);
        assert_eq!(sm.state().info().name, "Bar");
        sm.change(4);
        assert_eq!(sm.state().info().name, "Init");
        sm.start();
        sm.change(5);
        assert_eq!(sm.state().info().name, "Bar");
    }

    /// Test that we can access the values of domain variables via the runtime interface.
    #[test]
    fn runtime_domain_variables() {
        let mut sm = StateContextSm::new();
        sm.inc();
        sm.inc();
        {
            let tape: Log = *sm.variables().lookup("tape").unwrap().downcast().unwrap();
            assert_eq!(tape, vec!["w=3", "w=4", "w=5"]);
        }
        sm.tape.clear();
        sm.start();
        {
            let tape: Log = *sm.variables().lookup("tape").unwrap().downcast().unwrap();
            assert_eq!(*tape, vec!["a=3", "b=5", "x=15"]);
        }
    }

    /// Test that we can access the values of state variables via the runtime interface.
    #[test]
    fn runtime_state_variables() {
        let mut sm = StateContextSm::new();
        sm.inc();
        {
            let vars = sm.state().variables();
            let w: i32 = *vars.lookup("w").unwrap().downcast().unwrap();
            assert_eq!(w, 4);
            assert!(vars.lookup("a").is_none());
            assert!(vars.lookup("x").is_none());
            assert!(vars.lookup("y").is_none());
            assert!(vars.lookup("z").is_none());
            assert!(vars.lookup("log").is_none());
        }
        sm.inc();
        {
            let vars = sm.state().variables();
            let w: i32 = *vars.lookup("w").unwrap().downcast().unwrap();
            assert_eq!(w, 5);
        }
        sm.inc();
        sm.start();
        {
            let vars = sm.state().variables();
            let x: i32 = *vars.lookup("x").unwrap().downcast().unwrap();
            assert_eq!(x, 18);
            assert!(vars.lookup("a").is_none());
            assert!(vars.lookup("w").is_none());
            assert!(vars.lookup("y").is_none());
            assert!(vars.lookup("z").is_none());
            assert!(vars.lookup("log").is_none());
        }
        sm.inc();
        sm.next(10);
        {
            let vars = sm.state().variables();
            let z: i32 = *vars.lookup("z").unwrap().downcast().unwrap();
            assert_eq!(z, 119);
            assert!(vars.lookup("a").is_none());
            assert!(vars.lookup("w").is_none());
            assert!(vars.lookup("x").is_none());
            assert!(vars.lookup("y").is_none());
            assert!(vars.lookup("log").is_none());
        }
    }

    /// Test that we can access the values of state arguments via the runtime interface.
    #[test]
    fn runtime_state_arguments() {
        let mut sm = StateContextSm::new();
        {
            let args = sm.state().arguments();
            assert!(args.lookup("a").is_none());
            assert!(args.lookup("w").is_none());
            assert!(args.lookup("x").is_none());
            assert!(args.lookup("y").is_none());
            assert!(args.lookup("z").is_none());
            assert!(args.lookup("log").is_none());
        }
        sm.inc();
        sm.start();
        sm.inc();
        sm.next(10);
        {
            let args = sm.state().arguments();
            let y: i32 = *args.lookup("y").unwrap().downcast().unwrap();
            assert_eq!(y, 13);
            assert!(args.lookup("a").is_none());
            assert!(args.lookup("w").is_none());
            assert!(args.lookup("x").is_none());
            assert!(args.lookup("z").is_none());
            assert!(args.lookup("log").is_none());
        }
    }
}
