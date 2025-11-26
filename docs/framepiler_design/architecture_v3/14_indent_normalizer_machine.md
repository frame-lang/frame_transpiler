# Stage 14 — Python Indent Normalizer Machine (Self‑Hosting)

Purpose
- Replace ad‑hoc Rust string logic for Python handler indentation with a Frame system that implements the same algorithm and can be compiled to Rust.
- Make indentation rules explicit, testable, and reusable across V3 components.

Inputs
- A sequence of logical handler lines, each including:
  - The original line text (including leading spaces/tabs).
  - Flags derived from the MIR/expander:
    - `is_frame_expansion`: whether the line came from a Frame expansion (transition/forward/stack/system.return sugar).
    - `is_comment`: whether the trimmed line starts with `#`.
    - `is_blank`: whether the line is all whitespace.
- A guard indentation pad (`pad`), e.g. `"        "` for state‑less handlers, `"            "` for state‑guarded handlers.

Outputs
- A normalized sequence of Python lines such that:
  - Relative nesting is preserved (nested `def` / `if` / `else` blocks remain valid).
  - Frame expansion lines are aligned with the surrounding block indent instead of inheriting spurious extra indent from the source.
  - The first statement after a colon (`:`) is indented one level deeper than the colon line.
  - Handler sugar is applied:
    - `system.return` → `self._system_return_stack[-1]`.
    - `return expr` → `self._system_return_stack[-1] = expr; return`.
  - Comment‑only bodies emit a `pass` at the normalized pad.

High‑Level Algorithm
- Track:
  - `base_indent`: minimal non‑blank indent across the body, computed once per handler.
  - `prev_indent_norm`: last logical normalized indent.
  - `last_line_ended_with_colon`: whether the previous logical line ended with `:`.
- For each input line:
  - If `is_blank`: emit `pad + "\n"`, do not change tracking state.
  - Rewrite `system.return` in the text before further processing.
  - Compute `indent_orig` from leading spaces/tabs; `trimmed` from `line[indent_orig..].lstrip()`.
  - If `trimmed` is empty: emit `pad + "\n"`.
  - Determine whether the line is a Frame expansion:
    - `is_frame_expansion` flag from the host (expander) covers:
      - `next_compartment = FrameCompartment(...)`.
      - `_frame_transition`, `_frame_stack_push`, `_frame_stack_pop`, parent forward calls.
  - Choose `indent_norm`:
    - If `last_line_ended_with_colon`: `prev_indent_norm.unwrap_or(base_indent) + 4`.
    - Else if `is_frame_expansion`: `prev_indent_norm.unwrap_or(indent_orig)`.
    - Else: `indent_orig`.
    - Clamp `indent_norm >= base_indent`.
  - Compute `extra = " " * (indent_norm - base_indent)`.
  - Apply `return expr` sugar:
    - If `trimmed` starts with `return ` and has a non‑empty expression, emit:
      - `pad + extra + "self._system_return_stack[-1] = " + expr`
      - `pad + extra + "return"`
      - Update `prev_indent_norm = indent_norm`, `last_line_ended_with_colon = false`, continue.
  - Otherwise, emit `pad + extra + trimmed`, update `prev_indent_norm = indent_norm`, `last_line_ended_with_colon = trimmed.endswith(':')`.
- After all lines:
  - If the body contained no non‑comment statements, emit `pad + "pass"`.

Machine Sketch (Frame System)
- System: `IndentNormalizer`
  - Domain:
    - `lines: string[]` — input lines.
    - `flags_is_expansion: bool[]`
    - `flags_is_comment: bool[]`
    - `pad: string`
    - `base_indent: int`
    - `prev_indent_norm: int`
    - `has_prev_indent: bool`
    - `last_line_ended_with_colon: bool`
    - `has_non_comment: bool`
    - `out_lines: string[]`
    - `i: int` — current line index.
  - Interface:
    - `run(lines, flags_is_expansion, flags_is_comment, pad): string[]`
      - Seeds domain state and drives the machine; returns `out_lines`.
  - Machine:
    - `$ComputeBase`:
      - Iterate over `lines`, compute `base_indent` (minimal leading whitespace count on non‑blank lines).
      - Transition to `$Normalize`.
    - `$Normalize`:
      - For each line `i`:
        - Apply the algorithm above, append to `out_lines`.
      - After last line:
        - If `!has_non_comment`, append `pad + "pass"`.
      - Transition to `$Done`.
    - `$Done`:
      - `run(...)` returns `out_lines`.

Integration Plan
- Phase A (tests only):
  - Implement `IndentNormalizer` in a `.frs` file under `framec_tests/language_specific/rust/v3_internal/` with `@target rust`.
  - Add a small Rust harness (or Python/TS harness calling Rust) that:
    - Feeds a hard‑coded handler body (e.g., the `stopOnEntry` or `PythonDebugRuntime` cases) into `run(...)`.
    - Asserts that the normalized lines match the current `emit_py_handler_body` output in `mod.rs`.
  - Keep the existing Rust indentation helper as the production path; the machine is used only by tests.
- Phase B (compiler integration):
  - Generate Rust for `IndentNormalizer` as part of the build or vendored runtime.
  - Replace the ad‑hoc `emit_py_handler_body` logic in `mod.rs` by:
    - Computing the per‑line flags (`is_frame_expansion`, `is_comment`, etc.) in Rust.
    - Calling the generated Rust machine to normalize indentation and sugar per handler.
  - Maintain DFAs/DPDAs as the authoritative front‑end; the machine operates purely on the already‑segmented line stream.

Notes
- The machine is intentionally line‑oriented and stateless with respect to Python syntax beyond indentation and colons; all Python syntax awareness (strings, comments, etc.) remains in the body closers and native scanners.
- This keeps Stage 14 as a self‑hosting, DPDA‑style transformer that can be compiled and tested like any other Frame system, while respecting the existing scanning/lexing/parsing invariants.

