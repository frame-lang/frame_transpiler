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
        sm.hello();
        sm.hello();
        assert_eq!(sm.enter_log, vec!["hello B 42", "howdy A 0"]);
        assert_eq!(sm.exit_log, Log::new());
    }

    #[test]
    fn exit() {
        let mut sm = EventParams::new();
        sm.goodbye();
        sm.goodbye();
        assert_eq!(sm.enter_log, Log::new());
        assert_eq!(sm.exit_log, vec!["true goodbye A", "false tootles B"]);
    }

    #[test]
    fn both() {
        let mut sm = EventParams::new();
        sm.hello();
        sm.both();
        sm.both();
        sm.goodbye();
        assert_eq!(sm.enter_log, vec!["hello B 42", "sup A 101", "hi B -42"]);
        assert_eq!(
            sm.exit_log,
            vec!["true ciao B", "false bye A", "false tootles B"]
        );
    }
}
