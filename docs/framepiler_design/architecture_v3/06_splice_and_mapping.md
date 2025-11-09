# Stage 6 — Splice & Mapping

Purpose
- Combine native text and Frame-statement expansions into a single spliced body while maintaining a precise mapping from inserted spans back to their originating Frame statements.

Inputs
- `MixedBody` items and per‑item expansion snippets.

Outputs
- `SplicedBody { bytes: Vec<u8>, splice_map: Vec<SpliceEntry> }`
- `SpliceEntry { frame_span: ByteSpan, inserted_span: ByteSpan, kind: MirKind }`

Invariants
- Stable order; no overlap between inserted spans.
- Mapping must be sufficient to remap native parser diagnostics back to Frame/native origins.

Algorithm
- Iterate `MixedBody.items` left→right.
- For `NativeText{span}`: copy bytes directly.
- For `Mir(item)`:
  - Record current output offset as `insert_start`.
  - Write expansion bytes with the Frame statement’s indent preserved.
  - Record `insert_end` and push `SpliceEntry { frame_span: item.span, inserted_span: [insert_start, insert_end), kind }`.

Complexity
- O(n) in combined output size.

Test Hooks
- Round‑trip mapping checks: given an inserted byte index, map to originating Frame span.
- Edge: consecutive Frame statements; Frame statements at body start/end; empty native runs.
## Mapping Model (Addendum)

- Spliced body and mapping
  - `SplicedBodyV3 { text: String, splice_map: Vec<(ByteSpan /*target*/, Origin)> }`
  - `Origin::{ FrameStatement{ frame_line }, NativeText{ start_line, end_line } }`
  - Compose `SourceMapV3` directly from `splice_map` (byte‑based spans); byte→line index is used only for human diagnostics.

- Terminal handling
  - Preserved syntax headers (e.g., `elif:`) after a terminal Frame statement map to the originating Frame statement’s `frame_line`.

- Acceptance
  - Linear composition; deterministic spans; debug anchors/trailer reflect origins.
