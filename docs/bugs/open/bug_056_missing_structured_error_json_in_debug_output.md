# Bug #056: Missing structured error JSON in --debug-output mode

## Metadata
```yaml
bug_number: 056
title: "Missing structured error JSON in --debug-output mode"
status: Open
priority: High
category: Tooling
discovered_version: v0.86.25
fixed_version: 
reporter: Codex
assignee: 
created_date: 2025-11-13
resolved_date: 
```

## Description
When `framec` is invoked with `--debug-output`, syntax/semantic errors produce a non‑zero exit code with human‑readable diagnostics on stderr and no JSON on stdout. Tools expecting a JSON envelope (to surface precise errors in IDEs) cannot parse failures consistently.

## Reproduction Steps
1. Create an intentionally invalid Frame file `invalid.frm` (missing brace):
2. Run: `framec -l python_3 --debug-output invalid.frm`
3. Observe that the process exits non‑zero, prints plain text diagnostics, and emits no JSON on stdout.

## Test Case
```frame
system S {
    machine:
        $Start {
            $>() {
                -> $Next
        }
}
```

## Expected Behavior
- With `--debug-output`, errors should be emitted as a structured JSON envelope on stdout in addition to a non‑zero exit, e.g.:
```json
{
  "error": {
    "message": "Unclosed handler block",
    "code": "SyntaxError",
    "frameFile": "invalid.frm",
    "frameLine": 6,
    "frameColumn": 17
  },
  "metadata": { "frameVersion": "0.86.25" }
}
```

## Actual Behavior
- Plain text diagnostics on stderr only; stdout contains no JSON. Consumers that expect a JSON object fail to parse and must fall back to generic errors.

## Impact
- **Severity**: High — blocks robust IDE/debugger integration and automated tooling.
- **Scope**: All consumers of `--debug-output` across languages.
- **Workaround**: Parse stderr heuristically (fragile and language‑dependent).

## Technical Analysis
- The CLI flag is parsed (`framec/src/frame_c/cli.rs`), but error paths bypass the JSON writer used for successful debug output and write directly to stderr before exiting.

### Root Cause
- No centralized error → JSON emission path when `--debug-output` is set.

### Affected Files
- `framec/src/frame_c/cli.rs` – flag handling and top‑level command dispatch
- `framec/src/main.rs` (or equivalent entry) – error handling and process exit

## Proposed Solution
- When `--debug-output` is present, catch top‑level errors and emit a stable JSON envelope to stdout (do not intermingle with stderr), then exit non‑zero.

### Option 1: Centralized error reporter
- Add an error reporter gated by `debug_output` that serializes `{ error, metadata }` to stdout.
- Pros: Minimal surface change, consistent across subcommands.
- Cons: Requires auditing all early returns to ensure they are wrapped.

### Option 2: Wrapper around main dispatch
- Wrap the main command execution in a `match` that converts any `Err` into the JSON envelope when `debug_output` is true.
- Pros: Single chokepoint.
- Cons: Ensure lower layers do not partially write to stdout first.

## Test Coverage
- [ ] Unit test: CLI returns JSON error when `--debug-output` + invalid input
- [ ] Integration test: stderr contains human message; stdout contains JSON; exit is non‑zero
- [ ] Regression test: valid runs remain unchanged

## Related Issues
- Bug #057 – source map schema consistency

## Work Log
- 2025-11-13: Initial report — Codex

## Resolution
_Pending._

### Fix Summary
_Pending._

### Verification
_Pending._

### Lessons Learned
_Pending._

---
*Bug tracking policy version: 1.0*
