# Legacy Test Audit — Conversion/Retirement Plan (Python/TypeScript)

Scope: Non‑V3 test directories under `framec_tests/language_specific/*/*` that are not `v3_*`.

Summary (counts)

Python
- control_flow: 3
- core: 9
- data_types: 3
- event_handler_incremental: 7
- external_apis: 3
- negative: 2
- operators: 1
- regression: 4
- scoping: 13
- systems: 200
- torture: 3

TypeScript
- control_flow: 13
- core: 17
- data_types: 1
- declarations: 1
- external_apis: 3
- islands: 26
- operators: 1
- regression: 5
- runtime: 1
- scoping: 13
- systems: 200
- torture: 3

Other
- typescript_swc (core/promises/events/fs): 4 total (legacy SWC demos)
- llvm/basic: 18 (LLVM is on hold)

Policy
- V3 architecture is authoritative. We convert only fixtures that cover behaviors not already represented in V3 categories (prolog/imports/outline/closers/mir/mapping/expansion/validator and curated exec for core/control_flow/systems).
- Legacy tests that duplicate V3 coverage or rely on legacy semantics are retired.
- Tests requiring non‑hermetic toolchains in core phases remain retired; exec runs use curated harnesses only.

Recommended actions

Convert (targeted)
- python/typescript external_apis: defer to FID/Project work (Stage 12). Keep a short list of 2–3 API‑shape exemplars for future conversion.
- python/typescript regression: inspect messages; where unique policies are exercised (e.g., specific error shapes), convert into `v3_validator` negatives; otherwise retire.
- python/typescript scoping: if any fixture probes SOL detection beyond current V3 scoping coverage, convert up to 2–3; retire the rest.

Retire (safe to remove; V3 covers)
- python/typescript systems (200 each): superseded by V3 curated exec and validator suites.
- python/typescript core/control_flow/data_types/operators: covered by V3 categories with broader cases.
- python event_handler_incremental: superseded by V3 outline/validator tests for handlers‑in‑state and terminal‑last.
- python/typescript torture: non‑hermetic and not aligned to V3.
- typescript islands/runtime/declarations: pre‑V3 experiments; retire.
- typescript_swc: superseded by `v3_facade_smoke` and `native-ts` diagnostics mapping.
- llvm/basic: LLVM is on hold.

Process
1) Inventory (done): scripts/audit_legacy_tests.py lists legacy directories and counts.
2) Selection: for each language, pick at most 2–3 fixtures from external_apis/regression/scoping with unique value and convert them to V3 categories.
3) Retirement: move remaining legacy directories to `framec_tests/retired_legacy/<lang>/<category>` with a README noting the rationale and replacement coverage. Runner already ignores non‑V3 categories.
4) CI/Docs: update this audit as we convert. Keep conversion minimal; prefer V3 equivalents.

Notes
- No functional impact: legacy directories are not part of normal V3 runs. This audit formalizes their status and guides minimal, high‑signal conversions only.

