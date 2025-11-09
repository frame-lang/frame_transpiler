# Stage 1a — Body Closer (Python)

Purpose
- Determine the exact `close_byte` for a member body that begins with `{` for the Python target.
- Be robust against Python string constructs: single/double quotes, triple‑quoted strings, and f‑strings; and `#` line comments.

Algorithm (Deterministic Pushdown)
- Inputs: `src: &[u8]`, `open_idx: usize` (index of the opening `{`).
- State:
  - `brace_depth: i32 = 1`
  - `in_squote`, `in_dquote`, `in_tsquote`, `in_tdquote`, `in_comment`: booleans
  - `i: usize = open_idx + 1`
- Loop over bytes until EOF:
  - If `in_comment`: advance until `\n`, then `in_comment = false`; continue.
  - If in a string:
    - For single/double quotes: handle escapes `\\` and close on matching quote.
    - For triple quotes: close on matching triple sequence.
    - Continue.
  - Otherwise (not in string/comment):
    - On `#`: `in_comment = true`; continue.
    - On `'`/`"`: detect triple quotes first; enter corresponding string state; continue.
    - On `{`: `brace_depth += 1`; continue.
    - On `}`: `brace_depth -= 1`; if `brace_depth == 0`, return `Ok(i)` as `close_byte`.
  - `i += 1`.
- If EOF reached with `brace_depth != 0`, return an unterminated‑body error, including where the unterminated string (if any) began.

Edge Cases
- CRLF: treat `\r\n` as newline; no special handling needed beyond byte iteration.
- Unicode: scanning operates on bytes; delimiters are ASCII; for diagnostics, use a byte→(line,col) index.

Errors
- `PyCloserError::UnterminatedTripleQuote { start_span }`
- `PyCloserError::UnterminatedString { start_span }`
- `PyCloserError::UnterminatedBody { open_idx }`

Complexity
- O(n) in the size of the body.

Test Matrix
- Single/double quotes with escapes; triple quotes across many lines; nested braces in strings.
- f‑strings: braces inside f‑strings are ignored as part of the string region.
- Comments containing `}` or Frame-statement‑like lexemes are ignored.

Contract
- Called only from `ModulePartitionerV3`; downstream stages never re‑close bodies.
