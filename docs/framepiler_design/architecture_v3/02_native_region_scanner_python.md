# Stage 2 — Native Region Scanner (Python)

Purpose
- Segment a known body range into Native and Frame segments using a streaming, protected‑region‑aware scan. SOL‑anchored Frame statements are detected only when outside strings/comments.

Inputs
- `BodyPartition` range `[open_byte+1, close_byte)`
- Source bytes `&[u8]`

Outputs
- `Vec<Segment>` where:
  - `Segment::NativeText { start, end }`
  - `Segment::FrameSegment { start, end, kind_hint, indent }`
- `kind_hint` ∈ { Transition, Forward, StackOp } detected by FIRST‑set at SOL.
- `indent`: the exact indentation prefix (bytes) of the Frame statement line.

Invariants
- No regex; scan is byte‑wise and DPDA‑based for protected regions.
- `at_sol` is true immediately after a newline (LF/CRLF) and before the first non‑whitespace when not in protected regions.
- Unicode whitespace at SOL is accepted: tabs, ASCII space, NBSP (U+00A0), Zs block (U+2000..U+200B, U+202F, U+205F, U+3000). A BOM at body start is skipped.
- `system.return` is not recognized as a Frame statement.

Inline end markers
- A Frame segment ends at the earliest of:
  - end of line (LF/CRLF), or
  - the first top‑level semicolon `;`, or
  - the start of a `#` comment.
- The scanner emits the left `FrameSegment` and then a `NativeText` segment for the remainder on the same line.

Multi‑statement on a line
- Python allows `;` to separate multiple statements on one physical line; Frame statements follow this rule. A top‑level `;` or `#` delimits the Frame segment and the remainder of the line is native.
- Examples (valid): `=> $^; # trailing comment`, `=> $^; native()`
- Without a separator, non‑whitespace tokens after a Frame statement are invalid (parser error).

Algorithm
- Initialize `i = open+1`, `region_start = i`, states for strings (`'`, `\"`, triple quotes), f‑strings, and comments.
- Maintain `at_sol` flag:
  - On `\n` outside protected regions: set `at_sol = true`, remember `sol_idx = i+1`.
  - While `at_sol`, skip Unicode whitespace (see invariants); capture `indent` span; then test FIRST‑set:
    - Transition: `->` WS+ `$` state
    - Forward: `=>` WS+ `$^`
    - Stack op: `$$[+]` or `$$[-]` (canonical)
  - On match: flush preceding `NativeText` [region_start, sol_idx+indent_len), then find end of the Frame statement line (to `\n` or `close_byte`), emit `FrameSegment { start, end, kind_hint, indent }`, set `region_start = end`, continue with `i = end`.
- Update protected‑region states as in the Python body closer.

Errors
- None at segmentation time; malformed Frame statements are handled by the Frame Statement parser.

Complexity
- O(n) over body length.

Test Hooks
- Frame-statement-like tokens inside strings/comments are ignored.
- Unicode whitespace before Frame statements is accepted at SOL.
- Mixed native/Frame-statement lines produce correct segment boundaries.
## Addendum: Section Headers Boundaries for Import/Prolog Scans

Scans that collect module prolog and top‑level imports MUST stop at the first section header (SOL‑anchored, leading whitespace allowed):

- `system`, `machine`, `interface`, `actions`, `operations`, `domain`

Rationale: imports inside `actions:` or handlers are part of method bodies and must not be hoisted or used to advance the module outline start. This prevents skipping `machine:`/`actions:` during partitioning.

Recommended check: when encountering a potential `import …` or `from … import …`, confirm that no section header has been seen yet; otherwise, treat the line as body content.
