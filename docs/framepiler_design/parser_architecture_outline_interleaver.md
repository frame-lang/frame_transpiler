# Parser Architecture – OutlineScanner + Interleaver (Option A)

Document version: 1.0  
Authors: Frame team  
Status: Proposed (Implementation in branch `ts_interleaver_mvp`)

## Motivation

We need a deterministic, multi‑stage parsing architecture that:
- Allows native target language statements (TypeScript first) to coexist with Frame statements in member bodies (actions, operations, event handlers) without any body fences.
- Preserves Frame semantics (transitions, state stack operations, forwards) while keeping target code verbatim.
- Avoids heuristic scanners and fragile return‑analysis.
- Enables dual source mapping (Frame lines + target lines) and clean diagnostics.

This document defines Option A: a minimal, robust foundation centered on an OutlineScanner and a top‑level body interleaver for TypeScript. Python parity follows; SWC integration is validator‑only (not in the critical path).

## Goals

- Deterministic outline parsing of Frame structure; no regex‑driven or heuristic mixing.
- Top‑level segmentation of TypeScript member bodies into `[NativeSegment | FrameStmt]*`.
- Preserve all braces so nesting is correct; be aware of strings, comments, and template literals (`${…}`).
- Emit native code verbatim; emit Frame glue for transitions/stack/forward with early‑return semantics (in the visitor, not the scanner).
- First‑class source maps and diagnostics for both Frame and target lines.

## Non‑Goals (for this phase)

- No CLI or toolchain changes beyond what is necessary to compile; FID and adapters land after parser is stable.
- No SWC‑driven rewriting; SWC (or rustpython_parser) can be used later for validation only.
- No cross‑language abstraction of native statements; we embrace target syntax.

## Architecture Overview

The pipeline becomes explicitly multi‑stage:

1) OutlineScanner (Frame outline only)
- Recognizes Frame structural headers: module/system/actions/operations/domain/interface, plus state machine operators when they appear on their own lines.
- Records member bodies as contiguous ranges; does not tokenize native lines.
- Always preserves `{` and `}` to maintain correct nesting.

2) Body Segmenter (TypeScript interleaver)
- For each recorded body range, produces an ordered sequence of segments:
  - `NativeSegment { text: Arc<str>, start_line, end_line }`
  - `FrameStmt { kind: Transition | Forward | StackPush | StackPop, span }`
- Recognizes Frame statements only at top level within the body (not inside strings/comments/template literals or nested braces).

3) Parser Assembly
- The general parser builds AST nodes (actions, operations, handlers) and attaches an `ActionBody::Interleaved(Vec<BodySegment>)` (or equivalent) per member.
- For Frame‑only bodies, we continue to use `ActionBody::Frame`.

4) Visitor Emission
- `NativeSegment` lines are emitted verbatim in order.
- `FrameStmt` lines emit the appropriate glue:
  - Transition: emit transition code and return early.
  - State stack ops: push/pop semantics as per runtime/visitor needs.
  - Forward: parent forward glue.
- Public action wrappers remain in use (no `_action_*` leakage).

5) Diagnostics & Source Mapping
- Each segment carries a `SourceSpan` with both Frame and target line info.
- Diagnostics report dual locations; AST dump prints segments for debugging.

## Key Data Structures (proposed)

```rust
// Outline
struct OutlineItem {
    kind: OutlineKind,                // System, Action, Operation, Handler, etc.
    header_span: SourceSpan,          // Frame header location
    body_range: (usize /*start*/, usize /*end*/), // Frame line numbers (inclusive)
}

enum BodySegment {
    NativeSegment {
        text: Arc<str>,
        start_line: usize, // target lines
        end_line: usize,
    },
    FrameStmt {
        kind: FrameStmtKind,
        span: SourceSpan, // Frame line span
    },
}

enum FrameStmtKind { Transition, Forward, StackPush, StackPop }

enum ActionBody {
    Empty,
    Frame,                // current behavior
    Interleaved(Vec<BodySegment>),
}
```

`SourceSpan` already exists; we extend/attach target‑line metadata when building segments.

## Determinism & Token Rules

- Allowlist only for Frame starters at the body top‑level: `->`, `=>`, `$$[+]`, `$$[-]`, and any recognized header tokens (though headers typically won’t appear inside a body).
- Always preserve braces `{` `}` and track depth; do not classify native lines—treat them as `NativeSegment` unless a Frame token is matched at depth 0 and not inside strings/templates/comments.
- Strings: `'...'`, `"..."` with escapes; template literals `` `...${...}` `` with nested `${...}` tracked.
- Comments: `//` and `/* ... */` recognized and ignored for Frame tokenization.

## Error Handling & Diagnostics

- The segmenter is tolerant: native lines that the Frame parser doesn’t understand become `NativeSegment`.
- Frame tokens in invalid contexts (e.g., nested inside a string or comment) are treated as native, not Frame.
- When emitting diagnostics for Frame statements, report both the Frame file line and the local target line where available.

## Testing Strategy

Unit tests (parser + segmenter):
- Mixed bodies with multiple `NativeSegment`/`FrameStmt` interleavings.
- Template literals with nested `${...}` and braces.
- Comments containing `->`/`=>`.
- Multiple braces on lines; verify depth tracking.

Integration tests (compile‑only initially):
- `language_specific/typescript/regression/test_mixed_target_body_interleave.frm`
- Runtime protocol example with native TS calls: `runtime/test_runtime_protocol_native.frm`

AST dump / source maps:
- Verify that segment spans align with both Frame and target lines.

## Performance Considerations

- Single pass over body lines; per‑line classification is O(n) with small state machines for strings/templates/comments.
- No heavy parsing of native code; no SWC in the hot path.

## Extensibility

- TypeScript first. Python to follow using the same segmentation model with its own string/comment rules.
- SWC/rustpython_parser can be added as optional validators (not required to build or emit).
- Future targets plug in their body segmenters under the same OutlineScanner framework.

## Rollout Plan (alignment with Cross‑Language Plan)

1) Implement OutlineScanner and tests.
2) Implement TypeScript Body Segmenter and attach `ActionBody::Interleaved` to members.
3) Update TypeScript visitor to emit interleaved bodies.
4) Add compile‑only fixtures and validate.
5) Add diagnostics/source map tests.
6) Adopt the same pattern for Python (post‑055).

## Open Questions

- Do we add `ActionBody::Mixed` for backward compatibility or move directly to `Interleaved`? (Recommendation: add `Interleaved` and deprecate `Mixed`.)
- Do we store `NativeSegment` as raw lines or a single contiguous slice? (Recommendation: contiguous slice per block for minimal allocation, with per‑line mapping indices.)
- Where to house OutlineScanner code: reuse `scanner.rs` or split into `scanner_outline.rs`? (Recommendation: split for clarity; keep `scanner.rs` delegating to outline, then parser.)

---

This document is the authoritative guide for the Option A implementation. It prioritizes determinism and simplicity, enabling us to unblock TypeScript native bodies and Bug 055 without introducing fence syntax or heavy external dependencies in the critical path.

