# Release Notes — v0.86.46

**Date**: 2025-11-17  
**Type**: bug-fix / V3 runtime glue

## Summary

This release advances the V3 “going native” runtime for Python and TypeScript
by threading system/state parameters into the generated runtimes and tightening
the semantics around `$enter` dispatch. It keeps the DPDA scanners and MIR
semantics unchanged while improving module-level codegen.

## Changes

- **Python V3 runtime glue**
- `compile_module` (formerly `compile_module_demo`) now:
    - Uses Arcanum to choose the first declared state as the start state for
      the initial `FrameCompartment` instead of hard-coding `A`.
    - Parses `system` parameter lists (`$(start)`, `$>(enter)`, `domain`) via
      `parse_system_params` and partitions constructor arguments accordingly.
    - Seeds `state_args` and applies domain parameters as attributes on the
      generated system class.
    - Calls `_frame_router` with a synthetic `$enter` event on construction and
      on `_frame_transition`, aligning with the documented runtime model.
  - `PyExpanderV3` now correctly assigns transition `exit_args` on the newly
    constructed `FrameCompartment` (`next_compartment.exit_args`) instead of
    the current `compartment`, matching the abstract semantics.

- **TypeScript V3 runtime glue**
  - TypeScript module emitter now:
    - Reuses `parse_system_params` and `find_start_state_name` to partition
      flattened system parameters and choose the textual first state as the
      start state.
    - Emits a constructor that:
      - Splits `sysParams` into `startArgs`, `enterArgs`, and `domainArgs`.
      - Builds `stateArgs` from `startArgs` keyed by parameter name.
      - Applies domain arguments as overrides on matching fields when present.
      - Seeds `_compartment` as `new FrameCompartment('__Sys_state_Start', enterArgs, undefined, stateArgs)`
        and immediately fires a `$enter` event through `_frame_router`.
    - Updates `_frame_transition` to update `_compartment` and fire a `$enter`
      event for the new compartment.
    - Adds a simple `_stack` field plus `_frame_stack_push`/`_frame_stack_pop`
      that mirror the Python behavior.

- **Validation and tests**
  - `cargo build --release` passes with the new runtime glue.
  - Python V3 suites:
    - `python3 framec_tests/runner/frame_test_runner.py --languages python --categories all_v3 v3_capabilities --framec ./target/release/framec --transpile-only`
    - Result: 119/119 tests passing across 10 V3 categories.
  - TypeScript V3 suites:
    - `python3 framec_tests/runner/frame_test_runner.py --languages typescript --categories all_v3 v3_capabilities --framec ./target/release/framec --transpile-only`
    - Result: 109/109 tests passing across 9 V3 categories.

## Notes

- `system.return` and `system.method` placement rules remain enforced by
  semantic validation (E406/E407). Full runtime wiring of `system.return`
  remains partially implemented and will be finalized in a future release.
- The Python and TypeScript runtimes still treat some aspects of the abstract
  runtime (e.g., parent-forward and rich stack semantics) as stubs; these are
  intentionally documented as evolving in `architecture_v3/frame_runtime.md`.
