//! Test transition parameters, i.e. arguments passed to the enter/exit
//! handlers during a transition.

type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "transition_params.rs"));

impl TransitParams {
    pub fn entered(&mut self, msg: String, val: i16) {
        self.enter_log.push(format!("{} {}", msg, val));
    }
    pub fn exited(&mut self, val: bool, msg: String) {
        self.exit_log.push(format!("{} {}", val, msg));
    }
    pub fn transition_hook(&mut self, _current: TransitParamsState, _next: TransitParamsState) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enter() {
        let mut sm = TransitParams::new();
        sm.next();
        assert_eq!(sm.enter_log, vec!["hi A 1", "hi B 2"]);
        assert_eq!(sm.exit_log, Log::new());
    }

    #[test]
    fn enter_and_exit() {
        let mut sm = TransitParams::new();
        sm.next();
        sm.next();
        assert_eq!(sm.enter_log, vec!["hi A 1", "hi B 2", "hi again A 3"]);
        assert_eq!(sm.exit_log, vec!["true bye B"]);
    }
}
