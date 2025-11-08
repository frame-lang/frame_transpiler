# Native MixedBody Parsing via FIRST-Set Indexing (DPDA + Mini‑Parsers)

Status: Proposed (in progress)
Owner: compiler frontend
Last updated: 2025‑11‑08

## Goal

Parse native bodies (actions/operations/handlers) reliably and efficiently by:

- Using a per‑language DPDA textual closer to bound the body span (handles strings, comments, templates/f‑strings, and nested `${…}` where applicable).
- Performing a single streaming pass over the body to build a “SOLIndex” of Frame statements based on their FIRST sets at start‑of‑line (SOL) outside strings/comments.
- Delegating each detected directive to a tiny, directive‑specific mini‑parser to produce MIR (Mixed Intermediate Representation) statements.
- Emitting a canonical MixedBody: an ordered sequence of NativeText/NativeAst and MIR statements.

This replaces ad‑hoc per‑char segmentation logic with a simple, fast, and deterministic approach.

## Definitions

- DPDA closer: A deterministic pushdown automaton that locates the body’s closing `}` by tracking strings/comments and language‑specific constructs (TS: template literals with nested `${…}`; Py: triple quotes and f‑strings). It returns `close_line` or a characterized failure (e.g., “Unterminated template literal at line N”).
- SOL (start‑of‑line, outside strings/comments): The first non‑whitespace column of a line when the scanner is not inside a string/comment/template/f‑string state.
- FIRST(Directive): The set of ASCII tokens that can start a Frame statement at SOL.
  - Transition: `->`
  - Parent forward: `=>` (followed by optional whitespace and `$^`)
  - Stack ops: `$$[+]`, `$$[-]`
  - System return: `system.return` (followed by optional whitespace and `=`)

## Algorithm

1) Body boundary (per language)
   - Invoke the DPDA closer on the body’s opening `{` line to obtain `close_line` (or a precise failure). This bounds the slice for subsequent scanning.

2) Streaming scan (O(n), byte‑based)
   - Iterate bytes from the first body line to `close_line - 1` while tracking string/comment/template/f‑string states.
   - For each line, compute SOL = first non‑whitespace outside those states. Use `char.is_whitespace()` for Unicode safety; delimiter detection remains ASCII.
   - Accumulate native text between SOLs. When a SOL token is in FIRST(Directive), flush the native slice and delegate to a directive mini‑parser.

3) Mini‑parsers (per directive)
   - Transition (`-> $State(args?)`): parse state name after `$` and optional `(args…)` on the same line; produce `MirStatement::Transition { state, args }`.
   - Parent forward (`=> $^`): optionally verify `$^`; produce `MirStatement::ParentForward`.
   - Stack ops (`$$[+]`/`$$[-]`): produce `MirStatement::StackPush` / `MirStatement::StackPop`.
   - System return (`system.return = expr`): verify `=`; capture RHS text; produce `MirStatement::Return(expr_opt)`.
   - On parse failure, emit a directive‑specific error (e.g., “Missing `)` in state args”), with `frame_line` and the line’s snippet.

4) MixedBody assembly
   - Append `MixedBodyItem::NativeText` (or `NativeAst` if native parser succeeds) for native runs and `MixedBodyItem::Frame` for each MIR directive.
   - If no directives are found, attach a single native span (prefer native AST when available).

5) Emission and semantics
   - Visitors emit native segments verbatim and MIR glue deterministically.
   - Transitions/forward/stack are terminal in handlers; after such MIR, visitors suppress subsequent native text or warn “unreachable”.
   - System return rewrites:
     - Python: `system.return` → `self.return_stack[-1]` (normalize `true/false/null` → `True/False/None`).
     - TypeScript: `system.return` → `this.returnStack[this.returnStack.length - 1]`.

## Unicode Safety

- Delimiters are ASCII (`'`, `"`, `` ` ``, `{`, `}`, `$`, `\`, `[`, `]`, `-`, `=`, `\n`), which are 1‑byte in UTF‑8; scanning is byte‑safe.
- SOL uses `char.is_whitespace()` so NBSP and other Unicode spaces are treated correctly.
- We count lines using `\n`. CRLF is tolerated (CR handled as a normal byte).
- Native content can contain any Unicode; scanners never split a multi‑byte codepoint except by searching for ASCII delimiters.

## Error Handling

- DPDA closer returns `Ok(close_line)` or `Failure(kind)`, mapped to precise `ParseError` (e.g., “Unterminated triple‑quoted string at line N”).
- Mini‑parsers produce directive‑specific errors with frame lines and helpful suggestions.
- On failure, the compiler respects policy: treat the line as native and continue, or abort with a precise message (language‑dependent).

## Integration Plan

1) Add SOLIndex builder (per language)
   - Streaming pass over the body using existing DPDA states; record entries `{ line, kind, byte_span, line_text }` when FIRST matches at SOL.

2) Implement mini‑parsers
   - One per directive kind; consume within the line; return MIR and the consumed span; classify failures.

3) Replace segmenters’ inner loops
   - Keep the outer API (returns `Vec<BodySegment>`); inside, use SOLIndex + mini‑parsers instead of per‑char directive detection.

4) Wire into parser
   - For handlers, build `MixedBody` from segments using `build_mixed_body_from_segments`.
   - For actions/operations (native‑only policy), short‑circuit: if SOLIndex is empty, attach a single native span.

5) Tests
   - Positive and negative fixtures for each directive kind at SOL.
   - Strings/templates/f‑strings containing directive‑like tokens must not trigger SOLIndex.
   - Unicode whitespace at SOL, CRLF files, and large bodies for performance.

## Acceptance Criteria

- Single‑file TS/Py suites remain green.
- Directive detection is strictly SOL and never triggers inside strings/comments/templates.
- Errors are specific and actionable (unterminated constructs, malformed directives).
- Scanning is linear and free of pathological blow‑ups on large bodies.

## Notes

- This design formalizes the current approach (DPDA for closers + SOL detection) and simplifies directive handling via FIRST + mini‑parsers. It is correctness‑preserving and generally faster, while keeping behavior deterministic and well‑tested.

