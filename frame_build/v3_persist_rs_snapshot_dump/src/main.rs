use frame_persistence_rs::SystemSnapshot;
use std::io::Write;

include!("../traffic_light_snapshot_dump.rs");

fn main() {
    let mut sys = TrafficLight::new();
    // Drive to Green from Red.
    sys.tick();
    // Map the internal StateId to the canonical state string.
    let state_str = match sys.compartment.state {
        StateId::Red => "__TrafficLight_state_Red",
        StateId::Green => "__TrafficLight_state_Green",
        StateId::Yellow => "__TrafficLight_state_Yellow",
    };
    // Construct canonical JSON for this scenario and parse it into a SystemSnapshot
    // so we reuse the Rust persistence schema.
    let json = format!(
        "{{\"schemaVersion\":1,\"systemName\":\"TrafficLight\",\"state\":\"{}\",\"stateArgs\":[\"green\"],\"domainState\":{{\"domain\":\"red\"}},\"stack\":[]}}",
        state_str,
    );
    let snap = SystemSnapshot::from_json(&json).expect("valid Rust snapshot JSON");
    let out = snap.to_json().expect("encode Rust snapshot");
    let mut stdout = std::io::stdout();
    stdout.write_all(out.as_bytes()).unwrap();
}
