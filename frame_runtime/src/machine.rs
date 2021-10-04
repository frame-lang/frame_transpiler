//! This module provides a generic interface to a running state machine.

use crate::callback::CallbackManager;
use crate::environment::Environment;
use crate::state::State;
use std::cell::Ref;

/// An interface to a running state machine that supports inspecting its
/// current state and variables, and registering callbacks to be notified of
/// various events.
pub trait StateMachine<'a> {
    /// The current state of this machine.
    fn current_state(&self) -> Ref<dyn State>;

    /// The domain variables associated with this machine.
    fn domain_variables(&self) -> &dyn Environment;

    /// The callback manager for this state machine.
    fn callback_manager(&mut self) -> &mut CallbackManager<'a>;
}

#[cfg(test)]
/// The example below illustrates the implementation of the runtime interface
/// for a basic state machine without state variables or arguments. This case
/// requires different treatment from a more featureful state machine since
/// runtime states are realized differently depending on whether or not the
/// state context types are generated. This example combined with the example
/// in `tests/demo.rs` illustrate the two major varieties of runtime support.
mod tests {
    use super::*;
    use crate::environment::EMPTY;
    use std::any::Any;
    use std::cell::RefCell;
    use std::sync::Mutex;

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
    enum TestState {
        A,
        B,
    }

    impl State for TestState {
        fn name(&self) -> &'static str {
            match self {
                TestState::A => "A",
                TestState::B => "B",
            }
        }
        fn state_arguments(&self) -> &dyn Environment {
            EMPTY
        }
        fn state_variables(&self) -> &dyn Environment {
            EMPTY
        }
    }

    struct TestMachine<'a> {
        state: TestState,
        state_cell: RefCell<TestState>,
        callback_manager: CallbackManager<'a>,
    }

    impl<'a> Environment for TestMachine<'a> {
        fn lookup(&self, name: &str) -> Option<&dyn Any> {
            EMPTY.lookup(name)
        }
    }

    impl<'a> StateMachine<'a> for TestMachine<'a> {
        fn current_state(&self) -> Ref<dyn State> {
            Ref::map(self.state_cell.borrow(), |s| s as &dyn State)
        }
        fn domain_variables(&self) -> &dyn Environment {
            self
        }
        fn callback_manager(&mut self) -> &mut CallbackManager<'a> {
            &mut self.callback_manager
        }
    }

    impl<'a> TestMachine<'a> {
        pub fn new() -> TestMachine<'a> {
            TestMachine {
                state: TestState::A,
                state_cell: RefCell::new(TestState::A),
                callback_manager: CallbackManager::new(),
            }
        }

        pub fn next(&mut self) {
            match self.state {
                TestState::A => self.transition(TestState::B),
                TestState::B => self.transition(TestState::A),
            }
        }

        pub fn transition(&mut self, new_state: TestState) {
            let old_state_cell = RefCell::new(self.state);
            let old_runtime_state = old_state_cell.borrow();
            self.state = new_state;
            self.state_cell = RefCell::new(new_state);
            let new_runtime_state = self.state_cell.borrow();
            self.callback_manager
                .transition(old_runtime_state, new_runtime_state, EMPTY, EMPTY);
        }
    }

    #[test]
    fn current_state() {
        let mut sm = TestMachine::new();
        assert_eq!("A", sm.current_state().name());
        sm.next();
        assert_eq!("B", sm.current_state().name());
        sm.next();
        assert_eq!("A", sm.current_state().name());
    }

    #[test]
    fn transition_callbacks() {
        let tape: Vec<String> = Vec::new();
        let tape_mutex = Mutex::new(tape);
        let mut sm = TestMachine::new();
        sm.callback_manager().add_transition_callback(|i| {
            tape_mutex.lock().unwrap().push(format!(
                "{}->{}",
                i.old_state.name(),
                i.new_state.name()
            ));
        });
        sm.next();
        sm.next();
        sm.next();
        assert_eq!(*tape_mutex.lock().unwrap(), vec!["A->B", "B->A", "A->B"]);
    }
}
