# Stage 4 — MIR Assembly (MixedBody)

Purpose
- Convert segmented native/Frame‑statement slices into a stable, target‑agnostic MixedBody with precise spans and mapping, ready for validation and expansion.

Inputs
- `Vec<Segment>` (NativeText and FrameSegment) and parsed `MirItem`s.

Outputs
- `MixedBody { items: Vec<MixedBodyItem>, mapping: MixedBodyMapping }`
  - `MixedBodyItem::{ NativeText{ span }, Mir(MirItem) }`
  - `MixedBodyMapping`: auxiliary indices for fast span lookup and source‑map composition.

Invariants
- Preserve original ordering and byte spans.
- MIR contains only the three embedded Frame statements; no `system.return`.
- MixedBody is authoritative for validation and expansion; no additional parsing is performed at this stage.

Validation (at assembly time)
- Only Transition is terminal. A Transition must be the last executable statement within its containing block.
- Forwards and Stack operations are non‑terminal and may be followed by native statements on the same or subsequent lines (subject to the host language’s multi‑statement rules).
- Any native content after a Transition in the same block is flagged for diagnostics (policy violation).

Complexity
- Linear in the number of segments.

Test Hooks
- Mixed native/MIR sequences; terminal Frame statement at EOF; native code trailing a terminal Frame statement.
