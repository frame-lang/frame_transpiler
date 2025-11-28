//! Stage 15 – Rust persistence helpers for Frame systems.
//!
//! This crate provides a language-neutral `SystemSnapshot` model and helper
//! traits for Rust systems generated from Frame. It does not depend on any
//! particular runtime struct layout; instead, systems can implement
//! `SnapshotableSystem` to map their own fields into the snapshot shape.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Snapshot of a single compartment on the state stack.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrameCompartmentSnapshot {
    /// Logical state name at this stack frame.
    #[serde(rename = "state")]
    pub state: String,
    /// State arguments at this frame (mirrors `stateArgs` in JSON).
    #[serde(rename = "stateArgs")]
    pub state_args: Value,
}

/// Language-neutral snapshot of a Frame system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSnapshot {
    /// Snapshot schema version (`schemaVersion` in JSON).
    #[serde(rename = "schemaVersion")]
    pub schema_version: u32,
    /// System name (`systemName` in JSON).
    #[serde(rename = "systemName")]
    pub system_name: String,
    /// Current logical state (`state` in JSON).
    #[serde(rename = "state")]
    pub state: String,
    /// Current state arguments (`stateArgs` in JSON).
    #[serde(rename = "stateArgs")]
    pub state_args: Value,
    /// Domain / system-level state (`domainState` in JSON).
    #[serde(rename = "domainState")]
    pub domain_state: Value,
    /// Stack of prior compartments (`stack` in JSON).
    #[serde(rename = "stack")]
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

    /// Compare two snapshots for structural equality.
    ///
    /// Returns (equal, differences), where `differences` contains
    /// human-readable descriptions of any mismatched fields.
    pub fn compare(&self, other: &SystemSnapshot) -> (bool, Vec<String>) {
        let mut diffs = Vec::new();

        if self.schema_version != other.schema_version {
            diffs.push(format!(
                "schema_version: {} != {}",
                self.schema_version, other.schema_version
            ));
        }
        if self.system_name != other.system_name {
            diffs.push(format!(
                "system_name: {:?} != {:?}",
                self.system_name, other.system_name
            ));
        }
        if self.state != other.state {
            diffs.push(format!(
                "state: {:?} != {:?}",
                self.state, other.state
            ));
        }
        if self.state_args != other.state_args {
            diffs.push(format!(
                "state_args differ: {} != {}",
                self.state_args, other.state_args
            ));
        }
        if self.domain_state != other.domain_state {
            diffs.push(format!(
                "domain_state differ: {} != {}",
                self.domain_state, other.domain_state
            ));
        }
        if self.stack != other.stack {
            diffs.push(format!(
                "stack differ: {:?} != {:?}",
                self.stack, other.stack
            ));
        }

        (diffs.is_empty(), diffs)
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

    #[test]
    fn system_snapshot_compare_reports_differences() {
        let snap1 = SystemSnapshot {
            schema_version: 1,
            system_name: "S".to_string(),
            state: "A".to_string(),
            state_args: json!({"x": 1}),
            domain_state: json!({"d": true}),
            stack: Vec::new(),
        };
        let mut snap2 = snap1.clone();

        // Equal snapshots compare as equal with no diffs.
        let (eq, diffs) = snap1.compare(&snap2);
        assert!(eq);
        assert!(diffs.is_empty());

        // Change a field and ensure we see a difference.
        snap2.state = "B".to_string();
        let (eq2, diffs2) = snap1.compare(&snap2);
        assert!(!eq2);
        assert!(diffs2.iter().any(|d| d.contains("state:")));
    }

    #[test]
    fn system_snapshot_canonical_json_round_trip() {
        // Canonical JSON shape used across PRT languages (camelCase fields).
        let json = r#"
        {
            "schemaVersion": 1,
            "systemName": "TrafficLight",
            "state": "Red",
            "stateArgs": { "color": "red" },
            "domainState": { "timeout": 3.0, "retryCount": 1 },
            "stack": [
                { "state": "Green", "stateArgs": { "color": "green" } }
            ]
        }
        "#;

        let snap = SystemSnapshot::from_json(json).expect("parse canonical snapshot JSON");
        assert_eq!(snap.schema_version, 1);
        assert_eq!(snap.system_name, "TrafficLight");
        assert_eq!(snap.state, "Red");
        assert_eq!(snap.state_args["color"], "red");
        assert_eq!(snap.domain_state["timeout"], json!(3.0));
        assert_eq!(snap.domain_state["retryCount"], json!(1));
        assert_eq!(snap.stack.len(), 1);
        assert_eq!(snap.stack[0].state, "Green");
        assert_eq!(snap.stack[0].state_args["color"], json!("green"));

        // Round-trip through JSON should preserve structure.
        let json2 = snap.to_json_pretty().expect("encode snapshot to JSON");
        let snap2 = SystemSnapshot::from_json(&json2).expect("reparse round-tripped JSON");
        let (equal, diffs) = snap.compare(&snap2);
        assert!(equal, "expected snapshots to be equal, diffs: {:?}", diffs);
    }
}
