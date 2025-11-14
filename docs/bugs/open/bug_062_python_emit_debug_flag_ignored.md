# Bug #062: Python compile appends trailers when --emit-debug is not set

## Metadata
```yaml
bug_number: 062
title: "Python compile appends trailers when --emit-debug is not set"
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
Compiling with the module path without --emit-debug can still yield trailer blocks in the emitted .py, suggesting the flag is not respected or an env default enables trailers by default, contrary to docs.

## Reproduction Steps
1) Compile without --emit-debug:
   ./target/release/framec compile -l python_3 path/to/file.frm -o outdir
2) Inspect outdir/*.py — trailer blocks present in our environment.
3) Importing fails due to the same syntax issues as above.

## Expected Behavior
Without --emit-debug (and absent enabling env vars), no trailers should be appended; output should be runnable Python only.

## Actual Behavior
Trailer blocks are present even when --emit-debug is omitted.

## Proposed Solution
Ensure CLI honors --emit-debug strictly; only append trailers when flag is set (or specific env vars provided). Add a test for compile without trailers.

## Test Coverage
- [ ] Add compile test that imports a minimal module when compiled without --emit-debug

## Related Issues
- Bug #060

---
*Policy version: 1.0*
