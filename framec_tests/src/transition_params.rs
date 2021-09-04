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

    #[test]
    fn change_state() {
        let mut sm = TransitParams::new();
        assert_eq!(sm.state, TransitParamsState::Init);
        sm.change();
        assert_eq!(sm.state, TransitParamsState::A);
        sm.change();
        assert_eq!(sm.state, TransitParamsState::B);
        sm.change();
        assert_eq!(sm.state, TransitParamsState::A);
        assert!(sm.tape.is_empty());
    }

    #[test]
    fn change_and_transition() {
        let mut sm = TransitParams::new();
        sm.change();
        assert_eq!(sm.state, TransitParamsState::A);
        assert!(sm.tape.is_empty());
        sm.next();
        assert_eq!(sm.state, TransitParamsState::B);
        assert_eq!(sm.tape, vec!["bye A", "hi B", "42"]);
        sm.tape.clear();
        sm.change();
        assert_eq!(sm.state, TransitParamsState::A);
        assert!(sm.tape.is_empty());
        sm.change();
        sm.next();
        assert_eq!(sm.state, TransitParamsState::A);
        assert_eq!(sm.tape, vec!["true", "bye B", "hi again A"]);
    }
}
