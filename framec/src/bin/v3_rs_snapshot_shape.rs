use std::env;
use std::path::PathBuf;
use std::process::Command;

use frame_persistence_rs::SystemSnapshot;
use serde_json::json;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .to_path_buf()
}

fn canonical_snapshot_json() -> String {
    let snap = json!({
        "schemaVersion": 1,
        "systemName": "TrafficLight",
        "state": "Red",
        "stateArgs": { "color": "red" },
        "domainState": { "timeout": 3.0, "retryCount": 1 },
        "stack": [
            { "state": "Green", "stateArgs": { "color": "green" } }
        ],
    });
    serde_json::to_string(&snap).expect("serialize canonical snapshot")
}

fn check_python(canonical: &str, root: &PathBuf) -> Result<(), String> {
    let script = r#"
import json, sys, os
from pathlib import Path

repo = Path(sys.argv[1]).resolve()
sys.path.insert(0, str(repo))
from frame_persistence_py import snapshot_from_json, snapshot_to_json, compare_snapshots

text = sys.argv[2]
snap = snapshot_from_json(text)
out = snapshot_to_json(snap)
snap2 = snapshot_from_json(out)
equal, diffs = compare_snapshots(snap, snap2)
if not equal:
    print("Python snapshot round-trip mismatch:", diffs)
    sys.exit(1)
"#;

    let status = Command::new("python3")
        .arg("-c")
        .arg(script)
        .arg(root)
        .arg(canonical)
        .current_dir(root)
        .status()
        .map_err(|e| format!("failed to spawn python3: {e}"))?;

    if !status.success() {
        return Err(format!(
            "Python snapshot shape check failed with status {}",
            status
        ));
    }
    Ok(())
}

fn check_typescript(canonical: &str, root: &PathBuf) -> Result<(), String> {
    let script = r#"
const { snapshotFromJson, snapshotToJson, compareSnapshots } = require("./frame_persistence_ts");

function main(jsonText) {
  const snap = snapshotFromJson(jsonText);
  const out = snapshotToJson(snap);
  const snap2 = snapshotFromJson(out);
  const result = compareSnapshots(snap, snap2);
  if (!result.equal) {
    console.error("TypeScript snapshot round-trip mismatch:", result.differences);
    process.exit(1);
  }
}

main(process.argv[2]);
"#;

    let mut cmd = Command::new("node");
    cmd.arg("-e")
        .arg(script)
        .arg(root.to_string_lossy().to_string())
        .arg(canonical)
        .current_dir(root);

    // Make project-local modules (frame_persistence_ts, frame_runtime_ts) visible.
    if let Ok(existing) = env::var("NODE_PATH") {
        let merged = format!(
            "{}{}{}",
            root.display(),
            std::path::MAIN_SEPARATOR,
            existing
        );
        cmd.env("NODE_PATH", merged);
    } else {
        cmd.env("NODE_PATH", root);
    }

    let status = cmd
        .status()
        .map_err(|e| format!("failed to spawn node: {e}"))?;
    if !status.success() {
        return Err(format!(
            "TypeScript snapshot shape check failed with status {}",
            status
        ));
    }
    Ok(())
}

fn check_rust(canonical: &str) -> Result<(), String> {
    let snap = SystemSnapshot::from_json(canonical)
        .map_err(|e| format!("Rust SystemSnapshot::from_json failed: {e}"))?;
    let json = snap
        .to_json()
        .map_err(|e| format!("Rust SystemSnapshot::to_json failed: {e}"))?;
    let snap2 = SystemSnapshot::from_json(&json)
        .map_err(|e| format!("Rust SystemSnapshot::from_json round-trip failed: {e}"))?;
    let (equal, diffs) = snap.compare(&snap2);
    if !equal {
        return Err(format!(
            "Rust snapshot round-trip mismatch: {:?}",
            diffs
        ));
    }
    Ok(())
}

fn main() {
    let root = repo_root();
    let canonical = canonical_snapshot_json();

    if let Err(e) = check_python(&canonical, &root) {
        eprintln!("{e}");
        std::process::exit(1);
    }
    if let Err(e) = check_typescript(&canonical, &root) {
        eprintln!("{e}");
        std::process::exit(1);
    }
    if let Err(e) = check_rust(&canonical) {
        eprintln!("{e}");
        std::process::exit(1);
    }

    println!("v3_rs_snapshot_shape: Python, TypeScript, and Rust snapshot shape OK.");
}

