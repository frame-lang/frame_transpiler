# Stage 1 — Module Partitioning (ModulePartitionerV3)

Purpose
- Produce a deterministic outline of a `.frm` source file and exact byte ranges for every `{ … }` body, using per‑target textual body closers.
- Eliminate downstream “brace ownership” ambiguity; all later stages trust the recorded body bounds.
- Record prolog and language‑specific import partitions using SOL‑anchored, DPDA scanners (no regex).

Inputs
- Source bytes (`&[u8]`), file path, declared `Target` (from `@target` prolog). Prolog is required to be the first non‑whitespace token.

Outputs
- `ModulePartitions` with:
  - `prolog`: span of `@target` line
  - `imports`: one or more contiguous native import partitions (target‑specific text)
  - `bodies: Vec<BodyPartition>`
- `BodyPartition`:
  - `owner_id` (module artifact name; e.g., handler/action/operation)
  - `kind` (Handler | Action | Operation | Unknown)
  - `header_span` (SOL line preceding `{`)
  - `open_byte`, `close_byte` (inclusive `{` / matching `}` positions)

Invariants
- Single source of truth for body end: per‑target textual closers (DPDA) determine `close_byte`.
- No re‑closing downstream; parser and scanners consume within `[open_byte+1, close_byte)`.
- The `@target` prolog is the first non‑whitespace token; comments before prolog are disallowed by policy.

Algorithm
- PrologScannerV3 (SOL): ensure `@target <lang>` occurs as the first non‑whitespace line; record span.
- ImportScannerV3 (per‑language, SOL; DPDA; comment/string aware): record contiguous import partitions.
 - OutlineScannerV3 (SOL):
  - Recognize section lines at SOL: `interface:`, `actions:`, `operations:`, `machine:` (case‑insensitive; SOL‑anchored).
  - Within a section, detect function/handler headers as follows (Header Grammar by Section):
    - machine: bare identifier headers are allowed — `IDENT '(' … ')' '{'` (e.g., `e() { … }`).
    - actions/operations/interface: require an explicit function keyword — `fn NAME '(' … ')' '{'` or `async fn NAME '(' … ')' '{'`.
  - Only true headers (per the above) trigger the “missing '{' after module artifact header” error. Non‑headers like `print(...)` are ignored as native.
  - On a recognized header, dispatch to the per‑language BodyCloser to find the matching `}` and record `BodyPartition` with `owner_id`, `kind`, `header_span`, `open_byte`, `close_byte`.

Header Grammar by Section (authoritative)
- Sections: `interface:`, `actions:`, `operations:`, `machine:` drive header interpretation.
- machine:
  - Header: `IDENT '(' … ')' '{'` (no `fn` required)
  - Bodies inside are handlers; Frame statements are permitted at SOL within these bodies.
- actions / operations / interface:
  - Header: `fn NAME '(' … ')' '{'` or `async fn NAME '(' … ')' '{'`
  - Bodies inside are native‑only (Frame statements are disallowed by validation policy).
  - Missing `{` after a header produces a structured outline error.

Per‑Target Body Closers
- Python: triple‑quote/f‑string aware DPDA; tracks `'`, `"`, `'''`, `"""`, `#` comments; counts top‑level `{`/`}` inside the Frame shell.
- TypeScript: template/backtick aware DPDA; tracks `'`, `"`, backticks with `${}` nesting, `/*…*/`, `//` comments.

Error Handling
- Unterminated string/template: “unterminated <kind> starting here” with start span.
- Stray `}` closing brace before body opens or outside allowed depth.
- Prolog missing/invalid: “expected @target <lang> at start of file”.

Complexity
- O(n) over file length.

Interfaces (proposed)
- `struct ModulePartitionerV3;`
- `impl ModulePartitionerV3 { fn partition(src: &[u8], lang: TargetLanguage) -> Result<ModulePartitionsV3> }`
- `trait BodyCloserV3 { fn close_byte(&mut self, src: &[u8], open_idx: usize) -> Result<usize>; }`

Test Hooks
- Golden outlines for files with nested modules, multiple bodies, and complex strings/templates in bodies.
- Negatives for unterminated quotes/templates; stray braces; bad prolog.

Integration
- Downstream scanners receive `BodyPartition` and operate only within `[open_byte+1, close_byte)`.
## Addendum: Import Scanning Boundaries (Python)

Top‑level imports are only those that appear before the first module section header. Partitioning/scanning MUST stop import collection at the first occurrence of any of:

`system`, `machine`, `interface`, `actions`, `operations`, `domain` (SOL‑anchored; leading whitespace permitted).

Imports appearing inside `actions:` or state handlers are part of action/handler bodies and MUST NOT advance the outline start or be treated as module imports.

Example (imports inside actions are not top‑level imports):

```frame
@target python_3
system Example {
  actions:
    helper() {
      import sys
      self.s = sys.version
    }
  machine:
    $S { e() { self.helper() } }
}
```

The outline for `Example` begins at `system` and includes `machine`/`actions`; the `import sys` above is part of `_action_helper` and must not shift partitioning past `machine:`/`actions:`.
