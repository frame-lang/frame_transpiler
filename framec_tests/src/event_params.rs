type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "event_params.rs"));

impl EventParams {
    pub fn entered(&mut self, msg: String, val: i16) {
        self.enter_log.push(format!("{} {}", msg, val));
    }
    pub fn exited(&mut self, val: bool, msg: String) {
        self.exit_log.push(format!("{} {}", val, msg));
    }
    pub fn transition_hook(&mut self, _current: EventParamsState, _next: EventParamsState) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enter() {
        let mut sm = EventParams::new();
        sm.next();
        assert_eq!(sm.enter_log, vec!["hi A 1", "hi B 2"]);
        assert_eq!(sm.exit_log, Log::new());
    }

    #[test]
    fn enter_and_exit() {
        let mut sm = EventParams::new();
        sm.next();
        sm.next();
        assert_eq!(sm.enter_log, vec!["hi A 1", "hi B 2", "hi again A 3"]);
        assert_eq!(sm.exit_log, vec!["true bye B"]);
    }
}
