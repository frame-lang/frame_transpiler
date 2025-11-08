# Test Index Policy

- This repository maintains an index of tests and expected outcomes per active target language.
- We no longer maintain tests in a fictional/common hybrid syntax. All fixtures must be native per target language under `framec_tests/language_specific/<language>/...`.
- The legacy `common/tests/` set is being phased into a small, Frame‑semantics spec core. Language‑specific suites are authoritative for bodies.
- LLVM target is on indefinite hold. Do not add or maintain LLVM fixtures. Keep the index up to date, but do not track LLVM expectations.

## Index file

- Path: `framec_tests/TEST_INDEX.json`
- Structure (per test, per language):
  - `transpile`, `validate`, `execute` (boolean expected outcomes)
  - `negative` (boolean: test is expected to fail at validation/transpile)
  - `infinite` (boolean: test is intended to run indefinitely and is not executed)
  - `notes` (optional string)

Example (abbreviated):

```
{
  "metadata": {
    "version": 1,
    "active_languages": ["python", "typescript"],
    "on_hold": ["llvm"],
    "policy": "All fixtures are target‑native."
  },
  "tests": {
    "control_flow/test_simple_hsm_loop.frm": {
      "python": {"transpile": true, "validate": true, "execute": true, "negative": false, "infinite": false},
      "typescript": {"transpile": true, "validate": true, "execute": true, "negative": false, "infinite": false}
    }
  }
}
```

## Keeping the index current

- Use the runner flags to read and optionally update the index after a run:
  - `--index framec_tests/TEST_INDEX.json` to load and compare
  - `--update-index` to write actual results back into the index
- The index is the source of truth for expected outcomes; CI can compare actual to expected to detect regressions.

