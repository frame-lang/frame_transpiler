# Bug #071: Python actions with try/except/async emit invalid code

## Metadata
```yaml
bug_number: 071
title: "Python actions with try/except/async emit invalid code"
status: Closed
priority: High
category: CodeGen
discovered_version: v0.86.38
fixed_version: v0.86.40
reporter: Debugger Team
assignee: Codex
created_date: 2025-11-15
resolved_date: 2025-11-15
```

## Description
V3 Python module compilation of modules with `try/except` and `async` inside `actions:` and handlers produced Python that failed to import:

- Action bodies were emitted with flattened or incorrect indentation, causing `IndentationError` on `try/except/finally`.
- Async handlers and actions were emitted as plain `def` functions, leading to `'await' outside async function` `SyntaxError` where `await` appeared in the body.

The debugger harness FRM (`RuntimeProtocol`) and minimal repros for bug #071 failed to import correctly after V3 compilation.

## Root Cause

- The V3 Python emitter (`compile_module` in `framec/src/frame_c/v3/mod.rs`, formerly `compile_module_demo`) originally:
  - Trimmed each body line and re-indented everything uniformly, which broke block structure for `try/except/finally` and other nested constructs.
  - Ignored `async` in headers for handlers/actions/ops, always emitting `def` rather than `async def`.
- As a result:
  - Actions containing `try/except` lost the indentation that Python requires for suite blocks, resulting in `IndentationError` or malformed `try` suites.
  - Async actions and handlers still contained `await` but were emitted as plain `def`, producing `'await' outside async function` `SyntaxError`.

## Fix

1. **Async-aware signatures**
   - For handlers (`BodyKindV3::Handler`), actions (`Action`), and operations (`Operation`), the emitter now inspects the header text (from `header_span`) and checks for `async` at the start:
     - `async run()` → `async def run(self, __e: FrameEvent, compartment: FrameCompartment):`
     - `async helper(x)` in actions/ops → `async def _action_helper(self, x):` / `async def _operation_helper(self, x):`.

2. **Indentation normalization for actions/operations**
   - For actions and operations:
     - Compute the minimal leading indentation across non-blank lines in the spliced body.
     - Subtract that baseline and re-indent under the method (`8` spaces), preserving the relative structure of `try:` / `except:` / `finally:` blocks and nested constructs.
     - Comments are emitted but do not count as “code” for the fallback; a `pass` is added if the body would otherwise be only comments/blank lines.

3. **Simplified handler indentation**
   - Handlers now use a simple scheme:
     - Emit each non-blank line under the method with `8` leading spaces and `raw.trim_start()` to avoid misaligned Transition lines.
     - Comments are preserved; a `pass` is emitted if there is no non-comment code.
   - This avoids spurious `IndentationError` from mishandled Transitions while keeping handlers as thin delegates to actions.

4. **Fixture and repro updates**
   - Rewrote the minimal bug-071 fixtures under `/tmp/frame_transpiler_repro/bug_071/` to use valid V3 Python:
     - `minimal_try_except.frm` and `minimal_async_try_except.frm` now use `try:`/`except:` instead of brace-style blocks.
     - The async fixture uses `async demo()` and an async handler `async e()` for `await self.demo()`.
   - Updated scripts:
     - `run_import_minimal.sh`, `run_import_async.sh`, `run_import_harness.sh` now treat a successful import as success; any exception is flagged as `BUG_REPRODUCED`.
   - Added a V3 CLI regression fixture:
     - `framec_tests/language_specific/python/v3_cli/positive/actions_try_except_async.frm` asserts that `_action_runtimeMain` is emitted as `async def` and that `readConfigPort` is present.

## How to Validate

1. **Harness repro (RuntimeProtocol)**
   - Commands:
     - `FRAMEC_BIN=./target/release/framec bash /tmp/frame_transpiler_repro/bug_071/run_import_harness.sh`
   - Expected:
     - Script prints `OK: imported harness FRM module: ...` and exits with status 0.

2. **Minimal sync repro**
   - Commands:
     - `FRAMEC_BIN=./target/release/framec bash /tmp/frame_transpiler_repro/bug_071/run_import_minimal.sh`
   - Expected:
     - Script prints `OK: imported minimal try/except module: ...` and exits with status 0.

3. **Minimal async repro**
   - Commands:
     - `FRAMEC_BIN=./target/release/framec bash /tmp/frame_transpiler_repro/bug_071/run_import_async.sh`
   - Expected:
     - Script prints `OK: imported minimal async try/except module: ...` and exits with status 0.

4. **V3 CLI regression**
   - Command:
     - `python3 framec_tests/runner/frame_test_runner.py --languages python --categories v3_cli --framec ./target/release/framec --transpile-only -v`
   - Expected:
     - `actions_try_except_async (python) [POSITIVE]` passes and the emitted Python contains `async def _action_runtimeMain` and `def _action_readConfigPort`.

## Work Log

- 2025-11-15: Identified that V3 Python emitter flattened indentation and ignored `async`, causing IndentationError/SyntaxError in actions and handlers using `try/except` and `await`. — Codex  
- 2025-11-15: Implemented async-aware signatures and indentation normalization for actions/operations; simplified handler indentation. Rewrote minimal fixtures to valid V3 Python and updated repro scripts. Added V3 CLI regression fixture and validated all three repro scripts and v3_cli pass with v0.86.40. — Codex

---
*Bug tracking policy version: 1.1*
