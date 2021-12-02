use crate::*;

use frame_runtime::live::Machine;
use frame_runtime::unsync as runtime;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct FrameEvent {
    message: FrameMessage,
    arguments: Rc<RefCell<FrameEventArgs>>,
    ret: RefCell<FrameEventReturn>,
}

impl FrameEvent {
    fn new(message: FrameMessage, arguments: FrameEventArgs) -> FrameEvent {
        FrameEvent {
            message,
            arguments: Rc::new(RefCell::new(arguments)),
            ret: RefCell::new(FrameEventReturn::None),
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
        match self.ret.borrow().to_owned() {
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
        runtime::Empty::rc()
    }
    fn variables(&self) -> runtime::EnvironmentPtr {
        runtime::Empty::rc()
    }
}

struct FooStateContext {
    state_vars: Rc<RefCell<FooStateVars>>,
}

impl runtime::State<runtime::EnvironmentPtr> for FooStateContext {
    fn info(&self) -> &'static runtime::StateInfo {
        info::machine().states[1]
    }
    fn arguments(&self) -> runtime::EnvironmentPtr {
        runtime::Empty::rc()
    }
    fn variables(&self) -> runtime::EnvironmentPtr {
        self.state_vars.clone()
    }
}


struct BarStateContext {
    state_args: Rc<RefCell<BarStateArgs>>,
    state_vars: Rc<RefCell<BarStateVars>>,
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
    state_context: Rc<StateContext>,
    event_monitor: runtime::EventMonitor<'a>,
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
        let next_state_context = Rc::new(StateContext::Init(context));
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
        let frame_event = Rc::new(FrameEvent::new(
            FrameMessage::Enter(self.state),
            FrameEventArgs::None,
        ));
        self.handle_event(frame_event);
    }

    pub fn inc(&mut self, arg: i32) -> i32 {
        let frame_args = FrameEventArgs::Inc(IncArgs { arg });
        let frame_event = Rc::new(FrameEvent::new(FrameMessage::Inc, frame_args));
        self.handle_event(frame_event.clone());
        let return_value = match *frame_event.ret.borrow() {
            FrameEventReturn::Inc { return_value } => return_value,
            _ => panic!("Bad return value for inc"),
        };
        return_value
    }

    pub fn next(&mut self) {
        let frame_args = FrameEventArgs::None;
        let frame_event = Rc::new(FrameEvent::new(FrameMessage::Next, frame_args));
        self.handle_event(frame_event);
    }

    #[allow(clippy::single_match)]
    #[allow(unused_variables)]
    fn init_handler(&mut self, frame_event: Rc<FrameEvent>) {
        let this_state_context = self.state_context.init_context();
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

    fn foo_handler(&mut self, frame_event: Rc<FrameEvent>) {
        let this_state_context = self.state_context.foo_context();
        match frame_event.message {
            FrameMessage::Enter(_) => {
                this_state_context.state_vars.borrow_mut().x =
                    frame_event.arguments.borrow().foo_enter_args().init;
            }
            FrameMessage::Exit(_) => {}
            FrameMessage::Inc => {
                this_state_context.state_vars.borrow_mut().x +=
                    frame_event.arguments.borrow().inc_args().arg;
                frame_event.ret.replace(FrameEventReturn::Inc {
                    return_value: this_state_context.state_vars.borrow().x,
                });
            }
            FrameMessage::Next => {
                self.x += this_state_context.state_vars.borrow().x;
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

    fn bar_handler(&mut self, frame_event: Rc<FrameEvent>) {
        let this_state_context = self.state_context.bar_context();
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
                frame_event.ret.replace(FrameEventReturn::Inc {
                    return_value: this_state_context.state_vars.borrow().y,
                });
            }
            FrameMessage::Next => {
                self.y += this_state_context.state_vars.borrow().y;
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

    fn handle_event(&mut self, frame_event: Rc<FrameEvent>) {
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
        new_state_context: Rc<StateContext>,
    ) {
        // create and send exit event for old state
        let exit_event = Rc::new(FrameEvent::new(FrameMessage::Exit(self.state), exit_args));
        self.handle_event(exit_event.clone());

        // update state
        let old_state_context = self.state_context.clone();
        self.state = new_state;
        self.state_context = new_state_context.clone();

        // create enter event for new state
        let enter_event = Rc::new(FrameEvent::new(FrameMessage::Enter(self.state), enter_args));

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
        new_state_context: Rc<StateContext>,
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
