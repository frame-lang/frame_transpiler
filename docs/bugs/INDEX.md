# Frame Transpiler Bug Index

## Statistics
- **Total Bugs**: 25
- **Open**: 0
- **Fixed**: 1
- **Closed**: 24
- **Reopen**: 0
- **Won't Fix**: 1
- **Next Bug Number**: 071

## Quick Links
- [Bug Tracking Policy](BUG_TRACKING_POLICY.md)
- [Bug Template](TEMPLATE.md)
- [Open Bugs](open/)
- [Closed Bugs](closed/)


## Repro Shortcuts
- #070: /tmp/frame_transpiler_repro/bug_070/runtime_protocol.frm, /tmp/frame_transpiler_repro/bug_070/run.sh

- #067: /tmp/frame_transpiler_repro/bug_067/minimal_actions_missing.frm, /tmp/frame_transpiler_repro/bug_067/multiple_actions.frm, /tmp/frame_transpiler_repro/bug_067/run_validate.sh
- #068: /tmp/frame_transpiler_repro/bug_068/minimal_ts_import.frm, /tmp/frame_transpiler_repro/bug_068/run.sh
- #061: /tmp/frame_transpiler_repro/bug_061/minimal_py.frm, /tmp/frame_transpiler_repro/bug_061/run.sh
- #062: /tmp/frame_transpiler_repro/bug_062/minimal_py.frm, /tmp/frame_transpiler_repro/bug_062/run.sh

- #063: /tmp/frame_transpiler_repro/bug_063/minimal_py.frm, /tmp/frame_transpiler_repro/bug_063/run.sh

## Active Bugs (Open)

| Bug # | Title | Priority | Category | Status | Assignee |
|-------|-------|----------|----------|--------|----------|
| (none) |  |  |  |  |  |

## Fixed (awaiting closure by owning team)

| Bug # | Title | Fixed Version |
|-------|-------|----------------|
| [#069](fixed/bug_069_runner_relative_framec_path_cwd_failure.md) | Runner fails when CWD changes (relative framec path) | v0.86.37 |

## Recently Closed

- #065: Python runtime package not emitted by compile — fixed_version: v0.86.36
- #067: Python actions not emitted in output module — fixed_version: v0.86.35
- #068: TypeScript runtime import path incorrect for single-file — fixed_version: v0.86.35
- #070: Python module compile drops handlers/actions for complex FRM — fixed_version: v0.86.38
