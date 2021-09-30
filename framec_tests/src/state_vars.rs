include!(concat!(env!("OUT_DIR"), "/", "state_vars.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_variable() {
        let mut sm = StateVars::new();
        assert_eq!(sm.state, StateVarsState::A);
        assert_eq!(sm.state_context.a_context().borrow().state_vars.x, 0);
        sm.x(); // increment x
        sm.x(); // increment x
        assert_eq!(sm.state_context.a_context().borrow().state_vars.x, 2);
    }

    #[test]
    fn multiple_variables() {
        let mut sm = StateVars::new();
        sm.y(); // transition to B
        assert_eq!(sm.state, StateVarsState::B);
        assert_eq!(sm.state_context.b_context().borrow().state_vars.y, 10);
        assert_eq!(sm.state_context.b_context().borrow().state_vars.z, 100);
        sm.y(); // increment y
        sm.y(); // increment y
        sm.z(); // increment z
        sm.y(); // increment y
        assert_eq!(sm.state_context.b_context().borrow().state_vars.y, 13);
        assert_eq!(sm.state_context.b_context().borrow().state_vars.z, 101);
    }

    #[test]
    fn variables_are_reset() {
        let mut sm = StateVars::new();
        sm.x(); // increment x
        sm.x(); // increment x
        assert_eq!(sm.state_context.a_context().borrow().state_vars.x, 2);
        sm.z(); // transition to B
        sm.z(); // increment z
        sm.y(); // increment y
        sm.z(); // increment z
        assert_eq!(sm.state_context.b_context().borrow().state_vars.y, 11);
        assert_eq!(sm.state_context.b_context().borrow().state_vars.z, 102);
        sm.x(); // transition to A
        assert_eq!(sm.state_context.a_context().borrow().state_vars.x, 0);
        sm.y(); // transition to B
        assert_eq!(sm.state_context.b_context().borrow().state_vars.y, 10);
        assert_eq!(sm.state_context.b_context().borrow().state_vars.z, 100);
    }
}
