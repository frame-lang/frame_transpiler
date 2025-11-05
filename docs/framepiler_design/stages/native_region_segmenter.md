# Native Region Segmenter (per target)

Purpose
- Classify native body text into an ordered sequence of segments without reordering:
  - `BodySegment::Native { text, start_line, end_line }`
  - `BodySegment::Directive { kind, frame_line, line_text }`

Algorithm (TypeScript)
- Single pass with state variables:
  - `in_squote`, `in_dquote`, `in_template`, `tpl_expr_depth`
  - `in_block_comment`, `in_line_comment`, `brace_depth`
- Only detect directives at `brace_depth == 0` and outside strings/comments:
  - Transition: `-> $Name`
  - Forward: `=> $^`
  - Stack push/pop: `$$[+]`, `$$[-]`

Invariants
- Segment line spans map back to frame lines; no text reformatting.
- Directives are recognized verbatim and removed from the native run.

Edge Cases
- Nested template literals: `${ ... }` tracked via `tpl_expr_depth`.
- Comments containing directive tokens: ignored by detection.

Validation
- Fixtures for comments, strings, template literals, multi‑line blocks.
- Negative tests: directive‑like tokens inside strings/comments.

