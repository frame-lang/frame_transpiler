# Stage 1 — Module Partitioning (ModulePartitionerV3)

Purpose
- Produce a deterministic outline of a `.frm` source file and exact byte ranges for every `{ … }` body, using per‑target textual body closers.
- Eliminate downstream “brace ownership” ambiguity; all later stages trust the recorded body bounds.

Inputs
- Source bytes (`&[u8]`), file path, inferred or declared `Target` (from `@target` prolog).

Outputs
- `ModulePartitions` with:
  - `prolog`: span of `@target` and related metadata
  - `imports`: one or more contiguous native import partitions (target‑specific text)
  - `outline`: Frame outline partition (systems, blocks, headers)
  - `bodies: Vec<BodyPartition>`
- `BodyPartition`:
  - `owner_id` (system/member identity for linking)
  - `open_byte`, `close_byte` (inclusive `{` / matching `}` positions)
  - `target`
  - `byte_to_line_index` (optional precomputed index for diagnostics; lines unused in algorithms)

Invariants
- Single source of truth for body end: per‑target textual closers (DPDA) determine `close_byte`.
- No re‑closing downstream; parser and scanners consume within `[open_byte+1, close_byte)`.
- The `@target` prolog is the first non‑whitespace token; comments before prolog are disallowed by policy.

Algorithm
- Scan the file once to locate prolog and outline tokens.
- When encountering a member header with `{`, dispatch to the per‑target BodyCloser to find the matching `}`:
  - Python closer: see `01_body_closers_python.md`.
  - TypeScript closer: see `01_body_closers_typescript.md`.
- Record the `BodyPartition` with exact byte offsets.

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
- `impl ModulePartitionerV3 { fn partition(src: &[u8], path: &Path) -> Result<ModulePartitions> }`
- `trait BodyCloser { fn close(src: &[u8], open_idx: usize) -> Result<usize>; }`

Test Hooks
- Golden outlines for files with nested modules, multiple bodies, and complex strings/templates in bodies.
- Negatives for unterminated quotes/templates; stray braces; bad prolog.

Integration
- Downstream scanners receive `BodyPartition` and operate only within `[open_byte+1, close_byte)`.

