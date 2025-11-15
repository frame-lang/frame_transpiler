# Frame Transpiler Bug Index

## Statistics
- **Total Bugs**: 67
- **Open**: 1
- **Closed**: 64
- **Won't Fix**: 2
- **Next Bug Number**: 69

## Quick Links
- [Bug Tracking Policy](BUG_TRACKING_POLICY.md)
- [Bug Template](TEMPLATE.md)
- [Open Bugs](open/)
- [Closed Bugs](closed/)


## Repro Shortcuts

- #065: /tmp/frame_transpiler_repro/bug_065/minimal_runtime_pkg.frm, /tmp/frame_transpiler_repro/bug_065/run_check.sh, /tmp/frame_transpiler_repro/bug_065/run_validate.sh
- #067: /tmp/frame_transpiler_repro/bug_067/minimal_actions_missing.frm, /tmp/frame_transpiler_repro/bug_067/multiple_actions.frm, /tmp/frame_transpiler_repro/bug_067/run_validate.sh
- #068: /tmp/frame_transpiler_repro/bug_068/minimal_ts_import.frm, /tmp/frame_transpiler_repro/bug_068/run.sh
- #061: /tmp/frame_transpiler_repro/bug_061/minimal_py.frm, /tmp/frame_transpiler_repro/bug_061/run.sh
- #062: /tmp/frame_transpiler_repro/bug_062/minimal_py.frm, /tmp/frame_transpiler_repro/bug_062/run.sh

- #063: /tmp/frame_transpiler_repro/bug_063/minimal_py.frm, /tmp/frame_transpiler_repro/bug_063/run.sh

## Active Bugs

| Bug # | Title | Priority | Category | Status | Assignee |
|-------|-------|----------|----------|--------|----------|
| [#065](open/bug_065_python_runtime_package_not_emitted_by_compile.md) | Python runtime package not emitted by compile (frame_runtime_py missing) | High | Tooling | Reopened | Codex |

## Recently Resolved (awaiting closure by owning team)

- #067: Python actions not emitted in output module — fixed_version: v0.86.35
- #068: TypeScript runtime import path incorrect for single-file — fixed_version: v0.86.35
- #065: Python runtime package not emitted by compile — fixed_version: v0.86.36
