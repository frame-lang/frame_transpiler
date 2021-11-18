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
}
