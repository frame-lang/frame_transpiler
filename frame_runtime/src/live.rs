//! This module defines traits that enable reflecting on a running state machine.

use crate::env::{Environment, EMPTY};
use crate::event::CallbackManager;
use crate::info::*;
use std::cell::Ref;

/// An interface to a running state machine that supports inspecting its current state and
/// variables, and registering callbacks to be notified of various events.
pub trait MachineInstance<'a> {
    /// Static nformation about the state machine declaration that gave rise to this machine
    /// instance.
    fn info(&self) -> &'static dyn MachineInfo;

    /// The currently active state of this machine.
    fn state(&self) -> Ref<dyn StateInstance>;

    /// Environment containing the current values of the domain variables associated with this
    /// machine.
    fn variables(&self) -> &dyn Environment {
        EMPTY
    }

    /// The callback manager for this state machine.
    fn callback_manager(&mut self) -> &mut CallbackManager<'a>;
}

/// A snapshot of an active state within a running state machine. State arguments and variables are
/// not saved between visits, so these names are bound to values only when the state is "active". A
/// state is active when it is the current state or when it is immediately involved in a
/// transition.
pub trait StateInstance {
    /// Static information about the state declaration that gave rise to this state instance.
    fn info(&self) -> &'static dyn StateInfo;

    /// Environment containing the values of the state arguments passed to this state on
    /// transition. The names and types of the parameters these arguments are bound to can be found
    /// in `self.info().parameters`.
    fn arguments(&self) -> &dyn Environment {
        EMPTY
    }

    /// Environment containing the current values of the variables associated with this state. The
    /// names and types of the variables can be found in `self.info().variables`.
    fn variables(&self) -> &dyn Environment {
        EMPTY
    }
}

/// The example below illustrates the implementation of the runtime interface
/// for a basic state machine without state variables or arguments. This case
/// requires different treatment from a more featureful state machine since
/// runtime states are realized differently depending on whether or not the
/// state context types are generated. This example combined with the example
/// in `tests/demo.rs` illustrate the two major varieties of runtime support.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::info::{MachineInfo, StateInfo};
    use std::cell::RefCell;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Mutex;

    mod info {
        use crate::info::*;

        pub struct Machine {}
        pub struct StateA {}
        pub struct StateB {}
        pub const MACHINE: &Machine = &Machine {};
        pub const STATE_A: &StateA = &StateA {};
        pub const STATE_B: &StateB = &StateB {};

        impl MachineInfo for Machine {
            fn name(&self) -> &'static str {
                "TestMachine"
            }
            fn variables(&self) -> Vec<NameInfo> {
                vec![]
            }
            fn states(&self) -> Vec<&dyn StateInfo> {
                vec![STATE_A, STATE_B]
            }
            fn interface(&self) -> Vec<MethodInfo> {
                vec![MACHINE.events()[0].clone()]
            }
            fn actions(&self) -> Vec<MethodInfo> {
                vec![]
            }
            fn events(&self) -> Vec<MethodInfo> {
                vec![
                    MethodInfo {
                        name: "next",
                        parameters: vec![],
                        return_type: None,
                    },
                    MethodInfo {
                        name: "A:>",
                        parameters: vec![],
                        return_type: None,
                    },
                    MethodInfo {
                        name: "A:<",
                        parameters: vec![],
                        return_type: None,
                    },
                    MethodInfo {
                        name: "B:>",
                        parameters: vec![],
                        return_type: None,
                    },
                    MethodInfo {
                        name: "B:<",
                        parameters: vec![],
                        return_type: None,
                    },
                ]
            }
            fn transitions(&self) -> Vec<TransitionInfo> {
                vec![
                    TransitionInfo {
                        kind: TransitionKind::Transition,
                        event: MACHINE.events()[0].clone(),
                        label: "",
                        source: MACHINE.states()[0],
                        target: MACHINE.states()[1],
                    },
                    TransitionInfo {
                        kind: TransitionKind::ChangeState,
                        event: MACHINE.events()[0].clone(),
                        label: "",
                        source: MACHINE.states()[1],
                        target: MACHINE.states()[0],
                    },
                ]
            }
        }

        impl StateInfo for StateA {
            fn machine(&self) -> &dyn MachineInfo {
                MACHINE
            }
            fn name(&self) -> &'static str {
                "A"
            }
            fn parent(&self) -> Option<&dyn StateInfo> {
                None
            }
            fn parameters(&self) -> Vec<NameInfo> {
                vec![]
            }
            fn variables(&self) -> Vec<NameInfo> {
                vec![]
            }
            fn handlers(&self) -> Vec<MethodInfo> {
                vec![MACHINE.events()[0].clone()]
            }
        }

        impl StateInfo for StateB {
            fn machine(&self) -> &dyn MachineInfo {
                MACHINE
            }
            fn name(&self) -> &'static str {
                "B"
            }
            fn parent(&self) -> Option<&dyn StateInfo> {
                None
            }
            fn parameters(&self) -> Vec<NameInfo> {
                vec![]
            }
            fn variables(&self) -> Vec<NameInfo> {
                vec![]
            }
            fn handlers(&self) -> Vec<MethodInfo> {
                vec![MACHINE.events()[0].clone()]
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
    enum TestState {
        A,
        B,
    }

    impl StateInstance for TestState {
        fn info(&self) -> &'static dyn StateInfo {
            match self {
                TestState::A => info::STATE_A,
                TestState::B => info::STATE_B,
            }
        }
    }

    struct TestMachine<'a> {
        state: TestState,
        state_cell: RefCell<TestState>,
        callback_manager: CallbackManager<'a>,
    }

    impl<'a> MachineInstance<'a> for TestMachine<'a> {
        fn info(&self) -> &'static dyn MachineInfo {
            info::MACHINE
        }
        fn state(&self) -> Ref<dyn StateInstance> {
            Ref::map(self.state_cell.borrow(), |s| s as &dyn StateInstance)
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
                TestState::A => self.transition(&info::MACHINE.transitions()[0], TestState::B),
                TestState::B => self.transition(&info::MACHINE.transitions()[1], TestState::A),
            }
        }

        pub fn transition(&mut self, transition_info: &TransitionInfo, new_state: TestState) {
            let old_state_cell = RefCell::new(self.state);
            let old_runtime_state = old_state_cell.borrow();
            self.state = new_state;
            self.state_cell = RefCell::new(new_state);
            let new_runtime_state = self.state_cell.borrow();
            self.callback_manager.transition(
                transition_info,
                old_runtime_state,
                new_runtime_state,
                EMPTY,
                EMPTY,
            );
        }
    }

    #[test]
    fn static_info() {
        let sm = TestMachine::new();
        assert_eq!("TestMachine", sm.info().name());
        assert_eq!(0, sm.info().variables().len());
        assert_eq!(2, sm.info().states().len());
        assert_eq!(1, sm.info().interface().len());
        assert_eq!(0, sm.info().actions().len());
        assert_eq!(5, sm.info().events().len());
        assert_eq!(2, sm.info().transitions().len());
    }

    #[test]
    fn current_state() {
        let mut sm = TestMachine::new();
        assert_eq!("A", sm.state().info().name());
        sm.next();
        assert_eq!("B", sm.state().info().name());
        sm.next();
        assert_eq!("A", sm.state().info().name());
    }

    #[test]
    fn transition_callbacks() {
        let tape: Vec<String> = Vec::new();
        let tape_mutex = Mutex::new(tape);
        let mut sm = TestMachine::new();
        sm.callback_manager().add_transition_callback(|e| {
            tape_mutex.lock().unwrap().push(format!(
                "{}{}{}",
                e.old_state.info().name(),
                match e.info.kind {
                    TransitionKind::ChangeState => "->>",
                    TransitionKind::Transition => "->",
                },
                e.new_state.info().name()
            ));
        });
        sm.next();
        sm.next();
        sm.next();
        assert_eq!(*tape_mutex.lock().unwrap(), vec!["A->B", "B->>A", "A->B"]);
    }

    #[test]
    fn transition_static_info_agrees() {
        let agree = AtomicBool::new(false);
        let mut sm = TestMachine::new();
        sm.callback_manager().add_transition_callback(|e| {
            agree.store(
                e.info.source.name() == e.old_state.info().name()
                    && e.info.target.name() == e.new_state.info().name(),
                Ordering::Relaxed,
            );
        });
        sm.next();
        assert!(agree.load(Ordering::Relaxed));
        sm.next();
        assert!(agree.load(Ordering::Relaxed));
        sm.next();
        assert!(agree.load(Ordering::Relaxed));
    }
}
