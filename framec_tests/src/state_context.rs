//! Tests the interaction of several features (state variables, state
//! parameters, event parameters, event variables, return values) that are
//! implemented via state contexts.

type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "state_context.rs"));

impl StateContextSm {
    pub fn log(&mut self, name: String, val: i32) {
        self.tape.push(format!("{}={}", name, val));
    }

    pub fn transition_hook(&mut self, _current: StateContextSmState, _next: StateContextSmState) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interaction() {
        let mut sm = StateContextSm::new();
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
}
