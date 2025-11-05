# Module Partitioner

Purpose
- Identify module boundaries and produce an ordered list of partitions:
  - PrologPartition (e.g., `@target`),
  - NativeImportPartition(s),
  - FrameOutlinePartition (systems/blocks/headers),
  - BodyPartition (per member body with balanced brace ranges).

Inputs/Outputs
- Input: full file text (lines), optional CLI target.
- Output: `ModuleUnit { partitions: Vec<Partition> }` with line/column ranges.

Invariants
- Partitions do not overlap and cover the file (whitespace/comments allowed between).
- Body partition ranges are brace‑balanced and exclude the closing `}`.

Notes
- An implicit ModuleUnit exists without explicit `module {}`.
- Target prolog is optional; CLI may supply target.

Validation
- Unit tests for nested braces and mixed import formats.
- Integration: partitions feed scanner and parser with consistent ranges.

