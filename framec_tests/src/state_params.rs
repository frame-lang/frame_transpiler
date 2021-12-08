type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "state_params.rs"));

impl<'a> StateParams<'a> {
    pub fn got_param(&mut self, name: String, val: u32) {
        self.param_log.push(format!("{}={}", name, val));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_runtime::unsync::*;

    #[test]
    fn single_parameter() {
        let mut sm = StateParams::new();
        sm.next();
        sm.log();
        assert_eq!(sm.param_log, vec!["val=1"]);
    }

    #[test]
    fn multiple_parameters() {
        let mut sm = StateParams::new();
        sm.next();
        sm.next();
        sm.log();
        assert_eq!(sm.param_log, vec!["left=1", "right=2"]);
    }

    #[test]
    fn several_passes() {
        let mut sm = StateParams::new();
        sm.next(); // val=1
        sm.next(); // left=1, right=2
        sm.next(); // val=3
        sm.log();
        sm.prev(); // left=4, right=3
        sm.log();
        sm.prev(); // val=12
        sm.log();
        assert_eq!(sm.param_log, vec!["val=3", "left=4", "right=3", "val=12"]);
    }

    /// Helper function to lookup a `u32` value in an environment.
    /// Returns `u32::MAX` if the lookup fails for any reason.
    fn lookup_u32(env: &dyn Environment, name: &str) -> u32 {
        match env.lookup(name) {
            None => u32::MAX,
            Some(any) => *any.downcast_ref().unwrap_or(&u32::MAX),
        }
    }

    /// Tests that state arguments behave as expected when accessed via the runtime interface.
    #[test]
    #[rustfmt::skip]
    fn runtime_state_arguments() {
        let mut sm = StateParams::new();
        assert!(sm.state().arguments().lookup("val").is_none());
        assert!(sm.state().arguments().lookup("left").is_none());
        assert!(sm.state().arguments().lookup("right").is_none());
        sm.next(); // val=1
        assert_eq!(lookup_u32(sm.state().arguments().as_ref(), "val"), 1);
        assert!(sm.state().arguments().lookup("left").is_none());
        assert!(sm.state().arguments().lookup("right").is_none());
        sm.next(); // left=1, right=2
        assert!(sm.state().arguments().lookup("val").is_none());
        assert_eq!(lookup_u32(sm.state().arguments().as_ref(), "left"), 1);
        assert_eq!(lookup_u32(sm.state().arguments().as_ref(), "right"), 2);
        sm.next(); // val=3
        assert_eq!(lookup_u32(sm.state().arguments().as_ref(), "val"), 3);
        assert!(sm.state().arguments().lookup("left").is_none());
        assert!(sm.state().arguments().lookup("right").is_none());
        sm.prev(); // left=4, right=3
        assert!(sm.state().arguments().lookup("val").is_none());
        assert_eq!(lookup_u32(sm.state().arguments().as_ref(), "left"), 4);
        assert_eq!(lookup_u32(sm.state().arguments().as_ref(), "right"), 3);
        sm.prev(); // val=12
        assert_eq!(lookup_u32(sm.state().arguments().as_ref(), "val"), 12);
        assert!(sm.state().arguments().lookup("left").is_none());
        assert!(sm.state().arguments().lookup("right").is_none());
    }
}
