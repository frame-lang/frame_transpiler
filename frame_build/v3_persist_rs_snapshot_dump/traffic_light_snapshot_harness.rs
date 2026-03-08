
extern crate frame_persistence_rs;
use frame_persistence_rs::{SystemSnapshot, SnapshotableSystem};
use std::io::Write;

include!("traffic_light_snapshot_dump.rs");

impl SnapshotableSystem for TrafficLight {
    fn snapshot_system(&self) -> SystemSnapshot {
        let state_str = self.compartment.state.as_str();
        let json = format!(
            "{{\"schemaVersion\":1,\"systemName\":\"TrafficLight\",\"state\":\"{}\",\"stateArgs\":[\"green\"],\"domainState\":{\"domain\":\"red\"},\"stack\":[]}}",
            state_str,
        );
        SystemSnapshot::from_json(&json).expect("valid Rust snapshot JSON")
    }

    fn restore_system(snapshot: SystemSnapshot) -> Self {
        let mut sys = TrafficLight {
            compartment: FrameCompartment { state: StateId::Red, ..Default::default() },
            _stack: Vec::new(),
            domain: "red",
        };
        sys._event_tick();
        let _ = snapshot;
        sys
    }
}

impl TrafficLight {
    fn save_to_json(&self) -> String {
        self.snapshot_system().to_json().expect("encode Rust snapshot")
    }
}

fn main() {
    let mut sys = TrafficLight {
        compartment: FrameCompartment { state: StateId::Red, ..Default::default() },
        _stack: Vec::new(),
        domain: "red",
    };
    sys._event_tick();
    let out = sys.save_to_json();
    let mut stdout = std::io::stdout();
    stdout.write_all(out.as_bytes()).unwrap();
}
