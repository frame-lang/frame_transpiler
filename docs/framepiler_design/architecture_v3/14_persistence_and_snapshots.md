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
    snapshot. In early PRT implementations this MAY be stored in the same
    structural shape as the underlying runtime (`dict` or `list` in Python);
    cross-language adapters can re-map it to a canonical name-keyed form if
    needed.
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

System-Level Helpers and `@persist`
- In addition to the library functions above, V3 systems may opt in to
  type-centric convenience helpers by annotating the system header:

  ```frame
  @persist system TrafficLight($(color), domain) {
      // ...
  }
  ```

- When `@persist` is present, the long-term goal (tracked in `PLAN.md`) is
  for the per-language V3 module-path codegen to emit idiomatic helpers on
  the generated system type that delegate to the persistence libraries:
  - Python (class-level helpers):
    - `@classmethod def save_to_json(cls, system) -> str`
    - `@classmethod def restore_from_json(cls, text: str)`
  - TypeScript (static helpers):
    - `static saveToJson(system: S): string`
    - `static restoreFromJson(text: string): S`
  - Rust (inherent methods on the generated system struct):
    - `pub fn save_to_json(&self) -> String`
    - `pub fn restore_from_json(text: &str) -> Self`

- These helpers are thin, type-centric wrappers around the persistence
  libraries:
  - Python: `frame_persistence_py.snapshot_system` /
    `frame_persistence_py.restore_system` plus JSON helpers.
  - TypeScript: `frame_persistence_ts.snapshotSystem` /
    `frame_persistence_ts.restoreSystem` plus JSON helpers.
  - Rust: `frame_persistence_rs::SnapshotableSystem` /
    `frame_persistence_rs::SystemSnapshot` and their JSON helpers.
- A future extension (`@persist(save_name, restore_name)`) MAY allow
  per-system customization of helper names while keeping semantics aligned
  with the standard `save_to_json` / `restore_from_json` (or `saveToJson` /
  `restoreFromJson`) pattern.
- Implementation status:
  - The library-level helpers (`snapshot_system` / `restore_system` and
    their JSON counterparts) are implemented for all PRT languages.
  - System-level helpers driven by `@persist` are being introduced
    incrementally and will be wired into the V3 module-path codegen as part
    of the Stage 19 Rust-first tooling and persistence work.

Python (Stage 15 requirements)
- Do NOT rely on `jsonpickle.encode(self)` for full-system persistence in V3.
- Instead:
  - Implement `frame_persistence_py.snapshot_system(system)` that:
    - Inspects `_compartment`, `_stack`, and selected domain fields.
    - Produces a `SystemSnapshot` structure whose `stateArgs` field mirrors
      the runtime `FrameCompartment.state_args` value (typically a dict for
      start-state parameters, sometimes a list for positional transition
      args).
    - Optionally accepts a `domain_encoder(system) -> Mapping[str, Any]`
      callback that can override the default domain inference when complex
      object graphs must be projected into a stable DTO.
  - Implement `frame_persistence_py.restore_system(snapshot, system_factory)` that:
    - Instantiates a new system via `system_factory` (calling the Python
      constructor in a defined way).
      - Rebuilds the initial compartment and stack from the snapshot without
        firing `$enter` unexpectedly.
    - Optionally accepts a `domain_decoder(snapshot, system)` callback that
      can perform additional reconstruction of complex domain state after the
      generic attribute-based restore has run.
  - Provide helpers for encoding/decoding snapshots to JSON using standard
    libraries (`json`, optional `orjson`), with `schemaVersion` included.

TypeScript (Stage 15 requirements)
- Do NOT rely on `JSON.stringify(this)` for systems; it fails on cycles and
  loses class identity.
- Instead:
  - Implement `frame_persistence_ts.snapshotSystem(system)` that builds the
    DTO from:
    - `_compartment` (state name and args; `stateArgs` mirrors the runtime
      `FrameCompartment.stateArgs` shape).
    - Public domain fields (own, non-function properties whose names do not
      start with `_`).
    - `_stack` (when present).
    - Optional `encodeDomain(system)` callback in the options bag can
      override default domain inference when complex graphs are projected
      into DTOs.
  - Implement `frame_persistence_ts.restoreSystem(snapshot, factory)` that:
    - Constructs a new system instance via the provided `factory`.
    - Seeds `_compartment` and `_stack` with new `FrameCompartment` objects,
      using the snapshot's `state` / `stateArgs`.
    - Optional `decodeDomain(snapshot, system)` callback in the options bag
      can refine or replace the default attribute-based restore logic.
  - Provide JSON encode/decode helpers (`snapshotToJson`, `snapshotFromJson`)
    that preserve `schemaVersion`.

Rust (Stage 15 requirements)
- Use `serde` over a dedicated `SystemSnapshot` struct instead of serializing
  the live system struct directly.
- Implement a small helper crate `frame_persistence_rs` that exposes:
  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct FrameCompartmentSnapshot {
      pub state: String,
      pub state_args: serde_json::Value,
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct SystemSnapshot {
      pub schema_version: u32,
      pub system_name: String,
      pub state: String,
      pub state_args: serde_json::Value,
      pub domain_state: serde_json::Value,
      pub stack: Vec<FrameCompartmentSnapshot>,
  }

  pub trait SnapshotableSystem: Sized {
      fn snapshot_system(&self) -> SystemSnapshot;
      fn restore_system(snapshot: SystemSnapshot) -> Self;
  }
  ```
  Generated V3 Rust systems can implement `SnapshotableSystem` in their
  module, mapping their internal `compartment` / `_stack` / domain fields
  into this neutral shape.
- `SystemSnapshot` provides JSON helpers (`to_json`, `to_json_pretty`,
  `from_json`) using `serde_json`.

Practical How-To (Python / TypeScript / Rust)
- This section shows concrete, minimal examples for taking and restoring
  snapshots in each PRT language using the TrafficLight fixtures.

- Python (`frame_persistence_py`):
  - Fixture: `framec_tests/language_specific/python/v3_persistence/positive/traffic_light_persistence.frm`.
  - Typical usage:
    ```python
    from traffic_light_persistence_v3 import TrafficLight  # generated V3 module

    # Create and run the system to a non-trivial state.
    tl = TrafficLight("red", "red", None)
    tl.tick()  # e.g., Red -> Green

    # Take a snapshot and round-trip via JSON using
    # the generated class helpers.
    json_text = TrafficLight.save_to_json(tl)
    tl2 = TrafficLight.restore_from_json(json_text)
    # Continue from the restored state.
    tl2.tick()
    tl2.tick()
    ```

- TypeScript (`frame_persistence_ts`):
  - Fixture: `framec_tests/language_specific/typescript/v3_persistence/positive/traffic_light_persistence.frm`.
  - Typical usage (in the generated module’s `main`):
    ```ts
    const tl = new TrafficLight("red", "red", null);
    tl.tick(); // Red -> Green

    const jsonText = TrafficLight.saveToJson(tl);
    const tl2 = TrafficLight.restoreFromJson(jsonText);
    tl2.tick();
    tl2.tick();
    tl2.tick();
    ```

- Rust (`frame_persistence_rs`):
  - Fixture: `framec_tests/language_specific/rust/v3_persistence/positive/traffic_light_snapshot_dump.frm`.
  - Typical usage pattern in a small harness:
    ```rust
    use frame_persistence_rs::{SystemSnapshot};

    // Assume `TrafficLight` is the generated V3 Rust system and implements
    // SnapshotableSystem.
    fn main() {
        let mut tl = TrafficLight::new("red".into(), "red".into(), ());
        tl.tick(); // Red -> Green

        let snap = tl.snapshot_system();
        let json = snap.to_json_pretty().unwrap();
        let snap2 = SystemSnapshot::from_json(&json).unwrap();

        let mut tl2 = TrafficLight::restore_system(snap2);
        tl2.tick();
        tl2.tick();
    }
    ```
  - The exact constructor and type signatures depend on the generated V3
    system, but the snapshot/restore flow always follows:
    `system → SystemSnapshot → JSON → SystemSnapshot → system`.

Naming convention for generated helpers
- For systems annotated with `@persist`, V3 adopts a standard naming
  convention for marshalling helpers on the generated types:
  - Python:
    - `TrafficLight.save_to_json(system)` and
      `TrafficLight.restore_from_json(text)`
  - TypeScript:
    - `TrafficLight.saveToJson(system)` and
      `TrafficLight.restoreFromJson(text)`
  - Rust:
    - `impl SnapshotableSystem for TrafficLight` and inherent
      `TrafficLight::save_to_json(&self)` /
      `TrafficLight::restore_from_json(&str) -> Self`
- These helpers are **opt-in** (only for `@persist` systems) and delegate
  to the language-specific persistence libraries; they are intended to be
  the canonical marshalling surface for common workflows, while the
  underlying libraries (`frame_persistence_py`, `frame_persistence_ts`,
  `frame_persistence_rs`) remain available for advanced/custom encoders.

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

Cross-Language Validation (Stage 17 linkage)
- Schema-level shape:
  - Python/TypeScript: `tools/test_cross_language_snapshot_shape.py` uses a
    canonical `TrafficLight` snapshot JSON to exercise
    `frame_persistence_py` and `frame_persistence_ts`, round-tripping via
    their JSON helpers and comparing snapshots with `compare_snapshots`.
  - Rust:
    - `frame_persistence_rs` tests (`SystemSnapshot::from_json`,
      `SystemSnapshot::to_json_pretty`, and `SystemSnapshot::compare`)
      validate that the same canonical JSON shape can be parsed and
      re-emitted without structural differences.
    - A Rust-native tool, `v3_rs_snapshot_shape` (under
      `framec/src/bin/`), constructs the same canonical JSON and invokes
      the Python and TypeScript persistence helpers via subprocesses,
      ensuring that all three PRT targets agree on the snapshot DTO
      from a single Rust entrypoint.
- Runtime-level semantics:
  - Python/TypeScript: `tools/test_cross_language_snapshot_traffic_light.py`
    compiles and runs the `TrafficLight` persistence fixtures under the V3
    module path, captures their snapshots via the per-language helpers, and
    asserts structural equality on the resulting JSON.
  - Rust: a matching runtime-level harness is planned as a follow-on Stage 17
    task once a small V3 Rust system is wired to `frame_persistence_rs`.

Relationship to Stages 01–13
- Stage 15 depends on:
  - Stage 04/05/06 for accurate MIR/expansion/splice behavior (so state and
    transitions are well-defined).
  - Stage 10/11 for AST/symbol information (so state names and parameters are
    known).
- Stage 15 does **not** change core compilation semantics; it adds a
  persistence layer on top of the runtime model defined in
  `frame_runtime.md` and `codegen.md`.
