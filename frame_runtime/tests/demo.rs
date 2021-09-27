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

use frame_runtime::environment::{Environment, EMPTY};
use frame_runtime::state::State;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

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
            FrameEventReturn::Inc { return_value } => return_value.clone(),
            _ => panic!("Invalid return value"),
        }
    }
}

pub struct FrameEvent {
    message: FrameMessage,
    arguments: FrameEventArgs,
    ret: FrameEventReturn,
}

impl FrameEvent {
    fn new(message: FrameMessage, arguments: FrameEventArgs) -> FrameEvent {
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

enum FrameEventArgs {
    None,
    Inc(IncArgs),
    FooEnter(FooEnterArgs),
    FooExit(FooExitArgs),
    BarEnter(BarEnterArgs),
    BarExit(BarExitArgs),
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

impl State for InitStateContext {
    fn name(&self) -> &'static str {
        "Init"
    }
    fn state_arguments(&self) -> &dyn Environment {
        EMPTY
    }
    fn state_variables(&self) -> &dyn Environment {
        EMPTY
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

impl State for FooStateContext {
    fn name(&self) -> &'static str {
        "Foo"
    }
    fn state_arguments(&self) -> &dyn Environment {
        EMPTY
    }
    fn state_variables(&self) -> &dyn Environment {
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

impl State for BarStateContext {
    fn name(&self) -> &'static str {
        "Bar"
    }
    fn state_arguments(&self) -> &dyn Environment {
        &self.state_args
    }
    fn state_variables(&self) -> &dyn Environment {
        &self.state_vars
    }
}

enum StateContext {
    Init(RefCell<InitStateContext>),
    Foo(RefCell<FooStateContext>),
    Bar(RefCell<BarStateContext>),
}

impl StateContext {
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

pub struct Demo {
    state: DemoState,
    state_context: Rc<StateContext>,
    x: i32,
    y: i32,
}

impl Environment for Demo {
    fn lookup(&self, name: &str) -> Option<&dyn Any> {
        match name {
            "x" => Some(&self.x),
            "y" => Some(&self.y),
            _ => None,
        }
    }
}

impl Demo {
    pub fn new() -> Demo {
        let context = InitStateContext {};
        let next_state_context = Rc::new(StateContext::Init(RefCell::new(context)));
        let mut machine = Demo {
            state: DemoState::Init,
            x: 0,
            y: 0,
            state_context: next_state_context,
        };
        machine.initialize();
        machine
    }

    pub fn initialize(&mut self) {
        let mut e = FrameEvent::new(FrameMessage::Enter, FrameEventArgs::None);
        self.handle_event(&mut e);
    }

    pub fn inc(&mut self, arg: i32) -> i32 {
        let frame_args = FrameEventArgs::Inc(IncArgs { arg });
        let mut frame_event = FrameEvent::new(FrameMessage::Inc, frame_args);
        self.handle_event(&mut frame_event);
        match frame_event.ret {
            FrameEventReturn::Inc { return_value } => return_value.clone(),
            _ => panic!("Bad return value for inc"),
        }
    }

    pub fn next(&mut self) {
        let frame_args = FrameEventArgs::None;
        let mut frame_event = FrameEvent::new(FrameMessage::Next, frame_args);
        self.handle_event(&mut frame_event);
    }

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
                self.transition(exit_args, enter_args, DemoState::Foo, next_state_context);
                return;
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
                return;
            }
            FrameMessage::Exit => {
                return;
            }
            FrameMessage::Inc => {
                this_state_context.state_vars.x =
                    this_state_context.state_vars.x + frame_event.arguments.inc_args().arg;
                frame_event.ret = FrameEventReturn::Inc {
                    return_value: this_state_context.state_vars.x,
                };
                return;
            }
            FrameMessage::Next => {
                self.x = self.x + this_state_context.state_vars.x;
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
                self.transition(exit_args, enter_args, DemoState::Bar, next_state_context);
                return;
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
                return;
            }
            FrameMessage::Exit => {
                return;
            }
            FrameMessage::Inc => {
                this_state_context.state_vars.y =
                    this_state_context.state_vars.y + frame_event.arguments.inc_args().arg;
                frame_event.ret = FrameEventReturn::Inc {
                    return_value: this_state_context.state_vars.y,
                };
                return;
            }
            FrameMessage::Next => {
                self.y = self.y + this_state_context.state_vars.y;
                // Start change state
                let context = FooStateContext {
                    state_vars: FooStateVars { x: 0 },
                };
                let next_state_context = Rc::new(StateContext::Foo(RefCell::new(context)));
                drop(this_state_context);
                self.change_state(DemoState::Foo, next_state_context);
                return;
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
        exit_args: FrameEventArgs,
        enter_args: FrameEventArgs,
        new_state: DemoState,
        new_state_context: Rc<StateContext>,
    ) {
        let mut exit_event = FrameEvent::new(FrameMessage::Exit, exit_args);
        self.handle_event(&mut exit_event);
        self.state = new_state;
        self.state_context = Rc::clone(&new_state_context);
        let mut enter_event = FrameEvent::new(FrameMessage::Enter, enter_args);
        self.handle_event(&mut enter_event);
    }
    
    fn change_state(&mut self, new_state: DemoState, new_state_context: Rc<StateContext>) {
        self.state = new_state;
        self.state_context = Rc::clone(&new_state_context);
    }
}
