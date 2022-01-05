//! Tests the basic functionality of the state stack feature. This test case does not include any
//! features that require a state context.

type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "state_stack.rs"));

impl StateStack {
    pub fn log(&mut self, msg: String) {
        self.tape.push(msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_runtime::*;

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
        let mut sm = StateStack::new();
        let out = Rc::new(RefCell::new(String::new()));
        let out_cb = out.clone();
        sm.event_monitor_mut()
            .add_transition_callback(Callback::new("test", move |t: &Transition<StateStack>| {
                out_cb.replace(t.to_string());
            }));
        sm.to_c();
        sm.push();
        sm.to_b();
        sm.push();
        sm.to_a();
        sm.push(); // stack top-to-bottom: A, B, C
        sm.to_b();
        assert_eq!((*out).borrow().to_owned(), "A->B");
        sm.pop();
        assert_eq!((*out).borrow().to_owned(), "B->A");
        sm.pop_change();
        assert_eq!((*out).borrow().to_owned(), "A->>B");
        sm.push();
        sm.to_c();
        assert_eq!((*out).borrow().to_owned(), "B->C");
        sm.pop_change();
        assert_eq!((*out).borrow().to_owned(), "C->>B");
        sm.pop();
        assert_eq!((*out).borrow().to_owned(), "B->C");
    }

    /// Test that the targets of pop transitions are set correctly.
    #[test]
    fn pop_transition_target_info() {
        let mut sm = StateStack::new();
        sm.push();
        sm.to_b();
        sm.push();
        sm.to_c();

        let c_info = sm.state().info();
        assert_eq!(c_info.name, "C");
        assert!(!c_info.is_stack_pop);

        let c_out = c_info.outgoing_transitions();
        assert_eq!(c_out.len(), 5);
        assert_eq!(c_out[0].target.name, "A");
        assert_eq!(c_out[1].target.name, "B");
        assert_eq!(c_out[2].target.name, "C");
        assert_eq!(c_out[3].target.name, "$$[-]");
        assert_eq!(c_out[4].target.name, "$$[-]");

        assert!(!c_out[0].target.is_stack_pop);
        assert!(!c_out[1].target.is_stack_pop);
        assert!(!c_out[2].target.is_stack_pop);
        assert!(c_out[3].target.is_stack_pop);
        assert!(c_out[4].target.is_stack_pop);

        assert!(c_out[0].is_transition());
        assert!(c_out[1].is_transition());
        assert!(c_out[2].is_transition());
        assert!(c_out[3].is_transition());
        assert!(c_out[4].is_change_state());

        sm.pop();
        let b_info = sm.state().info();
        assert_eq!(b_info.name, "B");
        assert!(!b_info.is_stack_pop);

        let b_out = b_info.outgoing_transitions();
        assert_eq!(b_out.len(), 5);
        assert_eq!(b_out[0].target.name, "A");
        assert_eq!(b_out[1].target.name, "B");
        assert_eq!(b_out[2].target.name, "C");
        assert_eq!(b_out[3].target.name, "$$[-]");
        assert_eq!(b_out[4].target.name, "$$[-]");

        assert!(!b_out[0].target.is_stack_pop);
        assert!(!b_out[1].target.is_stack_pop);
        assert!(!b_out[2].target.is_stack_pop);
        assert!(b_out[3].target.is_stack_pop);
        assert!(b_out[4].target.is_stack_pop);

        assert!(b_out[0].is_transition());
        assert!(b_out[1].is_transition());
        assert!(b_out[2].is_transition());
        assert!(b_out[3].is_transition());
        assert!(b_out[4].is_change_state());

        sm.pop();
        let a_info = sm.state().info();
        assert_eq!(a_info.name, "A");
        assert!(!a_info.is_stack_pop);
    }
}
