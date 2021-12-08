use crate::*;

use frame_runtime::live::Machine;
use frame_runtime::sync as runtime;
use std::sync::{Arc, Mutex};

pub struct FrameEvent {
    message: FrameMessage,
    arguments: Arc<Mutex<FrameEventArgs>>,
    ret: Mutex<FrameEventReturn>,
}

impl FrameEvent {
    fn new(message: FrameMessage, arguments: FrameEventArgs) -> FrameEvent {
        FrameEvent {
            message,
            arguments: Arc::new(Mutex::new(arguments)),
            ret: Mutex::new(FrameEventReturn::None),
        }
    }
}

impl runtime::Event<runtime::EnvironmentPtr> for FrameEvent {
    fn info(&self) -> &runtime::MethodInfo {
        let msg = self.message.to_string();
        info::machine()
            .get_event(&msg)
            .unwrap_or_else(|| panic!("No runtime info for event: {}", msg))
    }
    fn arguments(&self) -> runtime::EnvironmentPtr {
        self.arguments.clone()
    }
    #[allow(clippy::clone_on_copy)]
    fn return_value(&self) -> Option<Box<dyn Any>> {
        match *self.ret.lock().unwrap() {
            FrameEventReturn::None => None,
            FrameEventReturn::Inc { return_value } => Some(Box::new(return_value.clone())),
        }
    }
}

struct InitStateContext {}

impl runtime::State<runtime::EnvironmentPtr> for InitStateContext {
    fn info(&self) -> &'static runtime::StateInfo {
        info::machine().states[0]
    }
    fn arguments(&self) -> runtime::EnvironmentPtr {
        runtime::Empty::arc()
    }
    fn variables(&self) -> runtime::EnvironmentPtr {
        runtime::Empty::arc()
    }
}

struct FooStateContext {
    state_vars: Arc<Mutex<FooStateVars>>,
}

impl runtime::State<runtime::EnvironmentPtr> for FooStateContext {
    fn info(&self) -> &'static runtime::StateInfo {
        info::machine().states[1]
    }
    fn arguments(&self) -> runtime::EnvironmentPtr {
        runtime::Empty::arc()
    }
    fn variables(&self) -> runtime::EnvironmentPtr {
        self.state_vars.clone()
    }
}

struct BarStateContext {
    state_args: Arc<Mutex<BarStateArgs>>,
    state_vars: Arc<Mutex<BarStateVars>>,
}

impl runtime::State<runtime::EnvironmentPtr> for BarStateContext {
    fn info(&self) -> &'static runtime::StateInfo {
        info::machine().states[2]
    }
    fn arguments(&self) -> runtime::EnvironmentPtr {
        self.state_args.clone()
    }
    fn variables(&self) -> runtime::EnvironmentPtr {
        self.state_vars.clone()
    }
}

enum StateContext {
    Init(InitStateContext),
    Foo(FooStateContext),
    Bar(BarStateContext),
}

impl StateContext {
    fn init_context(&self) -> &InitStateContext {
        match self {
            StateContext::Init(context) => context,
            _ => panic!("Failed conversion to InitStateContext"),
        }
    }
    fn foo_context(&self) -> &FooStateContext {
        match self {
            StateContext::Foo(context) => context,
            _ => panic!("Failed conversion to FooStateContext"),
        }
    }
    fn bar_context(&self) -> &BarStateContext {
        match self {
            StateContext::Bar(context) => context,
            _ => panic!("Failed conversion to BarStateContext"),
        }
    }
}

impl runtime::State<runtime::EnvironmentPtr> for StateContext {
    fn info(&self) -> &'static runtime::StateInfo {
        match self {
            StateContext::Init(context) => context.info(),
            StateContext::Foo(context) => context.info(),
            StateContext::Bar(context) => context.info(),
        }
    }
    fn arguments(&self) -> runtime::EnvironmentPtr {
        match self {
            StateContext::Init(context) => context.arguments(),
            StateContext::Foo(context) => context.arguments(),
            StateContext::Bar(context) => context.arguments(),
        }
    }
    fn variables(&self) -> runtime::EnvironmentPtr {
        match self {
            StateContext::Init(context) => context.variables(),
            StateContext::Foo(context) => context.variables(),
            StateContext::Bar(context) => context.variables(),
        }
    }
}

pub struct Demo<'a> {
    state: DemoState,
    state_context: Arc<StateContext>,
    event_monitor: runtime::EventMonitor<'a>,
    x: i32,
    y: i32,
}

impl<'a> Environment for Demo<'a> {
    fn lookup(&self, name: &str) -> Option<Box<dyn Any>> {
        match name {
            "x" => Some(Box::new(self.x)),
            "y" => Some(Box::new(self.y)),
            _ => None,
        }
    }
}

impl<'a> runtime::Machine<runtime::StatePtr, runtime::EventMonitor<'a>> for Demo<'a> {
    fn info(&self) -> &'static runtime::MachineInfo {
        info::machine()
    }
    fn state(&self) -> runtime::StatePtr {
        self.state_context.clone()
    }
    fn variables(&self) -> &dyn Environment {
        self
    }
    fn event_monitor(&self) -> &runtime::EventMonitor<'a> {
        &self.event_monitor
    }
    fn event_monitor_mut(&mut self) -> &mut runtime::EventMonitor<'a> {
        &mut self.event_monitor
    }
}

impl<'a> Demo<'a> {
    pub fn new() -> Demo<'a> {
        let context = InitStateContext {};
        let next_state_context = Arc::new(StateContext::Init(context));
        let event_monitor = runtime::EventMonitor::new(Some(0), Some(1));
        let mut machine = Demo {
            state: DemoState::Init,
            state_context: next_state_context,
            event_monitor,
            x: 0,
            y: 0,
        };
        machine.initialize();
        machine
    }

    fn initialize(&mut self) {
        let frame_event = Arc::new(FrameEvent::new(
            FrameMessage::Enter(self.state),
            FrameEventArgs::None,
        ));
        self.handle_event(frame_event);
    }

    pub fn inc(&mut self, arg: i32) -> i32 {
        let frame_args = FrameEventArgs::Inc(IncArgs { arg });
        let frame_event = Arc::new(FrameEvent::new(FrameMessage::Inc, frame_args));
        self.handle_event(frame_event.clone());
        let return_value = match *frame_event.ret.lock().unwrap() {
            FrameEventReturn::Inc { return_value } => return_value,
            _ => panic!("Bad return value for inc"),
        };
        return_value
    }

    pub fn next(&mut self) {
        let frame_args = FrameEventArgs::None;
        let frame_event = Arc::new(FrameEvent::new(FrameMessage::Next, frame_args));
        self.handle_event(frame_event);
    }

    #[allow(clippy::single_match)]
    #[allow(unused_variables)]
    fn init_handler(&mut self, frame_event: Arc<FrameEvent>) {
        let this_state_context = self.state_context.init_context();
        match frame_event.message {
            FrameMessage::Enter(_) => {
                // Start transition
                let exit_args = FrameEventArgs::None;
                let enter_args = FrameEventArgs::FooEnter(FooEnterArgs { init: 2 });
                let context = FooStateContext {
                    state_vars: Arc::new(Mutex::new(FooStateVars { x: 0 })),
                };
                let next_state_context = Arc::new(StateContext::Foo(context));
                self.transition(
                    info::machine().transitions[0],
                    exit_args,
                    enter_args,
                    DemoState::Foo,
                    next_state_context,
                );
            }
            _ => {}
        }
    }

    fn foo_handler(&mut self, frame_event: Arc<FrameEvent>) {
        let this_state_context = self.state_context.foo_context();
        match frame_event.message {
            FrameMessage::Enter(_) => {
                this_state_context.state_vars.lock().unwrap().x =
                    frame_event.arguments.lock().unwrap().foo_enter_args().init;
            }
            FrameMessage::Exit(_) => {}
            FrameMessage::Inc => {
                this_state_context.state_vars.lock().unwrap().x +=
                    frame_event.arguments.lock().unwrap().inc_args().arg;
                let mut ret = frame_event.ret.lock().unwrap();
                *ret = FrameEventReturn::Inc {
                    return_value: this_state_context.state_vars.lock().unwrap().x,
                };
            }
            FrameMessage::Next => {
                self.x += this_state_context.state_vars.lock().unwrap().x;
                // Start transition
                let exit_args = FrameEventArgs::FooExit(FooExitArgs {
                    done: this_state_context.state_vars.lock().unwrap().x,
                });
                let enter_args = FrameEventArgs::BarEnter(BarEnterArgs { start: 3 });
                let context = BarStateContext {
                    state_args: Arc::new(Mutex::new(BarStateArgs { tilt: 4 })),
                    state_vars: Arc::new(Mutex::new(BarStateVars { y: 0 })),
                };
                let next_state_context = Arc::new(StateContext::Bar(context));
                self.transition(
                    info::machine().transitions[1],
                    exit_args,
                    enter_args,
                    DemoState::Bar,
                    next_state_context,
                );
            }
        }
    }

    fn bar_handler(&mut self, frame_event: Arc<FrameEvent>) {
        let this_state_context = self.state_context.bar_context();
        match frame_event.message {
            FrameMessage::Enter(_) => {
                this_state_context.state_vars.lock().unwrap().y =
                    frame_event.arguments.lock().unwrap().bar_enter_args().start
                        + this_state_context.state_args.lock().unwrap().tilt;
            }
            FrameMessage::Exit(_) => {}
            FrameMessage::Inc => {
                this_state_context.state_vars.lock().unwrap().y +=
                    frame_event.arguments.lock().unwrap().inc_args().arg;
                let mut ret = frame_event.ret.lock().unwrap();
                *ret = FrameEventReturn::Inc {
                    return_value: this_state_context.state_vars.lock().unwrap().y,
                };
            }
            FrameMessage::Next => {
                self.y += this_state_context.state_vars.lock().unwrap().y;
                // Start change state
                let context = FooStateContext {
                    state_vars: Arc::new(Mutex::new(FooStateVars { x: 0 })),
                };
                let next_state_context = Arc::new(StateContext::Foo(context));
                self.change_state(
                    info::machine().transitions[2],
                    DemoState::Foo,
                    next_state_context,
                );
            }
        }
    }

    fn handle_event(&mut self, frame_event: Arc<FrameEvent>) {
        self.event_monitor_mut().event_sent(frame_event.clone());
        match self.state {
            DemoState::Init => self.init_handler(frame_event.clone()),
            DemoState::Foo => self.foo_handler(frame_event.clone()),
            DemoState::Bar => self.bar_handler(frame_event.clone()),
        }
        self.event_monitor_mut().event_handled(frame_event);
    }

    fn transition(
        &mut self,
        transition_info: &'static runtime::TransitionInfo,
        exit_args: FrameEventArgs,
        enter_args: FrameEventArgs,
        new_state: DemoState,
        new_state_context: Arc<StateContext>,
    ) {
        // create and send exit event for old state
        let exit_event = Arc::new(FrameEvent::new(FrameMessage::Exit(self.state), exit_args));
        self.handle_event(exit_event.clone());

        // update state
        let old_state_context = self.state_context.clone();
        self.state = new_state;
        self.state_context = new_state_context.clone();

        // create enter event for new state
        let enter_event = Arc::new(FrameEvent::new(FrameMessage::Enter(self.state), enter_args));

        // call transition callbacks
        self.event_monitor
            .transition_occurred(runtime::Transition::new(
                transition_info,
                old_state_context,
                new_state_context,
                exit_event,
                enter_event.clone(),
            ));

        // send enter event for new state
        self.handle_event(enter_event);
    }

    fn change_state(
        &mut self,
        transition_info: &'static runtime::TransitionInfo,
        new_state: DemoState,
        new_state_context: Arc<StateContext>,
    ) {
        // update state
        let old_state_context = self.state_context.clone();
        self.state = new_state;
        self.state_context = new_state_context.clone();

        // call change-state callbacks
        self.event_monitor
            .transition_occurred(runtime::Transition::new_change_state(
                transition_info,
                old_state_context,
                new_state_context,
            ));
    }
}

impl<'a> Default for Demo<'a> {
    fn default() -> Self {
        Demo::new()
    }
}

mod tests {
    use super::*;
    use frame_runtime::env::{Empty, Environment};
    use frame_runtime::live::Machine;
    use frame_runtime::sync::{Callback, EventPtr, Transition};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Mutex;

    /// Helper function to lookup an `i32` value in an environment.
    /// Returns -1 if the lookup fails for any reason.
    fn lookup_i32(env: &dyn Environment, name: &str) -> i32 {
        match env.lookup(name) {
            None => -1,
            Some(any) => *any.downcast_ref().unwrap_or(&-1),
        }
    }

    #[test]
    fn current_state() {
        let mut sm = Demo::new();
        assert_eq!("Foo", sm.state().info().name);
        sm.inc(3);
        assert_eq!("Foo", sm.state().info().name);
        sm.next();
        assert_eq!("Bar", sm.state().info().name);
        sm.inc(4);
        assert_eq!("Bar", sm.state().info().name);
        sm.next();
        assert_eq!("Foo", sm.state().info().name);
    }

    #[test]
    fn variables() {
        let mut sm = Demo::new();
        assert_eq!(0, lookup_i32(sm.variables(), "y"));
        assert!(sm.variables().lookup("z").is_none());
        assert!(sm.variables().lookup("arg").is_none());
        assert!(sm.variables().lookup("inc").is_none());
        sm.inc(3);
        sm.inc(4);
        assert_eq!(0, lookup_i32(sm.variables(), "x"));
        assert_eq!(0, lookup_i32(sm.variables(), "y"));
        sm.next();
        assert_eq!(9, lookup_i32(sm.variables(), "x"));
        assert_eq!(0, lookup_i32(sm.variables(), "y"));
        sm.inc(5);
        sm.inc(6);
        assert_eq!(9, lookup_i32(sm.variables(), "x"));
        assert_eq!(0, lookup_i32(sm.variables(), "y"));
        sm.next();
        assert_eq!(9, lookup_i32(sm.variables(), "x"));
        assert_eq!(18, lookup_i32(sm.variables(), "y"));
        sm.inc(7);
        sm.next();
        assert_eq!(16, lookup_i32(sm.variables(), "x"));
        assert_eq!(18, lookup_i32(sm.variables(), "y"));
    }

    #[test]
    fn state_variables() {
        let mut sm = Demo::new();
        assert_eq!(2, lookup_i32(sm.state().variables().as_ref(), "x"));
        assert!(sm.state().variables().lookup("y").is_none());
        sm.inc(3);
        sm.inc(4);
        assert_eq!(9, lookup_i32(sm.state().variables().as_ref(), "x"));
        sm.next();
        assert_eq!(7, lookup_i32(sm.state().variables().as_ref(), "y"));
        assert!(sm.state().variables().lookup("x").is_none());
        sm.inc(5);
        sm.inc(6);
        assert_eq!(18, lookup_i32(sm.state().variables().as_ref(), "y"));
        sm.next();
        assert_eq!(0, lookup_i32(sm.state().variables().as_ref(), "x"));
        sm.inc(7);
        assert_eq!(7, lookup_i32(sm.state().variables().as_ref(), "x"));
    }

    #[test]
    fn state_arguments() {
        let mut sm = Demo::new();
        assert!(sm.state().arguments().lookup("x").is_none());
        assert!(sm.state().arguments().lookup("y").is_none());
        assert!(sm.state().arguments().lookup("tilt").is_none());
        sm.next();
        assert!(sm.state().arguments().lookup("x").is_none());
        assert!(sm.state().arguments().lookup("y").is_none());
        assert_eq!(4, lookup_i32(sm.state().arguments().as_ref(), "tilt"));
        sm.next();
        assert!(sm.state().arguments().lookup("tilt").is_none());
    }

    #[test]
    fn event_sent_callbacks() {
        let tape: Vec<String> = Vec::new();
        let tape_mutex = Mutex::new(tape);
        let mut sm = Demo::new();
        sm.event_monitor_mut()
            .add_event_sent_callback(Callback::new(|e: &EventPtr| {
                tape_mutex.lock().unwrap().push(e.info().name.to_string());
            }));
        sm.inc(2);
        sm.next();
        sm.inc(3);
        sm.next();
        sm.next();
        assert_eq!(
            *tape_mutex.lock().unwrap(),
            vec!["inc", "next", "Foo:<", "Bar:>", "inc", "next", "next", "Foo:<", "Bar:>"]
        );
    }

    #[test]
    fn event_handled_callbacks() {
        let tape: Vec<String> = Vec::new();
        let tape_mutex = Mutex::new(tape);
        let mut sm = Demo::new();
        sm.event_monitor_mut()
            .add_event_handled_callback(Callback::new(|e: &EventPtr| {
                tape_mutex.lock().unwrap().push(e.info().name.to_string());
            }));
        sm.inc(2);
        sm.next();
        sm.inc(3);
        sm.next();
        sm.next();
        assert_eq!(
            *tape_mutex.lock().unwrap(),
            vec!["inc", "Foo:<", "Bar:>", "next", "inc", "next", "Foo:<", "Bar:>", "next"]
        );
    }

    #[test]
    fn event_sent_arguments() {
        let tape: Vec<String> = Vec::new();
        let tape_mutex = Mutex::new(tape);
        let mut sm = Demo::new();
        sm.event_monitor_mut()
            .add_event_sent_callback(Callback::new(|e: &EventPtr| {
                for param in e.info().parameters {
                    let name = param.name;
                    let val = lookup_i32(e.arguments().as_ref(), name);
                    tape_mutex.lock().unwrap().push(format!("{}={}", name, val));
                }
            }));
        sm.inc(5);
        sm.next(); // transition done=7, start=3
        sm.inc(6);
        sm.next(); // change-state
        sm.inc(7);
        sm.next(); // transition done=7, start=3
        assert_eq!(
            *tape_mutex.lock().unwrap(),
            vec!["arg=5", "done=7", "start=3", "arg=6", "arg=7", "done=7", "start=3"]
        );
    }

    #[test]
    fn event_handled_return() {
        let tape: Vec<i32> = Vec::new();
        let tape_mutex = Mutex::new(tape);
        let mut sm = Demo::new();
        sm.event_monitor_mut()
            .add_event_handled_callback(Callback::new(|e: &EventPtr| {
                let val = match e.return_value() {
                    None => -1,
                    Some(any) => *any.downcast_ref().unwrap_or(&-100),
                };
                tape_mutex.lock().unwrap().push(val);
            }));
        sm.inc(3); // 5
        sm.inc(5); // 10
        sm.next(); // transition
        sm.inc(5); // 12
        sm.inc(7); // 19
        sm.next(); // change-state
        sm.inc(3); // 3
        sm.inc(5); // 8
        sm.next(); // transition
        assert_eq!(
            *tape_mutex.lock().unwrap(),
            vec![5, 10, -1, -1, -1, 12, 19, -1, 3, 8, -1, -1, -1]
        );
    }

    #[test]
    fn transition_callbacks() {
        let tape: Vec<String> = Vec::new();
        let tape_mutex = Mutex::new(tape);
        let mut sm = Demo::new();
        sm.event_monitor_mut()
            .add_transition_callback(Callback::new(|t: &Transition| {
                tape_mutex
                    .lock()
                    .unwrap()
                    .push(format!("kind: {:?}", t.info.kind));
            }));
        sm.event_monitor_mut()
            .add_transition_callback(Callback::new(|t: &Transition| {
                tape_mutex
                    .lock()
                    .unwrap()
                    .push(format!("old: {}", t.old_state.info().name));
                tape_mutex
                    .lock()
                    .unwrap()
                    .push(format!("new: {}", t.new_state.info().name));
            }));
        sm.next();
        assert_eq!(
            *tape_mutex.lock().unwrap(),
            vec!["kind: Transition", "old: Foo", "new: Bar"]
        );
        tape_mutex.lock().unwrap().clear();
        sm.next();
        assert_eq!(
            *tape_mutex.lock().unwrap(),
            vec!["kind: ChangeState", "old: Bar", "new: Foo"]
        );
        tape_mutex.lock().unwrap().clear();
        sm.next();
        assert_eq!(
            *tape_mutex.lock().unwrap(),
            vec!["kind: Transition", "old: Foo", "new: Bar"]
        );
    }

    #[test]
    fn transition_info_id() {
        let tape: Vec<usize> = Vec::new();
        let tape_mutex = Mutex::new(tape);
        let mut sm = Demo::new();
        sm.event_monitor_mut()
            .add_transition_callback(Callback::new(|t: &Transition| {
                tape_mutex.lock().unwrap().push(t.info.id);
            }));
        sm.next();
        sm.inc(5);
        sm.next();
        sm.next();
        assert_eq!(*tape_mutex.lock().unwrap(), vec![1, 2, 1]);
    }

    #[test]
    fn transition_static_info_agrees() {
        let agree = AtomicBool::new(false);
        let mut sm = Demo::new();
        sm.event_monitor_mut()
            .add_transition_callback(Callback::new(|t: &Transition| {
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

    #[test]
    fn enter_exit_arguments() {
        let tape: Vec<(i32, i32, i32, i32)> = Vec::new();
        let tape_mutex = Mutex::new(tape);
        let mut sm = Demo::new();
        sm.event_monitor_mut()
            .add_transition_callback(Callback::new(|t: &Transition| {
                let exit = match t.exit_event.as_ref() {
                    Some(event) => event.arguments(),
                    None => Empty::arc(),
                };
                let enter = match t.enter_event.as_ref() {
                    Some(event) => event.arguments(),
                    None => Empty::arc(),
                };
                tape_mutex.lock().unwrap().push((
                    lookup_i32(exit.as_ref(), "done"),
                    lookup_i32(exit.as_ref(), "end"),
                    lookup_i32(enter.as_ref(), "init"),
                    lookup_i32(enter.as_ref(), "start"),
                ));
            }));
        sm.inc(10);
        sm.next(); // transition done=12, start=3
        sm.inc(10);
        sm.next(); // change-state
        sm.inc(10);
        sm.next(); // transition done=10, start=3
        assert_eq!(
            *tape_mutex.lock().unwrap(),
            vec![(12, -1, -1, 3,), (-1, -1, -1, -1), (10, -1, -1, 3)]
        );
    }

    #[test]
    fn machines_in_separate_threads() {
        use std::sync::mpsc;
        use std::thread;
        use std::time::Duration;

        let (tx1, rx) = mpsc::channel();
        let tx2 = tx1.clone();

        let thread1 = thread::spawn(move || {
            let mut sm1 = Demo::new();
            sm1.event_monitor_mut()
                .add_event_sent_callback(Callback::new(move |e: &EventPtr| {
                    tx1.send((1, e.info().name.to_string())).unwrap();
                }));
            sm1.inc(2); // inc
            thread::sleep(Duration::from_millis(20));
            sm1.next(); // next, Foo:<, Bar:>
            sm1.inc(3); // inc
            thread::sleep(Duration::from_millis(30));
            sm1.next(); // next
            thread::sleep(Duration::from_millis(20));
            sm1.next(); // next, Foo:<, Bar:>
        });

        let thread2 = thread::spawn(move || {
            let mut sm2 = Demo::new();
            sm2.event_monitor_mut()
                .add_event_sent_callback(Callback::new(move |e: &EventPtr| {
                    tx2.send((2, e.info().name.to_string())).unwrap();
                }));
            sm2.inc(2); // inc
            sm2.inc(3); // inc
            thread::sleep(Duration::from_millis(50));
            sm2.next(); // next, Foo:<, Bar:>
            sm2.inc(4); // inc
            sm2.next(); // next
        });

        thread1.join().unwrap();
        thread2.join().unwrap();

        let out: Vec<(u8, String)> = rx.iter().collect();
        assert_eq!(out.len(), 16);

        let mut out1 = Vec::new();
        let mut out2 = Vec::new();
        for (thread, event) in out {
            if thread == 1 {
                out1.push(event);
            } else if thread == 2 {
                out2.push(event);
            }
        }

        assert_eq!(
            out1,
            vec!["inc", "next", "Foo:<", "Bar:>", "inc", "next", "next", "Foo:<", "Bar:>"]
        );
        assert_eq!(
            out2,
            vec!["inc", "inc", "next", "Foo:<", "Bar:>", "inc", "next"]
        );
    }
}
