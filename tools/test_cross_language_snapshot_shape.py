#!/usr/bin/env python3
"""
Stage 17 – Cross-language snapshot shape smoke test.

This tool validates that the canonical SystemSnapshot JSON shape used in
Stage 15 can be consumed and re-emitted consistently by the Python and
TypeScript persistence helpers, and that Rust does the same via its own
unit tests (`frame_persistence_rs`).

It does NOT depend on running any generated V3 systems; instead it focuses
on the DTO layer only, using a single canonical TrafficLight snapshot
example.
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]


CANONICAL_SNAPSHOT = {
    "schemaVersion": 1,
    "systemName": "TrafficLight",
    "state": "Red",
    "stateArgs": {"color": "red"},
    "domainState": {"timeout": 3.0, "retryCount": 1},
    "stack": [
        {"state": "Green", "stateArgs": {"color": "green"}},
    ],
}


def check_python() -> None:
    """Verify Python persistence helpers round-trip the canonical snapshot."""
    sys.path.insert(0, str(REPO_ROOT))
    from frame_persistence_py import (
        SystemSnapshot,
        snapshot_from_json,
        snapshot_to_json,
        compare_snapshots,
    )

    text = json.dumps(CANONICAL_SNAPSHOT, sort_keys=True)
    snap = snapshot_from_json(text)
    assert isinstance(snap, SystemSnapshot)
    out = snapshot_to_json(snap)
    snap2 = snapshot_from_json(out)
    equal, diffs = compare_snapshots(snap, snap2)
    if not equal:
        raise SystemExit(f"Python snapshot round-trip mismatch: {diffs}")


def check_typescript() -> None:
    """Verify TypeScript helpers accept the canonical shape."""
    json_text = json.dumps(CANONICAL_SNAPSHOT)
    script = r"""
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

main(process.argv[1]);
"""
    env = dict(os.environ)
    # Make the local frame_runtime_ts visible as a module.
    env["NODE_PATH"] = str(REPO_ROOT)
    proc = subprocess.run(
        ["node", "-e", script, json_text],
        cwd=str(REPO_ROOT),
        text=True,
        capture_output=True,
        env=env,
    )
    if proc.returncode != 0:
        sys.stderr.write(proc.stdout)
        sys.stderr.write(proc.stderr)
        raise SystemExit("TypeScript snapshot round-trip failed")


def main() -> int:
    check_python()
    check_typescript()
    print("Cross-language snapshot shape (Python/TypeScript) OK.")
    print("Rust shape is validated by `cargo test -p frame_persistence_rs`.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
