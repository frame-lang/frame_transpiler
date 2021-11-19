//! This file tests features of the runtime system's event monitor.

include!(concat!(env!("OUT_DIR"), "/", "event_monitor.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use frame_runtime::*;
    use std::sync::Mutex;

    /// Test that event sent callbacks are triggered.
    #[test]
    fn event_sent() {
        let events = Mutex::new(Vec::new());
        let mut sm = EventMonitorSm::new();
        sm.event_monitor_mut().add_event_sent_callback(|e| {
            events.lock().unwrap().push(e.clone());
        });

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
        let events = Mutex::new(Vec::new());
        let mut sm = EventMonitorSm::new();
        sm.event_monitor_mut().add_event_handled_callback(|e| {
            events.lock().unwrap().push(e.clone());
        });

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
        let events = Mutex::new(Vec::new());
        let mut sm = EventMonitorSm::new();
        sm.event_monitor_mut().add_event_sent_callback(|e| {
            events.lock().unwrap().push(e.clone());
        });

        sm.transit(2);
        assert_eq!(EventMonitorSmState::A, sm.state);
        assert_eq!(10, events.lock().unwrap().len());
        let actual: Vec<&str> = events
            .lock()
            .unwrap()
            .iter()
            .map(|e| e.info().name)
            .collect();
        let expected = vec![
            "transit", "A:<", "B:>", "transit", "B:<", "C:>", "transit", "C:<", "D:>", "change",
        ];
        assert_eq!(expected, actual);

        events.lock().unwrap().clear();
        sm.change();
        sm.mult(4, 6);
        sm.transit(7);
        sm.change();
        sm.reset();

        let actual: Vec<&str> = events
            .lock()
            .unwrap()
            .iter()
            .map(|e| e.info().name)
            .collect();
        let expected = vec![
            "change", "mult", "transit", "B:<", "C:>", "transit", "C:<", "D:>",
            "change", // chain
            "change", "reset",
        ];
        assert_eq!(expected, actual);
    }

    /// Test that event handled callbacks are triggered in the expected order.
    #[test]
    fn event_handled_order() {
        let events = Mutex::new(Vec::new());
        let mut sm = EventMonitorSm::new();
        sm.event_monitor_mut().add_event_handled_callback(|e| {
            events.lock().unwrap().push(e.clone());
        });

        sm.transit(2);
        assert_eq!(EventMonitorSmState::A, sm.state);
        assert_eq!(10, events.lock().unwrap().len());
        let actual: Vec<&str> = events
            .lock()
            .unwrap()
            .iter()
            .map(|e| e.info().name)
            .collect();
        let expected = vec![
            "A:<", "B:<", "C:<", "change", "D:>", "transit", "C:>", "transit", "B:>", "transit",
        ];
        assert_eq!(expected, actual);

        events.lock().unwrap().clear();
        sm.change();
        sm.mult(4, 6);
        sm.transit(7);
        sm.change();
        sm.reset();

        let actual: Vec<&str> = events
            .lock()
            .unwrap()
            .iter()
            .map(|e| e.info().name)
            .collect();
        let expected = vec![
            "change", "mult", "B:<", "C:<", "change", "D:>", "transit", "C:>",
            "transit", // chain
            "change", "reset",
        ];
        assert_eq!(expected, actual);
    }
}
