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
- Source Maps & Codegen: breakpoint alignment and golden maps for representative fixtures.
- Validation: negatives per rule (terminal‑last; no Frame statements in actions/ops; per‑language native policies).

End‑to‑End
- Use `framec_tests` language‑specific suites in transpile‑only mode as the validation gate.
- V3 fixtures per language live under `language_specific/<lang>/{v3_prolog,v3_imports,v3_outline,v3_demos}` with positive and negative cases.

Performance
- Add budget checks for worst‑case fixtures (large triple‑quotes/templates) to guarantee O(n) behavior and no stalls.

CI Integration
- Run all 7 language suites for V3 categories on meaningful changes to scanning/segmentation/MIR/validator/expander. LLVM is out of scope (on hold).
