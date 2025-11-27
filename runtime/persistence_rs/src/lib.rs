//! Stage 15 – Rust persistence helpers for Frame systems.
//!
//! This crate provides a language-neutral `SystemSnapshot` model and helper
//! traits for Rust systems generated from Frame. It does not depend on any
//! particular runtime struct layout; instead, systems can implement
//! `SnapshotableSystem` to map their own fields into the snapshot shape.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Snapshot of a single compartment on the state stack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameCompartmentSnapshot {
    pub state: String,
    pub state_args: Value,
}

/// Language-neutral snapshot of a Frame system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSnapshot {
    pub schema_version: u32,
    pub system_name: String,
    pub state: String,
    pub state_args: Value,
    pub domain_state: Value,
    pub stack: Vec<FrameCompartmentSnapshot>,
}

impl SystemSnapshot {
    /// Encode the snapshot as compact JSON.
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    /// Encode the snapshot as pretty-printed JSON.
    pub fn to_json_pretty(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    /// Decode a snapshot from JSON.
    pub fn from_json(text: &str) -> serde_json::Result<Self> {
        serde_json::from_str(text)
    }
}

/// Trait for systems that can be snapshotted and restored.
///
/// Generated V3 Rust systems can implement this trait in their module to
/// integrate with the Stage 15 persistence model.
pub trait SnapshotableSystem: Sized {
    fn snapshot_system(&self) -> SystemSnapshot;
    fn restore_system(snapshot: SystemSnapshot) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Minimal stand-alone system used to sanity-check the snapshot model.
    #[derive(Debug, Clone)]
    struct TrafficLight {
        state: String,
        // For this test we keep state args and domain state simple; real
        // generated systems will mirror the V3 runtime layout.
        color: String,
        cycles: u32,
    }

    impl TrafficLight {
        fn new(start_color: &str) -> Self {
            Self {
                state: "Red".to_string(),
                color: start_color.to_string(),
                cycles: 0,
            }
        }

        fn tick(&mut self) {
            match self.state.as_str() {
                "Red" => {
                    self.state = "Green".to_string();
                    self.color = "green".to_string();
                }
                "Green" => {
                    self.state = "Yellow".to_string();
                    self.color = "yellow".to_string();
                }
                "Yellow" => {
                    self.state = "Red".to_string();
                    self.color = "red".to_string();
                    self.cycles += 1;
                }
                _ => {}
            }
        }
    }

    impl SnapshotableSystem for TrafficLight {
        fn snapshot_system(&self) -> SystemSnapshot {
            SystemSnapshot {
                schema_version: 1,
                system_name: "TrafficLight".to_string(),
                state: self.state.clone(),
                state_args: json!({ "color": self.color }),
                domain_state: json!({ "cycles": self.cycles }),
                stack: Vec::new(),
            }
        }

        fn restore_system(snapshot: SystemSnapshot) -> Self {
            let color = snapshot
                .state_args
                .get("color")
                .and_then(|v| v.as_str())
                .unwrap_or("red")
                .to_string();
            let cycles = snapshot
                .domain_state
                .get("cycles")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as u32;

            Self {
                state: snapshot.state,
                color,
                cycles,
            }
        }
    }

    #[test]
    fn traffic_light_snapshot_round_trip() {
        let mut tl = TrafficLight::new("red");
        // Advance to a non-trivial state.
        tl.tick(); // Green
        tl.tick(); // Yellow

        let snap = tl.snapshot_system();
        assert_eq!(snap.state, "Yellow");
        assert_eq!(snap.state_args["color"], "yellow");

        let json = snap.to_json().unwrap();
        let snap2 = SystemSnapshot::from_json(&json).unwrap();
        let mut restored = TrafficLight::restore_system(snap2);

        // Continue execution from the restored state.
        restored.tick(); // back to Red, cycles + 1
        assert_eq!(restored.state, "Red");
        assert_eq!(restored.color, "red");
        assert_eq!(restored.cycles, 1);
    }
}

