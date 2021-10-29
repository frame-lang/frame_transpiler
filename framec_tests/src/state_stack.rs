//! Tests the basic functionality of the state stack feature. This test case does not include any
//! features that require a state context.

type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "state_stack.rs"));

impl<'a> StateStack<'a> {
    pub fn log(&mut self, msg: String) {
        self.tape.push(msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_runtime::*;
    use std::sync::Mutex;

    /// Test that a pop restores a pushed state.
    #[test]
    fn push_pop() {
        let mut sm = StateStack::new();
        assert_eq!(sm.state, StateStackState::A);
        sm.push();
        sm.to_b();
        assert_eq!(sm.state, StateStackState::B);
        sm.pop();
        assert_eq!(sm.state, StateStackState::A);
    }

    /// Test that multiple states can be pushed and subsequently restored by pops, LIFO style.
    #[test]
    fn multiple_push_pops() {
        let mut sm = StateStack::new();
        assert_eq!(sm.state, StateStackState::A);
        sm.push();
        sm.to_c();
        sm.push();
        sm.to_a();
        sm.push();
        sm.push();
        sm.to_c(); // no push
        sm.to_b();
        sm.push();
        sm.to_c();
        sm.push(); // stack top-to-bottom: C, B, A, A, C, A
        sm.to_a();
        assert_eq!(sm.state, StateStackState::A);
        sm.pop();
        assert_eq!(sm.state, StateStackState::C);
        sm.to_a();
        assert_eq!(sm.state, StateStackState::A);
        sm.pop();
        assert_eq!(sm.state, StateStackState::B);
        sm.pop();
        assert_eq!(sm.state, StateStackState::A);
        sm.pop();
        assert_eq!(sm.state, StateStackState::A);
        sm.pop();
        assert_eq!(sm.state, StateStackState::C);
        sm.to_b();
        sm.push();
        sm.to_c();
        sm.push(); // stack top-to-bottom: C, B, A
        sm.to_a();
        sm.to_b();
        assert_eq!(sm.state, StateStackState::B);
        sm.pop();
        assert_eq!(sm.state, StateStackState::C);
        sm.pop();
        assert_eq!(sm.state, StateStackState::B);
        sm.pop();
        assert_eq!(sm.state, StateStackState::A);
    }

    /// Test that pop transitions trigger enter/exit events.
    #[test]
    fn pop_transition_events() {
        let mut sm = StateStack::new();
        sm.to_b();
        sm.push();
        sm.to_a();
        sm.push();
        sm.to_c();
        sm.push(); // stack top-to-bottom: C, A, B
        sm.to_a();
        sm.tape.clear();
        assert_eq!(sm.state, StateStackState::A);
        sm.pop();
        assert_eq!(sm.state, StateStackState::C);
        assert_eq!(sm.tape, vec!["A:<", "C:>"]);
        sm.tape.clear();
        sm.pop();
        sm.pop();
        assert_eq!(sm.state, StateStackState::B);
        assert_eq!(sm.tape, vec!["C:<", "A:>", "A:<", "B:>"]);
    }

    /// Test that pop change-states do not trigger enter/exit events.
    #[test]
    fn pop_change_state_no_events() {
        let mut sm = StateStack::new();
        sm.to_b();
        sm.push();
        sm.to_a();
        sm.push();
        sm.to_c();
        sm.push(); // stack top-to-bottom: C, A, B
        sm.to_a();
        sm.tape.clear();
        assert_eq!(sm.state, StateStackState::A);
        sm.pop_change();
        assert_eq!(sm.state, StateStackState::C);
        assert!(sm.tape.is_empty());
        sm.pop();
        sm.pop_change();
        assert_eq!(sm.state, StateStackState::B);
        assert_eq!(sm.tape, vec!["C:<", "A:>"]);
    }

    /// Test that pop transitions and change-states trigger callbacks.
    #[test]
    fn pop_transition_callbacks() {
        let out = Mutex::new(String::new());
        let mut sm = StateStack::new();
        sm.callback_manager().add_transition_callback(|event| {
            *out.lock().unwrap() = format!(
                "{}{}{}",
                event.old_state.info().name(),
                match event.info.kind {
                    TransitionKind::ChangeState => "->>",
                    TransitionKind::Transition => "->",
                },
                event.new_state.info().name(),
            );
        });
        sm.to_c();
        sm.push();
        sm.to_b();
        sm.push();
        sm.to_a();
        sm.push(); // stack top-to-bottom: A, B, C
        sm.to_b();
        assert_eq!(*out.lock().unwrap(), "A->B");
        sm.pop();
        assert_eq!(*out.lock().unwrap(), "B->A");
        sm.pop_change();
        assert_eq!(*out.lock().unwrap(), "A->>B");
        sm.push();
        sm.to_c();
        assert_eq!(*out.lock().unwrap(), "B->C");
        sm.pop_change();
        assert_eq!(*out.lock().unwrap(), "C->>B");
        sm.pop();
        assert_eq!(*out.lock().unwrap(), "B->C");
    }
}
