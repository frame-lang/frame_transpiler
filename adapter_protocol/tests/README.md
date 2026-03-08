# Adapter Protocol Verification Tests

This directory is the home for **verification tests** for the shared adapter
environment. These are stable tests that encode the semantics both teams have
agreed on, independent of how individual bugs were originally discovered.

## Relationship to Bug Tests

- Bug-specific repros live in `bug/bugs/bug_NNN_*.md` and (optionally)
  `bug/tests/bug_NNN/` as ad hoc scripts or fixtures.
- Once the expected behavior is clear, it should be captured here as one or
  more verification tests so regressions are caught automatically.
- Bugs should list:
  - Their repro command(s) under **Reproduction Steps** /
    **Build/Release Artifacts**, and
  - The verification test(s) here under **Verification Tests** (e.g.
    “covered by `adapter_protocol/scripts/run_adapter_smoke.sh` and
    `adapter_protocol/tests/...`”).

## Current Tests

- `../scripts/run_adapter_smoke.sh` + `../scripts/node_harness.ts`
  - Smoke test validating minimal AdapterProtocol semantics:
    - Guarded commands defer before handshake/ready.
    - Ready collapses deferred queue to a single in-flight guarded command.
    - `stopped` sets `isPaused === true` and updates stopped metadata.
  - Intended as the canonical cross-team adapter smoke.

Additional tests (e.g., more detailed AdapterProtocol scenarios or multi-step
debugger flows) should be added as small, self-contained scripts or fixtures
in this directory and referenced from the relevant bugs.

