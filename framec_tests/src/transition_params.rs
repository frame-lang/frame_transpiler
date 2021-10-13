//! Test transition parameters, i.e. arguments passed to the enter/exit
//! handlers during a transition.

type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "transition_params.rs"));

impl<'a> TransitParams<'a> {
    pub fn log(&mut self, msg: String) {
        self.tape.push(msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_runtime::transition::*;
    use std::sync::Mutex;

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

    #[test]
    /// Test that transition callbacks get event arguments.
    fn callbacks_get_event_args() {
        let out = Mutex::new(String::new());
        let mut sm = TransitParams::new();
        sm.callback_manager().add_transition_callback(|info| {
            let mut entry = String::new();
            info.exit_arguments.lookup("msg").map(|any| {
                entry.push_str(&format!("msg: {}, ", any.downcast_ref::<String>().unwrap()));
            });
            info.exit_arguments.lookup("val").map(|any| {
                entry.push_str(&format!("val: {}, ", any.downcast_ref::<bool>().unwrap()));
            });
            entry.push_str(&format!(
                "{}{}{}",
                info.old_state.name(),
                match info.kind {
                    TransitionKind::ChangeState => "->>",
                    TransitionKind::Transition => "->",
                },
                info.new_state.name(),
            ));
            info.enter_arguments.lookup("msg").map(|any| {
                entry.push_str(&format!(", msg: {}", any.downcast_ref::<String>().unwrap()));
            });
            info.enter_arguments.lookup("val").map(|any| {
                entry.push_str(&format!(", val: {}", any.downcast_ref::<i16>().unwrap()));
            });
            *out.lock().unwrap() = entry;
        });
        sm.next();
        assert_eq!(*out.lock().unwrap(), "Init->A, msg: hi A");
        sm.next();
        assert_eq!(*out.lock().unwrap(), "A->B, msg: hi B, val: 42");
        sm.next();
        assert_eq!(
            *out.lock().unwrap(),
            "msg: bye B, val: true, B->A, msg: hi again A"
        );
        sm.change();
        assert_eq!(*out.lock().unwrap(), "A->>B");
        sm.change();
        assert_eq!(*out.lock().unwrap(), "B->>A");
    }
}
