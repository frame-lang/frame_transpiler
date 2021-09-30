//! Tests the state stack feature when states have associated contexts.
//!
//! Most features of state contexts are not supported by state stacks. In
//! particular, state parameters and enter/exit parameters are not supported.
//! The reason is that when transitioning to a popped state, the state is not
//! known statically, so there is no way for the programmer to know what
//! arguments must be passed.
//!
//! However, state variables are supported by the state stack feature. The
//! interaction of those features is tested here.
//!
//! Additionally, the basic functionality of state stacks are tested again
//! here since pushing and popping with state contexts is a different code
//! path than pushing and popping without.

type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "state_context_stack.rs"));

impl<'a> StateContextStack<'a> {
    pub fn log(&mut self, msg: String) {
        self.tape.push(msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_runtime::transition::*;

    #[test]
    /// Test that a pop restores a pushed state.
    fn push_pop() {
        let mut sm = StateContextStack::new();
        assert_eq!(sm.state, StateContextStackState::A);
        sm.push();
        sm.to_b();
        assert_eq!(sm.state, StateContextStackState::B);
        sm.pop();
        assert_eq!(sm.state, StateContextStackState::A);
    }

    #[test]
    /// Test that multiple states can be pushed and subsequently restored by
    /// pops, LIFO style.
    fn multiple_push_pops() {
        let mut sm = StateContextStack::new();
        assert_eq!(sm.state, StateContextStackState::A);
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
        assert_eq!(sm.state, StateContextStackState::A);
        sm.pop();
        assert_eq!(sm.state, StateContextStackState::C);
        sm.to_a();
        assert_eq!(sm.state, StateContextStackState::A);
        sm.pop();
        assert_eq!(sm.state, StateContextStackState::B);
        sm.pop();
        assert_eq!(sm.state, StateContextStackState::A);
        sm.pop();
        assert_eq!(sm.state, StateContextStackState::A);
        sm.pop();
        assert_eq!(sm.state, StateContextStackState::C);
        sm.to_b();
        sm.push();
        sm.to_c();
        sm.push(); // stack top-to-bottom: C, B, A
        sm.to_a();
        sm.to_b();
        assert_eq!(sm.state, StateContextStackState::B);
        sm.pop();
        assert_eq!(sm.state, StateContextStackState::C);
        sm.pop();
        assert_eq!(sm.state, StateContextStackState::B);
        sm.pop();
        assert_eq!(sm.state, StateContextStackState::A);
    }

    #[test]
    /// Test that pop transitions trigger enter/exit events.
    fn pop_transition_events() {
        let mut sm = StateContextStack::new();
        sm.to_b();
        sm.push();
        sm.to_a();
        sm.push();
        sm.to_c();
        sm.push(); // stack top-to-bottom: C, A, B
        sm.to_a();
        sm.tape.clear();
        assert_eq!(sm.state, StateContextStackState::A);
        sm.pop();
        assert_eq!(sm.state, StateContextStackState::C);
        assert_eq!(sm.tape, vec!["A:<", "C:>"]);
        sm.tape.clear();
        sm.pop();
        sm.pop();
        assert_eq!(sm.state, StateContextStackState::B);
        assert_eq!(sm.tape, vec!["C:<", "A:>", "A:<", "B:>"]);
    }

    #[test]
    /// Test that pop change-states do not trigger enter/exit events.
    fn pop_change_state_no_events() {
        let mut sm = StateContextStack::new();
        sm.to_b();
        sm.push();
        sm.to_a();
        sm.push();
        sm.to_c();
        sm.push(); // stack top-to-bottom: C, A, B
        sm.to_a();
        sm.tape.clear();
        assert_eq!(sm.state, StateContextStackState::A);
        sm.pop_change();
        assert_eq!(sm.state, StateContextStackState::C);
        assert!(sm.tape.is_empty());
        sm.pop();
        sm.pop_change();
        assert_eq!(sm.state, StateContextStackState::B);
        assert_eq!(sm.tape, vec!["C:<", "A:>"]);
    }

    #[test]
    /// Test that state variables are restored after pop.
    fn pop_restores_state_variables() {
        let mut sm = StateContextStack::new();
        sm.inc();
        sm.inc();
        sm.push();
        assert_eq!(sm.state, StateContextStackState::A);
        assert_eq!(sm.value(), 2);
        sm.to_b();
        sm.inc();
        sm.push();
        assert_eq!(sm.state, StateContextStackState::B);
        assert_eq!(sm.value(), 5);
        sm.to_c();
        sm.inc();
        sm.inc();
        sm.inc();
        sm.push();
        assert_eq!(sm.state, StateContextStackState::C);
        assert_eq!(sm.value(), 30);
        sm.to_a();
        sm.inc();
        assert_eq!(sm.state, StateContextStackState::A);
        assert_eq!(sm.value(), 1);
        sm.pop();
        assert_eq!(sm.state, StateContextStackState::C);
        assert_eq!(sm.value(), 30);
        sm.pop();
        assert_eq!(sm.state, StateContextStackState::B);
        assert_eq!(sm.value(), 5);
        sm.to_a();
        sm.inc();
        sm.inc();
        sm.inc();
        sm.push();
        assert_eq!(sm.state, StateContextStackState::A);
        assert_eq!(sm.value(), 3);
        sm.to_c();
        sm.inc();
        assert_eq!(sm.state, StateContextStackState::C);
        assert_eq!(sm.value(), 10);
        sm.pop();
        assert_eq!(sm.state, StateContextStackState::A);
        assert_eq!(sm.value(), 3);
        sm.pop();
        assert_eq!(sm.state, StateContextStackState::A);
        assert_eq!(sm.value(), 2);
    }

    #[test]
    /// Test that pop transitions and change-states with state contexts trigger
    /// callbacks.
    fn pop_transition_callbacks() {
        let out: RefCell<String> = RefCell::new(String::new());
        let mut sm = StateContextStack::new();
        sm.callback_manager().add_transition_callback(|info| {
            *out.borrow_mut() = format!(
                "{}{}{}",
                info.old_state.name(),
                match info.kind {
                    TransitionKind::ChangeState => "->>",
                    TransitionKind::Transition => "->",
                },
                info.new_state.name(),
            );
        });
        sm.to_c();
        sm.push();
        sm.to_b();
        sm.inc();
        sm.push();
        sm.to_a();
        sm.inc();
        sm.push(); // stack top-to-bottom: A, B, C
        sm.to_b();
        assert_eq!(*out.borrow(), "A->B");
        sm.pop();
        assert_eq!(*out.borrow(), "B->A");
        sm.pop_change();
        assert_eq!(*out.borrow(), "A->>B");
        sm.push();
        sm.to_c();
        assert_eq!(*out.borrow(), "B->C");
        sm.pop_change();
        assert_eq!(*out.borrow(), "C->>B");
        sm.pop();
        assert_eq!(*out.borrow(), "B->C");
    }
}
