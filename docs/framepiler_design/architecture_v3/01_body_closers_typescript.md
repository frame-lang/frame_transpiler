# Stage 1b — Body Closer (TypeScript)

Purpose
- Determine the exact `close_byte` for a member body that begins with `{` for the TypeScript target.
- Be robust against TS/JS constructs that can contain braces: template literals with nested `${ … }`, strings with escapes, block and line comments.

Algorithm (Template‑Aware Deterministic Pushdown)
- Inputs: `src: &[u8]`, `open_idx: usize` (index of the opening `{`).
- State:
  - `brace_depth: i32 = 1`
  - `in_squote`, `in_dquote`, `in_template`, `in_block_comment`: booleans
  - `tpl_expr_depth: i32 = 0`
  - `i: usize = open_idx + 1`
- Loop over bytes until EOF:
  - If `in_block_comment`: advance and close on `*/`.
  - If `in_template`:
    - On `${`: `tpl_expr_depth += 1`.
    - On `}` with `tpl_expr_depth > 0`: `tpl_expr_depth -= 1` (does not affect `brace_depth`).
    - On backtick `` ` `` when `tpl_expr_depth == 0`: `in_template = false`.
    - Handle escapes `\\` as literal in template.
    - Continue.
  - If in a quoted string: handle escapes `\\` and close on matching quote.
  - Otherwise (not in string/template/comment):
    - On `//`: skip to end of line; continue.
    - On `/*`: `in_block_comment = true`; continue.
    - On backtick: `in_template = true`; continue.
    - On `'`/`"`: enter string; continue.
    - On `{`: `brace_depth += 1`; continue.
    - On `}`: `brace_depth -= 1`; if `brace_depth == 0`, return `Ok(i)` as `close_byte`.
  - `i += 1`.
- If EOF reached with `brace_depth != 0`, return an unterminated‑body error.

Edge Cases
- CRLF: handled transparently.
- NBSP/tabs: treated as non‑structural.

Errors
- `TsCloserError::UnterminatedTemplate { start_span }`
- `TsCloserError::UnterminatedString { start_span }`
- `TsCloserError::UnterminatedBody { open_idx }`

Complexity
- O(n) in the size of the body.

Contract
- Called only from `ModulePartitionerV3`; downstream stages never re‑close bodies.

