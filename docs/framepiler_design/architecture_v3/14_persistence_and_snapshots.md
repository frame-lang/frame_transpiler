# Stage 15 — Persistence & Snapshots (PRT, Mandatory)

Purpose
- Provide a first-class, per-language persistence model for V3 systems in the
  PRT targets (Python, TypeScript, Rust).
- Define a stable, language-neutral snapshot shape so systems can be saved,
  restored, and potentially moved across processes or languages without
  depending on ad-hoc serialization of the full object graph.

Scope
- Targets: Python, TypeScript, Rust (PRT). Stage 15 is **mandatory** for
  production workflows on these languages.
- Non-PRT languages (C/C++/Java/C#) MAY adopt the same snapshot schema later,
  but are not in scope for the initial Stage 15 implementation.
- Persistence covers **system state** only, not arbitrary application data:
  - Current state identifier and state parameters.
  - Domain/state variables owned by the system.
  - The state stack (for `$$[+]` / `$$[-]`).
  - Any additional runtime fields that are semantically relevant to the
    resumed behavior.

Out of Scope
- Storage backends (files, databases, message queues) — Stage 15 defines the
  encode/decode of system state, not how or where snapshots are stored.
- Live external resources (sockets, file handles, OS processes, debugger
  adapters) — these must be re-established by application code after a
  snapshot is restored.

Conceptual Model: SystemSnapshot
- Each PRT system has a well-defined snapshot representation:
  ```json
  {
    "schemaVersion": 1,
    "systemName": "TrafficLight",
    "state": "Red",
    "stateArgs": { "color": "red" },
    "domainState": {
      "timeout": 3.0,
      "retryCount": 1
    },
    "stack": [
      { "state": "Green", "stateArgs": { "color": "green" } }
    ]
  }
  ```
- Properties:
  - `schemaVersion`: integer, starts at 1 and is bumped only on breaking
    schema changes.
  - `systemName`: Frame system identifier.
  - `state`: logical state name (V3 treats this as the abstract state id,
    independent of target-specific mangling such as `"__Sys_state_Red"`).
  - `stateArgs`: map of state parameter names to values at the time of
    snapshot.
  - `domainState`: map of selected domain fields and other stable system
    variables. The exact set is under user control (see per-language APIs).
  - `stack`: array of prior compartments on the state stack, encoded in the
    same logical `(state, stateArgs)` form.

Design Goals
- **Language-neutral**: the snapshot uses only JSON-friendly primitives
  (strings, numbers, booleans, arrays, objects) so it can be serialized the
  same way across Python/TypeScript/Rust.
- **Minimal but sufficient**: captures just enough to resume behavior from the
  same logical point while avoiding transient fields (e.g., current event,
  enter/exit args, internal runtime book-keeping).
- **Explicit versioning**: `schemaVersion` enables safe evolution; Stage 15
  MUST treat unknown versions conservatively (e.g., refuse to restore or
  require an adapter).

Per-Language Libraries (mandatory for PRT)
- Each PRT target exposes a small persistence helper library:
  - Python: `frame_persistence_py`
  - TypeScript: `frame_persistence_ts`
  - Rust: `frame_persistence_rs`

- Each library provides:
  - `snapshot = snapshot_system(system: &System)`  
    (function name and signature idiomatic per language).
  - `system = restore_system(snapshot)`  
    creating a new runtime instance from the snapshot.

- These helpers:
  - Know how to map between the runtime representation
    (`FrameCompartment`, domain fields, stack, and `system.return` state
    where appropriate) and the `SystemSnapshot` structure.
  - Avoid serializing live resources or non-deterministic references.
  - Reconstruct a valid system:
    - Fresh `FrameCompartment` chain with correct `state` and `stateArgs`.
    - Clean transient fields (no in-flight events or half-completed
      transitions).
    - Domain fields populated from `domainState`.
    - Stack rebuilt from the snapshot.

Python (Stage 15 requirements)
- Do NOT rely on `jsonpickle.encode(self)` for full-system persistence in V3.
- Instead:
  - Implement `frame_persistence_py.snapshot_system(system)` that:
    - Inspects `_compartment`, `_stack`, and selected domain fields.
    - Produces a `SystemSnapshot` structure.
  - Implement `frame_persistence_py.restore_system(snapshot, system_factory)` that:
    - Instantiates a new system via `system_factory` (calling the Python
      constructor in a defined way).
    - Rebuilds the initial compartment and stack from the snapshot without
      firing `$enter` unexpectedly.
  - Provide helpers for encoding/decoding snapshots to JSON using standard
    libraries (`json`, optional `orjson`), with `schemaVersion` included.

TypeScript (Stage 15 requirements)
- Do NOT rely on `JSON.stringify(this)` for systems; it fails on cycles and
  loses class identity.
- Instead:
  - Implement `frame_persistence_ts.snapshot(system: System): SystemSnapshot`
    that builds the DTO from:
    - `_compartment` (state name and args).
    - Public domain fields.
    - `_stack`.
  - Implement `frame_persistence_ts.restore(snapshot: SystemSnapshot): System`
    that:
    - Constructs a new system instance.
    - Seeds `_compartment` and `_stack` according to the snapshot.
  - Provide JSON encode/decode helpers that preserve `schemaVersion`.

Rust (Stage 15 requirements)
- Use `serde` over a dedicated `SystemSnapshot` struct instead of serializing
  the live system struct directly.
- Define:
  ```rust
  #[derive(Serialize, Deserialize)]
  struct SystemSnapshot {
      schema_version: u32,
      system_name: String,
      state: StateId,
      state_args: serde_json::Value,
      domain_state: serde_json::Value,
      stack: Vec<FrameCompartmentSnapshot>,
  }
  ```
  (details may evolve, but the pattern must match the conceptual model).
- Implement:
  - `impl From<&System> for SystemSnapshot`
  - `impl System { fn from_snapshot(snapshot: SystemSnapshot) -> Self }`
  - JSON encode/decode via `serde_json`.

Integration with Workflows (mandatory)
- Stage 15 is a **required capability** for production workflows on PRT:
  - The PRT runtimes and codegen must support round-tripping a system through
    `snapshot → JSON → snapshot → system` without violating core semantics
    (state, domain, stack).
  - The bug process and test suites must include persistence tests for at
    least one representative system per language (e.g., a traffic light or
    adapter-like system).
- Tests:
  - Positive: snapshot/restore cycles that preserve behavior.
  - Negative: snapshots with unknown `schemaVersion` or mismatched
    `systemName` are rejected gracefully.

Relationship to Stages 01–13
- Stage 15 depends on:
  - Stage 04/05/06 for accurate MIR/expansion/splice behavior (so state and
    transitions are well-defined).
  - Stage 10/11 for AST/symbol information (so state names and parameters are
    known).
- Stage 15 does **not** change core compilation semantics; it adds a
  persistence layer on top of the runtime model defined in
  `frame_runtime.md` and `codegen.md`.
