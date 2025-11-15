# Bug #067: Python codegen omits `actions:` methods in output module (no `def _action_*`)

## Metadata
```yaml
bug_number: 067
title: "Python codegen omits actions methods in output module (no def _action_*)"
status: Closed
priority: High
category: CodeGen
discovered_version: v0.86.31
fixed_version: 
reporter: Codex
assignee: 
created_date: 2025-11-14
resolved_date: 
```

## Resolution
- Python module compile now emits methods for `actions:` and `operations:`:
  - Actions: `def _action_<name>(self, ...)`
  - Operations: `def _operation_<name>(self, ...)`
- Bodies are normalized and a `pass` is inserted for empty methods to keep code valid.

## Validation
- New fixture asserts presence of `_action_*` in compiled code:
  - `framec_tests/language_specific/python/v3_cli/positive/actions_emitted.frm`

