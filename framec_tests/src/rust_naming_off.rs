//! Test that Frame still works with the `follow_rust_naming` feature disabled.

type Log = Vec<i32>;
include!(concat!(env!("OUT_DIR"), "/", "rust_naming_off.rs"));

#[allow(non_snake_case)]
impl RustNaming {
    pub fn snake_action(&mut self, arg: i32) {
        self.snake_log.push(arg);
    }
    pub fn CamelAction(&mut self, arg: i32) {
        self.CamelLog.push(arg);
    }
    pub fn action123(&mut self, arg: i32) {
        self.log123.push(arg);
    }
    pub fn logFinal(&mut self, arg: i32) {
        self.finalLog.push(arg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test that the generated file compiles.
    fn follow_rust_naming_compiles() {}

    #[test]
    /// Test that the generated state machine works and that events are
    /// named as expected.
    fn follow_rust_naming_works() {
        let mut sm = RustNaming::new();

        sm.snake_event(1);
        assert_eq!(sm.state, RustNamingState::snake_state);
        sm.snake_event(2);
        assert_eq!(sm.state, RustNamingState::Init);
        sm.snake_event(1);
        assert_eq!(sm.state, RustNamingState::snake_state);
        sm.CamelEvent(3);
        assert_eq!(sm.state, RustNamingState::Init);
        sm.snake_event(1);
        assert_eq!(sm.state, RustNamingState::snake_state);
        sm.event123(4);
        assert_eq!(sm.state, RustNamingState::Init);
        assert_eq!(sm.finalLog, vec![1103, 1104, 1105]);
        sm.finalLog.clear();

        sm.CamelEvent(11);
        assert_eq!(sm.state, RustNamingState::CamelState);
        sm.snake_event(2);
        assert_eq!(sm.state, RustNamingState::Init);
        sm.CamelEvent(11);
        assert_eq!(sm.state, RustNamingState::CamelState);
        sm.CamelEvent(3);
        assert_eq!(sm.state, RustNamingState::Init);
        sm.CamelEvent(11);
        assert_eq!(sm.state, RustNamingState::CamelState);
        sm.event123(4);
        assert_eq!(sm.state, RustNamingState::Init);
        assert_eq!(sm.finalLog, vec![1213, 1214, 1215]);
        sm.finalLog.clear();

        sm.event123(21);
        assert_eq!(sm.state, RustNamingState::state123);
        sm.snake_event(2);
        assert_eq!(sm.state, RustNamingState::Init);
        sm.event123(21);
        assert_eq!(sm.state, RustNamingState::state123);
        sm.CamelEvent(3);
        assert_eq!(sm.state, RustNamingState::Init);
        sm.event123(21);
        assert_eq!(sm.state, RustNamingState::state123);
        sm.event123(4);
        assert_eq!(sm.state, RustNamingState::Init);
        assert_eq!(sm.finalLog, vec![1323, 1324, 1325]);

        assert_eq!(sm.snake_log, vec![1103, 1213, 1323]);
        assert_eq!(sm.CamelLog, vec![1104, 1214, 1324]);
        assert_eq!(sm.log123, vec![1105, 1215, 1325]);
    }

    #[test]
    /// Test that dynamic interface calls are renamed correctly.
    fn interface_calls() {
        let mut sm = RustNaming::new();
        sm.call(String::from("snake_event"), 1);
        sm.call(String::from("CamelEvent"), 2);
        sm.call(String::from("event123"), 3);
        sm.call(String::from("snake_event"), 4);
        sm.call(String::from("CamelEvent"), 5);
        sm.call(String::from("event123"), 6);
        assert_eq!(sm.finalLog, vec![1103, 1307, 1211]);
        assert_eq!(sm.snake_log, vec![1307]);
        assert_eq!(sm.CamelLog, vec![1103]);
        assert_eq!(sm.log123, vec![1211]);
    }
}
