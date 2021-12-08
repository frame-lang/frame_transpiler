//! Tests the interaction of several features (state variables, state parameters, event parameters,
//! event variables, return values) that are implemented via state contexts, with the runtime
//! system enabled.
//!
//! This is the same state machine as `state_context.rs` but with `runtime_support=true`. The first
//! few tests are redundant with the ones in that module, but the code generation code path is very
//! different with `runtime_support` enabled, so these tests are doing work.
//!
//! Similarly, a few of the runtime system tests here test similar functionality as the tests in
//! `basic.rs`, but the code path when state contexts are present is very different than without,
//! so again, the seeming redundancy is warranted.

type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "state_context_runtime.rs"));

impl<'a> StateContextSm<'a> {
    pub fn log(&mut self, name: String, val: i32) {
        self.tape.push(format!("{}={}", name, val));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_runtime::unsync::*;

    #[test]
    fn initial_state() {
        let mut sm = StateContextSm::new();
        let r = sm.inc();
        assert_eq!(r, 4);
        sm.log_state();
        assert_eq!(sm.tape, vec!["w=3", "w=4", "w=4"]);
    }

    #[test]
    fn transition() {
        let mut sm = StateContextSm::new();
        sm.inc();
        sm.inc();
        sm.tape.clear();

        sm.start();
        assert_eq!(sm.tape, vec!["a=3", "b=5", "x=15"]);
        sm.tape.clear();

        sm.inc();
        let r = sm.inc();
        assert_eq!(r, 17);
        assert_eq!(sm.tape, vec!["x=16", "x=17"]);
        sm.tape.clear();

        sm.next(3);
        assert_eq!(sm.tape, vec!["c=10", "x=27", "a=30", "y=17", "z=47"]);
        sm.tape.clear();

        sm.inc();
        sm.inc();
        let r = sm.inc();
        assert_eq!(r, 50);
        assert_eq!(sm.tape, vec!["z=48", "z=49", "z=50"]);
    }

    #[test]
    fn change_state() {
        let mut sm = StateContextSm::new();
        sm.inc();
        sm.inc();
        sm.start();
        sm.tape.clear();

        sm.inc();
        assert_eq!(sm.tape, vec!["x=16"]);
        sm.tape.clear();

        sm.change(10);
        sm.log_state();
        assert_eq!(sm.tape, vec!["y=26", "z=0"]);
        sm.tape.clear();

        sm.inc();
        sm.change(100);
        sm.log_state();
        assert_eq!(sm.state, StateContextSmState::Init);
        assert_eq!(sm.tape, vec!["z=1", "tmp=127", "w=0"]);
    }

    /// Test that the machine name from the runtime interface is correct.
    #[test]
    fn machine_name() {
        let info = StateContextSm::machine_info();
        assert_eq!(info.name, "StateContextSm");
    }

    /// Test that the state names from the runtime interface are correct.
    #[test]
    fn state_names() {
        let info = StateContextSm::machine_info();
        let states = info.states;
        assert_eq!(states.len(), 3);
        assert!(states.iter().any(|s| s.name == "Init"));
        assert!(states.iter().any(|s| s.name == "Foo"));
        assert!(states.iter().any(|s| s.name == "Bar"));
    }

    /// Test that the name of the initial state from the runtime interface is correct.
    #[test]
    fn initial_state_name() {
        let info = StateContextSm::machine_info();
        let init = info.initial_state();
        assert!(init.is_some());
        assert_eq!(init.unwrap().name, "Init");
    }

    /// Test that the state variable declarations from the runtime interface are correct.
    #[test]
    #[allow(clippy::blacklisted_name)]
    fn state_variables() {
        let info = StateContextSm::machine_info();
        let init = info.get_state("Init").unwrap();
        let foo = info.get_state("Foo").unwrap();
        let bar = info.get_state("Bar").unwrap();
        assert_eq!(init.variables.len(), 1);
        assert_eq!(foo.variables.len(), 1);
        assert_eq!(bar.variables.len(), 1);

        let w = init.get_variable("w");
        let x = foo.get_variable("x");
        let z = bar.get_variable("z");
        assert!(w.is_some());
        assert!(x.is_some());
        assert!(z.is_some());
        assert_eq!(w.as_ref().unwrap().name, "w");
        assert_eq!(x.as_ref().unwrap().name, "x");
        assert_eq!(z.as_ref().unwrap().name, "z");
        assert_eq!(w.as_ref().unwrap().vtype, "i32");
        assert_eq!(x.as_ref().unwrap().vtype, "i32");
        assert_eq!(z.as_ref().unwrap().vtype, "i32");
    }

    /// Test that the state parameter declarations from the runtime interface are correct.
    #[test]
    #[allow(clippy::blacklisted_name)]
    fn state_parameters() {
        let info = StateContextSm::machine_info();
        let init = info.get_state("Init").unwrap();
        let foo = info.get_state("Foo").unwrap();
        let bar = info.get_state("Bar").unwrap();
        assert_eq!(init.parameters.len(), 0);
        assert_eq!(foo.parameters.len(), 0);
        assert_eq!(bar.parameters.len(), 1);

        let y = bar.get_parameter("y");
        assert!(y.is_some());
        assert_eq!(y.as_ref().unwrap().name, "y");
        assert_eq!(y.as_ref().unwrap().vtype, "i32");
    }

    /// Test that the handler names from the runtime interface are correct.
    #[test]
    #[allow(clippy::blacklisted_name)]
    fn state_handler_names() {
        let info = StateContextSm::machine_info();
        let init = info.get_state("Init").unwrap();
        let foo = info.get_state("Foo").unwrap();
        let bar = info.get_state("Bar").unwrap();
        assert_eq!(init.handlers.len(), 4);
        assert_eq!(foo.handlers.len(), 6);
        assert_eq!(bar.handlers.len(), 4);

        assert!(init.handlers.iter().any(|m| m.name == "Init:>"));
        assert!(init.handlers.iter().any(|m| m.name == "Inc"));
        assert!(init.handlers.iter().any(|m| m.name == "LogState"));
        assert!(init.handlers.iter().any(|m| m.name == "Start"));

        assert!(foo.handlers.iter().any(|m| m.name == "Foo:>"));
        assert!(foo.handlers.iter().any(|m| m.name == "Foo:<"));
        assert!(foo.handlers.iter().any(|m| m.name == "LogState"));
        assert!(foo.handlers.iter().any(|m| m.name == "Inc"));
        assert!(foo.handlers.iter().any(|m| m.name == "Next"));
        assert!(foo.handlers.iter().any(|m| m.name == "Change"));

        assert!(bar.handlers.iter().any(|m| m.name == "Bar:>"));
        assert!(bar.handlers.iter().any(|m| m.name == "LogState"));
        assert!(bar.handlers.iter().any(|m| m.name == "Inc"));
        assert!(bar.handlers.iter().any(|m| m.name == "Change"));
    }

    /// Test that the handler signatures from the runtime interface are correct.
    #[test]
    #[allow(clippy::blacklisted_name)]
    fn state_handler_signatures() {
        let info = StateContextSm::machine_info();
        let init = info.get_state("Init").unwrap();
        let foo = info.get_state("Foo").unwrap();
        let bar = info.get_state("Bar").unwrap();

        let init_inc = init.get_handler("Inc").unwrap();
        let foo_enter = foo.get_handler("Foo:>").unwrap();
        let foo_exit = foo.get_handler("Foo:<").unwrap();
        let foo_inc = foo.get_handler("Inc").unwrap();
        let foo_change = foo.get_handler("Change").unwrap();
        let bar_inc = bar.get_handler("Inc").unwrap();
        let bar_change = bar.get_handler("Change").unwrap();

        assert_eq!(init_inc.name, "Inc");
        assert_eq!(init_inc.parameters.len(), 0);
        assert_eq!(init_inc.return_type, Some("i32"));

        assert_eq!(init_inc, foo_inc);
        assert_eq!(init_inc, bar_inc);

        assert_eq!(foo_enter.name, "Foo:>");
        assert_eq!(foo_enter.parameters.len(), 2);
        assert_eq!(foo_enter.parameters[0].name, "a");
        assert_eq!(foo_enter.parameters[0].vtype, "i32");
        assert_eq!(foo_enter.parameters[1].name, "b");
        assert_eq!(foo_enter.parameters[1].vtype, "i32");
        assert!(foo_enter.return_type.is_none());

        assert_eq!(foo_exit.name, "Foo:<");
        assert_eq!(foo_exit.parameters.len(), 1);
        assert_eq!(foo_exit.parameters[0].name, "c");
        assert_eq!(foo_exit.parameters[0].vtype, "i32");
        assert!(foo_exit.return_type.is_none());

        assert_eq!(foo_change.name, "Change");
        assert_eq!(foo_change.parameters.len(), 1);
        assert_eq!(foo_change.parameters[0].name, "arg");
        assert_eq!(foo_change.parameters[0].vtype, "i32");
        assert!(foo_change.return_type.is_none());

        assert_eq!(foo_change, bar_change);
    }

    /// Test that the interface names from the runtime interface are correct.
    #[test]
    fn interface_names() {
        let info = StateContextSm::machine_info();
        let methods = info.interface;
        assert_eq!(methods.len(), 5);
        assert!(methods.iter().any(|m| m.name == "Start"));
        assert!(methods.iter().any(|m| m.name == "LogState"));
        assert!(methods.iter().any(|m| m.name == "Inc"));
        assert!(methods.iter().any(|m| m.name == "Next"));
        assert!(methods.iter().any(|m| m.name == "Change"));
    }

    /// Test that the interface signatures from the runtime interface are correct.
    #[test]
    fn interface_signatures() {
        let info = StateContextSm::machine_info();
        let start = info.get_event("Start").unwrap();
        let log_state = info.get_event("LogState").unwrap();
        let inc = info.get_event("Inc").unwrap();
        let next = info.get_event("Next").unwrap();
        let change = info.get_event("Change").unwrap();

        assert_eq!(start.name, "Start");
        assert_eq!(start.parameters.len(), 0);
        assert!(start.return_type.is_none());

        assert_eq!(log_state.name, "LogState");
        assert_eq!(log_state.parameters.len(), 0);
        assert!(log_state.return_type.is_none());

        assert_eq!(inc.name, "Inc");
        assert_eq!(inc.parameters.len(), 0);
        assert_eq!(inc.return_type, Some("i32"));

        assert_eq!(next.name, "Next");
        assert_eq!(next.parameters.len(), 1);
        assert_eq!(next.parameters[0].name, "arg");
        assert_eq!(next.parameters[0].vtype, "i32");
        assert!(next.return_type.is_none());

        assert_eq!(change.name, "Change");
        assert_eq!(change.parameters.len(), 1);
        assert_eq!(change.parameters[0].name, "arg");
        assert_eq!(change.parameters[0].vtype, "i32");
        assert!(change.return_type.is_none());
    }

    /// Test that the event names from the runtime interface are correct.
    #[test]
    fn event_names() {
        let info = StateContextSm::machine_info();
        let methods = info.events;
        assert_eq!(methods.len(), 11);
        assert!(methods.iter().any(|m| m.name == "Start"));
        assert!(methods.iter().any(|m| m.name == "LogState"));
        assert!(methods.iter().any(|m| m.name == "Inc"));
        assert!(methods.iter().any(|m| m.name == "Next"));
        assert!(methods.iter().any(|m| m.name == "Change"));
        assert!(methods.iter().any(|m| m.name == "Init:>"));
        assert!(methods.iter().any(|m| m.name == "Init:<"));
        assert!(methods.iter().any(|m| m.name == "Foo:>"));
        assert!(methods.iter().any(|m| m.name == "Foo:<"));
        assert!(methods.iter().any(|m| m.name == "Bar:>"));
        assert!(methods.iter().any(|m| m.name == "Bar:<"));
    }

    /// Test that enter/exit events from the runtime interface have the correct signatures.
    #[test]
    fn enter_exit_event_signatures() {
        let info = StateContextSm::machine_info();
        let init_enter = info.get_event("Init:>").unwrap();
        let foo_enter = info.get_event("Foo:>").unwrap();
        let foo_exit = info.get_event("Foo:<").unwrap();
        let bar_enter = info.get_event("Bar:>").unwrap();

        assert_eq!(init_enter.name, "Init:>");
        assert_eq!(init_enter.parameters.len(), 0);
        assert!(init_enter.return_type.is_none());

        assert_eq!(foo_enter.name, "Foo:>");
        assert_eq!(foo_enter.parameters.len(), 2);
        assert_eq!(foo_enter.parameters[0].name, "a");
        assert_eq!(foo_enter.parameters[0].vtype, "i32");
        assert_eq!(foo_enter.parameters[1].name, "b");
        assert_eq!(foo_enter.parameters[1].vtype, "i32");
        assert!(foo_enter.return_type.is_none());

        assert_eq!(foo_exit.name, "Foo:<");
        assert_eq!(foo_exit.parameters.len(), 1);
        assert_eq!(foo_exit.parameters[0].name, "c");
        assert_eq!(foo_exit.parameters[0].vtype, "i32");
        assert!(foo_exit.return_type.is_none());

        assert_eq!(bar_enter.name, "Bar:>");
        assert_eq!(bar_enter.parameters.len(), 1);
        assert_eq!(bar_enter.parameters[0].name, "a");
        assert_eq!(bar_enter.parameters[0].vtype, "i32");
        assert!(bar_enter.return_type.is_none());
    }

    /// Test that the action names from the runtime interface are correct.
    #[test]
    fn action_names() {
        let info = StateContextSm::machine_info();
        let methods = info.actions;
        assert_eq!(methods.len(), 1);
        assert!(methods.iter().any(|m| m.name == "log"));
    }

    /// Test that the action signatures from the runtime interface are correct.
    #[test]
    fn action_signatures() {
        let info = StateContextSm::machine_info();
        let log = info.get_action("log").unwrap();
        assert_eq!(log.name, "log");
        assert_eq!(log.parameters.len(), 2);
        assert_eq!(log.parameters[0].name, "name");
        assert_eq!(log.parameters[0].vtype, "String");
        assert_eq!(log.parameters[1].name, "val");
        assert_eq!(log.parameters[1].vtype, "i32");
        assert!(log.return_type.is_none());
    }

    /// Test that the domain variable declarations from the runtime interface are correct.
    #[test]
    fn domain_variable() {
        let info = StateContextSm::machine_info();
        let names = info.variables;
        assert_eq!(names.len(), 1);

        let tape = info.get_variable("tape");
        assert!(tape.is_some());
        assert_eq!(tape.as_ref().unwrap().name, "tape");
        assert_eq!(tape.as_ref().unwrap().vtype, "Log");
    }

    /// Test that transitions obtained from the runtime interface are correct.
    #[test]
    #[allow(clippy::blacklisted_name)]
    fn state_transitions() {
        let info = StateContextSm::machine_info();
        assert_eq!(info.transitions.len(), 4);

        let init = info.get_state("Init").unwrap();
        let foo = info.get_state("Foo").unwrap();
        let bar = info.get_state("Bar").unwrap();

        assert_eq!(init.incoming_transitions().len(), 1);
        assert_eq!(init.outgoing_transitions().len(), 1);
        assert_eq!(foo.incoming_transitions().len(), 1);
        assert_eq!(foo.outgoing_transitions().len(), 2);
        assert_eq!(bar.incoming_transitions().len(), 2);
        assert_eq!(bar.outgoing_transitions().len(), 1);

        let init_in = &init.incoming_transitions()[0];
        let init_out = &init.outgoing_transitions()[0];
        let foo_in = &foo.incoming_transitions()[0];
        let foo_out_1 = &foo.outgoing_transitions()[0];
        let foo_out_2 = &foo.outgoing_transitions()[1];
        let bar_in_1 = &bar.incoming_transitions()[0];
        let bar_in_2 = &bar.incoming_transitions()[1];
        let bar_out = &bar.outgoing_transitions()[0];

        assert!(init_out.is_transition());
        assert_eq!(init_out.label, "transition 1");
        assert_eq!(init_out.source.name, "Init");
        assert_eq!(init_out.target.name, "Foo");
        assert_eq!(init_out.event.name, "Start");

        assert!(foo_out_1.is_transition());
        assert_eq!(foo_out_1.label, "transition 2");
        assert_eq!(foo_out_1.source.name, "Foo");
        assert_eq!(foo_out_1.target.name, "Bar");
        assert_eq!(foo_out_1.event.name, "Next");

        assert!(foo_out_2.is_change_state());
        assert_eq!(foo_out_2.label, "change-state 1");
        assert_eq!(foo_out_2.source.name, "Foo");
        assert_eq!(foo_out_2.target.name, "Bar");
        assert_eq!(foo_out_2.event.name, "Change");

        assert!(bar_out.is_change_state());
        assert_eq!(bar_out.label, "change-state 2");
        assert_eq!(bar_out.source.name, "Bar");
        assert_eq!(bar_out.target.name, "Init");
        assert_eq!(bar_out.event.name, "Change");

        assert_eq!(init_out.label, foo_in.label);
        assert_eq!(foo_out_1.label, bar_in_1.label);
        assert_eq!(foo_out_2.label, bar_in_2.label);
        assert_eq!(bar_out.label, init_in.label);
    }

    /// Test that we can access the current state via the runtime interface.
    #[test]
    fn runtime_current_state() {
        let mut sm = StateContextSm::new();
        assert_eq!(sm.state().info().name, "Init");
        sm.start();
        assert_eq!(sm.state().info().name, "Foo");
        sm.next(3);
        assert_eq!(sm.state().info().name, "Bar");
        sm.change(4);
        assert_eq!(sm.state().info().name, "Init");
        sm.start();
        sm.change(5);
        assert_eq!(sm.state().info().name, "Bar");
    }

    /// Test that we can access the values of domain variables via the runtime interface.
    #[test]
    fn runtime_domain_variables() {
        let mut sm = StateContextSm::new();
        sm.inc();
        sm.inc();
        {
            let tape: Log = *sm.variables().lookup("tape").unwrap().downcast().unwrap();
            assert_eq!(tape, vec!["w=3", "w=4", "w=5"]);
        }
        sm.tape.clear();
        sm.start();
        {
            let tape: Log = *sm.variables().lookup("tape").unwrap().downcast().unwrap();
            assert_eq!(*tape, vec!["a=3", "b=5", "x=15"]);
        }
    }

    /// Test that we can access the values of state variables via the runtime interface.
    #[test]
    fn runtime_state_variables() {
        let mut sm = StateContextSm::new();
        sm.inc();
        {
            let vars = sm.state().variables();
            let w: i32 = *vars.lookup("w").unwrap().downcast().unwrap();
            assert_eq!(w, 4);
            assert!(vars.lookup("a").is_none());
            assert!(vars.lookup("x").is_none());
            assert!(vars.lookup("y").is_none());
            assert!(vars.lookup("z").is_none());
            assert!(vars.lookup("log").is_none());
        }
        sm.inc();
        {
            let vars = sm.state().variables();
            let w: i32 = *vars.lookup("w").unwrap().downcast().unwrap();
            assert_eq!(w, 5);
        }
        sm.inc();
        sm.start();
        {
            let vars = sm.state().variables();
            let x: i32 = *vars.lookup("x").unwrap().downcast().unwrap();
            assert_eq!(x, 18);
            assert!(vars.lookup("a").is_none());
            assert!(vars.lookup("w").is_none());
            assert!(vars.lookup("y").is_none());
            assert!(vars.lookup("z").is_none());
            assert!(vars.lookup("log").is_none());
        }
        sm.inc();
        sm.next(10);
        {
            let vars = sm.state().variables();
            let z: i32 = *vars.lookup("z").unwrap().downcast().unwrap();
            assert_eq!(z, 119);
            assert!(vars.lookup("a").is_none());
            assert!(vars.lookup("w").is_none());
            assert!(vars.lookup("x").is_none());
            assert!(vars.lookup("y").is_none());
            assert!(vars.lookup("log").is_none());
        }
    }

    /// Test that we can access the values of state arguments via the runtime interface.
    #[test]
    fn runtime_state_arguments() {
        let mut sm = StateContextSm::new();
        {
            let args = sm.state().arguments();
            assert!(args.lookup("a").is_none());
            assert!(args.lookup("w").is_none());
            assert!(args.lookup("x").is_none());
            assert!(args.lookup("y").is_none());
            assert!(args.lookup("z").is_none());
            assert!(args.lookup("log").is_none());
        }
        sm.inc();
        sm.start();
        sm.inc();
        sm.next(10);
        {
            let args = sm.state().arguments();
            let y: i32 = *args.lookup("y").unwrap().downcast().unwrap();
            assert_eq!(y, 13);
            assert!(args.lookup("a").is_none());
            assert!(args.lookup("w").is_none());
            assert!(args.lookup("x").is_none());
            assert!(args.lookup("z").is_none());
            assert!(args.lookup("log").is_none());
        }
    }
}
