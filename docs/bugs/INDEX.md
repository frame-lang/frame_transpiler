# Frame Transpiler Bug Index

## Statistics
- **Total Bugs**: 30
- **Open**: 0
- **Fixed**: 0
- **Closed**: 30
- **Reopen**: 0
- **Won't Fix**: 0
## Quick Links
- [Bug Tracking Policy](BUG_TRACKING_POLICY.md)
- [Bug Template](TEMPLATE.md)
- [Open Bugs](open/)
- [Closed Bugs](closed/)
- [Fixed Bugs](fixed/)

## Repro Shortcuts
- #075: `/tmp/frame_transpiler_repro/bug_075/minimal_redeclare_next_compartment.frm`, scripts `run.sh`, `run_validate.sh`

- #074: `/tmp/frame_transpiler_repro/bug_074/minimal_ts_emit_issues.frm`, script `run.sh`

- #073: `/tmp/frame_transpiler_repro/bug_073/ts_dup_methods.frm`, script `run.sh`

- #071: `/tmp/frame_transpiler_repro/bug_071/{minimal_try_except.frm, minimal_async_try_except.frm, harness.frm}`, scripts `run_import_minimal.sh`, `run_import_async.sh`, `run_import_harness.sh`
- #070: `/tmp/frame_transpiler_repro/bug_070/runtime_protocol.frm`, `/tmp/frame_transpiler_repro/bug_070/run_check_handlers.sh`
- #069: `/tmp/frame_transpiler_repro/bug_069/minimal_dup_iface.frm`, `/tmp/frame_transpiler_repro/bug_069/runtime_protocol.frm`
- #067: `/tmp/frame_transpiler_repro/bug_067/minimal_actions_missing.frm`, `/tmp/frame_transpiler_repro/bug_067/multiple_actions.frm`, `/tmp/frame_transpiler_repro/bug_067/run_validate.sh`
- #068: `/tmp/frame_transpiler_repro/bug_068/minimal_ts_import.frm`, `/tmp/frame_transpiler_repro/bug_068/run.sh`
- #061: `/tmp/frame_transpiler_repro/bug_061/minimal_py.frm`, `/tmp/frame_transpiler_repro/bug_061/run.sh`
- #062: `/tmp/frame_transpiler_repro/bug_062/minimal_py.frm`, `/tmp/frame_transpiler_repro/bug_062/run.sh`
- #063: `/tmp/frame_transpiler_repro/bug_063/minimal_py.frm`, `/tmp/frame_transpiler_repro/bug_063/run.sh`

## Active Bugs (Open)

_(none)_
## Fixed (awaiting closure by owning team)

| Bug # | Title | Fixed Version |
|-------|-------|----------------|
| 073 | TypeScript generator emits duplicate class methods per state | v0.86.42 |
| 074 | TypeScript generator omits actions/domain declarations and drops interface params | v0.86.43 |

## Recently Closed

- #071: Python actions with try/except/async emit invalid code — `fixed_version: v0.86.40`
- #070: Python module compile drops handlers/actions for complex FRM — `fixed_version: v0.86.39`
- #069: Runner fails when CWD changes (relative framec path) — `fixed_version: v0.86.37`
- #072: Python action call prefix mismatch (self.log vs _action_log) — `fixed_version: v0.86.41`

## Reopen

_(none)_
