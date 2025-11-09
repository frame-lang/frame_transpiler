# Stage 2b — Native Region Scanner (TypeScript)

Purpose
- Segment a known body range into Native and Frame segments using a streaming, template‑aware scan. SOL‑anchored Frame statements are detected only when outside strings/templates/comments.

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
- Template/backtick aware; tracks nested `${ … }` expressions with a counter.
- `at_sol` computed only outside protected regions.
- `system.return` is not recognized as a Frame statement.

Algorithm
- Initialize `i = open+1`, `region_start = i`, protected‑region states for `'`, `\"`, `` `…` ``, `/*…*/`, `//`.
- Maintain `at_sol` and test FIRST‑set at SOL:
  - Transition: `-` then `>` then space/tab then `$`.
  - Forward: `=` then `>` then space/tab then `$` then `^`.
  - Stack op: `$` `$` then `+` or `-`.
- On match: flush preceding `NativeText`, capture Frame statement line end at `\n` or `close_byte`, emit `FrameSegment`, advance `region_start` and `i`.
- Update protected‑region states like the TS body closer.

Errors
- None during segmentation; malformed Frame statements are handled by the Frame Statement parser.

Complexity
- O(n) over body length.

Test Hooks
- Frame-statement tokens inside backticks or comments are ignored.
- Template literals with nested `${…}` do not produce false positives at SOL.
