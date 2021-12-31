//! This file illustrates what generated code that realizes the runtime interface should look like
//! for a basic state machine without state variables or arguments. This case requires different
//! treatment from a more featureful state machine since runtime states are realized differently
//! depending on whether or not the state context types are generated. This example combined with
//! the example in `tests/demo.rs` illustrate the two major varieties of runtime support.
//!
//! The organization of the code in this file is slightly different than what Framec would produce
//! in order to support testing both the `sync` and `unsync` variants of the library without too
//! much redundancy.
//!
//! Frame spec:
//! ```
//! #Simple
//!     -interface-
//!     next
//!     -machine-
//!     $A
//!         |next|
//!             -> $B ^
//!     $B
//!         |next|
//!             -> $B ^
//!     -actions-
//!     -domain-
//! ```

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum TestState {
    A,
    B,
}

mod info {
    use frame_runtime::info::*;
    use once_cell::sync::OnceCell;

    pub fn machine() -> &'static MachineInfo {
        if MACHINE_CELL.get().is_none() {
            let _ = MACHINE_CELL.set(MACHINE);
        }
        MACHINE
    }

    static MACHINE: &MachineInfo = &MachineInfo {
        name: "Simple",
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

mod sync {
    use super::*;
    use frame_runtime::sync as runtime;
    use std::any::Any;
    use std::sync::Arc;

    impl runtime::State<runtime::EnvironmentPtr> for TestState {
        fn info(&self) -> &'static runtime::StateInfo {
            match self {
                TestState::A => info::machine().states[0],
                TestState::B => info::machine().states[1],
            }
        }
        fn arguments(&self) -> runtime::EnvironmentPtr {
            runtime::Empty::arc()
        }
        fn variables(&self) -> runtime::EnvironmentPtr {
            runtime::Empty::arc()
        }
    }

    pub struct Simple<'a> {
        state: TestState,
        state_rc: Arc<TestState>,
        event_monitor: runtime::EventMonitor<'a>,
    }

    impl<'a> runtime::Environment for Simple<'a> {
        fn is_empty(&self) -> bool {
            true
        }
        fn lookup(&self, _name: &str) -> Option<Box<dyn Any>> {
            None
        }
    }

    impl<'a> runtime::Machine<runtime::StatePtr, runtime::EventMonitor<'a>> for Simple<'a> {
        fn info(&self) -> &'static runtime::MachineInfo {
            info::machine()
        }
        fn state(&self) -> runtime::StatePtr {
            self.state_rc.clone()
        }
        fn variables(&self) -> &dyn runtime::Environment {
            self
        }
        fn event_monitor(&self) -> &runtime::EventMonitor<'a> {
            &self.event_monitor
        }
        fn event_monitor_mut(&mut self) -> &mut runtime::EventMonitor<'a> {
            &mut self.event_monitor
        }
    }

    impl<'a> Simple<'a> {
        pub fn new() -> Simple<'a> {
            Simple {
                state: TestState::A,
                state_rc: Arc::new(TestState::A),
                event_monitor: runtime::EventMonitor::default(),
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
            transition_info: &'static runtime::TransitionInfo,
            new_state: TestState,
        ) {
            let old_state_rc = self.state_rc.clone();
            self.state = new_state;
            self.state_rc = Arc::new(new_state);
            self.event_monitor
                .transition_occurred(runtime::Transition::new_change_state(
                    transition_info,
                    old_state_rc,
                    self.state_rc.clone(),
                ));
        }
    }
}

mod unsync {
    use super::*;
    use frame_runtime::unsync as runtime;
    use std::any::Any;
    use std::rc::Rc;

    impl runtime::State<runtime::EnvironmentPtr> for TestState {
        fn info(&self) -> &'static runtime::StateInfo {
            match self {
                TestState::A => info::machine().states[0],
                TestState::B => info::machine().states[1],
            }
        }
        fn arguments(&self) -> runtime::EnvironmentPtr {
            runtime::Empty::rc()
        }
        fn variables(&self) -> runtime::EnvironmentPtr {
            runtime::Empty::rc()
        }
    }

    pub struct Simple<'a> {
        state: TestState,
        state_rc: Rc<TestState>,
        event_monitor: runtime::EventMonitor<'a>,
    }

    impl<'a> runtime::Environment for Simple<'a> {
        fn is_empty(&self) -> bool {
            true
        }
        fn lookup(&self, _name: &str) -> Option<Box<dyn Any>> {
            None
        }
    }

    impl<'a> runtime::Machine<runtime::StatePtr, runtime::EventMonitor<'a>> for Simple<'a> {
        fn info(&self) -> &'static runtime::MachineInfo {
            info::machine()
        }
        fn state(&self) -> runtime::StatePtr {
            self.state_rc.clone()
        }
        fn variables(&self) -> &dyn runtime::Environment {
            self
        }
        fn event_monitor(&self) -> &runtime::EventMonitor<'a> {
            &self.event_monitor
        }
        fn event_monitor_mut(&mut self) -> &mut runtime::EventMonitor<'a> {
            &mut self.event_monitor
        }
    }

    impl<'a> Simple<'a> {
        pub fn new() -> Simple<'a> {
            Simple {
                state: TestState::A,
                state_rc: Rc::new(TestState::A),
                event_monitor: runtime::EventMonitor::default(),
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
            transition_info: &'static runtime::TransitionInfo,
            new_state: TestState,
        ) {
            let old_state_rc = self.state_rc.clone();
            self.state = new_state;
            self.state_rc = Rc::new(new_state);
            self.event_monitor
                .transition_occurred(runtime::Transition::new_change_state(
                    transition_info,
                    old_state_rc,
                    self.state_rc.clone(),
                ));
        }
    }
}

mod tests {
    use super::*;
    use frame_runtime::info::*;
    use frame_runtime::live::Machine;
    use frame_runtime::sync as srt;
    use frame_runtime::unsync as urt;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Mutex;

    #[test]
    fn static_info() {
        let sm = unsync::Simple::new();
        assert_eq!("Simple", sm.info().name);
        assert_eq!(0, sm.info().variables.len());
        assert_eq!(2, sm.info().states.len());
        assert_eq!(1, sm.info().interface.len());
        assert_eq!(0, sm.info().actions.len());
        assert_eq!(5, sm.info().events.len());
        assert_eq!(2, sm.info().transitions.len());
    }

    #[test]
    fn current_state_sync() {
        let mut sm = sync::Simple::new();
        assert_eq!("A", sm.state().info().name);
        sm.next();
        assert_eq!("B", sm.state().info().name);
        sm.next();
        assert_eq!("A", sm.state().info().name);
    }

    #[test]
    fn current_state_unsync() {
        let mut sm = unsync::Simple::new();
        assert_eq!("A", sm.state().info().name);
        sm.next();
        assert_eq!("B", sm.state().info().name);
        sm.next();
        assert_eq!("A", sm.state().info().name);
    }

    #[test]
    fn transition_callbacks_sync() {
        let tape: Vec<String> = Vec::new();
        let tape_mutex = Mutex::new(tape);
        let mut sm = sync::Simple::new();
        sm.event_monitor_mut()
            .add_transition_callback(srt::Callback::new("test", |t: &srt::Transition| {
                tape_mutex.lock().unwrap().push(format!(
                    "{}{}{}",
                    t.old_state.info().name,
                    match t.info.kind {
                        TransitionKind::ChangeState => "->>",
                        TransitionKind::Transition => "->",
                    },
                    t.new_state.info().name
                ));
            }));
        sm.next();
        sm.next();
        sm.next();
        assert_eq!(*tape_mutex.lock().unwrap(), vec!["A->B", "B->>A", "A->B"]);
    }

    #[test]
    fn transition_callbacks_unsync() {
        let tape: Vec<String> = Vec::new();
        let tape_mutex = Mutex::new(tape);
        let mut sm = unsync::Simple::new();
        sm.event_monitor_mut()
            .add_transition_callback(urt::Callback::new("test", |t: &urt::Transition| {
                tape_mutex.lock().unwrap().push(format!(
                    "{}{}{}",
                    t.old_state.info().name,
                    match t.info.kind {
                        TransitionKind::ChangeState => "->>",
                        TransitionKind::Transition => "->",
                    },
                    t.new_state.info().name
                ));
            }));
        sm.next();
        sm.next();
        sm.next();
        assert_eq!(*tape_mutex.lock().unwrap(), vec!["A->B", "B->>A", "A->B"]);
    }

    #[test]
    fn transition_info_id() {
        let tape: Vec<usize> = Vec::new();
        let tape_mutex = Mutex::new(tape);
        let mut sm = unsync::Simple::new();
        sm.event_monitor_mut()
            .add_transition_callback(urt::Callback::new("test", |t: &urt::Transition| {
                tape_mutex.lock().unwrap().push(t.info.id);
            }));
        sm.next();
        sm.next();
        sm.next();
        assert_eq!(*tape_mutex.lock().unwrap(), vec![0, 1, 0]);
    }

    #[test]
    fn transition_static_info_agrees() {
        let agree = AtomicBool::new(false);
        let mut sm = sync::Simple::new();
        sm.event_monitor_mut()
            .add_transition_callback(srt::Callback::new("test", |t: &srt::Transition| {
                agree.store(
                    t.info.source.name == t.old_state.info().name
                        && t.info.target.name == t.new_state.info().name,
                    Ordering::Relaxed,
                );
            }));
        sm.next();
        assert!(agree.load(Ordering::Relaxed));
        sm.next();
        assert!(agree.load(Ordering::Relaxed));
        sm.next();
        assert!(agree.load(Ordering::Relaxed));
    }
}
