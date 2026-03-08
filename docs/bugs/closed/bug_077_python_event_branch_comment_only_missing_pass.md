# Bug #077: Python event branch with comment-only body misses 'pass' (IndentationError)

## Metadata
```yaml
bug_number: 077
title: "Python event branch with comment-only body misses 'pass' (IndentationError)"
status: Fixed
priority: High
category: CodeGen
discovered_version: v0.86.46
fixed_version: v0.86.47
reporter: vscode_editor (Codex)
assignee: framepiler team
created_date: 2025-11-17
resolved_date: 2025-11-18
```

## Description
When generating Python for a machine event that has a comment-only body (e.g.,
`# Already terminated`), the emitter previously produced an `elif` block with
only a comment and no executable statement. Python requires at least one
statement in a block; the result was an
`IndentationError: expected an indented block after 'elif'`.

This was analogous to the earlier fix for comment-only actions (see #064), but
occurred specifically in the event/handler emission path for machine branches.

## Reproduction Steps (original)

1. Use framec v0.86.46.
2. Compile the minimal FRM and import the generated module:
   - `FRAMEC=/Users/marktruluck/projects/frame_transpiler/target/release/framec`
   - `/tmp/frame_transpiler_repro/bug_077/minimal_event_comment_only.frm`
   - Generated: `/tmp/frame_transpiler_repro/bug_077/minimal_event_comment_only.py`
3. Import test prints `IMPORT_FAIL: IndentationError ...`.

Minimal example:

```frame
@target python_3

system EmptyElif {
  interface:
    run()

  machine:
    $Idle { run() { -> $Terminated } }
    $Terminated { run() { # Already terminated } }
}
```

## Expected Behavior

- Generated Python should include a `pass` (or `...`) in any event/handler
  branch whose body is comment-only, ensuring valid syntax.
- Importing the generated module should succeed without `IndentationError`.

## Actual Behavior (pre-fix)

- The `elif` branch for `$Terminated` contained only a comment; Python import
  failed with `IndentationError: expected an indented block after 'elif'`.

## Fix Summary (v0.86.47)

**Files**:

- `framec/src/frame_c/v3/mod.rs` (Python emitter for V3 module handlers)
- `framec_tests/language_specific/python/v3_systems_runtime/positive/empty_elif_comment_only_exec.frm`

**Emitter change**:

- In the Python handler emission path, both single-state and multi-state event
  handlers now track whether any non-comment line is emitted for a branch:
  - Comments are preserved.
  - If a branch’s normalized body is comment-only, a `pass` statement is
    appended at the correct indentation level.
- This mirrors the prior comment-only handling for actions (Bug #064) and
  ensures that every `if`/`elif` branch has at least one executable statement.

**Tests**:

- Added V3 runtime regression:
  - `framec_tests/language_specific/python/v3_systems_runtime/positive/empty_elif_comment_only_exec.frm`
    - Defines a system with a `$Terminated.run()` handler whose body is only a
      comment.
    - `fn main()` drives the system; the test is executed via the V3 runner.
- Runner command:
  - `python3 framec_tests/runner/frame_test_runner.py --languages python --categories v3_systems_runtime --framec ./target/release/framec --run --include-common`
  - Result: 2/2 tests passing in `v3_systems_runtime` (TrafficLight + this fixture).

This confirms that the generator now emits `pass` for comment-only event
branches and that the generated Python modules import and execute without
`IndentationError`.

