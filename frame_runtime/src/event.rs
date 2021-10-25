//! This module provides infrastructure to support registering and invoking callbacks that notify
//! clients of events within a running state machine.

use crate::env::Environment;
use crate::info::TransitionInfo;
use crate::live::StateInstance;
use std::cell::Ref;

/// An event indicating a transition between two states.
pub struct TransitionEvent<'a> {
    /// Information about the transition statement that triggered this event.
    pub info: &'static TransitionInfo,

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
        info: &'static TransitionInfo,
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
    use crate::info::*;
    use std::cell::RefCell;
    use std::sync::Mutex;

    const MACHINE_INFO: &MachineInfo = &MachineInfo {
        name: "Test",
        variables: vec![],
        states: vec![
            StateInfo {
                machine: &MACHINE_INFO,
                name: "A",
                parent: None,
                parameters: vec![],
                variables: vec![],
                handlers: vec![&MACHINE_INFO.events[0]],
            },
            StateInfo {
                machine: &MACHINE_INFO,
                name: "B",
                parent: None,
                parameters: vec![],
                variables: vec![],
                handlers: vec![&MACHINE_INFO.events[0]],
            },
        ],
        events: vec![MethodInfo {
            name: "next",
            parameters: vec![],
            return_type: None,
        }],
        actions: vec![],
        transitions: vec![
            TransitionInfo {
                kind: TransitionKind::Transition,
                event: &MACHINE_INFO.events[0],
                label: "",
                source: &MACHINE_INFO.states[0],
                target: &MACHINE_INFO.states[1],
            },
            TransitionInfo {
                kind: TransitionKind::ChangeState,
                event: &MACHINE_INFO.events[0],
                label: "",
                source: &MACHINE_INFO.states[1],
                target: &MACHINE_INFO.states[0],
            },
        ],
    };

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
    enum TestState {
        A,
        B,
    }

    impl StateInstance for TestState {
        fn info(&self) -> &StateInfo {
            match self {
                TestState::A => &MACHINE_INFO.states[0],
                TestState::B => &MACHINE_INFO.states[1],
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
                .push(format!("old: {}", e.old_state.info().name))
        });
        cm.add_transition_callback(|e| {
            tape_mutex
                .lock()
                .unwrap()
                .push(format!("new: {}", e.new_state.info().name))
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
            &MACHINE_INFO.transitions[0],
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
            &MACHINE_INFO.transitions[1],
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
