# Frame Transpiler Bug Index

## Statistics
- **Total Bugs**: 27
- **Open**: 0
- **Fixed**: 3
- **Closed**: 24
- **Reopen**: 0
- **Won't Fix**: 1
- **Next Bug Number**: 073

## Quick Links
- [Bug Tracking Policy](BUG_TRACKING_POLICY.md)
- [Bug Template](TEMPLATE.md)
- [Open Bugs](open/)
- [Closed Bugs](closed/)
- [Fixed Bugs](fixed/)

## Repro Shortcuts
- #071: `/tmp/frame_transpiler_repro/bug_071/{minimal_try_except.frm, minimal_async_try_except.frm, harness.frm}`, scripts `run_import_minimal.sh`, `run_import_async.sh`, `run_import_harness.sh`
- #070: `/tmp/frame_transpiler_repro/bug_070/runtime_protocol.frm`, `/tmp/frame_transpiler_repro/bug_070/run_check_handlers.sh`
- #069: `/tmp/frame_transpiler_repro/bug_069/minimal_dup_iface.frm`, `/tmp/frame_transpiler_repro/bug_069/runtime_protocol.frm`
- #067: `/tmp/frame_transpiler_repro/bug_067/minimal_actions_missing.frm`, `/tmp/frame_transpiler_repro/bug_067/multiple_actions.frm`, `/tmp/frame_transpiler_repro/bug_067/run_validate.sh`
- #068: `/tmp/frame_transpiler_repro/bug_068/minimal_ts_import.frm`, `/tmp/frame_transpiler_repro/bug_068/run.sh`
- #061: `/tmp/frame_transpiler_repro/bug_061/minimal_py.frm`, `/tmp/frame_transpiler_repro/bug_061/run.sh`
- #062: `/tmp/frame_transpiler_repro/bug_062/minimal_py.frm`, `/tmp/frame_transpiler_repro/bug_062/run.sh`
- #063: `/tmp/frame_transpiler_repro/bug_063/minimal_py.frm`, `/tmp/frame_transpiler_repro/bug_063/run.sh`

## Active Bugs (Open)

| Bug # | Title | Priority | Category | Status | Assignee |
|-------|-------|----------|----------|--------|----------|
| (none) |  |  |  |  |  |

## Fixed (awaiting closure by owning team)

| Bug # | Title | Fixed Version |
|-------|-------|----------------|
| [#069](fixed/bug_069_runner_relative_framec_path_cwd_failure.md) | Runner fails when CWD changes (relative framec path) | v0.86.37 |
| [#070](fixed/bug_070_python_module_compile_drops_handlers_for_complex_frm.md) | Python module compile drops handlers/actions for complex FRM | v0.86.39 |
| [#072](fixed/bug_072_python_action_call_prefix_mismatch.md) | Python action call prefix mismatch (self.log vs _action_log) | v0.86.41 |

## Recently Closed

- #071: Python actions with try/except/async emit invalid code — `fixed_version: v0.86.40`

