//! This module demonstrates what generated code that realizes the runtime
//! interfaces should look like.
//!
//! Frame spec:
//! ```
//! #Demo
//!     -interface-
//!     inc [arg:i32] : i32
//!     next
//!     -machine-
//!     $Init
//!         |>| -> (2) $Foo ^
//!
//!     $Foo
//!         var x:i32 = 0
//!
//!         |>| [init:i32]
//!             x = init ^
//!
//!         |<| [done:i32] ^
//!
//!         |inc| [arg:i32]
//!             x = x + arg
//!             ^(x)
//!
//!         |next|
//!             #.x = #.x + x
//!             (x) -> (3) $Bar (4) ^
//!
//!     $Bar [tilt:i32]
//!         var y:i32 = 0
//!
//!         |>| [start:i32]
//!             y = start + tilt ^
//!
//!         |<| [end:i32] ^
//!
//!         |inc| [arg:i32]
//!             y = y + arg
//!             ^(y)
//!
//!         |next|
//!             #.y = #.y + y
//!             ->> $Foo ^
//!     -actions-
//!     -domain-
//!     var x:i32 = 0
//!     var y:i32 = 0
//! ##
//! ```

use frame_runtime::*;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

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
        name: "Demo",
        variables: &[
            NameInfo {
                name: "x",
                vtype: "i32",
            },
            NameInfo {
                name: "y",
                vtype: "i32",
            },
        ],
        states: &[STATE_INIT, STATE_FOO, STATE_BAR],
        interface: &[EVENTS[0], EVENTS[1]],
        actions: ACTIONS,
        events: EVENTS,
        transitions: TRANSITIONS,
    };
    static MACHINE_CELL: OnceCell<&MachineInfo> = OnceCell::new();
    static STATE_INIT: &StateInfo = &StateInfo {
        machine_cell: &MACHINE_CELL,
        name: "Init",
        parent: None,
        parameters: &[],
        variables: &[],
        handlers: &[EVENTS[4]],
        is_stack_pop: false,
    };
    static STATE_FOO: &StateInfo = &StateInfo {
        machine_cell: &MACHINE_CELL,
        name: "Foo",
        parent: None,
        parameters: &[],
        variables: &[NameInfo {
            name: "x",
            vtype: "i32",
        }],
        handlers: &[EVENTS[4], EVENTS[5], EVENTS[0], EVENTS[1]],
        is_stack_pop: false,
    };
    static STATE_BAR: &StateInfo = &StateInfo {
        machine_cell: &MACHINE_CELL,
        name: "Bar",
        parent: None,
        parameters: &[NameInfo {
            name: "tilt",
            vtype: "i32",
        }],
        variables: &[NameInfo {
            name: "y",
            vtype: "i32",
        }],
        handlers: &[EVENTS[6], EVENTS[7], EVENTS[0], EVENTS[1]],
        is_stack_pop: false,
    };
    const ACTIONS: &[&MethodInfo] = &[];
    const EVENTS: &[&MethodInfo] = &[
        &MethodInfo {
            name: "inc",
            parameters: &[NameInfo {
                name: "arg",
                vtype: "i32",
            }],
            return_type: Some("i32"),
        },
        &MethodInfo {
            name: "next",
            parameters: &[],
            return_type: None,
        },
        &MethodInfo {
            name: "Bar:>",
            parameters: &[NameInfo {
                name: "start",
                vtype: "i32",
            }],
            return_type: None,
        },
        &MethodInfo {
            name: "Foo:>",
            parameters: &[NameInfo {
                name: "init",
                vtype: "i32",
            }],
            return_type: None,
        },
        &MethodInfo {
            name: "Init:>",
            parameters: &[],
            return_type: None,
        },
        &MethodInfo {
            name: "Bar:<",
            parameters: &[NameInfo {
                name: "end",
                vtype: "i32",
            }],
            return_type: None,
        },
        &MethodInfo {
            name: "Foo:<",
            parameters: &[NameInfo {
                name: "done",
                vtype: "i32",
            }],
            return_type: None,
        },
        &MethodInfo {
            name: "Init:<",
            parameters: &[],
            return_type: None,
        },
    ];
    static TRANSITIONS: &[&TransitionInfo] = &[
        &TransitionInfo {
            id: 0,
            kind: TransitionKind::Transition,
            event: EVENTS[4],
            label: "",
            source: STATE_INIT,
            target: STATE_FOO,
        },
        &TransitionInfo {
            id: 1,
            kind: TransitionKind::Transition,
            event: EVENTS[1],
            label: "",
            source: STATE_FOO,
            target: STATE_BAR,
        },
        &TransitionInfo {
            id: 2,
            kind: TransitionKind::ChangeState,
            event: EVENTS[1],
            label: "",
            source: STATE_BAR,
            target: STATE_FOO,
        },
    ];
}

enum FrameMessage {
    Enter(DemoState),
    Exit(DemoState),
    Inc,
    Next,
}

impl std::fmt::Display for FrameMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FrameMessage::Enter(DemoState::Init) => write!(f, "Init:>"),
            FrameMessage::Enter(DemoState::Foo) => write!(f, "Foo:>"),
            FrameMessage::Enter(DemoState::Bar) => write!(f, "Bar:>"),
            FrameMessage::Exit(DemoState::Init) => write!(f, "Init:<"),
            FrameMessage::Exit(DemoState::Foo) => write!(f, "Foo:<"),
            FrameMessage::Exit(DemoState::Bar) => write!(f, "Bar:<"),
            FrameMessage::Inc => write!(f, "inc"),
            FrameMessage::Next => write!(f, "next"),
        }
    }
}

enum FrameEventReturn {
    None,
    Inc { return_value: i32 },
}

impl FrameEventReturn {
    #[allow(dead_code)]
    fn get_inc_ret(&self) -> i32 {
        match self {
            FrameEventReturn::Inc { return_value } => *return_value,
            _ => panic!("Invalid return value"),
        }
    }
}

pub struct FrameEvent {
    message: FrameMessage,
    arguments: Rc<RefCell<FrameEventArgs>>,
    ret: FrameEventReturn,
}

impl FrameEvent {
    fn new(message: FrameMessage, arguments: FrameEventArgs) -> FrameEvent {
        FrameEvent {
            message,
            arguments: Rc::new(RefCell::new(arguments)),
            ret: FrameEventReturn::None,
        }
    }
}

impl MethodInstance for FrameEvent {
    fn info(&self) -> &MethodInfo {
        let msg = self.message.to_string();
        info::machine()
            .get_event(&msg)
            .unwrap_or_else(|| panic!("No runtime info for event: {}", msg))
    }
    fn arguments(&self) -> Rc<dyn Environment> {
        self.arguments.clone()
    }
    fn return_value(&self) -> Option<&dyn Any> {
        match self.ret {
            FrameEventReturn::None => None,
            FrameEventReturn::Inc { ref return_value } => Some(return_value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum DemoState {
    Init,
    Foo,
    Bar,
}

struct IncArgs {
    arg: i32,
}

impl Environment for IncArgs {
    fn lookup(&self, name: &str) -> Option<&dyn Any> {
        match name {
            "arg" => Some(&self.arg),
            _ => None,
        }
    }
}

struct FooEnterArgs {
    init: i32,
}

impl Environment for FooEnterArgs {
    fn lookup(&self, name: &str) -> Option<&dyn Any> {
        match name {
            "init" => Some(&self.init),
            _ => None,
        }
    }
}

struct FooExitArgs {
    done: i32,
}

impl Environment for FooExitArgs {
    fn lookup(&self, name: &str) -> Option<&dyn Any> {
        match name {
            "done" => Some(&self.done),
            _ => None,
        }
    }
}

struct BarEnterArgs {
    start: i32,
}

impl Environment for BarEnterArgs {
    fn lookup(&self, name: &str) -> Option<&dyn Any> {
        match name {
            "start" => Some(&self.start),
            _ => None,
        }
    }
}

struct BarExitArgs {
    end: i32,
}

impl Environment for BarExitArgs {
    fn lookup(&self, name: &str) -> Option<&dyn Any> {
        match name {
            "end" => Some(&self.end),
            _ => None,
        }
    }
}

#[allow(dead_code)]
enum FrameEventArgs {
    None,
    Inc(IncArgs),
    FooEnter(FooEnterArgs),
    FooExit(FooExitArgs),
    BarEnter(BarEnterArgs),
    BarExit(BarExitArgs),
}

impl Environment for FrameEventArgs {
    fn lookup(&self, name: &str) -> Option<&dyn Any> {
        match self {
            FrameEventArgs::None => None,
            FrameEventArgs::Inc(args) => args.lookup(name),
            FrameEventArgs::FooEnter(args) => args.lookup(name),
            FrameEventArgs::FooExit(args) => args.lookup(name),
            FrameEventArgs::BarEnter(args) => args.lookup(name),
            FrameEventArgs::BarExit(args) => args.lookup(name),
        }
    }
}

impl FrameEventArgs {
    fn inc_args(&self) -> &IncArgs {
        match self {
            FrameEventArgs::Inc(args) => args,
            _ => panic!("Failed conversion to IncArgs"),
        }
    }
    fn foo_enter_args(&self) -> &FooEnterArgs {
        match self {
            FrameEventArgs::FooEnter(args) => args,
            _ => panic!("Failed conversion to FooEnterArgs"),
        }
    }
    #[allow(dead_code)]
    fn foo_exit_args(&self) -> &FooExitArgs {
        match self {
            FrameEventArgs::FooExit(args) => args,
            _ => panic!("Failed conversion to FooExitArgs"),
        }
    }
    fn bar_enter_args(&self) -> &BarEnterArgs {
        match self {
            FrameEventArgs::BarEnter(args) => args,
            _ => panic!("Failed conversion to BarEnterArgs"),
        }
    }
    #[allow(dead_code)]
    fn bar_exit_args(&self) -> &BarExitArgs {
        match self {
            FrameEventArgs::BarExit(args) => args,
            _ => panic!("Failed conversion to BarExitArgs"),
        }
    }
}

struct InitStateContext {}

impl StateInstance for InitStateContext {
    fn info(&self) -> &'static StateInfo {
        info::machine().states[0]
    }
}

struct FooStateVars {
    x: i32,
}

impl Environment for FooStateVars {
    fn lookup(&self, name: &str) -> Option<&dyn Any> {
        match name {
            "x" => Some(&self.x),
            _ => None,
        }
    }
}

struct FooStateContext {
    state_vars: Rc<RefCell<FooStateVars>>,
}

impl StateInstance for FooStateContext {
    fn info(&self) -> &'static StateInfo {
        info::machine().states[1]
    }
    fn variables(&self) -> Rc<dyn Environment> {
        self.state_vars.clone()
    }
}

struct BarStateArgs {
    tilt: i32,
}

impl Environment for BarStateArgs {
    fn lookup(&self, name: &str) -> Option<&dyn Any> {
        match name {
            "tilt" => Some(&self.tilt),
            _ => None,
        }
    }
}

struct BarStateVars {
    y: i32,
}

impl Environment for BarStateVars {
    fn lookup(&self, name: &str) -> Option<&dyn Any> {
        match name {
            "y" => Some(&self.y),
            _ => None,
        }
    }
}

struct BarStateContext {
    state_args: Rc<RefCell<BarStateArgs>>,
    state_vars: Rc<RefCell<BarStateVars>>,
}

impl StateInstance for BarStateContext {
    fn info(&self) -> &'static StateInfo {
        info::machine().states[2]
    }
    fn arguments(&self) -> Rc<dyn Environment> {
        self.state_args.clone()
    }
    fn variables(&self) -> Rc<dyn Environment> {
        self.state_vars.clone()
    }
}

enum StateContext {
    Init(InitStateContext),
    Foo(FooStateContext),
    Bar(BarStateContext),
}

impl StateContext {
    // fn as_state_instance(&self) -> Ref<dyn StateInstance> {
    //     match self {
    //         StateContext::Init(context) => Ref::map(context.borrow(), |c| c as &dyn StateInstance),
    //         StateContext::Foo(context) => Ref::map(context.borrow(), |c| c as &dyn StateInstance),
    //         StateContext::Bar(context) => Ref::map(context.borrow(), |c| c as &dyn StateInstance),
    //     }
    // }
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

impl StateInstance for StateContext {
    fn info(&self) -> &'static StateInfo {
        match self {
            StateContext::Init(context) => context.info(),
            StateContext::Foo(context) => context.info(),
            StateContext::Bar(context) => context.info(),
        }
    }
    fn arguments(&self) -> Rc<dyn Environment> {
        match self {
            StateContext::Init(context) => context.arguments(),
            StateContext::Foo(context) => context.arguments(),
            StateContext::Bar(context) => context.arguments(),
        }
    }
    fn variables(&self) -> Rc<dyn Environment> {
        match self {
            StateContext::Init(context) => context.variables(),
            StateContext::Foo(context) => context.variables(),
            StateContext::Bar(context) => context.variables(),
        }
    }
}

struct DomainVars {
    x: i32,
    y: i32,
}

impl Environment for DomainVars {
    fn lookup(&self, name: &str) -> Option<&dyn Any> {
        match name {
            "x" => Some(&self.x),
            "y" => Some(&self.y),
            _ => None,
        }
    }
}

pub struct Demo<'a> {
    state: DemoState,
    state_context: Rc<StateContext>,
    event_monitor: EventMonitor<'a>,
    domain_vars: Rc<RefCell<DomainVars>>,
}

impl<'a> MachineInstance<'a> for Demo<'a> {
    fn info(&self) -> &'static MachineInfo {
        info::machine()
    }
    fn state(&self) -> Rc<dyn StateInstance> {
        self.state_context.clone()
    }
    fn variables(&self) -> Rc<dyn Environment> {
        self.domain_vars.clone()
    }
    fn event_monitor(&self) -> &EventMonitor<'a> {
        &self.event_monitor
    }
    fn event_monitor_mut(&mut self) -> &mut EventMonitor<'a> {
        &mut self.event_monitor
    }
}

impl<'a> Demo<'a> {
    pub fn new() -> Demo<'a> {
        let context = InitStateContext {};
        let next_state_context = Rc::new(StateContext::Init(context));
        let event_monitor = EventMonitor::new(Some(3), Some(2));
        let domain_vars = Rc::new(RefCell::new(DomainVars { x: 0, y: 0 }));
        let mut machine = Demo {
            state: DemoState::Init,
            state_context: next_state_context,
            event_monitor,
            domain_vars,
        };
        machine.initialize();
        machine
    }

    fn initialize(&mut self) {
        let mut e = FrameEvent::new(FrameMessage::Enter(self.state), FrameEventArgs::None);
        self.handle_event(&mut e);
    }

    pub fn inc(&mut self, arg: i32) -> i32 {
        let frame_args = FrameEventArgs::Inc(IncArgs { arg });
        let mut frame_event = FrameEvent::new(FrameMessage::Inc, frame_args);
        self.handle_event(&mut frame_event);
        match frame_event.ret {
            FrameEventReturn::Inc { return_value } => return_value,
            _ => panic!("Bad return value for inc"),
        }
    }

    pub fn next(&mut self) {
        let frame_args = FrameEventArgs::None;
        let mut frame_event = FrameEvent::new(FrameMessage::Next, frame_args);
        self.handle_event(&mut frame_event);
    }

    #[allow(clippy::single_match)]
    #[allow(unused_variables)]
    fn init_handler(&mut self, frame_event: &mut FrameEvent) {
        let state_context_clone = Rc::clone(&self.state_context);
        let this_state_context = state_context_clone.init_context();
        match frame_event.message {
            FrameMessage::Enter(_) => {
                // Start transition
                let exit_args = FrameEventArgs::None;
                let enter_args = FrameEventArgs::FooEnter(FooEnterArgs { init: 2 });
                let context = FooStateContext {
                    state_vars: Rc::new(RefCell::new(FooStateVars { x: 0 })),
                };
                let next_state_context = Rc::new(StateContext::Foo(context));
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

    fn foo_handler(&mut self, frame_event: &mut FrameEvent) {
        let state_context_clone = Rc::clone(&self.state_context);
        let this_state_context = state_context_clone.foo_context();
        match frame_event.message {
            FrameMessage::Enter(_) => {
                this_state_context.state_vars.borrow_mut().x =
                    frame_event.arguments.borrow().foo_enter_args().init;
            }
            FrameMessage::Exit(_) => {}
            FrameMessage::Inc => {
                this_state_context.state_vars.borrow_mut().x +=
                    frame_event.arguments.borrow().inc_args().arg;
                frame_event.ret = FrameEventReturn::Inc {
                    return_value: this_state_context.state_vars.borrow().x,
                };
            }
            FrameMessage::Next => {
                self.domain_vars.borrow_mut().x += this_state_context.state_vars.borrow().x;
                // Start transition
                let exit_args = FrameEventArgs::FooExit(FooExitArgs {
                    done: this_state_context.state_vars.borrow().x,
                });
                let enter_args = FrameEventArgs::BarEnter(BarEnterArgs { start: 3 });
                let context = BarStateContext {
                    state_args: Rc::new(RefCell::new(BarStateArgs { tilt: 4 })),
                    state_vars: Rc::new(RefCell::new(BarStateVars { y: 0 })),
                };
                let next_state_context = Rc::new(StateContext::Bar(context));
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

    fn bar_handler(&mut self, frame_event: &mut FrameEvent) {
        let state_context_clone = Rc::clone(&self.state_context);
        let this_state_context = state_context_clone.bar_context();
        match frame_event.message {
            FrameMessage::Enter(_) => {
                this_state_context.state_vars.borrow_mut().y =
                    frame_event.arguments.borrow().bar_enter_args().start
                        + this_state_context.state_args.borrow().tilt;
            }
            FrameMessage::Exit(_) => {}
            FrameMessage::Inc => {
                this_state_context.state_vars.borrow_mut().y +=
                    frame_event.arguments.borrow().inc_args().arg;
                frame_event.ret = FrameEventReturn::Inc {
                    return_value: this_state_context.state_vars.borrow().y,
                };
            }
            FrameMessage::Next => {
                self.domain_vars.borrow_mut().y += this_state_context.state_vars.borrow().y;
                // Start change state
                let context = FooStateContext {
                    state_vars: Rc::new(RefCell::new(FooStateVars { x: 0 })),
                };
                let next_state_context = Rc::new(StateContext::Foo(context));
                self.change_state(
                    info::machine().transitions[2],
                    DemoState::Foo,
                    next_state_context,
                );
            }
        }
    }

    fn handle_event(&mut self, frame_event: &mut FrameEvent) {
        match self.state {
            DemoState::Init => self.init_handler(frame_event),
            DemoState::Foo => self.foo_handler(frame_event),
            DemoState::Bar => self.bar_handler(frame_event),
        }
    }

    fn transition(
        &mut self,
        transition_info: &'static TransitionInfo,
        exit_args: FrameEventArgs,
        enter_args: FrameEventArgs,
        new_state: DemoState,
        new_state_context: Rc<StateContext>,
    ) {
        // exit event for old state
        let mut exit_event = FrameEvent::new(FrameMessage::Exit(self.state), exit_args);
        self.handle_event(&mut exit_event);

        // update state
        let old_state_context = self.state_context.clone();
        self.state = new_state;
        self.state_context = new_state_context.clone();

        // enter event for new state
        let mut enter_event = FrameEvent::new(FrameMessage::Enter(self.state), enter_args);
        self.handle_event(&mut enter_event);

        // call transition callbacks
        self.event_monitor.transition_occurred(
            transition_info,
            old_state_context,
            new_state_context,
            Some(Rc::new(exit_event)),
            Some(Rc::new(enter_event)),
        );
    }

    fn change_state(
        &mut self,
        transition_info: &'static TransitionInfo,
        new_state: DemoState,
        new_state_context: Rc<StateContext>,
    ) {
        // update state
        let old_state_context = self.state_context.clone();
        self.state = new_state;
        self.state_context = new_state_context.clone();

        // call change-state callbacks
        self.event_monitor.transition_occurred(
            transition_info,
            old_state_context,
            new_state_context,
            None,
            None,
        );
    }
}

impl<'a> Default for Demo<'a> {
    fn default() -> Self {
        Demo::new()
    }
}

// Begin testing code

/// Helper function to lookup an `i32` value in an environment.
/// Returns -1 if the lookup fails for any reason.
fn lookup_i32(env: Rc<dyn Environment>, name: &str) -> i32 {
    match env.lookup(name) {
        None => -1,
        Some(any) => *any.downcast_ref().unwrap_or(&-1),
    }
}

#[test]
fn machine_info() {
    let sm = Demo::new();
    assert_eq!("Demo", sm.info().name);
    assert_eq!(2, sm.info().variables.len());
    assert_eq!(3, sm.info().states.len());
    assert_eq!(2, sm.info().interface.len());
    assert_eq!(0, sm.info().actions.len());
    assert_eq!(8, sm.info().events.len());
    assert_eq!(3, sm.info().transitions.len());
}

#[test]
fn domain_variable_info() {
    let sm = Demo::new();
    let x = sm.info().get_variable("x");
    let y = sm.info().get_variable("y");
    let z = sm.info().get_variable("z");
    assert!(x.is_some());
    assert!(y.is_some());
    assert!(z.is_none());
    assert_eq!("i32", x.unwrap().vtype);
    assert_eq!("i32", y.unwrap().vtype);
}

#[test]
#[allow(clippy::blacklisted_name)]
fn state_info() {
    let sm = Demo::new();
    let init = sm.info().get_state("Init");
    let foo = sm.info().get_state("Foo");
    let bar = sm.info().get_state("Bar");
    let baz = sm.info().get_state("Baz");
    assert!(init.is_some());
    assert!(foo.is_some());
    assert!(bar.is_some());
    assert!(baz.is_none());
    assert_eq!(1, init.unwrap().handlers.len());
    assert_eq!(4, foo.unwrap().handlers.len());
    assert_eq!(4, bar.unwrap().handlers.len());
    assert_eq!(0, init.unwrap().variables.len());
    assert_eq!(1, foo.unwrap().variables.len());
    assert_eq!(1, bar.unwrap().variables.len());
    assert_eq!(0, init.unwrap().parameters.len());
    assert_eq!(0, foo.unwrap().parameters.len());
    assert_eq!(1, bar.unwrap().parameters.len());
}

#[test]
#[allow(clippy::blacklisted_name)]
fn transition_info() {
    let sm = Demo::new();
    let foo = sm.info().get_state("Foo").unwrap();
    let incoming = foo.incoming_transitions();
    let outgoing = foo.outgoing_transitions();
    assert_eq!(2, incoming.len());
    assert_eq!(1, outgoing.len());

    assert_eq!("Init:>", incoming[0].event.name);
    assert_eq!("Init", incoming[0].source.name);
    assert_eq!("Foo", incoming[0].target.name);
    assert!(incoming[0].is_transition());
    assert!(!incoming[0].is_change_state());

    assert_eq!("next", incoming[1].event.name);
    assert_eq!("Bar", incoming[1].source.name);
    assert_eq!("Foo", incoming[1].target.name);
    assert!(!incoming[1].is_transition());
    assert!(incoming[1].is_change_state());

    assert_eq!("next", outgoing[0].event.name);
    assert_eq!("Foo", outgoing[0].source.name);
    assert_eq!("Bar", outgoing[0].target.name);
    assert!(outgoing[0].is_transition());
    assert!(!outgoing[0].is_change_state());
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
    assert_eq!(0, lookup_i32(sm.variables(), "x"));
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
    assert_eq!(2, lookup_i32(sm.state().variables(), "x"));
    assert!(sm.state().variables().lookup("y").is_none());
    sm.inc(3);
    sm.inc(4);
    assert_eq!(9, lookup_i32(sm.state().variables(), "x"));
    sm.next();
    assert_eq!(7, lookup_i32(sm.state().variables(), "y"));
    assert!(sm.state().variables().lookup("x").is_none());
    sm.inc(5);
    sm.inc(6);
    assert_eq!(18, lookup_i32(sm.state().variables(), "y"));
    sm.next();
    assert_eq!(0, lookup_i32(sm.state().variables(), "x"));
    sm.inc(7);
    assert_eq!(7, lookup_i32(sm.state().variables(), "x"));
}

#[test]
#[rustfmt::skip]
fn state_arguments() {
    let mut sm = Demo::new();
    assert!(sm.state().arguments().lookup("x").is_none());
    assert!(sm.state().arguments().lookup("y").is_none());
    assert!(sm.state().arguments().lookup("tilt").is_none());
    sm.next();
    assert!(sm.state().arguments().lookup("x").is_none());
    assert!(sm.state().arguments().lookup("y").is_none());
    assert_eq!(4, lookup_i32(sm.state().arguments(), "tilt"));
    sm.next();
    assert!(sm.state().arguments().lookup("tilt").is_none());
}

#[test]
#[rustfmt::skip]
fn transition_callbacks() {
    let tape: Vec<String> = Vec::new();
    let tape_mutex = Mutex::new(tape);
    let mut sm = Demo::new();
    sm.event_monitor_mut().add_transition_callback(|e| {
        tape_mutex
            .lock()
            .unwrap()
            .push(format!("kind: {:?}", e.info.kind));
    });
    sm.event_monitor_mut().add_transition_callback(|e| {
        tape_mutex
            .lock()
            .unwrap()
            .push(format!("old: {}", e.old_state.info().name));
        tape_mutex
            .lock()
            .unwrap()
            .push(format!("new: {}", e.new_state.info().name));
    });
    sm.next();
    assert_eq!(*tape_mutex.lock().unwrap(), vec!["kind: Transition", "old: Foo", "new: Bar"]);
    tape_mutex.lock().unwrap().clear();
    sm.next();
    assert_eq!(*tape_mutex.lock().unwrap(), vec!["kind: ChangeState", "old: Bar", "new: Foo"]);
    tape_mutex.lock().unwrap().clear();
    sm.next();
    assert_eq!(*tape_mutex.lock().unwrap(), vec!["kind: Transition", "old: Foo", "new: Bar"]);
}

#[test]
fn transition_info_id() {
    let tape: Vec<usize> = Vec::new();
    let tape_mutex = Mutex::new(tape);
    let mut sm = Demo::new();
    sm.event_monitor_mut().add_transition_callback(|e| {
        tape_mutex.lock().unwrap().push(e.info.id);
    });
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

#[test]
fn enter_exit_arguments() {
    let tape: Vec<(i32, i32, i32, i32)> = Vec::new();
    let tape_mutex = Mutex::new(tape);
    let mut sm = Demo::new();
    sm.event_monitor_mut().add_transition_callback(|i| {
        let exit = match i.exit_event.as_ref() {
            Some(event) => event.arguments(),
            None => Empty::new_rc(),
        };
        let enter = match i.enter_event.as_ref() {
            Some(event) => event.arguments(),
            None => Empty::new_rc(),
        };
        tape_mutex.lock().unwrap().push((
            lookup_i32(exit.clone(), "done"),
            lookup_i32(exit, "end"),
            lookup_i32(enter.clone(), "init"),
            lookup_i32(enter, "start"),
        ));
    });
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

// #[test]
// fn smcat_renderer() {
//     let smcat = smcat::Renderer::new(Box::new(smcat::SimpleStyle {}));
//     println!("Static rendering:\n{}", smcat.render_static(info::machine()));
//     let mut sm = Demo::new();
//     println!("Live rendering 1:\n{}", smcat.render_live(&sm, Some(0)));
//     sm.next();
//     println!("Live rendering 2:\n{}", smcat.render_live(&sm, Some(1)));
//     sm.next();
//     println!("Live rendering 3:\n{}", smcat.render_live(&sm, Some(2)));
// }
