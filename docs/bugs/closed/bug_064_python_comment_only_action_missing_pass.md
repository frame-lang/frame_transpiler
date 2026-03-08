# Bug #064: Python codegen for comment-only action body emits no pass → IndentationError

## Metadata
```yaml
bug_number: 064
title: "Python codegen for comment-only action body emits no pass → IndentationError"
status: Closed
priority: High
category: CodeGen
discovered_version: v0.86.31
fixed_version: v0.86.31
reporter: Codex
assignee: Codex
created_date: 2025-11-14
resolved_date: 2025-11-14
```

## Description
When a Python handler body contained only comments/whitespace, the compiled method suite lacked an actual statement, triggering `IndentationError` on import.

## Fix Summary
- Normalize spliced method body lines (left-strip, re-indent to method level) and insert a `pass` when no real code remains after stripping.
- Applies to Python module compile path (non-demo).

## Verification
- Non-debug compile: import succeeds.
- Debug compile (`--emit-debug`): import succeeds; trailers are safely embedded (triple-quoted) and extracted by the runner.

---
*Resolution Owner: Codex*
