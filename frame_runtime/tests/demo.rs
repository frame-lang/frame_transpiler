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
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

mod info {
    use frame_runtime::info::*;

    pub struct Machine {}
    pub struct StateInit {}
    pub struct StateFoo {}
    pub struct StateBar {}
    pub const MACHINE: &Machine = &Machine {};
    pub const STATE_INIT: &StateInit = &StateInit {};
    pub const STATE_FOO: &StateFoo = &StateFoo {};
    pub const STATE_BAR: &StateBar = &StateBar {};

    impl MachineInfo for Machine {
        fn name(&self) -> &'static str {
            "Demo"
        }
        fn variables(&self) -> Vec<NameInfo> {
            vec![
                NameInfo {
                    name: "x",
                    vtype: "i32",
                },
                NameInfo {
                    name: "y",
                    vtype: "i32",
                },
            ]
        }
        fn states(&self) -> Vec<&dyn StateInfo> {
            vec![STATE_INIT, STATE_FOO, STATE_BAR]
        }
        fn events(&self) -> Vec<MethodInfo> {
            vec![
                MethodInfo {
                    name: "inc",
                    parameters: vec![NameInfo {
                        name: "arg",
                        vtype: "i32",
                    }],
                    return_type: Some("i32"),
                },
                MethodInfo {
                    name: "next",
                    parameters: vec![],
                    return_type: None,
                },
                MethodInfo {
                    name: "Init:>",
                    parameters: vec![],
                    return_type: None,
                },
                MethodInfo {
                    name: "Init:<",
                    parameters: vec![],
                    return_type: None,
                },
                MethodInfo {
                    name: "Foo:>",
                    parameters: vec![NameInfo {
                        name: "init",
                        vtype: "i32",
                    }],
                    return_type: None,
                },
                MethodInfo {
                    name: "Foo:<",
                    parameters: vec![NameInfo {
                        name: "done",
                        vtype: "i32",
                    }],
                    return_type: None,
                },
                MethodInfo {
                    name: "Bar:>",
                    parameters: vec![NameInfo {
                        name: "start",
                        vtype: "i32",
                    }],
                    return_type: None,
                },
                MethodInfo {
                    name: "Bar:<",
                    parameters: vec![NameInfo {
                        name: "end",
                        vtype: "i32",
                    }],
                    return_type: None,
                },
            ]
        }
        fn interface(&self) -> Vec<MethodInfo> {
            vec![MACHINE.events()[0].clone(), MACHINE.events()[1].clone()]
        }
        fn actions(&self) -> Vec<MethodInfo> {
            vec![]
        }
        fn transitions(&self) -> Vec<TransitionInfo> {
            vec![
                TransitionInfo {
                    kind: TransitionKind::Transition,
                    event: MACHINE.events()[2].clone(),
                    label: "",
                    source: MACHINE.states()[0],
                    target: MACHINE.states()[1],
                },
                TransitionInfo {
                    kind: TransitionKind::Transition,
                    event: MACHINE.events()[1].clone(),
                    label: "",
                    source: MACHINE.states()[1],
                    target: MACHINE.states()[2],
                },
                TransitionInfo {
                    kind: TransitionKind::ChangeState,
                    event: MACHINE.events()[1].clone(),
                    label: "",
                    source: MACHINE.states()[2],
                    target: MACHINE.states()[1],
                },
            ]
        }
    }

    impl StateInfo for StateInit {
        fn machine(&self) -> &dyn MachineInfo {
            MACHINE
        }
        fn name(&self) -> &'static str {
            "Init"
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
            vec![MethodInfo {
                name: ">",
                parameters: vec![],
                return_type: None,
            }]
        }
    }

    impl StateInfo for StateFoo {
        fn machine(&self) -> &dyn MachineInfo {
            MACHINE
        }
        fn name(&self) -> &'static str {
            "Foo"
        }
        fn parent(&self) -> Option<&dyn StateInfo> {
            None
        }
        fn parameters(&self) -> Vec<NameInfo> {
            vec![]
        }
        fn variables(&self) -> Vec<NameInfo> {
            vec![NameInfo {
                name: "x",
                vtype: "i32",
            }]
        }
        fn handlers(&self) -> Vec<MethodInfo> {
            vec![
                MACHINE.events()[4].clone(),
                MACHINE.events()[5].clone(),
                MACHINE.events()[0].clone(),
                MACHINE.events()[1].clone(),
            ]
        }
    }

    impl StateInfo for StateBar {
        fn machine(&self) -> &dyn MachineInfo {
            MACHINE
        }
        fn name(&self) -> &'static str {
            "Bar"
        }
        fn parent(&self) -> Option<&dyn StateInfo> {
            None
        }
        fn parameters(&self) -> Vec<NameInfo> {
            vec![NameInfo {
                name: "tilt",
                vtype: "i32",
            }]
        }
        fn variables(&self) -> Vec<NameInfo> {
            vec![NameInfo {
                name: "y",
                vtype: "i32",
            }]
        }
        fn handlers(&self) -> Vec<MethodInfo> {
            vec![
                MACHINE.events()[6].clone(),
                MACHINE.events()[7].clone(),
                MACHINE.events()[0].clone(),
                MACHINE.events()[1].clone(),
            ]
        }
    }
}

enum FrameMessage {
    Enter,
    Exit,
    Inc,
    Next,
}

impl std::fmt::Display for FrameMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FrameMessage::Enter => write!(f, "Enter"),
            FrameMessage::Exit => write!(f, "Exit"),
            FrameMessage::Inc => write!(f, "Inc"),
            FrameMessage::Next => write!(f, "Next"),
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

pub struct FrameEvent<'a> {
    message: FrameMessage,
    arguments: &'a FrameEventArgs,
    ret: FrameEventReturn,
}

impl<'a> FrameEvent<'a> {
    fn new(message: FrameMessage, arguments: &'a FrameEventArgs) -> FrameEvent<'a> {
        FrameEvent {
            message,
            arguments,
            ret: FrameEventReturn::None,
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
            FrameEventArgs::None => EMPTY.lookup(name),
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

impl StateInstance for DemoState {
    fn info(&self) -> &'static dyn StateInfo {
        match self {
            DemoState::Init => info::STATE_INIT,
            DemoState::Foo => info::STATE_FOO,
            DemoState::Bar => info::STATE_BAR,
        }
    }
}

impl StateInstance for InitStateContext {
    fn info(&self) -> &'static dyn StateInfo {
        info::STATE_INIT
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
    state_vars: FooStateVars,
}

impl StateInstance for FooStateContext {
    fn info(&self) -> &'static dyn StateInfo {
        info::STATE_FOO
    }
    fn variables(&self) -> &dyn Environment {
        &self.state_vars
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
    state_args: BarStateArgs,
    state_vars: BarStateVars,
}

impl StateInstance for BarStateContext {
    fn info(&self) -> &'static dyn StateInfo {
        info::STATE_BAR
    }
    fn arguments(&self) -> &dyn Environment {
        &self.state_args
    }
    fn variables(&self) -> &dyn Environment {
        &self.state_vars
    }
}

enum StateContext {
    Init(RefCell<InitStateContext>),
    Foo(RefCell<FooStateContext>),
    Bar(RefCell<BarStateContext>),
}

impl StateContext {
    fn as_state_instance(&self) -> Ref<dyn StateInstance> {
        match self {
            StateContext::Init(context) => Ref::map(context.borrow(), |c| c as &dyn StateInstance),
            StateContext::Foo(context) => Ref::map(context.borrow(), |c| c as &dyn StateInstance),
            StateContext::Bar(context) => Ref::map(context.borrow(), |c| c as &dyn StateInstance),
        }
    }
    fn init_context(&self) -> &RefCell<InitStateContext> {
        match self {
            StateContext::Init(context) => context,
            _ => panic!("Failed conversion to InitStateContext"),
        }
    }
    fn foo_context(&self) -> &RefCell<FooStateContext> {
        match self {
            StateContext::Foo(context) => context,
            _ => panic!("Failed conversion to FooStateContext"),
        }
    }
    fn bar_context(&self) -> &RefCell<BarStateContext> {
        match self {
            StateContext::Bar(context) => context,
            _ => panic!("Failed conversion to BarStateContext"),
        }
    }
}

pub struct Demo<'a> {
    state: DemoState,
    state_context: Rc<StateContext>,
    callback_manager: CallbackManager<'a>,
    x: i32,
    y: i32,
}

impl<'a> Environment for Demo<'a> {
    fn lookup(&self, name: &str) -> Option<&dyn Any> {
        match name {
            "x" => Some(&self.x),
            "y" => Some(&self.y),
            _ => None,
        }
    }
}

impl<'a> MachineInstance<'a> for Demo<'a> {
    fn info(&self) -> &'static dyn MachineInfo {
        info::MACHINE
    }
    fn state(&self) -> Ref<dyn StateInstance> {
        self.state_context.as_ref().as_state_instance()
    }
    fn variables(&self) -> &dyn Environment {
        self
    }
    fn callback_manager(&mut self) -> &mut CallbackManager<'a> {
        &mut self.callback_manager
    }
}

impl<'a> Demo<'a> {
    pub fn new() -> Demo<'a> {
        let context = InitStateContext {};
        let next_state_context = Rc::new(StateContext::Init(RefCell::new(context)));
        let mut machine = Demo {
            state: DemoState::Init,
            state_context: next_state_context,
            callback_manager: CallbackManager::new(),
            x: 0,
            y: 0,
        };
        machine.initialize();
        machine
    }

    pub fn initialize(&mut self) {
        let mut e = FrameEvent::new(FrameMessage::Enter, &FrameEventArgs::None);
        self.handle_event(&mut e);
    }

    pub fn inc(&mut self, arg: i32) -> i32 {
        let frame_args = FrameEventArgs::Inc(IncArgs { arg });
        let mut frame_event = FrameEvent::new(FrameMessage::Inc, &frame_args);
        self.handle_event(&mut frame_event);
        match frame_event.ret {
            FrameEventReturn::Inc { return_value } => return_value,
            _ => panic!("Bad return value for inc"),
        }
    }

    pub fn next(&mut self) {
        let frame_args = FrameEventArgs::None;
        let mut frame_event = FrameEvent::new(FrameMessage::Next, &frame_args);
        self.handle_event(&mut frame_event);
    }

    #[allow(clippy::single_match)]
    fn init_handler(&mut self, frame_event: &mut FrameEvent) {
        let state_context_clone = Rc::clone(&self.state_context);
        let this_state_context = state_context_clone.init_context().borrow_mut();
        match frame_event.message {
            FrameMessage::Enter => {
                // Start transition
                let exit_args = FrameEventArgs::None;
                let enter_args = FrameEventArgs::FooEnter(FooEnterArgs { init: 2 });
                let context = FooStateContext {
                    state_vars: FooStateVars { x: 0 },
                };
                let next_state_context = Rc::new(StateContext::Foo(RefCell::new(context)));
                drop(this_state_context);
                self.transition(
                    &info::MACHINE.transitions()[0],
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
        let mut this_state_context = state_context_clone.foo_context().borrow_mut();
        match frame_event.message {
            FrameMessage::Enter => {
                this_state_context.state_vars.x = frame_event.arguments.foo_enter_args().init;
            }
            FrameMessage::Exit => {}
            FrameMessage::Inc => {
                this_state_context.state_vars.x += frame_event.arguments.inc_args().arg;
                frame_event.ret = FrameEventReturn::Inc {
                    return_value: this_state_context.state_vars.x,
                };
            }
            FrameMessage::Next => {
                self.x += this_state_context.state_vars.x;
                // Start transition
                let exit_args = FrameEventArgs::FooExit(FooExitArgs {
                    done: this_state_context.state_vars.x,
                });
                let enter_args = FrameEventArgs::BarEnter(BarEnterArgs { start: 3 });
                let context = BarStateContext {
                    state_args: BarStateArgs { tilt: 4 },
                    state_vars: BarStateVars { y: 0 },
                };
                let next_state_context = Rc::new(StateContext::Bar(RefCell::new(context)));
                drop(this_state_context);
                self.transition(
                    &info::MACHINE.transitions()[1],
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
        let mut this_state_context = state_context_clone.bar_context().borrow_mut();
        match frame_event.message {
            FrameMessage::Enter => {
                this_state_context.state_vars.y = frame_event.arguments.bar_enter_args().start
                    + this_state_context.state_args.tilt;
            }
            FrameMessage::Exit => {}
            FrameMessage::Inc => {
                this_state_context.state_vars.y += frame_event.arguments.inc_args().arg;
                frame_event.ret = FrameEventReturn::Inc {
                    return_value: this_state_context.state_vars.y,
                };
            }
            FrameMessage::Next => {
                self.y += this_state_context.state_vars.y;
                // Start change state
                let context = FooStateContext {
                    state_vars: FooStateVars { x: 0 },
                };
                let next_state_context = Rc::new(StateContext::Foo(RefCell::new(context)));
                drop(this_state_context);
                self.change_state(
                    &info::MACHINE.transitions()[2],
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
        transition_info: &TransitionInfo,
        exit_args: FrameEventArgs,
        enter_args: FrameEventArgs,
        new_state: DemoState,
        new_state_context: Rc<StateContext>,
    ) {
        // exit event for old state
        let mut exit_event = FrameEvent::new(FrameMessage::Exit, &exit_args);
        self.handle_event(&mut exit_event);

        // update state
        let old_state_context = Rc::clone(&self.state_context);
        let old_state_instance = old_state_context.as_ref().as_state_instance();
        self.state = new_state;
        self.state_context = Rc::clone(&new_state_context);
        let new_state_instance = new_state_context.as_state_instance();

        // call transition callbacks
        self.callback_manager.transition(
            transition_info,
            old_state_instance,
            new_state_instance,
            &exit_args,
            &enter_args,
        );

        // enter event for new state
        let mut enter_event = FrameEvent::new(FrameMessage::Enter, &enter_args);
        self.handle_event(&mut enter_event);
    }

    fn change_state(
        &mut self,
        transition_info: &TransitionInfo,
        new_state: DemoState,
        new_state_context: Rc<StateContext>,
    ) {
        // update state
        let old_state_context = Rc::clone(&self.state_context);
        let old_state_instance = old_state_context.as_ref().as_state_instance();
        self.state = new_state;
        self.state_context = Rc::clone(&new_state_context);
        let new_state_instance = new_state_context.as_state_instance();

        // call change-state callbacks
        self.callback_manager.transition(
            transition_info,
            old_state_instance,
            new_state_instance,
            EMPTY,
            EMPTY,
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
fn lookup_i32(env: &(impl Environment + ?Sized), name: &str) -> i32 {
    match env.lookup(name) {
        None => -1,
        Some(any) => *any.downcast_ref().unwrap_or(&-1),
    }
}

#[test]
fn machine_info() {
    let sm = Demo::new();
    assert_eq!("Demo", sm.info().name());
    assert_eq!(2, sm.info().variables().len());
    assert_eq!(3, sm.info().states().len());
    assert_eq!(2, sm.info().interface().len());
    assert_eq!(0, sm.info().actions().len());
    assert_eq!(8, sm.info().events().len());
    assert_eq!(3, sm.info().transitions().len());
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
    assert_eq!(1, init.unwrap().handlers().len());
    assert_eq!(4, foo.unwrap().handlers().len());
    assert_eq!(4, bar.unwrap().handlers().len());
    assert_eq!(0, init.unwrap().variables().len());
    assert_eq!(1, foo.unwrap().variables().len());
    assert_eq!(1, bar.unwrap().variables().len());
    assert_eq!(0, init.unwrap().parameters().len());
    assert_eq!(0, foo.unwrap().parameters().len());
    assert_eq!(1, bar.unwrap().parameters().len());
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
    assert_eq!("Init", incoming[0].source.name());
    assert_eq!("Foo", incoming[0].target.name());
    assert!(incoming[0].is_transition());
    assert!(!incoming[0].is_change_state());

    assert_eq!("next", incoming[1].event.name);
    assert_eq!("Bar", incoming[1].source.name());
    assert_eq!("Foo", incoming[1].target.name());
    assert!(!incoming[1].is_transition());
    assert!(incoming[1].is_change_state());

    assert_eq!("next", outgoing[0].event.name);
    assert_eq!("Foo", outgoing[0].source.name());
    assert_eq!("Bar", outgoing[0].target.name());
    assert!(outgoing[0].is_transition());
    assert!(!outgoing[0].is_change_state());
}

#[test]
fn current_state() {
    let mut sm = Demo::new();
    assert_eq!("Foo", sm.state().info().name());
    sm.inc(3);
    assert_eq!("Foo", sm.state().info().name());
    sm.next();
    assert_eq!("Bar", sm.state().info().name());
    sm.inc(4);
    assert_eq!("Bar", sm.state().info().name());
    sm.next();
    assert_eq!("Foo", sm.state().info().name());
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
    sm.callback_manager().add_transition_callback(|e| {
        tape_mutex
            .lock()
            .unwrap()
            .push(format!("kind: {:?}", e.info.kind));
    });
    sm.callback_manager().add_transition_callback(|e| {
        tape_mutex
            .lock()
            .unwrap()
            .push(format!("old: {}", e.old_state.info().name()));
        tape_mutex
            .lock()
            .unwrap()
            .push(format!("new: {}", e.new_state.info().name()));
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
fn transition_static_info_agrees() {
    let agree = AtomicBool::new(false);
    let mut sm = Demo::new();
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

#[test]
fn enter_exit_arguments() {
    let tape: Vec<(i32, i32, i32, i32)> = Vec::new();
    let tape_mutex = Mutex::new(tape);
    let mut sm = Demo::new();
    sm.callback_manager().add_transition_callback(|i| {
        let enter = i.enter_arguments;
        let exit = i.exit_arguments;
        tape_mutex.lock().unwrap().push((
            lookup_i32(enter, "init"),
            lookup_i32(enter, "start"),
            lookup_i32(exit, "done"),
            lookup_i32(exit, "end"),
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
        vec![(-1, 3, 12, -1), (-1, -1, -1, -1), (-1, 3, 10, -1)]
    );
}
