//! This module provides infrastructure to support registering and invoking callbacks that notify
//! clients of events within a running state machine.

use crate::info::TransitionInfo;
use crate::live::*;
use std::collections::VecDeque;
use std::rc::Rc;

/// Event manager.
pub struct EventMonitor<'c> {
    events_capacity: Option<usize>,
    transitions_capacity: Option<usize>,
    events: VecDeque<Rc<dyn MethodInstance>>,
    transitions: VecDeque<TransitionInstance>,
    transition_callbacks: Vec<Box<dyn FnMut(&TransitionInstance) + Send + 'c>>,
}

impl<'c> EventMonitor<'c> {
    /// Create a new event monitor. The arguments indicate the number of events and transitions to
    /// maintain as history.
    pub fn new(events_capacity: Option<usize>, transitions_capacity: Option<usize>) -> Self {
        EventMonitor {
            events_capacity,
            transitions_capacity,
            events: new_deque(&events_capacity),
            transitions: new_deque(&transitions_capacity),
            transition_callbacks: Vec::new(),
        }
    }

    /// Register a callback to be called on each transition.
    pub fn add_transition_callback(
        &mut self,
        callback: impl FnMut(&TransitionInstance) + Send + 'c,
    ) {
        self.transition_callbacks.push(Box::new(callback));
    }

    /// Track that a Frame event occurred, calling any relevant callbacks and saving it to the
    /// history. This method should not be directly invoked for enter/exit events. Instead,
    /// process the entire transition via `transition_occurred` (which calls this method for the
    /// enter/exit events).
    pub fn event_occurred(&mut self, event: Rc<dyn MethodInstance>) {
        push_to_deque(&self.events_capacity, &mut self.events, event);
    }

    /// Track that a transition occurred with the provided arguments, calling all of the transition
    /// callbacks and saving it to the history.
    pub fn transition_occurred(
        &mut self,
        info: &'static TransitionInfo,
        old_state: Rc<dyn StateInstance>,
        new_state: Rc<dyn StateInstance>,
        exit_event: Option<Rc<dyn MethodInstance>>,
        enter_event: Option<Rc<dyn MethodInstance>>,
    ) {
        if let Some(event) = &exit_event {
            self.event_occurred(event.clone())
        }
        if let Some(event) = &enter_event {
            self.event_occurred(event.clone())
        }
        let transition = TransitionInstance {
            info,
            old_state,
            new_state,
            exit_event,
            enter_event,
        };
        push_to_deque(
            &self.transitions_capacity,
            &mut self.transitions,
            transition.clone(),
        );
        for c in &mut self.transition_callbacks {
            (**c)(&transition);
        }
    }

    /// Get the event history.
    pub fn event_history(&self) -> &VecDeque<Rc<dyn MethodInstance>> {
        &self.events
    }

    /// Get the transition history.
    pub fn transition_history(&self) -> &VecDeque<TransitionInstance> {
        &self.transitions
    }

    /// Clear the event history.
    pub fn clear_event_history(&mut self) {
        self.events = new_deque(&self.events_capacity);
    }

    /// Clear the transition history.
    pub fn clear_transition_history(&mut self) {
        self.transitions = new_deque(&self.transitions_capacity);
    }

    /// Set the number of events to maintain in the history. If `None`, the number of elements is
    /// unlimited.
    pub fn set_event_history_capacity(&mut self, capacity: Option<usize>) {
        resize_deque(&capacity, &mut self.events);
        self.events_capacity = capacity;
    }

    /// Set the number of transitions to maintain in the history. If `None`, the number of elements
    /// is unlimited.
    pub fn set_transition_history_capacity(&mut self, capacity: Option<usize>) {
        resize_deque(&capacity, &mut self.transitions);
        self.transitions_capacity = capacity;
    }

    /// Get the most recent transition. This will return `None` if either the state machine has not
    /// transitioned yet or if the capacity of the transition history is set to 0.
    pub fn last_transition(&self) -> Option<&TransitionInstance> {
        self.transitions.back()
    }
}

impl<'c> Default for EventMonitor<'c> {
    fn default() -> Self {
        EventMonitor::new(Some(0), Some(1))
    }
}

/// Helper function to add an element to a possibly finite-sized deque.
fn push_to_deque<T>(capacity: &Option<usize>, deque: &mut VecDeque<T>, elem: T) {
    match *capacity {
        Some(cap) => {
            if cap > 0 {
                if deque.len() >= cap {
                    deque.pop_front();
                }
                deque.push_back(elem);
            }
        }
        None => deque.push_back(elem),
    };
}

/// Helper function to resize a possibly finite-sized deque.
fn resize_deque<T>(new_capacity: &Option<usize>, deque: &mut VecDeque<T>) {
    if let Some(cap) = *new_capacity {
        if deque.len() < cap {
            deque.reserve_exact(cap - deque.len());
        }
        while deque.len() > cap {
            deque.pop_front();
        }
    }
}

/// Helper function to create a possibly finite-sized deque.
fn new_deque<T>(capacity: &Option<usize>) -> VecDeque<T> {
    match *capacity {
        Some(cap) => VecDeque::with_capacity(cap),
        None => VecDeque::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::info::StateInfo;
    use std::sync::Mutex;

    mod info {
        use crate::info::*;
        use once_cell::sync::OnceCell;

        pub fn machine() -> &'static MachineInfo {
            if MACHINE_CELL.get().is_none() {
                let _ = MACHINE_CELL.set(MACHINE);
            }
            MACHINE
        }

        static MACHINE: &MachineInfo = &MachineInfo {
            name: "TestMachine",
            variables: &[],
            states: &[STATE_A, STATE_B],
            interface: &[EVENTS[0]],
            actions: ACTIONS,
            events: EVENTS,
            transitions: TRANSITIONS,
        };
        static MACHINE_CELL: OnceCell<&MachineInfo> = OnceCell::new();
        static STATE_A: &StateInfo = &StateInfo {
            machine_cell: &MACHINE_CELL,
            name: "A",
            parent: None,
            parameters: &[],
            variables: &[],
            handlers: &[EVENTS[0]],
            is_stack_pop: false,
        };
        static STATE_B: &StateInfo = &StateInfo {
            machine_cell: &MACHINE_CELL,
            name: "B",
            parent: None,
            parameters: &[],
            variables: &[],
            handlers: &[EVENTS[0]],
            is_stack_pop: false,
        };
        const ACTIONS: &[&MethodInfo] = &[];
        const EVENTS: &[&MethodInfo] = &[
            &MethodInfo {
                name: "next",
                parameters: &[],
                return_type: None,
            },
            &MethodInfo {
                name: "A:>",
                parameters: &[],
                return_type: None,
            },
            &MethodInfo {
                name: "B:>",
                parameters: &[],
                return_type: None,
            },
            &MethodInfo {
                name: "A:<",
                parameters: &[],
                return_type: None,
            },
            &MethodInfo {
                name: "B:<",
                parameters: &[],
                return_type: None,
            },
        ];
        static TRANSITIONS: &[&TransitionInfo] = &[
            &TransitionInfo {
                id: 0,
                kind: TransitionKind::Transition,
                event: EVENTS[0],
                label: "",
                source: STATE_A,
                target: STATE_B,
            },
            &TransitionInfo {
                id: 1,
                kind: TransitionKind::ChangeState,
                event: EVENTS[0],
                label: "",
                source: STATE_B,
                target: STATE_A,
            },
        ];
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
    enum TestState {
        A,
        B,
    }

    impl StateInstance for TestState {
        fn info(&self) -> &'static StateInfo {
            match self {
                TestState::A => info::machine().states[0],
                TestState::B => info::machine().states[1],
            }
        }
    }

    #[test]
    fn callbacks_are_called() {
        let tape: Vec<String> = Vec::new();
        let tape_mutex = Mutex::new(tape);
        let mut em = EventMonitor::default();
        em.add_transition_callback(|e| {
            tape_mutex
                .lock()
                .unwrap()
                .push(format!("old: {}", e.old_state.info().name))
        });
        em.add_transition_callback(|e| {
            tape_mutex
                .lock()
                .unwrap()
                .push(format!("new: {}", e.new_state.info().name))
        });
        em.add_transition_callback(|e| {
            tape_mutex
                .lock()
                .unwrap()
                .push(format!("kind: {:?}", e.info.kind))
        });

        let a_rc = Rc::new(TestState::A);
        let b_rc = Rc::new(TestState::B);
        em.transition_occurred(
            info::machine().transitions[0],
            a_rc.clone(),
            b_rc.clone(),
            None,
            None,
        );
        assert_eq!(
            *tape_mutex.lock().unwrap(),
            vec!["old: A", "new: B", "kind: Transition"]
        );
        tape_mutex.lock().unwrap().clear();

        em.transition_occurred(info::machine().transitions[1], b_rc, a_rc, None, None);
        assert_eq!(
            *tape_mutex.lock().unwrap(),
            vec!["old: B", "new: A", "kind: ChangeState"]
        );
    }
}
