include!(concat!(env!("OUT_DIR"), "/", "state_vars.rs"));

impl StateVars {
    pub fn transition_hook(&mut self, _current: StateVarsState, _next: StateVarsState) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_variable() {
        let sm = StateVars::new();
        assert_eq!(sm.state, StateVarsState::A);
        // check x = 0
        sm.x(); // increment x
        sm.x(); // increment x
        // check x = 2
    }

    #[test]
    fn multiple_variables() {
        let sm = StateVars::new();
        sm.y(); // transition to B
        assert_eq!(sm.state, StateVarsState::B);
        // check y = 10
        // check z = 100
        sm.y(); // increment y
        sm.y(); // increment y
        sm.z(); // increment z
        sm.y(); // increment y
        // check y = 13
        // check z = 101
    }
    
    #[test]
    fn variables_are_reset() {
        let sm = StateVars::new();
        sm.x(); // increment x
        sm.x(); // increment x
        // check x = 2
        sm.z(); // transition to B
        sm.z(); // increment z
        sm.y(); // increment y
        sm.z(); // increment z
        // check y = 11
        // check z = 102
        sm.x(); // transition to A
        // check x = 0
        sm.y(); // transition to B
        // check y = 10
        // check z = 100
    }
}
