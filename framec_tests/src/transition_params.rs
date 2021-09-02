//! Test transition parameters, i.e. arguments passed to the enter/exit
//! handlers during a transition.

type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "transition_params.rs"));

impl TransitParams {
    pub fn log(&mut self, msg: String) {
        self.tape.push(format!("{}", msg));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enter() {
        let mut sm = TransitParams::new();
        sm.next();
        assert_eq!(sm.tape, vec!["hi A"]);
    }
    
    #[test]
    fn enter_and_exit() {
        let mut sm = TransitParams::new();
        sm.next();
        sm.tape.clear();
        sm.next();
        assert_eq!(sm.tape, vec!["bye A", "hi B", "42"]);
        sm.tape.clear();
        sm.next();
        assert_eq!(sm.tape, vec!["true", "bye B", "hi again A"]);
    }
}
