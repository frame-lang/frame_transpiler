# Bug #066: TypeScript action body boolean operators (||, &&) rejected by legacy CLI (version skew)

## Metadata
```yaml
bug_number: 066
title: "TypeScript action body boolean operators (||, &&) rejected by legacy CLI (version skew)"
status: Closed
priority: Medium
category: Tooling
discovered_version: v0.85.3 (bundled CLI)
fixed_version: v0.86.32
reporter: Codex
assignee: 
created_date: 2025-11-14
resolved_date: 2025-11-15
```

## Resolution
- Not a compiler bug in current releases. The bundled workspace CLI was outdated and enforced non-TS rules in action bodies.
- Action: Update the bundled CLI to v0.86.31+ and add a compatibility note: action bodies are native TS and allow `||`/`&&`.

## Validation
- Recompiled the minimal fixture with v0.86.31+ `framec compile -l typescript` — success.
- Added doc notes; no code changes required.

