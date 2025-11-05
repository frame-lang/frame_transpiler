# Framepiler “Island Architecture” Plan (TypeScript First)

Document version: 1.0  
Status: Proposed (execute on branch `ts_interleaver_mvp`)  
Scope: Scanning/Parsing + Visitor emission for TypeScript using OutlineScanner + Interleaver. CLI/FID deferred until parser is stable.

## Intent

Build the new pipeline incrementally and deterministically. We start from the smallest, self‑contained “islands” (empty systems/functions/operations), validate each, then expand surface area (native lines, transitions, forwards, state‑stack ops, control flow, strings/templates, comments) until we cover the real‑world patterns needed for Bug 055.

No hacks, no shortcuts. Every step includes concrete validation with focused fixtures and explicit exit criteria.

## Ground Rules

- Stay on branch `ts_interleaver_mvp` (based on `old_parsers`).
- Do not modify CLI/FID during Phases I–III to keep the work tight. (FID comes after parser stability.)
- Use native comments and native import syntax; no body fences.
- Preserve `{}` on every line and maintain deterministic top‑level segmentation.
- Keep tests hermetic and minimal; each adds exactly one new concept.

## How We Validate

- Build: `cargo build --release`
- Transpile only: `python3 framec_tests/runner/frame_test_runner.py --languages typescript --transpile-only --framec ./target/release/framec --categories language_specific_typescript_islands`
- Spot run a single test by file path if useful (same runner, but restrict to that file’s category or path).
- Success = transpiler exit code 0 and generated `.ts` written under `framec_tests/generated/typescript/`.

## Test Fixture Convention

- Category root: `framec_tests/language_specific/typescript_islands/`
- Name tests by concept and order them numerically to make progress obvious.
- Each test starts with `@target typescript` and focuses on one new behavior.

---

## Progress

- Phase I (Skeletons): [3/4] tasks complete (OutlineScanner pending)
- Phase II (Interleaver Basics): [0/5]
- Phase III (Frame Stmts in TS bodies): [0/5]

## Phase I — Skeletons (no native code yet)

Goal: Prove we can parse minimal systems/operations/actions/domain with empty bodies and emit compilable TS skeletons through the new pipeline.

1. I‑1: Empty system skeleton
- Fixture: `typescript_islands/01_empty_system.frm`
- Contents:
  ```
  @target typescript
  system Empty {
      operations:
      interface:
      machine:
      actions:
      domain:
  }
  ```
- Tasks:
  - [ ] OutlineScanner recognizes headers and block order
  - [x] Parser produces AST for an empty system (no states, no members)
  - [x] Visitor emits a minimal TS module (runtime prelude + class or container as current visitor dictates)
- Acceptance:
  - [x] Transpile succeeds; output file non‑empty

2. I‑2: Empty operation
- Fixture: `typescript_islands/02_empty_operation.frm`
- Contents:
  ```
  @target typescript
  system OpsOnly {
      operations:
      op1() {}
      interface:
      machine:
      actions:
      domain:
  }
  ```
- Tasks:
  - [x] Parser accepts an operation with empty braces
  - [x] Visitor emits a method stub in TS
- Acceptance:
  - [x] Transpile succeeds; TS contains an op1 method

3. I‑3: Empty action
- Fixture: `typescript_islands/03_empty_action.frm`
- Contents:
  ```
  @target typescript
  system ActionsOnly {
      operations:
      interface:
      machine:
      actions:
      doNothing() {}
      domain:
  }
  ```
- Tasks:
  - [x] Parser accepts an empty action body
  - [x] Visitor emits a callable action wrapper + body
- Acceptance:
  - [x] Transpile succeeds; action wrapper present; no unreachable‑return diagnostics

4. I‑4: Empty machine (no states)
- Fixture: `typescript_islands/04_empty_machine.frm`
- Contents:
  ```
  @target typescript
  system MachineOnly {
      operations:
      interface:
      machine:
      actions:
      domain:
  }
  ```
- Tasks:
  - [x] Parser tolerates machine without states (existing validations apply)
- Acceptance:
  - [x] Transpile succeeds; no state code emitted beyond boilerplate

5. I‑5: Domain skeleton (no fields)
- Fixture: `typescript_islands/05_empty_domain.frm`
- Contents:
  ```
  @target typescript
  system DomainOnly {
      operations:
      interface:
      machine:
      actions:
      domain:
  }
  ```
- Tasks:
  - [x] Parser handles empty domain block
- Acceptance:
  - [x] Transpile succeeds

Exit Criteria for Phase I
- All five fixtures transpile successfully using the new OutlineScanner path (even if the interleaver yields empty segments).

---

## Phase II — Interleaver Basics (native first, no Frame statements)

Goal: Prove the TypeScript body interleaver can retain native code verbatim and keep structural correctness (braces, strings, comments).

6. II‑1: Single native statement
- Fixture: `typescript_islands/06_native_single_line.frm`
- Contents:
  ```
  @target typescript
  system NativeLine {
      operations:
      op1() {
          const x = 1;
      }
      interface:
      machine:
      actions:
      domain:
  }
  ```
- Tasks:
  - [ ] Interleaver segments body to one NativeSegment
  - [ ] Preserve braces and output the line unchanged
- Acceptance:
  - [ ] Transpile succeeds; TS has `const x = 1;` inside op1()

7. II‑2: Multi‑line native block (if/else)
- Fixture: `typescript_islands/07_native_if_else.frm`
- Adds:
  ```
  if (x > 0) { x++; } else { x--; }
  ```
- Acceptance: 
  - [ ] Transpile succeeds; emitted TS matches input structure

8. II‑3: Native comments (// and /* ... */)
- Fixture: `typescript_islands/08_native_comments.frm`
- Adds both comment styles; ensure interleaver does not misclassify.
- Acceptance: 
  - [ ] Transpile succeeds; comments appear verbatim

9. II‑4: Strings and escapes
- Fixture: `typescript_islands/09_strings_escapes.frm`
- Includes single/double quoted strings with escapes.
- Acceptance: 
  - [ ] No splitting inside strings; verbatim emission

10. II‑5: Template literals with ${…}
- Fixture: `typescript_islands/10_template_literals.frm`
- Includes nested `${ a ? `${b}` : c }`.
- Acceptance: 
  - [ ] Depth tracking stable; verbatim emission

Exit Criteria for Phase II
- [ ] All five fixtures pass, demonstrating robust NativeSegment handling across common TS constructs

---

## Phase III — Frame Statements at Top Level (inside TS bodies)

Goal: Introduce Frame statements one at a time and confirm glue emission + early return semantics work.

11. III‑1: Single transition
- Fixture: `typescript_islands/11_transition_basic.frm`
- Body contains only: `-> $Start` (or another simple state).
- Acceptance:
  - [ ] Transpile succeeds; visitor emits transition glue and early return

12. III‑2: Parent forward
- Fixture: `typescript_islands/12_forward_parent.frm`
- Body contains only: `=> $^`
- Acceptance:
  - [ ] Transpile succeeds; forward glue emitted

13. III‑3: State stack push/pop
- Fixture: `typescript_islands/13_stack_push_pop.frm`
- Body contains only: `-> $$[+]` then `-> $$[-]` in separate handlers.
- Acceptance:
  - [ ] Transpile succeeds; stack helpers wired; early return maintained

14. III‑4: Interleaved native + one Frame stmt
- Fixture: `typescript_islands/14_interleave_one_frame_stmt.frm`
- Example body:
  ```
  const a = 1;
  -> $S1
  ```
- Acceptance:
  - [ ] Native lines emitted verbatim; transition glue correct and returns

15. III‑5: Multiple Frame stmts interleaved
- Fixture: `typescript_islands/15_interleave_multi_frame_stmts.frm`
- Acceptance:
  - [ ] All transitions/forwards push/pop emitted correctly, with returns at the right places

Exit Criteria for Phase III
- [ ] All fixtures pass; top‑level Frame statements are correctly recognized and emitted alongside native code

---

## Phase IV — Control Flow & Nesting Stress

Goal: Demonstrate segmentation remains correct across complex native constructs while still recognizing Frame statements at top level only.

16. IV‑1: Loops (for/while)
- Fixture: `typescript_islands/16_loops.frm`
- Acceptance: Native loops intact; no mis‑segmentation.

17. IV‑2: Try/catch/finally
- Fixture: `typescript_islands/17_try_catch.frm`
- Acceptance: Verbatim emission; Frame statements still recognized at top level only.

18. IV‑3: Async/await
- Fixture: `typescript_islands/18_async_await.frm`
- Acceptance: Interleaver tolerant to `async function` and `await` usage; emission correct.

19. IV‑4: Mixed braces on single line
- Fixture: `typescript_islands/19_mixed_braces_single_line.frm`
- Acceptance: Depth tracking correct even with `}{` patterns in literals or tight formatting.

20. IV‑5: Comments containing Frame tokens
- Fixture: `typescript_islands/20_comments_containing_frame_tokens.frm`
- Include `// -> $S` and `/* => $^ */`; ensure they are not parsed as Frame statements.

Exit Criteria for Phase IV
- All fixtures pass; segmentation is robust against realistic code patterns.

---

## Phase V — Diagnostics & Source Maps

Goal: Add dual mapping and confirm error reporting pinpoints both Frame and target lines.

21. V‑1: Source map spans on segments
- Unit tests attach and assert spans for `NativeSegment` and `FrameStmt`.
- Acceptance: AST dump (or equivalent) shows spans; unit asserts match expected lines.

22. V‑2: Error line reporting
- Negative test where a Frame stmt appears in an invalid context; confirm diagnostic contains Frame and target line numbers.
- Acceptance: Runner marks test as negative and passes; diagnostics verified.

---

## Phase VI — Regression: Realistic Mini‑Specs

Goal: Combine multiple concepts into small but realistic systems.

23. VI‑1: Minimal protocol action
- Fixture: `typescript_islands/23_min_protocol.frm`
- Contains: a native import, a simple action with native lines + a single transition.
- Acceptance: Transpile succeeds; emitted TS compiles in `tsc` (optional for this phase).

24. VI‑2: Basic event handling with interleave
- Fixture: `typescript_islands/24_basic_event_interleave.frm`
- Acceptance: Transpile succeeds; glue + native preserved.

25. VI‑3: Socket‑like skeleton (compile‑only)
- Fixture: `typescript_islands/25_socket_skeleton.frm`
- Acceptance: Transpile succeeds; no runtime execution required yet.

---

## Phase VII — FID (post‑parser stabilization)

- Reintroduce the FID loader and manifest once the parser and visitor are stable (separate plan document exists).
- Use compile‑only sockets fixture to validate symbol discovery.

---

## Exit Criteria for the Workstream

- All Phase I–IV fixtures pass via the runner in transpile‑only mode.
- Phase V diagnostics/source maps assertions pass.
- Phase VI mini‑specs compile (transpile‑only), optional tsc pass is green where configured.

## Rollback & Safety

- Keep changes localized (scanner_outline.rs + parser interleaver + TS visitor emission paths).
- If a step regresses earlier fixtures, stop and fix before proceeding.
- No commits without review; stage diffs for inspection at each phase boundary.

---

This plan enumerates every step from empty skeletons to interleaved bodies with robust segmentation and diagnostics. Once complete, we will proceed to FID integration and then revisit Python parity using the same OutlineScanner model.
