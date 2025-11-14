# Bug #061: Python compile appends C-style trailer blocks that break Python syntax

## Metadata
```yaml
bug_number: 061
title: "Python compile appends C-style trailer blocks that break Python syntax"
status: Open
priority: High
category: Tooling
discovered_version: v0.86.28
fixed_version: 
reporter: Codex
assignee: 
created_date: 2025-11-14
resolved_date: 
```

## Description
Python compile appends debug trailers using C-style block markers (/*#name# … #name#*/) directly in the emitted .py file. These markers are not valid Python syntax and can appear after an indented def/class header, producing IndentationError and preventing import.

## Reproduction Steps
1) Compile any module with or without --emit-debug:
   ./target/release/framec compile -l python_3 --emit-debug path/to/file.frm -o outdir
2) Inspect outdir/*.py — trailer blocks like /*#frame-map# … #frame-map#*/ appear inline.
3) Import the module; observe syntax errors (IndentationError after function header).

## Expected Behavior
Trailers should be embedded using Python-safe constructs (triple-quoted strings, or # per line), placed at module scope after code blocks so import is never broken.

## Actual Behavior
Trailer blocks use /* … */ and can be injected at positions that break indentation or syntax.

## Proposed Solution
For Python target, wrap trailers in a top-level triple-quoted string or emit as # comments; ensure placement at module scope after code, separated by a blank line.

## Test Coverage
- [ ] Add compile test that imports and runs a minimal handler
- [ ] Verify trailers present and parseable without breaking import

## Related Issues
- Bug #060

---
*Policy version: 1.0*
