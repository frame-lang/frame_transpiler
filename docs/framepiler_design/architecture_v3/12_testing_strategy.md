# Testing Strategy (V3)

Goals
- Ensure determinism and correctness per stage with hermetic, compile‑only tests where runtime dependencies exist. Keep tests O(n) in fixture size.

Per‑Stage Tests
- Partitioning: golden partitions for complex bodies; negatives for unterminated strings/templates and stray braces.
- Body closers: focused unit tests for DPDA state transitions; edge cases for quotes/templates across many lines.
- Segmentation: fixtures with Frame‑statement‑like lexemes inside strings/comments; unicode whitespace at SOL; mixed native/Frame‑statement sequences.
- Frame Statement Parser: transitions with nested parentheses and strings; invalid/malformed Frame statements.
- MIR Assembly: terminal Frame statement enforcement; mapping preservation.
- Directive Expansion: nested conditionals; ensure expansions do not break `elif/else/except/finally` or `else if` chains.
- Splice & Mapping: round‑trip mapping checks; consecutive Frame statements; boundary at body start/end.
- Native Parse (optional): syntax errors mapping back to Frame spans; policy enforcement diagnostics.
- Source Maps & Codegen: breakpoint alignment and golden maps for representative fixtures.
- Validation: negative fixtures per rule (Python/TS policies; terminal last; disallowed Frame statements in actions/ops).

End‑to‑End
- Use the existing `framec_tests` language‑specific suites in transpile‑only mode as the default validation gate. Expand with new V3 fixtures under `language_specific_{python,typescript}`.

Performance
- Add budget checks for worst‑case fixtures (large triple‑quotes/templates) to guarantee O(n) behavior and no stalls.

CI Integration
- Run Python and TS suites on every meaningful change touching scanning/segmentation/MIR/visitors. LLVM is out of scope (on hold).
