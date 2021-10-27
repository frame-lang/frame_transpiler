//! Tests the interaction of several features (state variables, state parameters, event parameters,
//! event variables, return values) that are implemented via state contexts, with the runtime
//! system enabled.
//!
//! This is the same state machine as `state_context.rs` but with `runtime_support=true`.

type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "state_context_runtime.rs"));

impl<'a> StateContextSm<'a> {
    pub fn log(&mut self, name: String, val: i32) {
        self.tape.push(format!("{}={}", name, val));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        sm.inc();
        sm.inc();
        sm.start();
        sm.tape.clear();

        sm.inc();
        assert_eq!(sm.tape, vec!["x=16"]);
        sm.tape.clear();

        sm.change(10);
        sm.log_state();
        assert_eq!(sm.tape, vec!["y=26", "z=0"]);
        sm.tape.clear();

        sm.inc();
        sm.change(100);
        sm.log_state();
        assert_eq!(sm.state, StateContextSmState::Init);
        assert_eq!(sm.tape, vec!["z=1", "tmp=127", "w=0"]);
    }

    // TODO add runtime support tests
}
