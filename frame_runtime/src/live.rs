//! This module defines traits that enable reflecting on a running state machine.

use crate::env::{Empty, Environment};
use crate::event::EventMonitor;
use crate::info::*;
use std::any::Any;
use std::cell::Ref;
use std::rc::Rc;

/// An interface to a running state machine that supports inspecting its current state and
/// variables, and registering callbacks to be notified of various events.
pub trait MachineInstance<'a> {
    /// Static information about the state machine declaration that gave rise to this machine
    /// instance.
    fn info(&self) -> &'static MachineInfo;

    /// The currently active state of this machine.
    fn state(&self) -> Rc<dyn StateInstance>;

    /// Environment containing the current values of the domain variables associated with this
    /// machine. The variable names can be obtained from `variable_declarations`.
    fn variables(&self) -> &dyn Environment;

    /// Get an immutable reference to this machine's event monitor, suitable for querying the
    /// transition/event history.
    fn event_monitor(&self) -> &EventMonitor<'a>;

    /// Get a mutable reference to this machine's event monitor, suitable for registering callbacks
    /// to be notified of Frame events.
    fn event_monitor_mut(&mut self) -> &mut EventMonitor<'a>;
}

/// A snapshot of an active state within a running state machine. State arguments and variables are
/// not saved between visits, so these names are bound to values only when the state is "active". A
/// state is active when it is the current state or when it is immediately involved in a
/// transition.
pub trait StateInstance {
    /// Static information about the state declaration that gave rise to this state instance.
    fn info(&self) -> &'static StateInfo;

    /// Environment containing the values of the state arguments passed to this state on
    /// transition. The names and types of the parameters these arguments are bound to can be found
    /// at `self.info().parameters`.
    fn arguments(&self) -> Rc<dyn Environment> {
        Empty::new_rc()
    }

    /// Environment containing the current values of the variables associated with this state. The
    /// names and types of the variables can be found at `self.info().variables`.
    fn variables(&self) -> Rc<dyn Environment> {
        Empty::new_rc()
    }
}

/// A method instance represents a particular invocation of an event or action.
pub trait MethodInstance {
    /// The signature of the event that occurred
    fn info(&self) -> &MethodInfo;

    /// The arguments passed to this method. The names and types of the parameters these arguments
    /// are bound to can be found at `self.info().parameters`.
    fn arguments(&self) -> Rc<dyn Environment> {
        Empty::new_rc()
    }

    /// The return value, if any, whose type can be found at `self.info().return_type`.
    fn return_value(&self) -> Option<Ref<dyn Any>> {
        None
    }
}

/// Captures the occurrence of a transition between two states.
#[derive(Clone)]
pub struct TransitionInstance {
    /// Information about the transition statement that triggered this transition.
    pub info: &'static TransitionInfo,

    /// The source state instance immediately before the transition.
    pub old_state: Rc<dyn StateInstance>,

    /// The target state instance immediately after the transition.
    pub new_state: Rc<dyn StateInstance>,

    /// The exit event sent to the source state. Will be `None` for a change-state transition.
    pub exit_event: Option<Rc<dyn MethodInstance>>,

    /// The enter event sent to the target state. Will be `None` for a change-state transition.
    pub enter_event: Option<Rc<dyn MethodInstance>>,
}

impl TransitionInstance {
    /// Create a transition instance for a standard transition with exit/enter events.
    pub fn transition(
        info: &'static TransitionInfo,
        old_state: Rc<dyn StateInstance>,
        new_state: Rc<dyn StateInstance>,
        exit_event: Rc<dyn MethodInstance>,
        enter_event: Rc<dyn MethodInstance>,
    ) -> TransitionInstance {
        TransitionInstance {
            info,
            old_state,
            new_state,
            exit_event: Some(exit_event),
            enter_event: Some(enter_event),
        }
    }

    /// Create a transition instance for a change-state transition without exit/enter events.
    pub fn change_state(
        info: &'static TransitionInfo,
        old_state: Rc<dyn StateInstance>,
        new_state: Rc<dyn StateInstance>,
    ) -> TransitionInstance {
        TransitionInstance {
            info,
            old_state,
            new_state,
            exit_event: None,
            enter_event: None,
        }
    }

    /// Get the arguments from the exit event or an empty environment if there is no exit event.
    pub fn exit_arguments(&self) -> Rc<dyn Environment> {
        match &self.exit_event {
            Some(event) => event.arguments().clone(),
            None => Empty::new_rc(),
        }
    }

    /// Get the arguments from the enter event or an empty environment if there is no enter event.
    pub fn enter_arguments(&self) -> Rc<dyn Environment> {
        match &self.enter_event {
            Some(event) => event.arguments().clone(),
            None => Empty::new_rc(),
        }
    }
}

/// The example below illustrates the implementation of the runtime interface for a basic state
/// machine without state variables or arguments. This case requires different treatment from a
/// more featureful state machine since runtime states are realized differently depending on
/// whether or not the state context types are generated. This example combined with the example
/// in `tests/demo.rs` illustrate the two major varieties of runtime support.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::info::{MachineInfo, StateInfo};
    use std::sync::atomic::{AtomicBool, Ordering};
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

    struct TestMachine<'a> {
        state: TestState,
        state_rc: Rc<TestState>,
        event_monitor: EventMonitor<'a>,
    }

    impl<'a> Environment for TestMachine<'a> {
        fn is_empty(&self) -> bool {
            true
        }
        fn lookup(&self, _name: &str) -> Option<&dyn Any> {
            None
        }
    }

    impl<'a> MachineInstance<'a> for TestMachine<'a> {
        fn info(&self) -> &'static MachineInfo {
            info::machine()
        }
        fn state(&self) -> Rc<dyn StateInstance> {
            self.state_rc.clone()
        }
        fn variables(&self) -> &dyn Environment {
            self
        }
        fn event_monitor(&self) -> &EventMonitor<'a> {
            &self.event_monitor
        }
        fn event_monitor_mut(&mut self) -> &mut EventMonitor<'a> {
            &mut self.event_monitor
        }
    }

    impl<'a> TestMachine<'a> {
        pub fn new() -> TestMachine<'a> {
            TestMachine {
                state: TestState::A,
                state_rc: Rc::new(TestState::A),
                event_monitor: EventMonitor::default(),
            }
        }

        pub fn next(&mut self) {
            match self.state {
                TestState::A => self.transition(info::machine().transitions[0], TestState::B),
                TestState::B => self.transition(info::machine().transitions[1], TestState::A),
            }
        }

        pub fn transition(
            &mut self,
            transition_info: &'static TransitionInfo,
            new_state: TestState,
        ) {
            let old_state_rc = self.state_rc.clone();
            self.state = new_state;
            self.state_rc = Rc::new(new_state);
            self.event_monitor
                .transition_occurred(TransitionInstance::change_state(
                    transition_info,
                    old_state_rc,
                    self.state_rc.clone(),
                ));
        }
    }

    #[test]
    fn static_info() {
        let sm = TestMachine::new();
        assert_eq!("TestMachine", sm.info().name);
        assert_eq!(0, sm.info().variables.len());
        assert_eq!(2, sm.info().states.len());
        assert_eq!(1, sm.info().interface.len());
        assert_eq!(0, sm.info().actions.len());
        assert_eq!(5, sm.info().events.len());
        assert_eq!(2, sm.info().transitions.len());
    }

    #[test]
    fn current_state() {
        let mut sm = TestMachine::new();
        assert_eq!("A", sm.state().info().name);
        sm.next();
        assert_eq!("B", sm.state().info().name);
        sm.next();
        assert_eq!("A", sm.state().info().name);
    }

    #[test]
    fn transition_callbacks() {
        let tape: Vec<String> = Vec::new();
        let tape_mutex = Mutex::new(tape);
        let mut sm = TestMachine::new();
        sm.event_monitor_mut().add_transition_callback(|e| {
            tape_mutex.lock().unwrap().push(format!(
                "{}{}{}",
                e.old_state.info().name,
                match e.info.kind {
                    TransitionKind::ChangeState => "->>",
                    TransitionKind::Transition => "->",
                },
                e.new_state.info().name
            ));
        });
        sm.next();
        sm.next();
        sm.next();
        assert_eq!(*tape_mutex.lock().unwrap(), vec!["A->B", "B->>A", "A->B"]);
    }

    #[test]
    fn transition_info_id() {
        let tape: Vec<usize> = Vec::new();
        let tape_mutex = Mutex::new(tape);
        let mut sm = TestMachine::new();
        sm.event_monitor_mut().add_transition_callback(|e| {
            tape_mutex.lock().unwrap().push(e.info.id);
        });
        sm.next();
        sm.next();
        sm.next();
        assert_eq!(*tape_mutex.lock().unwrap(), vec![0, 1, 0]);
    }

    #[test]
    fn transition_static_info_agrees() {
        let agree = AtomicBool::new(false);
        let mut sm = TestMachine::new();
        sm.event_monitor_mut().add_transition_callback(|e| {
            agree.store(
                e.info.source.name == e.old_state.info().name
                    && e.info.target.name == e.new_state.info().name,
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
