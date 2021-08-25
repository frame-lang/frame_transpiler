type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "state_params.rs"));

impl StateParams {
    pub fn got_param(&mut self, name: String, val: u32) {
        self.param_log.push(format!("{}={}", name, val));
    }
    pub fn transition_hook(&mut self, _current: StateParamsState, _next: StateParamsState) {}
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
