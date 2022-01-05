//! This file tests features of the runtime system's event monitor.

include!(concat!(env!("OUT_DIR"), "/", "event_monitor.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use frame_runtime::*;
    use std::sync::{Arc, Mutex};

    /// Test that event sent callbacks are triggered.
    #[test]
    fn event_sent() {
        let mut sm = EventMonitorSm::new();
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_cb = events.clone();
        sm.event_monitor_mut()
            .add_event_sent_callback(Callback::new(
                "test",
                move |e: &<EventMonitorSm as Machine>::EventPtr| {
                    events_cb.lock().unwrap().push(e.clone());
                },
            ));

        sm.mult(3, 5);
        sm.change();
        assert_eq!(2, events.lock().unwrap().len());
        let e1 = events.lock().unwrap()[0].clone();
        let e2 = events.lock().unwrap()[1].clone();
        assert_eq!("mult", e1.info().name);
        assert_eq!("change", e2.info().name);

        sm.reset();
        assert_eq!(3, events.lock().unwrap().len());
        let e3 = events.lock().unwrap()[2].clone();
        assert_eq!("reset", e3.info().name);
    }

    /// Test that event handled callbacks are triggered.
    #[test]
    fn event_handled() {
        let mut sm = EventMonitorSm::new();
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_cb = events.clone();
        sm.event_monitor_mut()
            .add_event_handled_callback(Callback::new(
                "test",
                move |e: &<EventMonitorSm as Machine>::EventPtr| {
                    events_cb.lock().unwrap().push(e.clone());
                },
            ));

        sm.mult(3, 5);
        sm.change();
        assert_eq!(2, events.lock().unwrap().len());
        let e1 = events.lock().unwrap()[0].clone();
        let e2 = events.lock().unwrap()[1].clone();
        assert_eq!("mult", e1.info().name);
        assert_eq!("change", e2.info().name);

        sm.reset();
        assert_eq!(3, events.lock().unwrap().len());
        let e3 = events.lock().unwrap()[2].clone();
        assert_eq!("reset", e3.info().name);
    }

    /// Test that event sent callbacks are triggered in the expected order.
    #[test]
    fn event_sent_order() {
        let mut sm = EventMonitorSm::new();
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_cb = events.clone();
        sm.event_monitor_mut()
            .add_event_sent_callback(Callback::new(
                "test",
                move |e: &<EventMonitorSm as Machine>::EventPtr| {
                    events_cb.lock().unwrap().push(e.info().name);
                },
            ));

        sm.transit(2);
        assert_eq!(EventMonitorSmState::A, sm.state);
        assert_eq!(10, events.lock().unwrap().len());
        let expected = vec![
            "transit", "A:<", "B:>", "transit", "B:<", "C:>", "transit", "C:<", "D:>", "change",
        ];
        assert_eq!(expected, *events.lock().unwrap());

        events.lock().unwrap().clear();
        sm.change();
        sm.mult(4, 6);
        sm.transit(7);
        sm.change();
        sm.reset();

        let expected = vec![
            "change", "mult", // appetizer
            "transit", "B:<", "C:>", "transit", "C:<", "D:>", "change", // main course
            "change", "reset", // dessert
        ];
        assert_eq!(expected, *events.lock().unwrap());
    }

    /// Test that event handled callbacks are triggered in the expected order.
    #[test]
    fn event_handled_order() {
        let mut sm = EventMonitorSm::new();
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_cb = events.clone();
        sm.event_monitor_mut()
            .add_event_handled_callback(Callback::new(
                "test",
                move |e: &<EventMonitorSm as Machine>::EventPtr| {
                    events_cb.lock().unwrap().push(e.info().name);
                },
            ));

        sm.transit(2);
        assert_eq!(EventMonitorSmState::A, sm.state);
        assert_eq!(10, events.lock().unwrap().len());
        let expected = vec![
            "A:<", "B:<", "C:<", "change", "D:>", "transit", "C:>", "transit", "B:>", "transit",
        ];
        assert_eq!(expected, *events.lock().unwrap());

        events.lock().unwrap().clear();
        sm.change();
        sm.mult(4, 6);
        sm.transit(7);
        sm.change();
        sm.reset();

        let expected = vec![
            "change", "mult", // appetizer
            "B:<", "C:<", "change", "D:>", "transit", "C:>", "transit", // main course
            "change", "reset", // dessert
        ];
        assert_eq!(expected, *events.lock().unwrap());
    }

    /// Test that transition callbacks are triggered in the expected order.
    #[test]
    fn transition_order() {
        let mut sm = EventMonitorSm::new();
        let transits = Arc::new(Mutex::new(Vec::new()));
        let transits_cb = transits.clone();
        sm.event_monitor_mut()
            .add_transition_callback(Callback::new(
                "test",
                move |t: &Transition<EventMonitorSm>| {
                    transits_cb.lock().unwrap().push(t.to_string());
                },
            ));

        sm.transit(2);
        assert_eq!(4, transits.lock().unwrap().len());
        let expected = vec!["A->B", "B->C", "C->D", "D->>A"];
        assert_eq!(expected, *transits.lock().unwrap());

        transits.lock().unwrap().clear();
        sm.change();
        sm.mult(4, 6);
        sm.transit(7);
        sm.change();
        sm.mult(7, 9);
        sm.change();
        sm.reset();
        let expected = vec!["A->>B", "B->C", "C->D", "D->>A", "A->>B", "B->>C", "C->>A"];
        assert_eq!(expected, *transits.lock().unwrap());
    }

    /// Test that event and transition callbacks are triggered in the expected relative orders.
    #[test]
    fn event_transition_order() {
        let mut sm = EventMonitorSm::new();
        let sent = Arc::new(Mutex::new(Vec::new()));
        let handled = Arc::new(Mutex::new(Vec::new()));
        let sent_cb1 = sent.clone();
        let sent_cb2 = sent.clone();
        let handled_cb1 = handled.clone();
        let handled_cb2 = handled.clone();
        sm.event_monitor_mut()
            .add_event_sent_callback(Callback::new(
                "sent",
                move |e: &<EventMonitorSm as Machine>::EventPtr| {
                    sent_cb1.lock().unwrap().push(e.info().name.to_string());
                },
            ));
        sm.event_monitor_mut()
            .add_event_handled_callback(Callback::new(
                "handled",
                move |e: &<EventMonitorSm as Machine>::EventPtr| {
                    handled_cb1.lock().unwrap().push(e.info().name.to_string());
                },
            ));
        sm.event_monitor_mut()
            .add_transition_callback(Callback::new(
                "transition",
                move |t: &Transition<EventMonitorSm>| {
                    sent_cb2.lock().unwrap().push(t.to_string());
                    handled_cb2.lock().unwrap().push(t.to_string());
                },
            ));

        sm.transit(2);
        assert_eq!(14, sent.lock().unwrap().len());
        assert_eq!(14, handled.lock().unwrap().len());

        let sent_expected = vec![
            "transit", "A:<", "A->B", "B:>", // A->B
            "transit", "B:<", "B->C", "C:>", // B->C
            "transit", "C:<", "C->D", "D:>", // C->D
            "change", "D->>A", // D->>A
        ];
        let handled_expected = vec![
            "A:<", "A->B", "B:<", "B->C", "C:<", "C->D", "D->>A", // going down
            "change", "D:>", "transit", "C:>", "transit", "B:>", "transit", // coming back
        ];
        assert_eq!(sent_expected, *sent.lock().unwrap());
        assert_eq!(handled_expected, *handled.lock().unwrap());

        sent.lock().unwrap().clear();
        handled.lock().unwrap().clear();
        sm.change();
        sm.mult(4, 6);
        sm.transit(7);
        sm.change();
        sm.mult(7, 9);
        sm.change();
        sm.reset();
        let sent_expected = vec![
            "change", "A->>B", "mult", // change, mult
            "transit", "B:<", "B->C", "C:>", // transit
            "transit", "C:<", "C->D", "D:>", // ...
            "change", "D->>A", // ...
            "change", "A->>B", "mult", // change, mult
            "change", "B->>C", // change
            "reset", "C->>A", // reset
        ];
        let handled_expected = vec![
            "A->>B", "change", "mult", // change, mult
            "B:<", "B->C", "C:<", "C->D", "D->>A", // transit
            "change", "D:>", "transit", "C:>", "transit", // ...
            "A->>B", "change", "mult", // change, mult
            "B->>C", "change", // change
            "C->>A", "reset", // reset
        ];
        assert_eq!(sent_expected, *sent.lock().unwrap());
        assert_eq!(handled_expected, *handled.lock().unwrap());
    }

    /// Test that event sent callbacks receive the proper argument environments.
    #[test]
    fn event_sent_arguments() {
        let mut sm = EventMonitorSm::new();
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_cb = events.clone();
        sm.event_monitor_mut()
            .add_event_sent_callback(Callback::new(
                "test",
                move |e: &<EventMonitorSm as Machine>::EventPtr| {
                    if e.info().name == "mult" {
                        assert!(!e.arguments().is_empty());
                    } else if e.info().name == "change" {
                        assert!(e.arguments().is_empty());
                    }
                    events_cb.lock().unwrap().push(e.clone());
                },
            ));
        sm.mult(3, 5);
        sm.change();
        sm.mult(4, 6);

        let events = events.lock().unwrap();
        assert_eq!(3, events.len());

        assert_eq!("mult", events[0].info().name);
        let args = events[0].arguments();
        let a_opt = args.lookup("a");
        let b_opt = args.lookup("b");
        let c_opt = args.lookup("c");
        assert!(a_opt.is_some());
        assert!(b_opt.is_some());
        assert!(c_opt.is_none());
        assert_eq!(3, *a_opt.unwrap().downcast_ref::<i32>().unwrap());
        assert_eq!(5, *b_opt.unwrap().downcast_ref::<i32>().unwrap());

        assert_eq!("change", events[1].info().name);
        assert!(events[1].arguments().is_empty());

        assert_eq!("mult", events[2].info().name);
        let args = events[2].arguments();
        let a_opt = args.lookup("a");
        let b_opt = args.lookup("b");
        let c_opt = args.lookup("c");
        assert!(a_opt.is_some());
        assert!(b_opt.is_some());
        assert!(c_opt.is_none());
        assert_eq!(4, *a_opt.unwrap().downcast_ref::<i32>().unwrap());
        assert_eq!(6, *b_opt.unwrap().downcast_ref::<i32>().unwrap());
    }

    /// Test that event sent callbacks do not receive a return value at the time they're called,
    /// but that the return value is later added to the event.
    #[test]
    fn event_sent_return_value() {
        let mut sm = EventMonitorSm::new();
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_cb = events.clone();
        sm.event_monitor_mut()
            .add_event_sent_callback(Callback::new(
                "test",
                move |e: &<EventMonitorSm as Machine>::EventPtr| {
                    assert!(e.return_value().is_none());
                    events_cb.lock().unwrap().push(e.clone());
                },
            ));
        sm.mult(3, 5);
        sm.change();
        sm.mult(4, 6);

        let events = events.lock().unwrap();
        assert_eq!(3, events.len());

        assert_eq!("mult", events[0].info().name);
        let ret_opt = events[0].return_value();
        assert!(ret_opt.is_some());
        assert_eq!(15, *ret_opt.unwrap().downcast_ref::<i32>().unwrap());

        assert_eq!("change", events[1].info().name);
        let ret_opt = events[1].return_value();
        assert!(ret_opt.is_some());
        assert_eq!(2, *ret_opt.unwrap().downcast_ref::<u32>().unwrap());

        assert_eq!("mult", events[2].info().name);
        let ret_opt = events[2].return_value();
        assert!(ret_opt.is_some());
        assert_eq!(24, *ret_opt.unwrap().downcast_ref::<i32>().unwrap());
    }

    /// Test that event handled callbacks receive a return value at the time they're called.
    #[test]
    fn event_handled_return_value() {
        let mut sm = EventMonitorSm::new();
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_cb = events.clone();
        sm.event_monitor_mut()
            .add_event_handled_callback(Callback::new(
                "test",
                move |e: &<EventMonitorSm as Machine>::EventPtr| {
                    if e.return_value().is_some() {
                        events_cb.lock().unwrap().push(e.clone());
                    }
                },
            ));
        sm.mult(3, 5);
        sm.change();
        sm.mult(4, 6);
        sm.transit(3);

        let events = events.lock().unwrap();
        assert_eq!(4, events.len());
        assert_eq!(
            15,
            *events[0]
                .return_value()
                .unwrap()
                .downcast_ref::<i32>()
                .unwrap()
        );
        assert_eq!(
            2,
            *events[1]
                .return_value()
                .unwrap()
                .downcast_ref::<u32>()
                .unwrap()
        );
        assert_eq!(
            24,
            *events[2]
                .return_value()
                .unwrap()
                .downcast_ref::<i32>()
                .unwrap()
        );
        assert_eq!(
            32,
            *events[3]
                .return_value()
                .unwrap()
                .downcast_ref::<u32>()
                .unwrap()
        );
    }

    /// Test that the event history contains the initial enter event.
    #[test]
    fn event_history_initial_enter() {
        let sm = EventMonitorSm::new();
        let history = sm.event_monitor().event_history();
        assert_eq!(1, history.len());
        assert_eq!("A:>", history.newest().unwrap().info().name);
    }

    /// Test that the event history capacity works as expected.
    #[test]
    fn event_history_capacity() {
        let mut sm = EventMonitorSm::new();
        assert_eq!(Some(5), sm.event_monitor().event_history().capacity());
        sm.event_monitor_mut().clear_event_history();

        sm.change();
        sm.mult(3, 5);
        let history = sm.event_monitor().event_history();
        assert_eq!(2, history.len());
        assert_eq!("change", history.as_deque()[0].info().name);
        assert_eq!("mult", history.as_deque()[1].info().name);

        sm.transit(5);
        let history = sm.event_monitor().event_history();
        assert_eq!(5, history.len());
        let actual: Vec<&str> = history.iter().map(|e| e.info().name).collect();
        let expected = vec!["C:>", "transit", "C:<", "D:>", "change"];
        assert_eq!(expected, actual);

        sm.event_monitor_mut().set_event_history_capacity(Some(7));
        sm.mult(4, 6);
        sm.mult(5, 7);
        sm.change();
        let history = sm.event_monitor().event_history();
        assert_eq!(7, history.len());
        let actual: Vec<&str> = history.iter().map(|e| e.info().name).collect();
        let expected = vec!["transit", "C:<", "D:>", "change", "mult", "mult", "change"];
        assert_eq!(expected, actual);

        sm.event_monitor_mut().set_event_history_capacity(Some(3));
        let history = sm.event_monitor().event_history();
        assert_eq!(3, history.len());
        let actual: Vec<&str> = history.iter().map(|e| e.info().name).collect();
        let expected = vec!["mult", "mult", "change"];
        assert_eq!(expected, actual);

        sm.change();
        let history = sm.event_monitor().event_history();
        assert_eq!(3, history.len());
        let actual: Vec<&str> = history.iter().map(|e| e.info().name).collect();
        let expected = vec!["mult", "change", "change"];
        assert_eq!(expected, actual);

        sm.event_monitor_mut().set_event_history_capacity(None);
        sm.reset();
        sm.transit(3);
        let history = sm.event_monitor().event_history();
        assert_eq!(14, history.len());
    }

    /// Test that the transition history capacity works as expected.
    #[test]
    fn transition_history_capacity() {
        let mut sm = EventMonitorSm::new();
        assert_eq!(Some(3), sm.event_monitor().transition_history().capacity());
        assert!(sm.event_monitor().transition_history().is_empty());

        sm.change();
        sm.mult(3, 5);
        sm.reset();
        let history = sm.event_monitor().transition_history();
        assert_eq!(2, history.len());
        assert_eq!("A->>B", history.as_deque()[0].to_string());
        assert_eq!("B->>A", history.as_deque()[1].to_string());

        sm.transit(5);
        let history = sm.event_monitor().transition_history();
        assert_eq!(3, history.len());
        let actual: Vec<String> = history.iter().map(|t| t.to_string()).collect();
        let expected = vec!["B->C", "C->D", "D->>A"];
        assert_eq!(expected, actual);

        sm.event_monitor_mut()
            .set_transition_history_capacity(Some(6));
        sm.mult(5, 7);
        sm.transit(3);
        let history = sm.event_monitor().transition_history();
        assert_eq!(6, history.len());
        let actual: Vec<String> = history.iter().map(|t| t.to_string()).collect();
        let expected = vec!["C->D", "D->>A", "A->B", "B->C", "C->D", "D->>A"];
        assert_eq!(expected, actual);

        sm.event_monitor_mut()
            .set_transition_history_capacity(Some(3));
        let history = sm.event_monitor().transition_history();
        assert_eq!(3, history.len());
        let actual: Vec<String> = history.iter().map(|t| t.to_string()).collect();
        let expected = vec!["B->C", "C->D", "D->>A"];
        assert_eq!(expected, actual);

        sm.change();
        let history = sm.event_monitor().transition_history();
        assert_eq!(3, history.len());
        let actual: Vec<String> = history.iter().map(|t| t.to_string()).collect();
        let expected = vec!["C->D", "D->>A", "A->>B"];
        assert_eq!(expected, actual);

        sm.event_monitor_mut().set_transition_history_capacity(None);
        sm.reset();
        sm.transit(4);
        sm.transit(5);
        let history = sm.event_monitor().transition_history();
        assert_eq!(12, history.len());
    }

    /// Test that return values are set in events stored in the history.
    #[test]
    fn event_history_return_value() {
        let mut sm = EventMonitorSm::new();
        sm.event_monitor_mut().clear_event_history();

        sm.change();
        sm.mult(3, 5);
        sm.change();
        sm.reset();
        let history = sm.event_monitor().event_history();
        assert!(history.as_deque()[0].return_value().is_some());
        assert!(history.as_deque()[1].return_value().is_some());
        assert!(history.as_deque()[2].return_value().is_some());
        assert!(history.as_deque()[3].return_value().is_none());
        assert_eq!(
            2,
            *history.as_deque()[0]
                .return_value()
                .unwrap()
                .downcast_ref::<u32>()
                .unwrap()
        );
        assert_eq!(
            15,
            *history.as_deque()[1]
                .return_value()
                .unwrap()
                .downcast_ref::<i32>()
                .unwrap()
        );
        assert_eq!(
            12,
            *history.as_deque()[2]
                .return_value()
                .unwrap()
                .downcast_ref::<u32>()
                .unwrap()
        );
    }
}
