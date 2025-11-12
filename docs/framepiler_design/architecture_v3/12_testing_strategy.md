# Testing Strategy (V3)

Goals
- Ensure determinism and correctness per stage with hermetic, compile‑only tests where runtime dependencies exist. Keep tests O(n) in fixture size. All behavioral tests live in the Python runner.

Per‑Stage Tests (all 7 languages)
- Partitioning: golden partitions for complex bodies; negatives for unterminated strings/templates and stray braces.
- Body closers: DPDA state edge cases (quotes/templates/verbatim/raw/interp/preprocessor).
- Segmentation: SOL‑lexeme negatives inside strings/comments; unicode whitespace at SOL; mixed native/Frame‑statement sequences.
- Frame Statement Parser: transitions with nested parentheses and strings; invalid/malformed Frame statements.
- MIR Assembly: terminal Frame statement enforcement; mapping preservation.
- Frame Statement Expansion: nested conditionals; ensure expansions do not break `elif/else/except/finally` (Py) or `else if` (TS/Java/C#) chains.
- Splice & Mapping: round‑trip mapping checks; consecutive Frame statements; boundary at body start/end.
- Native Parse (runtime‑optional): syntax errors mapping back to Frame spans; indentation/format diagnostics.
  - Adapters (feature‑gated): `native-ts`, `native-rs`, `native-c`, `native-cpp`, `native-java`, `native-csharp`.
- Source Maps & Codegen: breakpoint alignment and golden maps for representative fixtures.
- Validation: negatives per rule (terminal‑last; no Frame statements in actions/ops; per‑language native policies).

End‑to‑End
- Use `framec_tests` language‑specific suites as the primary gate. By default, most V3 suites are build/validate‑only (transpile‑only + structural checks).
- Executable facade strict tests: `v3_facade_smoke` includes per‑language harnesses that extract wrapper calls from spliced output and run them with no‑op wrappers. The runner sets `--validate-native` for this category and surfaces native diagnostics when adapters are enabled. Supported languages and tools:
  - TypeScript: tsc + node; optional SWC adapter (`native-ts`) for strict native parsing.
  - Python: direct execution with Python; no‑op wrappers injected. Optional Tree‑sitter adapter (`native-py`) for strict native parsing.
  - Rust: rustc; optional syn adapter (`native-rs`).
  - C/C++: clang/gcc or clang++/g++; optional Tree‑sitter adapters (`native-c`, `native-cpp`).
  - Java/C#: javac/java and csc/mcs (+mono); optional Tree‑sitter adapters (`native-java`, `native-csharp`). If toolchains are missing, execution is cleanly skipped.
- Other V3 suites (prolog/imports/outline/closers/mir/mapping/expansion/validator) remain build/validate‑only; these demos do not emit full runnable programs.

Performance
- Add budget checks for worst‑case fixtures (large triple‑quotes/templates) to guarantee O(n) behavior and no stalls.

CI Integration
- Run all 7 language suites for V3 categories on meaningful changes to scanning/segmentation/MIR/validator/expander. LLVM is out of scope (on hold).

Curated Exec (Python/TypeScript)
- Purpose: end-to-end execution for selected categories (`v3_core`, `v3_control_flow`, `v3_systems`) to validate production glue and control-flow insertion.
- Enable with runner flag `--exec-v3` and author fixtures with inline expectations:
  - Python: `# @run-expect: <regex>` lines
  - TypeScript: `// @run-expect: <regex>` lines
- The curated harness asserts expected markers in output (`TRANSITION:`, `FORWARD:PARENT`, `STACK:PUSH|POP`). It executes expansions only (no branch evaluation).
- Example:
```
python3 framec_tests/runner/frame_test_runner.py \
  --languages python typescript \
  --categories v3_control_flow v3_core v3_systems \
  --framec ./target/release/framec \
  --exec-v3 --run -v
```

JUnit Artifacts
- All V3 workflows upload JUnit XML reports suitable for CI dashboards and local inspection:
  - v3_all.yml (transpile-only): `reports/v3_all_junit.xml`
  - v3_exec_smoke.yml: `reports/v3_exec_smoke_junit.xml`
  - v3_curated_exec.yml: `reports/v3_curated_exec_junit.xml`
- Locally, request JUnit via:
```
python3 framec_tests/runner/frame_test_runner.py \
  --languages python typescript \
  --categories v3_control_flow \
  --framec ./target/release/framec \
  --exec-v3 --run \
  --validation-format junit \
  --junit report.xml
```
