//! Tests the state stack feature when states have associated contexts.
//!
//! Most features of state contexts are not supported by state stacks. In particular, state
//! parameters and enter/exit parameters are not supported. The reason is that when transitioning
//! to a popped state, the state is not known statically, so there is no way for the programmer to
//! know what arguments must be passed.
//!
//! However, state variables are supported by the state stack feature. The interaction of those
//! features is tested here.
//!
//! Additionally, the basic functionality of state stacks are tested again here since pushing and
//! popping with state contexts is a different code path than pushing and popping without.

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
    use frame_runtime::*;
    use std::sync::Mutex;

    /// Test that a pop restores a pushed state.
    #[test]
    fn push_pop() {
        let mut sm = StateContextStack::new();
        assert_eq!(sm.state, StateContextStackState::A);
        sm.push();
        sm.to_b();
        assert_eq!(sm.state, StateContextStackState::B);
        sm.pop();
        assert_eq!(sm.state, StateContextStackState::A);
    }

    /// Test that multiple states can be pushed and subsequently restored by pops, LIFO style.
    #[test]
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

    /// Test that pop transitions trigger enter/exit events.
    #[test]
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

    /// Test that pop change-states do not trigger enter/exit events.
    #[test]
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

    /// Test that state variables are restored after pop.
    #[test]
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

    /// Test that push stores a snapshot of the current values of state variables. Any changes to
    /// state variables after a push should not be reflected after that state is popped.
    #[test]
    fn push_stores_state_variable_snapshot() {
        let mut sm = StateContextStack::new();
        sm.inc();
        sm.inc();
        sm.push();
        assert_eq!(sm.state, StateContextStackState::A);
        assert_eq!(sm.value(), 2);
        sm.inc();
        sm.inc();
        assert_eq!(sm.value(), 4);

        sm.to_b();
        sm.inc();
        sm.push();
        assert_eq!(sm.state, StateContextStackState::B);
        assert_eq!(sm.value(), 5);
        sm.inc();
        sm.inc();
        assert_eq!(sm.value(), 15);

        sm.to_c();
        sm.inc();
        sm.inc();
        sm.inc();
        sm.push();
        assert_eq!(sm.state, StateContextStackState::C);
        assert_eq!(sm.value(), 30);
        sm.inc();
        assert_eq!(sm.value(), 40);

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
        sm.inc();
        assert_eq!(sm.value(), 4);

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

    /// Test that pop transitions and change-states with state contexts trigger callbacks.
    #[test]
    fn pop_transition_callbacks() {
        let out = Mutex::new(String::new());
        let mut sm = StateContextStack::new();
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
        sm.inc();
        sm.push();
        sm.to_a();
        sm.inc();
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

    /// Test that the targets of pop transitions are set correctly.
    #[test]
    fn pop_transition_target_info() {
        let mut sm = StateContextStack::new();
        sm.inc();
        sm.push();
        sm.to_b();
        sm.inc();
        sm.push();
        sm.inc();
        sm.to_c();
        sm.inc();

        let c_info = sm.state().info();
        assert_eq!(c_info.name(), "C");
        assert!(!c_info.is_stack_pop());

        let c_out = c_info.outgoing_transitions();
        assert_eq!(c_out.len(), 5);
        assert_eq!(c_out[0].target.name(), "A");
        assert_eq!(c_out[1].target.name(), "B");
        assert_eq!(c_out[2].target.name(), "C");
        assert_eq!(c_out[3].target.name(), "$$[-]");
        assert_eq!(c_out[4].target.name(), "$$[-]");

        assert!(!c_out[0].target.is_stack_pop());
        assert!(!c_out[1].target.is_stack_pop());
        assert!(!c_out[2].target.is_stack_pop());
        assert!(c_out[3].target.is_stack_pop());
        assert!(c_out[4].target.is_stack_pop());

        assert!(c_out[0].is_transition());
        assert!(c_out[1].is_transition());
        assert!(c_out[2].is_transition());
        assert!(c_out[3].is_transition());
        assert!(c_out[4].is_change_state());

        sm.pop();
        let b_info = sm.state().info();
        assert_eq!(b_info.name(), "B");
        assert!(!b_info.is_stack_pop());

        let b_out = b_info.outgoing_transitions();
        assert_eq!(b_out.len(), 5);
        assert_eq!(b_out[0].target.name(), "A");
        assert_eq!(b_out[1].target.name(), "B");
        assert_eq!(b_out[2].target.name(), "C");
        assert_eq!(b_out[3].target.name(), "$$[-]");
        assert_eq!(b_out[4].target.name(), "$$[-]");

        assert!(!b_out[0].target.is_stack_pop());
        assert!(!b_out[1].target.is_stack_pop());
        assert!(!b_out[2].target.is_stack_pop());
        assert!(b_out[3].target.is_stack_pop());
        assert!(b_out[4].target.is_stack_pop());

        assert!(b_out[0].is_transition());
        assert!(b_out[1].is_transition());
        assert!(b_out[2].is_transition());
        assert!(b_out[3].is_transition());
        assert!(b_out[4].is_change_state());

        sm.pop();
        let a_info = sm.state().info();
        assert_eq!(a_info.name(), "A");
        assert!(!a_info.is_stack_pop());
    }

    /// Test that the values of state variables accessed via the runtime interface are correct
    /// after a pop transition.
    #[test]
    fn runtime_state_after_pop() {
        let mut sm = StateContextStack::new();
        sm.inc();
        sm.inc(); // x = 2
        sm.push();
        sm.to_b();
        sm.inc(); // y = 5
        sm.push();
        sm.inc(); // y = 10

        assert_eq!(
            *sm.state()
                .variables()
                .lookup("y")
                .unwrap()
                .downcast_ref::<i32>()
                .unwrap(),
            10
        );
        assert!(sm.state().variables().lookup("x").is_none());
        assert!(sm.state().variables().lookup("z").is_none());

        sm.to_c();
        sm.inc();
        sm.inc(); // z = 20

        assert_eq!(
            *sm.state()
                .variables()
                .lookup("z")
                .unwrap()
                .downcast_ref::<i32>()
                .unwrap(),
            20
        );
        assert!(sm.state().variables().lookup("x").is_none());
        assert!(sm.state().variables().lookup("y").is_none());

        sm.pop();

        assert_eq!(
            *sm.state()
                .variables()
                .lookup("y")
                .unwrap()
                .downcast_ref::<i32>()
                .unwrap(),
            5
        );
        assert!(sm.state().variables().lookup("x").is_none());
        assert!(sm.state().variables().lookup("z").is_none());

        sm.pop();

        assert_eq!(
            *sm.state()
                .variables()
                .lookup("x")
                .unwrap()
                .downcast_ref::<i32>()
                .unwrap(),
            2
        );
        assert!(sm.state().variables().lookup("y").is_none());
        assert!(sm.state().variables().lookup("z").is_none());
    }
}
