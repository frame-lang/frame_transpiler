# Framepiler Architecture (V3) — Authoritative Overview

Purpose
- Provide a single, up‑to‑date overview of the V3 architecture that the transpiler, debugger, and runtime teams can rely on. This page links to stage‑specific specs under `architecture_v3/` and includes a concise glossary and policies.

Core Principles
- Deterministic scanning/parsing only. No regex for language syntax. All per‑language scanners are DPDA‑style and aware of strings/comments/templates.
- SOL‑only Frame statements. Frame tokens are recognized only at start of line (leading whitespace allowed) in handler bodies. They are ignored inside strings/comments/templates.
- MixedBody/MIR is authoritative. Frame semantics (Transition/Forward/Stack) are carried as MIR items; native code remains native.
- Terminal rule. Transition is terminal within its containing block; forward/stack are non‑terminal.
- Hermetic by default. Native parser integrations are pure‑Rust and feature‑gated. Builds and tests avoid external toolchains.

Pipeline (Module path)
1) Partition module into imports/prolog/bodies with brace closers per language.
2) Native region scan per body to find SOL‑anchored Frame statements safely.
3) Frame statement parse (balanced parens; strict heads/args).
4) MIR assembly with spans and terminal checks.
5) Validation (E‑codes) including terminal‑last, disallow Frame in actions/ops, unknown state, parent‑forward policy.
6) Expansion to target code (per language), then splice with origin mapping.
7) Optional debug trailers (errors‑json, frame‑map, visitor‑map, debug‑manifest, native‑symbols).

Validation (selected E‑codes)
- E400: Transition must be last in its containing block.
- E401: Frame statements are not allowed in actions/operations.
- E402: Unknown state (Arcanum symbol table backed in module path).
- E403: Forward to parent requires declared parent (module demos).
- E404: Handler found outside a state block.
- E405: Advisory state parameter arity mismatch (flag‑gated).

Symbol Table (Arcanum)
- Built from the module outline (systems/machines/states with optional parents and state param lists).
- Drives E402/E403 resolution in module validation. Remains the source of truth for Frame‑side symbols.

Debug/Mapping Artifacts
- errors‑json: structured diagnostics (stable `code` + `message`), gated by `FRAME_ERROR_JSON=1` or `--emit-debug`.
- frame‑map: high‑level origin map from splice, gated by `FRAME_MAP_TRAILER=1` or `--emit-debug`.
- visitor‑map v2: line+column mapping for module path (Py/TS today), emitted with frame‑map in module mode.
- debug‑manifest v2: system name, states (compiled IDs), and handlers (names, compiled IDs, params).
- native‑symbols (advisory): parser‑backed snapshots of handler params (flag‑gated).

Native Parser Policy (Hermetic)
- Default‑on pure‑Rust parsers where pinned: Python (RustPython), TypeScript (SWC), Rust (syn). Others are feature‑gated (tree‑sitter family) until pinned.
- Parsers are advisory aids (diagnostics, snapshots) and never override Frame semantics.

Glossary
- SOL: Start‑of‑line. Frame statements only recognized at SOL in handler bodies.
- DPDA: Deterministic pushdown automaton; describes our string/comment/template‑aware scanners.
- MixedBody: Interleaved sequence of native spans and Frame statements with spans.
- MIR: Minimal intermediate representation of Frame statements (Transition, Forward, Stack) with spans.
- Arcanum: Module‑level symbol table for systems/machines/states (+ parents, params).
- Splice Map: Mapping from spliced target code back to source (frame/native) spans.
- Visitor Map: Derived line/column mapping used by debuggers for stepping/breakpoints.
- Debug Trailers: JSON payloads embedded in generated code between sentinel comments for hermetic extraction.
- Facade Mode: Native parse adapters used to surface mapped native diagnostics; wrappers used for exec smoke.

Status & Links
- Stage specs and plans live under `docs/framepiler_design/architecture_v3/`.
  - Overview: architecture_v3_overview.md
  - Stage index: 00_stage_index.md
  - Validation: 09_validation.md
  - AST & Symbols: 10_ast_and_symbol_integration.md
  - Error taxonomy: 11_error_taxonomy.md
  - Testing strategy: 12_testing_strategy.md
  - PLAN (status/roadmap): PLAN.md

How To (Quick)
- Compile module (non‑demo): `framec compile -l python_3 --emit-debug path/to/module.frm > out.py`
- Compile project: `framec compile-project -l typescript -o out_dir path/to/dir --validation-only`
- Emit advisory native snapshots: set `FRAME_NATIVE_SYMBOL_SNAPSHOT=1` (Py/TS).

Notes
- Demos remain for hermetic exec smoke; production uses `compile`/`compile-project`.
- Keep changes deterministic and documented; update stage docs upon policy changes.
- Rust default: StateId enum is default‑on for module compile; set `FRAME_RUST_STATE_ENUM=0` to fall back to string state IDs.
