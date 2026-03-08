# Bug #063: Python module compile still emits non-runnable output (regression) — references Bug #060

## Metadata
```yaml
bug_number: 063
title: "Python module compile still emits non-runnable output (regression) — references Bug #060"
status: Closed
priority: High
category: Tooling
discovered_version: v0.86.28
fixed_version: v0.86.31
assignee: Codex
created_date: 2025-11-14
resolved_date: 2025-11-14

## Description
Python compile still generates non-runnable modules due to invalid trailer embedding inside function bodies and/or invalid C-style comment blocks. This regresses the intent of #060 and conflicts with #061/#062 expectations.

## Reproduction Steps
1. Ensure FRAMEC binary at v0.86.30: `/Users/marktruluck/projects/frame_transpiler/target/release/framec -V` → `framec 0.86.30`.
2. Run repro script: `/tmp/frame_transpiler_repro/bug_063/run.sh` (uses FRAMEC_BIN and emits to a temp directory).
3. Observe import failure message `BUG_REPRODUCED: IndentationError …`.
4. Generated file path (example): `/tmp/frame_transpiler_repro/bug_063/out/minimal_py.py`.

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
- Generated module imports without syntax errors when compiled with `-l python_3`.
- With `--emit-debug`, debug trailers are syntactically valid and placed at module scope (e.g., triple-quoted Python string literal), not inside class/method bodies.
- Without `--emit-debug`, no trailers are emitted.

## Actual Behavior
- Import fails with IndentationError due to trailer blocks injected immediately after a method definition, where Python expects an indented block.
- Trailer content uses C-style `/* ... */` which is not valid Python unless wrapped appropriately, and is currently injected within an indented context.

```
File: /tmp/frame_transpiler_repro/bug_063/out/minimal_py.py
Error: IndentationError: expected an indented block after function definition on line 14

Relevant lines:
(See also snippet below)
```

```python
# Snippet from generated file around offending area
<will be updated dynamically by repro; see path above>
```

## Impact
- Severity: High — blocks using generated Python modules from Frame-only workflow.
- Scope: All Python targets with trailers; affects debugger rebuild pipeline and validation tests.
- Workaround: None in pure Frame-only flow. Manual post-processing to strip/relocate trailers is not acceptable per policy.

## Technical Analysis
- Trailer blocks (frame-map, visitor-map, debug-manifest, errors-json) are emitted using C-style delimiters and placed immediately after method bodies.
- In Python, this placement violates indentation rules and syntax.
- Even with `--emit-debug`, trailers must be placed at module scope and be syntactically valid in Python (triple-quoted strings or comments), not inside method indentation.

## Proposed Solution
- Emit trailers at module scope only (after class definitions) and wrap in triple-quoted strings (or `if False:` blocks) to maintain syntactic validity.
- Respect `--emit-debug`: do not emit trailers unless the flag is set.
- Ensure no C-style comments are present in Python output; use Python string literals or `#`-prefixed lines.
- Add generator invariant: never insert trailer content within an indented method/class block for Python.

## Test Coverage
- [ ] Add unit test to ensure generated Python imports with and without `--emit-debug`.
- [ ] Regression test that asserts no `/* ... */` tokens appear in Python output.
- [ ] Integration test using the provided /tmp repro artifacts.

## Related Issues
- Bug #060 — Python module compile outputs non-runnable file
- Bug #061 — Python trailer block style invalid (C-style comments)
- Bug #062 — `--emit-debug` appears ignored (trailers emitted unconditionally)

## Work Log
- 2025-11-15: Reopened with fresh repro on v0.86.30; added /tmp paths and error details — Codex

## Validation Assets
- Repro FRM: `/tmp/frame_transpiler_repro/bug_063/minimal_py.frm`
- Runner: `/tmp/frame_transpiler_repro/bug_063/run.sh`
- Generated file: `/tmp/frame_transpiler_repro/bug_063/out/minimal_py.py`

---
*Bug tracking policy version: 1.0*
\n### Generated Snippet (for reference)
\n```python
    14	    def e(self, __e: FrameEvent, compartment: FrameCompartment):
    15	        # no-op }
    16	        
    17	
    18	/*#frame-map#
    19	{"map":[{"targetStart":81,"targetEnd":100,"origin":"native","sourceStart":1,"sourceEnd":20 }] ,"version":1,"schemaVersion":1}
    20	#frame-map#*/
    21	
    22	/*#visitor-map#
```
