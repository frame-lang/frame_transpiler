# Release Notes — v0.86.39 (2025-11-15)

Type: Bug-fix (V3 Python module emit)

## Highlights

- **Bug #070 — Python module compile drops handlers/actions for complex FRM**
  - Fixed partitioning so that Python import scanning stops at the first V3 section or `system` header.
    - Prevents `import` statements inside `actions:` bodies from being treated as top-level imports, which previously caused the outline scan to start near `domain:` and miss all handlers/actions.
  - Extended the V3 outline scanner to recognize `async` function headers inside `machine:` and `actions:` sections:
    - Correctly discovers `async run()` handlers in state blocks and `async` actions such as `runtimeMain`, `handleCommand`, etc.
  - Python module emitter now reliably produces:
    - State handlers (e.g., `def run(self, __e: FrameEvent, compartment: FrameCompartment):`), and
    - `_action_*` methods for every action body in the FRM harness.
  - The harness repro script now fails as expected (no `BUG_REPRODUCED` message) against `runtime_protocol.frm`, confirming handlers/actions are present.

## Notes

- This release supersedes the previous 0.86.38 attempt for Bug #070 by fully addressing the complex harness FRM, not just simpler interface/actions fixtures.
- Existing V3 CLI/debugger fixtures continue to pass; legacy pre‑V3 async fixtures still report `E111` and remain candidates for retirement or migration.

## Version

- Workspace version bumped to **0.86.39**; `framec --version` reports `0.86.39`.

