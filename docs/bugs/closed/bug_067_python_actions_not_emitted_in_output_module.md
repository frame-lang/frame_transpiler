# Bug #067: Python codegen omits `actions:` methods in output module (no `def _action_*`)

## Metadata
```yaml
bug_number: 067
title: "Python codegen omits actions methods in output module (no def _action_*)"
status: Closed
priority: High
category: CodeGen
discovered_version: v0.86.31
fixed_version: v0.86.33
reporter: Codex
assignee: 
created_date: 2025-11-14
resolved_date: 2025-11-15
```

## Resolution
- Python module compile now emits methods for `actions:` and `operations:`:
  - Actions: `def _action_<name>(self, ...)`
  - Operations: `def _operation_<name>(self, ...)`

