//! This file illustrates what generated code that realizes the runtime interface should look like
//! for a basic state machine without state variables or arguments. This case requires different
//! treatment from a more featureful state machine since runtime states are realized differently
//! depending on whether or not the state context types are generated. This example combined with
//! the example in `tests/demo.rs` illustrate the two major varieties of runtime support.
//!
//! The code corresponds to the generated code with the `thread_safe` feature enabled. For such a
//! simple state machine, the non-thread-safe variant can be obtained by just swapping out `Arc`
//! pointers with `Rc` pointers.
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

use frame_runtime as runtime;
use frame_runtime::Machine;
use std::any::Any;
use std::sync::Arc;

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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum FrameMessage {
    Next,
}

impl std::fmt::Display for FrameMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FrameMessage::Next => write!(f, "next"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct FrameEvent {
    message: FrameMessage,
}

impl FrameEvent {
    fn new(message: FrameMessage) -> FrameEvent {
        FrameEvent { message }
    }
}

impl runtime::Event<Simple> for FrameEvent {
    fn info(&self) -> &runtime::MethodInfo {
        let msg = self.message.to_string();
        info::machine()
            .get_event(&msg)
            .unwrap_or_else(|| panic!("No runtime info for event: {}", msg))
    }
    fn arguments(&self) -> <Simple as runtime::Machine>::EnvironmentPtr {
        runtime::Empty::arc()
    }
    fn return_value(&self) -> Option<Box<dyn Any>> {
        None
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum SimpleState {
    A,
    B,
}

impl runtime::State<Simple> for SimpleState {
    fn info(&self) -> &'static runtime::StateInfo {
        match self {
            SimpleState::A => info::machine().states[0],
            SimpleState::B => info::machine().states[1],
        }
    }
    fn arguments(&self) -> <Simple as runtime::Machine>::EnvironmentPtr {
        runtime::Empty::arc()
    }
    fn variables(&self) -> <Simple as runtime::Machine>::EnvironmentPtr {
        runtime::Empty::arc()
    }
}

pub struct Simple {
    state: SimpleState,
    state_rc: Arc<SimpleState>,
    event_monitor: runtime::EventMonitor<Self>,
}

impl runtime::Environment for Simple {
    fn is_empty(&self) -> bool {
        true
    }
    fn lookup(&self, _name: &str) -> Option<Box<dyn Any>> {
        None
    }
}

impl runtime::Machine for Simple {
    type EnvironmentPtr = Arc<dyn runtime::Environment>;
    type StatePtr = Arc<dyn runtime::State<Self>>;
    type EventPtr = Arc<dyn runtime::Event<Self>>;
    type EventFn = runtime::CallbackSend<Self::EventPtr>;
    type TransitionFn = runtime::CallbackSend<runtime::Transition<Self>>;
    fn info(&self) -> &'static runtime::MachineInfo {
        info::machine()
    }
    fn state(&self) -> Self::StatePtr {
        self.state_rc.clone()
    }
    fn variables(&self) -> &dyn runtime::Environment {
        self
    }
    fn event_monitor(&self) -> &runtime::EventMonitor<Self> {
        &self.event_monitor
    }
    fn event_monitor_mut(&mut self) -> &mut runtime::EventMonitor<Self> {
        &mut self.event_monitor
    }
    fn empty_environment() -> Self::EnvironmentPtr {
        runtime::Empty::arc()
    }
}

impl Simple {
    pub fn new() -> Simple {
        Simple {
            state: SimpleState::A,
            state_rc: Arc::new(SimpleState::A),
            event_monitor: runtime::EventMonitor::default(),
        }
    }

    pub fn next(&mut self) {
        let frame_event = Arc::new(FrameEvent::new(FrameMessage::Next));
        self.handle_event(frame_event);
    }

    fn handle_event(&mut self, frame_event: Arc<FrameEvent>) {
        self.event_monitor_mut().event_sent(frame_event.clone());
        match self.state {
            SimpleState::A => self.a_handler(frame_event.clone()),
            SimpleState::B => self.b_handler(frame_event.clone()),
        }
        self.event_monitor_mut().event_handled(frame_event);
    }

    fn a_handler(&mut self, frame_event: Arc<FrameEvent>) {
        match frame_event.message {
            FrameMessage::Next => {
                self.transition(info::machine().transitions[0], SimpleState::B);
            }
        }
    }

    fn b_handler(&mut self, frame_event: Arc<FrameEvent>) {
        match frame_event.message {
            FrameMessage::Next => {
                self.transition(info::machine().transitions[1], SimpleState::A);
            }
        }
    }

    pub fn transition(
        &mut self,
        transition_info: &'static runtime::TransitionInfo,
        new_state: SimpleState,
    ) {
        let old_state_rc = self.state_rc.clone();
        self.state = new_state;
        self.state_rc = Arc::new(new_state);
        self.event_monitor
            .transition_occurred(runtime::Transition::new_change_state(
                transition_info,
                old_state_rc as <Simple as runtime::Machine>::StatePtr,
                self.state_rc.clone() as <Simple as runtime::Machine>::StatePtr,
            ));
    }
}

impl Default for Simple {
    fn default() -> Self {
        Simple::new()
    }
}

// TODO: Add some basic event tests.
mod tests {
    use super::*;
    use frame_runtime::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Mutex;

    #[test]
    fn static_info() {
        let sm = Simple::new();
        assert_eq!("Simple", sm.info().name);
        assert_eq!(0, sm.info().variables.len());
        assert_eq!(2, sm.info().states.len());
        assert_eq!(1, sm.info().interface.len());
        assert_eq!(0, sm.info().actions.len());
        assert_eq!(5, sm.info().events.len());
        assert_eq!(2, sm.info().transitions.len());
    }

    #[test]
    fn current_state() {
        let mut sm = Simple::new();
        assert_eq!("A", sm.state().info().name);
        sm.next();
        assert_eq!("B", sm.state().info().name);
        sm.next();
        assert_eq!("A", sm.state().info().name);
    }

    #[test]
    fn transition_callbacks() {
        let tape = Arc::new(Mutex::new(Vec::new()));
        let tape_cb = tape.clone();
        let mut sm = Simple::new();
        sm.event_monitor_mut()
            .add_transition_callback(CallbackSend::new("test", move |t: &Transition<Simple>| {
                tape_cb.lock().unwrap().push(format!(
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
        assert_eq!(*tape.lock().unwrap(), vec!["A->B", "B->>A", "A->B"]);
    }

    #[test]
    fn transition_info_id() {
        let tape = Arc::new(Mutex::new(Vec::new()));
        let tape_cb = tape.clone();
        let mut sm = Simple::new();
        sm.event_monitor_mut()
            .add_transition_callback(CallbackSend::new("test", move |t: &Transition<Simple>| {
                tape_cb.lock().unwrap().push(t.info.id);
            }));
        sm.next();
        sm.next();
        sm.next();
        assert_eq!(*tape.lock().unwrap(), vec![0, 1, 0]);
    }

    #[test]
    fn transition_static_info_agrees() {
        let agree = Arc::new(AtomicBool::new(false));
        let agree_cb = agree.clone();
        let mut sm = Simple::new();
        sm.event_monitor_mut()
            .add_transition_callback(CallbackSend::new("test", move |t: &Transition<Simple>| {
                agree_cb.store(
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
