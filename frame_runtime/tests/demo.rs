//! This file illustrates what generated code that realizes the runtime interface should look like
//! for a state machine that uses most of Frame's basic features. This example combined with the
//! example in `tests/simple.rs` illustrate the two major varieties of runtime support.
//!
//! The organization of the code in this file is different than what Framec would produce in order
//! to support testing both the `sync` and `unsync` variants of the library without too much
//! redundancy. See the submodules `demo::sync` and `demo::unsync` for the variant-specific parts.
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

use frame_runtime::env::Environment;
use std::any::Any;

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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum DemoState {
    Init,
    Foo,
    Bar,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
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

#[derive(Clone)]
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

#[derive(Clone)]
struct IncArgs {
    arg: i32,
}

impl Environment for IncArgs {
    fn lookup(&self, name: &str) -> Option<Box<dyn Any>> {
        match name {
            "arg" => Some(Box::new(self.arg)),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct FooEnterArgs {
    init: i32,
}

impl Environment for FooEnterArgs {
    fn lookup(&self, name: &str) -> Option<Box<dyn Any>> {
        match name {
            "init" => Some(Box::new(self.init)),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct FooExitArgs {
    done: i32,
}

impl Environment for FooExitArgs {
    fn lookup(&self, name: &str) -> Option<Box<dyn Any>> {
        match name {
            "done" => Some(Box::new(self.done)),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct BarEnterArgs {
    start: i32,
}

impl Environment for BarEnterArgs {
    fn lookup(&self, name: &str) -> Option<Box<dyn Any>> {
        match name {
            "start" => Some(Box::new(self.start)),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct BarExitArgs {
    end: i32,
}

impl Environment for BarExitArgs {
    fn lookup(&self, name: &str) -> Option<Box<dyn Any>> {
        match name {
            "end" => Some(Box::new(self.end)),
            _ => None,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
enum FrameEventArgs {
    None,
    Inc(IncArgs),
    FooEnter(FooEnterArgs),
    FooExit(FooExitArgs),
    BarEnter(BarEnterArgs),
    BarExit(BarExitArgs),
}

impl Environment for FrameEventArgs {
    fn lookup(&self, name: &str) -> Option<Box<dyn Any>> {
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

#[derive(Clone)]
struct FooStateVars {
    x: i32,
}

impl Environment for FooStateVars {
    fn lookup(&self, name: &str) -> Option<Box<dyn Any>> {
        match name {
            "x" => Some(Box::new(self.x)),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct BarStateArgs {
    tilt: i32,
}

impl Environment for BarStateArgs {
    fn lookup(&self, name: &str) -> Option<Box<dyn Any>> {
        match name {
            "tilt" => Some(Box::new(self.tilt)),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct BarStateVars {
    y: i32,
}

impl Environment for BarStateVars {
    fn lookup(&self, name: &str) -> Option<Box<dyn Any>> {
        match name {
            "y" => Some(Box::new(self.y)),
            _ => None,
        }
    }
}

// Note: these submodules contain parts of the example specific to the `sync` and `unsync` variants
// of the runtime system. They're enclosed in this `demo` module so that we can put the files
// implementing the submodules in a subdirectory of `tests` so they won't be interpreted as
// separate integration tests. It's useful to have the variant-specific code in separate files to
// support inspecting the differences with `diff`.
pub mod demo {
    #[rustfmt::skip] // https://github.com/rust-lang/rustfmt/issues/5119
    pub mod sync;
    #[rustfmt::skip]
    pub mod unsync;
}

// Begin testing code

mod tests {
    use super::demo::*;
    use frame_runtime::machine::Machine;

    #[test]
    fn machine_info() {
        let sm = unsync::Demo::new();
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
        let sm = sync::Demo::new();
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
        let sm = sync::Demo::new();
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
        let sm = unsync::Demo::new();
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

    use indoc::indoc;

    const SMCAT_STATIC: &str = indoc! {r#"
        initial,
        Init,
        Foo,
        Bar;

        initial -> Init;
        Init -> Foo : "  Init:>  ";
        Foo -> Bar : "  next  ";
        Bar -> Foo [color="grey"] : "  next  ";
        "#};

    const SMCAT_LIVE_1: &str = indoc! {r#"
        initial,
        Init,
        Foo [active color="red"],
        Bar;

        initial -> Init;
        Init -> Foo [color="red" width=2] : "  Init:>  ";
        Foo -> Bar : "  next  ";
        Bar -> Foo [color="grey"] : "  next  ";
        "#};

    const SMCAT_LIVE_2: &str = indoc! {r#"
        initial,
        Init,
        Foo,
        Bar [active color="red"];

        initial -> Init;
        Init -> Foo : "  Init:>  ";
        Foo -> Bar [color="red" width=2] : "  next  ";
        Bar -> Foo [color="grey"] : "  next  ";
        "#};

    const SMCAT_LIVE_3: &str = indoc! {r#"
        initial,
        Init,
        Foo [active color="red"],
        Bar;

        initial -> Init;
        Init -> Foo : "  Init:>  ";
        Foo -> Bar : "  next  ";
        Bar -> Foo [color="pink" width=2] : "  next  ";
        "#};

    #[test]
    fn smcat_render_static() {
        use frame_runtime::smcat::*;
        let smcat = Renderer::new(SimpleStyle);
        assert_eq!(smcat.render_static(super::info::machine()), SMCAT_STATIC);
    }

    #[test]
    fn smcat_render_live() {
        use crate::demo::sync::*;
        use frame_runtime::smcat::*;
        let smcat = Renderer::new(SimpleStyle);

        let mut sm = Demo::new();
        assert_eq!(smcat.render_live(&sm), SMCAT_LIVE_1);
        sm.next();
        assert_eq!(smcat.render_live(&sm), SMCAT_LIVE_2);
        sm.next();
        assert_eq!(smcat.render_live(&sm), SMCAT_LIVE_3);
    }
}
