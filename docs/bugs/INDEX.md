# Frame Transpiler Bug Index

## Statistics
- **Total Bugs**: 64
- **Open**: 6
- **Resolved**: 56
- **Won't Fix**: 2
- **Next Bug Number**: 65

## Quick Links
- [Bug Tracking Policy](BUG_TRACKING_POLICY.md)
- [Bug Template](TEMPLATE.md)
- [Open Bugs](open/)
- [Closed Bugs](closed/)


## Repro Shortcuts

- #061: /tmp/frame_transpiler_repro/bug_061/minimal_py.frm, /tmp/frame_transpiler_repro/bug_061/run.sh
- #062: /tmp/frame_transpiler_repro/bug_062/minimal_py.frm, /tmp/frame_transpiler_repro/bug_062/run.sh

- #063: /tmp/frame_transpiler_repro/bug_063/minimal_py.frm, /tmp/frame_transpiler_repro/bug_063/run.sh

## Active Bugs

| Bug # | Title | Priority | Category | Status | Assignee |
|-------|-------|----------|----------|--------|----------|

## Recently Resolved

| Bug # | Title | Priority | Category | Status | Fixed Version |
|-------|-------|----------|----------|--------|---------------|
| [#062](closed/bug_062_python_emit_debug_flag_ignored.md) | Python compile appends trailers when --emit-debug is not set | High | Tooling | Resolved | v0.86.29 |
| [#063](closed/bug_063_python_module_compile_not_runnable_regression.md) | Python module compile still emits non-runnable output (regression) — references Bug #060 | High | Tooling | Resolved | v0.86.31 |
| [#064](closed/bug_064_python_comment_only_action_missing_pass.md) | Python codegen for comment-only action body emits no pass → IndentationError | High | CodeGen | Resolved | v0.86.31 |
