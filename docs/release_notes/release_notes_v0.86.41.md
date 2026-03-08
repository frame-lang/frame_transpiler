# Release Notes — v0.86.41 (2025-11-15)

Type: Bug-fix (Python action call wrappers)

## Highlights

- **Bug #072 — Python action call prefix mismatch**
  - Previously, V3 Python emitted actions as `_action_<name>` but did not expose public methods with the FRM action names. Call sites in handlers/actions that used `self.log(...)` or `self.handle()` therefore failed at runtime with `AttributeError`.
  - The emitter now:
    - Keeps `_action_<name>` as the internal implementation.
    - Emits public wrappers `def name(self, ...)` (or `async def name(self, ...)` for async actions) that forward to `_action_<name>(...)`, reconstructing the call argument list from the header parameters.
  - This preserves FRM semantics for intra-system action calls while keeping internal `_action_*` hooks available for tooling and tests.

## Version

- Workspace version bumped to **0.86.41**; `framec --version` reports `0.86.41`.

