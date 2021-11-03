//! This module provides infrastructure to support registering and invoking callbacks that notify
//! clients of events within a running state machine.

use crate::env::Environment;
use crate::info::TransitionInfo;
use crate::live::StateInstance;
use std::cell::Ref;

/// An event indicating a transition between two states.
pub struct TransitionEvent<'a> {
    /// Information about the transition statement that triggered this event.
    pub info: &'a TransitionInfo,

    /// The source state instance immediately before the transition.
    pub old_state: Ref<'a, dyn StateInstance>,

    /// The target state instance immediately after the transition.
    pub new_state: Ref<'a, dyn StateInstance>,

    /// Arguments to the exit handler of the source state.
    pub exit_arguments: &'a dyn Environment,

    /// Arguments to the enter handler of the target state.
    pub enter_arguments: &'a dyn Environment,
}

/// Callback manager.
pub struct CallbackManager<'c> {
    transition_callbacks: Vec<Box<dyn FnMut(&TransitionEvent) + Send + 'c>>,
}

impl<'c, 't> CallbackManager<'c> {
    /// Create a new callback manager.
    pub fn new() -> Self {
        CallbackManager {
            transition_callbacks: Vec::new(),
        }
    }

    /// Register a callback to be called on each transition.
    pub fn add_transition_callback(&mut self, callback: impl FnMut(&TransitionEvent) + Send + 'c) {
        self.transition_callbacks.push(Box::new(callback));
    }

    /// Generate a transition event with the provided arguments and invoke all of the transition
    /// callbacks.
    pub fn transition(
        &mut self,
        info: &'t TransitionInfo,
        old_state: Ref<'t, dyn StateInstance>,
        new_state: Ref<'t, dyn StateInstance>,
        exit_arguments: &'t dyn Environment,
        enter_arguments: &'t dyn Environment,
    ) {
        let event = TransitionEvent {
            info,
            old_state,
            new_state,
            exit_arguments,
            enter_arguments,
        };
        for c in &mut self.transition_callbacks {
            (**c)(&event);
        }
    }
}

impl<'c> Default for CallbackManager<'c> {
    fn default() -> Self {
        CallbackManager::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::EMPTY;
    use crate::info::{MachineInfo, StateInfo};
    use std::cell::RefCell;
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
                        id: 0,
                        kind: TransitionKind::Transition,
                        event: MACHINE.events()[0].clone(),
                        label: "",
                        source: MACHINE.states()[0],
                        target: MACHINE.states()[1],
                    },
                    TransitionInfo {
                        id: 1,
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

    #[test]
    fn callbacks_are_called() {
        let tape: Vec<String> = Vec::new();
        let tape_mutex = Mutex::new(tape);
        let mut cm = CallbackManager::new();
        cm.add_transition_callback(|e| {
            tape_mutex
                .lock()
                .unwrap()
                .push(format!("old: {}", e.old_state.info().name()))
        });
        cm.add_transition_callback(|e| {
            tape_mutex
                .lock()
                .unwrap()
                .push(format!("new: {}", e.new_state.info().name()))
        });
        cm.add_transition_callback(|e| {
            tape_mutex
                .lock()
                .unwrap()
                .push(format!("kind: {:?}", e.info.kind))
        });

        let a_rc = RefCell::new(TestState::A);
        let b_rc = RefCell::new(TestState::B);
        cm.transition(
            &info::MACHINE.transitions()[0],
            a_rc.borrow(),
            b_rc.borrow(),
            EMPTY,
            EMPTY,
        );
        assert_eq!(
            *tape_mutex.lock().unwrap(),
            vec!["old: A", "new: B", "kind: Transition"]
        );
        tape_mutex.lock().unwrap().clear();

        cm.transition(
            &info::MACHINE.transitions()[1],
            b_rc.borrow(),
            a_rc.borrow(),
            EMPTY,
            EMPTY,
        );
        assert_eq!(
            *tape_mutex.lock().unwrap(),
            vec!["old: B", "new: A", "kind: ChangeState"]
        );
    }
}
