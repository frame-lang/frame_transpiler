# TypeScript Body Boundary Detection (Template‑Aware)

Status: Active for operations, actions, and event handlers (guarded)
Last updated: 2025‑11‑06

## Purpose

Locate the matching closing `}` for a TypeScript body reliably without being confused by language constructs that contain braces, namely template literals with nested `${ … }`, strings with escapes, and comments.

## What “template‑aware” means

- Recognizes backtick template literals (`` `...` ``) and tracks nested template expressions `${ ... }` with a counter `tpl_expr_depth`.
- While inside a template literal, braces are treated as part of the template expression unless `tpl_expr_depth == 0` and the backtick closes.
- Braces are only counted towards body nesting when we are not inside:
  - a single‑quoted string `'...'` (with `\` escapes)
  - a double‑quoted string `"..."` (with `\` escapes)
  - a template literal `` `...` `` (unless outside `${ ... }`)
  - a block comment `/* ... */`
  - a line comment `// ...` (rest of the line is ignored)

In short, it “knows” about template strings (and other literal/comment forms) and only counts `{` and `}` at the top level of JavaScript/TypeScript code.

## Algorithm (high‑level)

- Inputs: `body_start_line` (1‑based line where `{` opens the body), `source` split by lines.
- Initialize state: `brace_depth = 1` (we already consumed `{`), `in_squote = in_dquote = in_template = in_block_comment = false`, `tpl_expr_depth = 0`.
- For each line starting at `body_start_line + 1`:
  - Scan bytes left→right and update states:
    - Enter strings on `'` or `"` when not inside another string/comment/template.
    - Inside strings: handle escapes `\\` and close on matching quote.
    - Enter template on backtick when not inside another string/comment.
    - Inside template: `${` increments `tpl_expr_depth`, `}` decrements when `tpl_expr_depth > 0`, close on backtick only when `tpl_expr_depth == 0`.
    - Enter block comment on `/*`, leave on `*/`; stop at line comment `//`.
  - Only when not in any string/comment/template:
    - `{` increments `brace_depth`.
    - `}` decrements `brace_depth` and if `brace_depth == 0`, record `close_line` and stop.
- Output: `close_line` (the line that contains the closing `}` of the body).

## Whitespace and newlines

- Handles Unix LF and Windows CRLF (`\r\n`).
- Treats NBSP (`U+00A0`) and tabs the same as other non‑structural characters.

## Tested behaviors

- Nested template literals with `${ ... }` expressions.
- Frame‑statement‑like tokens inside block/line comments are ignored.
- CRLF newlines do not affect detection.
- NBSP and tabs do not affect detection.

See unit tests: `framec/src/frame_c/parser.rs` (module `ts_textual_scan_tests`).

## Current usage and rollout

- Active: used for operations, actions, and event handlers. Guarded by backtick detection; when backticks are present we use the textual closer, otherwise token‑depth is sufficient.
- Validation: full single‑file TS suite passes (transpile‑only). Negative fixtures cover unterminated templates and related failures.

## Limitations and future work

- This is a boundary detector, not a parser; it does not build AST. For mixed bodies we still rely on the NativeRegionSegmenter + MixedBody.
- Eventually, Frame‑statement expansions (MIR) will be emitted via SWC AST (B2 codegen) for deterministic formatting and precise source maps.
