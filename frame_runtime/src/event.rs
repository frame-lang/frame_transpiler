//! This module defines events, callbacks, and the runtime system's event monitor.

use crate::callback::IsCallback;
use crate::env::Environment;
use crate::history::History;
use crate::info::MethodInfo;
use crate::machine::{Machine, State};
use crate::transition::Transition;
use std::any::Any;
use std::ops::Deref;

/// Captures the occurence of a particular event or action.
pub trait Event<M: Machine + ?Sized>
where
    <M::EnvironmentPtr as Deref>::Target: Environment,
    <M::EventPtr as Deref>::Target: Event<M>,
    <M::StatePtr as Deref>::Target: State<M>,
{
    /// The signature of the event that occurred
    fn info(&self) -> &MethodInfo;

    /// The arguments passed to this method. The names and types of the parameters these arguments
    /// are bound to can be found at `self.info().parameters`.
    fn arguments(&self) -> M::EnvironmentPtr;

    /// The return value, if any, whose type can be found at `self.info().return_type`.
    fn return_value(&self) -> Option<Box<dyn Any>> {
        None
    }
}

/// An event monitor maintains a history of previous Frame events and transitions and enables
/// registering callbacks that will be automatically invoked whenever an event or transition occurs
/// in a running state machine.
pub struct EventMonitor<M: Machine + ?Sized>
where
    <M::EnvironmentPtr as Deref>::Target: Environment,
    <M::EventPtr as Deref>::Target: Event<M>,
    <M::StatePtr as Deref>::Target: State<M>,
{
    event_history: History<M::EventPtr>,
    transition_history: History<Transition<M>>,
    event_sent_callbacks: Vec<M::EventFn>,
    event_handled_callbacks: Vec<M::EventFn>,
    transition_callbacks: Vec<M::TransitionFn>,
}

impl<M: Machine> EventMonitor<M>
where
    <M::EnvironmentPtr as Deref>::Target: Environment,
    <M::EventPtr as Deref>::Target: Event<M>,
    <M::StatePtr as Deref>::Target: State<M>,
{
    /// Create a new event monitor with the given capacities for the event history and
    /// transition history. See the documentation for [History::capacity].
    pub fn new(event_capacity: Option<usize>, transition_capacity: Option<usize>) -> Self {
        EventMonitor {
            event_history: History::new(event_capacity),
            transition_history: History::new(transition_capacity),
            event_sent_callbacks: Vec::new(),
            event_handled_callbacks: Vec::new(),
            transition_callbacks: Vec::new(),
        }
    }

    /// Register a callback to be invoked when an event is sent but before it has been handled.
    /// Use this when you want the notification order for events to reflect the order that the
    /// events are triggered, but don't care about the return value of handled events.
    ///
    /// When an event triggers a transition, callbacks will be invoked for the related events in
    /// the following order:
    ///
    ///  * triggering event
    ///  * exit event for the old state, if any
    ///  * enter event for the new state, if any
    pub fn add_event_sent_callback(&mut self, callback: M::EventFn) {
        self.event_sent_callbacks.push(callback);
    }

    /// Register a callback to be invoked after an event has been *completely* handled. Use this
    /// when you want the method instance argument to contain the return value of the event, if
    /// any.
    ///
    /// When an event triggers a transition, callbacks will be invoked for the related events in
    /// the following order:
    ///
    ///  * exit event for the old state, if any
    ///  * enter event for the new state, if any
    ///  * triggering event
    pub fn add_event_handled_callback(&mut self, callback: M::EventFn) {
        self.event_handled_callbacks.push(callback);
    }

    /// Register a callback to be called on each transition. Callbacks will be invoked after the
    /// exit event for the old state has been handled, and before the enter event for the new
    /// state has been sent.
    pub fn add_transition_callback(&mut self, callback: M::TransitionFn) {
        self.transition_callbacks.push(callback);
    }

    /// Remove all event-sent callbacks with the given name.
    pub fn remove_event_sent_callback(&mut self, name: &str) {
        self.event_sent_callbacks.retain(|c| c.name() != name);
    }

    /// Remove all event-handled callbacks with the given name.
    pub fn remove_event_handled_callback(&mut self, name: &str) {
        self.event_handled_callbacks.retain(|c| c.name() != name);
    }

    /// Remove all transition callbacks with the given name.
    pub fn remove_transition_callback(&mut self, name: &str) {
        self.transition_callbacks.retain(|c| c.name() != name);
    }

    /// Track that a Frame event was sent, calling any relevant callbacks and saving it to the
    /// history. Clients shouldn't need to call this method. It will be called by code generated by
    /// Framec.
    pub fn event_sent(&mut self, event: M::EventPtr) {
        for c in &mut self.event_sent_callbacks {
            c.apply(&event);
        }
        self.event_history.add(event);
    }

    /// Track that a previously sent Frame event has been completely handled, calling any relevant
    /// callbacks. Note that the event will have already been added to the history when the initial
    /// `event_sent` call was made. However, it's useful to be able to notify clients here since
    /// now the return value, if any, will be set. Clients shouldn't need to call this method. It
    /// will be called by code generated by Framec.
    pub fn event_handled(&mut self, event: M::EventPtr) {
        for c in &mut self.event_handled_callbacks {
            c.apply(&event);
        }
    }

    /// Track that a transition occurred with the provided arguments, calling all of the transition
    /// callbacks and saving it to the history. Clients shouldn't need to call this method. It will
    /// be called by code generated by Framec.
    pub fn transition_occurred(&mut self, transition: Transition<M>) {
        for c in &mut self.transition_callbacks {
            c.apply(&transition);
        }
        self.transition_history.add(transition);
    }

    /// Get the history of handled events. New events are added to the back of the `VecDeque`, so
    /// the oldest saved event will be at index `0` and the most recent event can be obtained by
    /// [std::collections::VecDeque::back].
    pub fn event_history(&self) -> &History<M::EventPtr> {
        &self.event_history
    }

    /// Get the history of transitions that occurred. New transitions are added to the back of the
    /// `VecDeque`, so the oldest saved transition will be at index `0` and the most recent
    /// transition can be obtained by [std::collections::VecDeque::back].
    pub fn transition_history(&self) -> &History<Transition<M>> {
        &self.transition_history
    }

    /// Clear the event history.
    pub fn clear_event_history(&mut self) {
        self.event_history.clear();
    }

    /// Clear the transition history.
    pub fn clear_transition_history(&mut self) {
        self.transition_history.clear();
    }

    /// Set the number of events to maintain in the history. If `None`, the number of events is
    /// unlimited.
    pub fn set_event_history_capacity(&mut self, capacity: Option<usize>) {
        self.event_history.set_capacity(capacity);
    }

    /// Set the number of transitions to maintain in the history. If `None`, the number of
    /// transitions is unlimited.
    pub fn set_transition_history_capacity(&mut self, capacity: Option<usize>) {
        self.transition_history.set_capacity(capacity);
    }
}

impl<M: Machine> Default for EventMonitor<M>
where
    <M::EnvironmentPtr as Deref>::Target: Environment,
    <M::EventPtr as Deref>::Target: Event<M>,
    <M::StatePtr as Deref>::Target: State<M>,
{
    fn default() -> Self {
        EventMonitor::new(Some(0), Some(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::callback::*;
    use crate::env::*;
    use crate::info::*;
    use std::any::Any;
    use std::cell::RefCell;
    use std::rc::Rc;

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
            name: "Dummy",
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

    struct Dummy;

    impl Machine for Dummy {
        type EnvironmentPtr = Rc<dyn Environment>;
        type StatePtr = Rc<dyn State<Self>>;
        type EventPtr = Rc<dyn Event<Self>>;
        type EventFn = Callback<Self::EventPtr>;
        type TransitionFn = Callback<Transition<Self>>;
        fn state(&self) -> Self::StatePtr {
            panic!("not implemented")
        }
        fn variables(&self) -> &dyn Environment {
            panic!("not implemented")
        }
        fn event_monitor(&self) -> &EventMonitor<Self> {
            panic!("not implemented")
        }
        fn event_monitor_mut(&mut self) -> &mut EventMonitor<Self> {
            panic!("not implemented")
        }
        fn machine_info() -> &'static MachineInfo {
            info::machine()
        }
        fn empty_environment() -> Self::EnvironmentPtr {
            Empty::rc()
        }
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
    enum TestState {
        A,
        B,
    }

    impl State<Dummy> for TestState {
        fn info(&self) -> &'static StateInfo {
            match self {
                TestState::A => info::machine().states[0],
                TestState::B => info::machine().states[1],
            }
        }
        fn arguments(&self) -> <Dummy as Machine>::EnvironmentPtr {
            Empty::rc()
        }
        fn variables(&self) -> <Dummy as Machine>::EnvironmentPtr {
            Empty::rc()
        }
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
    enum FrameMessage {
        Enter(TestState),
        Exit(TestState),
        Next,
    }

    impl std::fmt::Display for FrameMessage {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                FrameMessage::Enter(s) => write!(f, "{:?}:>", s),
                FrameMessage::Exit(s) => write!(f, "{:?}:<", s),
                FrameMessage::Next => write!(f, "next"),
            }
        }
    }

    impl Event<Dummy> for FrameMessage {
        fn info(&self) -> &MethodInfo {
            info::machine().get_event(&self.to_string()).unwrap()
        }
        fn arguments(&self) -> <Dummy as Machine>::EnvironmentPtr {
            Empty::rc()
        }
        fn return_value(&self) -> Option<Box<dyn Any>> {
            None
        }
    }

    #[test]
    fn event_sent_callbacks() {
        let tape = Rc::new(RefCell::new(Vec::new()));
        let tape_cb = tape.clone();
        let mut em = EventMonitor::<Dummy>::default();
        em.add_event_sent_callback(Callback::new(
            "test",
            move |e: &<Dummy as Machine>::EventPtr| {
                tape_cb.borrow_mut().push(e.info().name.to_string())
            },
        ));
        em.event_sent(Rc::new(FrameMessage::Next));
        em.event_sent(Rc::new(FrameMessage::Enter(TestState::A)));
        em.event_sent(Rc::new(FrameMessage::Enter(TestState::B)));
        em.event_sent(Rc::new(FrameMessage::Next));
        em.event_sent(Rc::new(FrameMessage::Exit(TestState::A)));
        em.event_sent(Rc::new(FrameMessage::Exit(TestState::B)));
        em.event_sent(Rc::new(FrameMessage::Next));
        assert_eq!(
            *tape.borrow(),
            vec!["next", "A:>", "B:>", "next", "A:<", "B:<", "next"]
        );
    }

    #[test]
    fn event_handled_callbacks() {
        let tape = Rc::new(RefCell::new(Vec::new()));
        let tape_cb = tape.clone();
        let mut em = EventMonitor::<Dummy>::default();
        em.add_event_handled_callback(Callback::new(
            "test",
            move |e: &<Dummy as Machine>::EventPtr| {
                tape_cb.borrow_mut().push(e.info().name.to_string())
            },
        ));
        em.event_handled(Rc::new(FrameMessage::Exit(TestState::B)));
        em.event_handled(Rc::new(FrameMessage::Enter(TestState::A)));
        em.event_handled(Rc::new(FrameMessage::Next));
        em.event_handled(Rc::new(FrameMessage::Exit(TestState::A)));
        em.event_handled(Rc::new(FrameMessage::Enter(TestState::B)));
        em.event_handled(Rc::new(FrameMessage::Next));
        assert_eq!(
            *tape.borrow(),
            vec!["B:<", "A:>", "next", "A:<", "B:>", "next"]
        );
    }

    #[test]
    fn transition_callbacks() {
        let tape = Rc::new(RefCell::new(Vec::new()));
        let tape_cb1 = tape.clone();
        let tape_cb2 = tape.clone();
        let tape_cb3 = tape.clone();
        let mut em = EventMonitor::<Dummy>::default();
        em.add_transition_callback(Callback::new("old", move |t: &Transition<Dummy>| {
            tape_cb1
                .borrow_mut()
                .push(format!("old: {}", t.old_state.info().name))
        }));
        em.add_transition_callback(Callback::new("new", move |t: &Transition<Dummy>| {
            tape_cb2
                .borrow_mut()
                .push(format!("new: {}", t.new_state.info().name))
        }));
        em.add_transition_callback(Callback::new("kind", move |t: &Transition<Dummy>| {
            tape_cb3
                .borrow_mut()
                .push(format!("kind: {:?}", t.info.kind))
        }));

        let a_rc = Rc::new(TestState::A);
        let b_rc = Rc::new(TestState::B);
        em.transition_occurred(Transition::new_change_state(
            info::machine().transitions[0],
            a_rc.clone() as <Dummy as Machine>::StatePtr,
            b_rc.clone() as <Dummy as Machine>::StatePtr,
        ));
        assert_eq!(*tape.borrow(), vec!["old: A", "new: B", "kind: Transition"]);
        tape.borrow_mut().clear();

        em.transition_occurred(Transition::new_change_state(
            info::machine().transitions[1],
            b_rc as <Dummy as Machine>::StatePtr,
            a_rc as <Dummy as Machine>::StatePtr,
        ));
        assert_eq!(
            *tape.borrow(),
            vec!["old: B", "new: A", "kind: ChangeState"]
        );
    }

    #[test]
    fn remove_transition_callbacks() {
        let tape = Rc::new(RefCell::new(Vec::new()));
        let tape_cb1 = tape.clone();
        let tape_cb2 = tape.clone();
        let tape_cb3 = tape.clone();
        let mut em = EventMonitor::<Dummy>::default();
        em.add_transition_callback(Callback::new("old", move |t: &Transition<Dummy>| {
            tape_cb1
                .borrow_mut()
                .push(format!("old: {}", t.old_state.info().name))
        }));
        em.add_transition_callback(Callback::new("new", move |t: &Transition<Dummy>| {
            tape_cb2
                .borrow_mut()
                .push(format!("new: {}", t.new_state.info().name))
        }));
        em.add_transition_callback(Callback::new("kind", move |t: &Transition<Dummy>| {
            tape_cb3
                .borrow_mut()
                .push(format!("kind: {:?}", t.info.kind))
        }));

        let a_rc = Rc::new(TestState::A);
        let b_rc = Rc::new(TestState::B);
        em.transition_occurred(Transition::new_change_state(
            info::machine().transitions[0],
            a_rc.clone() as <Dummy as Machine>::StatePtr,
            b_rc.clone() as <Dummy as Machine>::StatePtr,
        ));
        assert_eq!(*tape.borrow(), vec!["old: A", "new: B", "kind: Transition"]);
        tape.borrow_mut().clear();

        em.remove_transition_callback("kind");
        em.transition_occurred(Transition::new_change_state(
            info::machine().transitions[0],
            a_rc.clone() as <Dummy as Machine>::StatePtr,
            b_rc.clone() as <Dummy as Machine>::StatePtr,
        ));
        assert_eq!(*tape.borrow(), vec!["old: A", "new: B"]);
        tape.borrow_mut().clear();

        let tape_cb4 = tape.clone();
        let tape_cb5 = tape.clone();
        em.add_transition_callback(Callback::new("old", move |t: &Transition<Dummy>| {
            tape_cb4
                .borrow_mut()
                .push(format!("old: {}", t.old_state.info().name))
        }));
        em.add_transition_callback(Callback::new("kind", move |t: &Transition<Dummy>| {
            tape_cb5
                .borrow_mut()
                .push(format!("kind: {:?}", t.info.kind))
        }));
        em.transition_occurred(Transition::new_change_state(
            info::machine().transitions[0],
            a_rc.clone() as <Dummy as Machine>::StatePtr,
            b_rc.clone() as <Dummy as Machine>::StatePtr,
        ));
        assert_eq!(
            *tape.borrow(),
            vec!["old: A", "new: B", "old: A", "kind: Transition"]
        );
        tape.borrow_mut().clear();

        em.remove_transition_callback("old");
        em.transition_occurred(Transition::new_change_state(
            info::machine().transitions[0],
            a_rc as <Dummy as Machine>::StatePtr,
            b_rc as <Dummy as Machine>::StatePtr,
        ));
        assert_eq!(*tape.borrow(), vec!["new: B", "kind: Transition"]);
    }

    #[test]
    fn event_history_finite() {
        let mut em = EventMonitor::<Dummy>::new(Some(5), Some(1));
        assert!(em.event_history().is_empty());

        em.event_sent(Rc::new(FrameMessage::Next));
        em.event_sent(Rc::new(FrameMessage::Enter(TestState::A)));
        em.event_sent(Rc::new(FrameMessage::Enter(TestState::B)));
        assert_eq!(
            em.event_history()
                .iter()
                .map(|e| e.info().name)
                .collect::<Vec<&str>>(),
            vec!["next", "A:>", "B:>"]
        );

        em.event_sent(Rc::new(FrameMessage::Exit(TestState::B)));
        em.event_sent(Rc::new(FrameMessage::Next));
        em.event_sent(Rc::new(FrameMessage::Exit(TestState::A)));
        em.event_sent(Rc::new(FrameMessage::Next));
        assert_eq!(
            em.event_history()
                .iter()
                .map(|e| e.info().name)
                .collect::<Vec<&str>>(),
            vec!["B:>", "B:<", "next", "A:<", "next"]
        );

        em.clear_event_history();
        assert!(em.event_history().is_empty());

        em.event_sent(Rc::new(FrameMessage::Enter(TestState::A)));
        em.event_sent(Rc::new(FrameMessage::Enter(TestState::B)));
        assert_eq!(
            em.event_history()
                .iter()
                .map(|e| e.info().name)
                .collect::<Vec<&str>>(),
            vec!["A:>", "B:>"]
        );
    }

    #[test]
    fn event_history_infinite() {
        let mut em = EventMonitor::<Dummy>::new(None, Some(1));
        assert!(em.event_history().is_empty());

        em.event_sent(Rc::new(FrameMessage::Next));
        em.event_sent(Rc::new(FrameMessage::Enter(TestState::A)));
        em.event_sent(Rc::new(FrameMessage::Enter(TestState::B)));
        assert_eq!(
            em.event_history()
                .iter()
                .map(|e| e.info().name)
                .collect::<Vec<&str>>(),
            vec!["next", "A:>", "B:>"]
        );

        em.event_sent(Rc::new(FrameMessage::Exit(TestState::B)));
        em.event_sent(Rc::new(FrameMessage::Next));
        em.event_sent(Rc::new(FrameMessage::Exit(TestState::A)));
        em.event_sent(Rc::new(FrameMessage::Next));
        assert_eq!(
            em.event_history()
                .iter()
                .map(|e| e.info().name)
                .collect::<Vec<&str>>(),
            vec!["next", "A:>", "B:>", "B:<", "next", "A:<", "next"]
        );

        em.clear_event_history();
        assert!(em.event_history().is_empty());

        em.event_sent(Rc::new(FrameMessage::Enter(TestState::A)));
        em.event_sent(Rc::new(FrameMessage::Enter(TestState::B)));
        assert_eq!(
            em.event_history()
                .iter()
                .map(|e| e.info().name)
                .collect::<Vec<&str>>(),
            vec!["A:>", "B:>"]
        );
    }

    #[test]
    fn event_history_disabled() {
        let mut em = EventMonitor::<Dummy>::new(Some(0), Some(1));
        assert!(em.event_history().is_empty());

        em.event_sent(Rc::new(FrameMessage::Next));
        em.event_sent(Rc::new(FrameMessage::Enter(TestState::A)));
        em.event_sent(Rc::new(FrameMessage::Enter(TestState::B)));
        assert!(em.event_history().is_empty());

        em.clear_event_history();
        assert!(em.event_history().is_empty());
    }

    #[test]
    fn transition_history_finite() {
        let mut em = EventMonitor::<Dummy>::new(Some(0), Some(3));
        let a = Rc::new(TestState::A);
        let b = Rc::new(TestState::B);
        let a2b = Transition::new_change_state(
            info::machine().transitions[0],
            a.clone() as <Dummy as Machine>::StatePtr,
            b.clone() as <Dummy as Machine>::StatePtr,
        );
        let b2a = Transition::new_change_state(
            info::machine().transitions[1],
            b as <Dummy as Machine>::StatePtr,
            a as <Dummy as Machine>::StatePtr,
        );

        assert!(em.transition_history().newest().is_none());
        assert!(em.transition_history().is_empty());

        em.transition_occurred(a2b.clone());
        em.transition_occurred(b2a.clone());
        assert_eq!(em.transition_history().len(), 2);

        let last = em.transition_history().newest().unwrap();
        let first = em.transition_history().as_deque().get(0).unwrap();
        assert_eq!(last.info.id, 1);
        assert_eq!(last.old_state.info().name, "B");
        assert_eq!(last.new_state.info().name, "A");
        assert_eq!(first.info.id, 0);
        assert_eq!(first.old_state.info().name, "A");
        assert_eq!(first.new_state.info().name, "B");

        em.transition_occurred(b2a.clone());
        em.transition_occurred(a2b.clone());
        assert_eq!(em.transition_history().len(), 3);
        assert_eq!(em.transition_history().newest().unwrap().info.id, 0);
        assert_eq!(
            em.transition_history().as_deque().get(1).unwrap().info.id,
            1
        );
        assert_eq!(
            em.transition_history().as_deque().get(0).unwrap().info.id,
            1
        );

        em.clear_transition_history();
        assert!(em.transition_history().is_empty());
        em.transition_occurred(b2a);
        em.transition_occurred(a2b);
        assert_eq!(em.transition_history().len(), 2);
    }

    #[test]
    fn transition_history_infinite() {
        let mut em = EventMonitor::<Dummy>::new(Some(0), None);
        let a = Rc::new(TestState::A);
        let b = Rc::new(TestState::B);
        let a2b = Transition::new_change_state(
            info::machine().transitions[0],
            a.clone() as <Dummy as Machine>::StatePtr,
            b.clone() as <Dummy as Machine>::StatePtr,
        );
        let b2a = Transition::new_change_state(
            info::machine().transitions[1],
            b as <Dummy as Machine>::StatePtr,
            a as <Dummy as Machine>::StatePtr,
        );

        assert!(em.transition_history().newest().is_none());
        assert!(em.transition_history().is_empty());

        em.transition_occurred(a2b.clone());
        em.transition_occurred(b2a.clone());
        assert_eq!(em.transition_history().len(), 2);

        let last = em.transition_history().newest().unwrap();
        let first = em.transition_history().as_deque().get(0).unwrap();
        assert_eq!(last.info.id, 1);
        assert_eq!(last.old_state.info().name, "B");
        assert_eq!(last.new_state.info().name, "A");
        assert_eq!(first.info.id, 0);
        assert_eq!(first.old_state.info().name, "A");
        assert_eq!(first.new_state.info().name, "B");

        em.transition_occurred(b2a.clone());
        em.transition_occurred(a2b.clone());
        assert_eq!(em.transition_history().len(), 4);
        assert_eq!(em.transition_history().newest().unwrap().info.id, 0);
        assert_eq!(
            em.transition_history().as_deque().get(2).unwrap().info.id,
            1
        );
        assert_eq!(
            em.transition_history().as_deque().get(1).unwrap().info.id,
            1
        );
        assert_eq!(
            em.transition_history().as_deque().get(0).unwrap().info.id,
            0
        );

        em.clear_transition_history();
        assert!(em.transition_history().is_empty());
        em.transition_occurred(b2a);
        em.transition_occurred(a2b);
        assert_eq!(em.transition_history().len(), 2);
    }

    #[test]
    fn transition_history_disabled() {
        let mut em = EventMonitor::<Dummy>::new(Some(0), Some(0));
        let a = Rc::new(TestState::A);
        let b = Rc::new(TestState::B);
        let a2b = Transition::new_change_state(
            info::machine().transitions[0],
            a.clone() as <Dummy as Machine>::StatePtr,
            b.clone() as <Dummy as Machine>::StatePtr,
        );
        let b2a = Transition::new_change_state(
            info::machine().transitions[1],
            b as <Dummy as Machine>::StatePtr,
            a as <Dummy as Machine>::StatePtr,
        );

        assert!(em.transition_history().newest().is_none());
        assert!(em.transition_history().is_empty());

        em.transition_occurred(a2b);
        em.transition_occurred(b2a);
        assert!(em.transition_history().newest().is_none());
        assert!(em.transition_history().is_empty());

        em.clear_transition_history();
        assert!(em.transition_history().newest().is_none());
        assert!(em.transition_history().is_empty());
    }
}
