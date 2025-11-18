# Frame Transpiler Bug Index

## Statistics
- **Total Bugs**: 34
- **Open**: 0
- **Fixed**: 3
- **Closed**: 31
- **Reopen**: 0
- **Won't Fix**: 0

## Quick Links
- [Bug Tracking Policy](BUG_TRACKING_POLICY.md)
- [Bug Template](TEMPLATE.md)
- [Open Bugs](open/)
- [Closed Bugs](closed/)
- [Fixed Bugs](fixed/)

## Repro Shortcuts
- #078: uses #073/#074 validators: `/tmp/frame_transpiler_repro/bug_073/run_validate.sh`, `/tmp/frame_transpiler_repro/bug_074/run_validate.sh`
- #079: `/tmp/frame_transpiler_repro/bug_079/run_validate.sh`
- #077: `/tmp/frame_transpiler_repro/bug_077/minimal_event_comment_only.frm`
- #076: `/tmp/frame_transpiler_repro/bug_076/adapter_protocol.frm`, scripts `run.sh`, `run_validate.sh`
- #075: `/tmp/frame_transpiler_repro/bug_075/minimal_redeclare_next_compartment.frm`, scripts `run.sh`, `run_validate.sh`
- #074: `/tmp/frame_transpiler_repro/bug_074/minimal_ts_emit_issues.frm`, scripts `run.sh`, `run_validate.sh`
- #073: `/tmp/frame_transpiler_repro/bug_073/ts_dup_methods.frm`, scripts `run.sh`, `run_validate.sh`
- #071: `/tmp/frame_transpiler_repro/bug_071/{minimal_try_except.frm, minimal_async_try_except.frm, harness.frm}`, scripts `run_import_minimal.sh`, `run_import_async.sh`, `run_import_harness.sh`
- #070: `/tmp/frame_transpiler_repro/bug_070/runtime_protocol.frm`, `/tmp/frame_transpiler_repro/bug_070/run_check_handlers.sh`
- #069: `/tmp/frame_transpiler_repro/bug_069/minimal_dup_iface.frm`, `/tmp/frame_transpiler_repro/bug_069/runtime_protocol.frm`
- #068: `/tmp/frame_transpiler_repro/bug_068/minimal_ts_import.frm`, `run.sh`
- #067: `/tmp/frame_transpiler_repro/bug_067/minimal_actions_missing.frm`, `/tmp/frame_transpiler_repro/bug_067/multiple_actions.frm`, `run_validate.sh`
- #063: `/tmp/frame_transpiler_repro/bug_063/minimal_py.frm`, `run.sh`
- #062: `/tmp/frame_transpiler_repro/bug_062/minimal_py.frm`, `run.sh`
- #061: `/tmp/frame_transpiler_repro/bug_061/minimal_py.frm`, `run.sh`

## Active Bugs (Open)

_(none)_

## Fixed (awaiting closure by owning team)

| Bug # | Title | Fixed Version |
|-------|-------|----------------|
| 073 | TypeScript generator emits duplicate class methods per state | v0.86.49 |
| 074 | TypeScript generator omits actions/domain declarations and drops interface params | v0.86.49 |
| 078 | TS runtime d.ts mismatch with generator calls | v0.86.49 |
