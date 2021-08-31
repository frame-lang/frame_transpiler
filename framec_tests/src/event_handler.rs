//! Test event handler parameters, local variables, and return values.

type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "event_handler.rs"));

impl EventHandler {
    pub fn log(&mut self, msg: String, val: i32) {
        self.tape.push(format!("{}={}", msg, val));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_parameter() {
        let mut sm = EventHandler::new();
        sm.log_it(2);
        assert_eq!(sm.tape, vec!["x=2"]);
    }

    #[test]
    fn compute_two_parameters() {
        let mut sm = EventHandler::new();
        sm.log_add(-3, 10);
        assert_eq!(sm.tape, vec!["a=-3", "b=10", "a+b=7"]);
    }

    #[test]
    fn return_local_variable() {
        let mut sm = EventHandler::new();
        let ret = sm.log_return(13, 21);
        assert_eq!(sm.tape, vec!["a=13", "b=21", "r=34"]);
        assert_eq!(ret, 34);
    }

    #[test]
    fn pass_result() {
        let mut sm = EventHandler::new();
        sm.pass_add(5, -12);
        assert_eq!(sm.tape, vec!["p=-7"]);
    }

    #[test]
    fn pass_and_return_result() {
        let mut sm = EventHandler::new();
        let ret = sm.pass_return(101, -59);
        assert_eq!(sm.tape, vec!["r=42", "p=42"]);
        assert_eq!(ret, 42);
    }
}
