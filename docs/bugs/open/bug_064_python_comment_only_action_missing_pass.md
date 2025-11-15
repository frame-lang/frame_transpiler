# Bug #064: Python codegen for comment-only action body emits no pass → IndentationError

## Metadata
```yaml
bug_number: 064
title: "Python codegen for comment-only action body emits no pass → IndentationError"
status: Resolved
priority: High
category: CodeGen
discovered_version: v0.86.31
fixed_version: v0.86.31
reporter: Codex
assignee: 
created_date: 2025-11-14
resolved_date: 2025-11-14
```

## Description
When an action body contains only a comment (no executable statements), the Python generator emits an empty function suite (only a comment line), which triggers `IndentationError` on import. This is independent of debug trailers.

## Reproduction Steps
1. Ensure framec is v0.86.31
2. Non-debug repro: `/tmp/frame_transpiler_repro/bug_064/run_non_debug.sh`
3. Debug repro: `/tmp/frame_transpiler_repro/bug_064/run_debug.sh`

## Test Case
```frame
@target python_3

system Minimal {
    machine:
        $Start {
            e() { # no-op }
        }
}
```

## Expected Behavior
- The generated method body should contain at least a `pass` when no executable statements exist after comment stripping.
- Module imports cleanly with and without `--emit-debug`.

## Actual Behavior
- Non-debug: `IndentationError: expected an indented block after function definition`.
- Debug: same error; trailers (now correctly at module scope in triple-quoted strings) are not the cause.

## Impact
- Severity: High — minimal comment-only actions break module import.
- Scope: All Python targets where comment-only action bodies occur.
- Workaround: Avoid comment-only bodies; add a dummy statement.

## Technical Analysis
- After stripping comments/whitespace, the generator should detect an empty body and emit a `pass` statement indented to the method block level.
- Current output leaves only `# ...` which Python ignores in the suite, leading to `IndentationError`.

## Proposed Solution
- In Python backend, if the normalized action body is empty (or comment-only), insert `pass`.
- Add tests covering both empty body `{}` and comment-only `{ # ... }`.

## Validation Assets
- Non-debug script: `/tmp/frame_transpiler_repro/bug_064/run_non_debug.sh`
- Debug script: `/tmp/frame_transpiler_repro/bug_064/run_debug.sh`
- FRM: `/tmp/frame_transpiler_repro/bug_064/minimal_comment_only.frm`

## Related Issues
- Bug #063 (resolved): Trailer placement fixed; this is distinct.

## Work Log
- YYYY-MM-DD: Initial report with /tmp repro — Codex

---
*Bug tracking policy version: 1.0*
