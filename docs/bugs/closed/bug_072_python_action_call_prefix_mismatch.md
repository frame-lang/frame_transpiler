# Bug #072: Python action call prefix mismatch (`self.log(...)` vs `_action_log`) leads to AttributeError

## Metadata
```yaml
bug_number: 072
title: "Python action call prefix mismatch (self.log vs _action_log)"
status: Closed
priority: Medium
category: CodeGen
discovered_version: v0.86.40
fixed_version: v0.86.41
reporter: Codex
assignee: 
created_date: 2025-11-15
resolved_date: 2025-11-15
```

## Description
When an action called another action by its FRM name (e.g., `self.log("...")`), the Python emitter generated the callee as `_action_log` but did not expose any public `log` method. At runtime, this resulted in `AttributeError: 'CallMismatch' object has no attribute 'log'` even though the generated module included `def _action_log(self, message): ...`.

## Root Cause
- V3 Python module emission consistently prefixed generated action/operation methods with `_action_` / `_operation_`.
- The emitter did not provide any public wrappers for those actions, so call sites inside handlers/actions that used FRM names (e.g., `self.log(...)`, `self.handle()`) accessed attributes that did not exist on the generated class.
- Call-site rewriting was not performed; the compiler relied entirely on the prefixed names.

## Fix
- In `framec/src/frame_c/v3/mod.rs` (Python module emission):
  - **Internal implementation**: actions and operations are still emitted as `_action_<name>` / `_operation_<name>` with signatures derived from header spans (and `async` detection where applicable).
  - **Public wrappers**: for each action `foo`:
    - A wrapper method is emitted with the FRM name:
      - Sync:
        - `def foo(self): return self._action_foo()` or
        - `def foo(self, x, y): return self._action_foo(x, y)`
      - Async:
        - `async def foo(self): return await self._action_foo()` or
        - `async def foo(self, x, y): return await self._action_foo(x, y)`
    - Parameter lists used in the wrapper call are built by stripping annotations/defaults from the header (best-effort), so `log(message: str = "x")` translates to `_action_log(message)`.
  - Handlers and other call sites can continue to use FRM names (e.g., `self.log("hello")`, `self.handle()`), and the wrappers delegate to the internal `_action_*` implementations.

## How to Validate

### Minimal repro

- FRM: `/tmp/frame_transpiler_repro/bug_072/minimal_call_mismatch.frm`
  ```frame
  @target python_3

  system CallMismatch {
      actions:
          log(message) {
              # log sink
              self.last = message
          }
          handle() {
              # Calls 'log' without _action_ prefix; generator emits _action_log
              self.log("hello")
          }
      machine:
          $S {
              e() { self.handle() }
          }
      domain:
          last = ""
  }
  ```
- Script: `/tmp/frame_transpiler_repro/bug_072/run_check.sh`
  - After the fix, the generated `minimal_call_mismatch.py` contains both `_action_log` and the public `log` wrapper, and the mismatch condition (`_action_log` with no `log`) is no longer present.
  - Optional: import and execute `e` to confirm there is no `AttributeError`.

### V3 CLI regression (optional follow-up)

- Add a V3 CLI fixture (pattern already used in `actions_try_except_async.frm`) asserting:
  - `def _action_log(self, message)` exists, and
  - `def log(self, message)` wrapper exists.

## Work Log
- 2025-11-15: Verified with framec v0.86.42; unit tests pass in vscode_editor — Owner closure

- 2025-11-15: Identified that internal `_action_*` methods did not have public wrappers; actions calling other actions by FRM name failed at runtime. — Codex  
- 2025-11-15: Implemented public wrappers for actions/operations while retaining `_action_*`/`_operation_*` as internal implementation; verified that the minimal repro generates `log`/`handle` wrappers and no longer exhibits AttributeError. — Codex

---
*Bug tracking policy version: 1.1*

