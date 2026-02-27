# Stage 5e — Frame Statement Expansion (C#)

Purpose
- Minimal expansion for MIR Frame statements in C# bodies: emit comment-only markers at the correct indentation. Full Roslyn-aware formatting remains an optional facade (Stage 07).

Inputs
- `MixedBody` MIR items; indent derived from each Frame-statement line.

Outputs
- C# comment lines (`// ...`) preserving the line’s indentation.

Expansions (Minimal)
- Transition `-> $State(args?)` → `// frame:transition State(args)`
- Forward `=> $^` → `// frame:forward`
- Stack ops `$$+` / `$$-` → `// frame:stack_push` / `// frame:stack_pop`

Indentation
- Use exactly the leading whitespace from the Frame-statement line.

Terminal Semantics
- Transitions are terminal within their containing block (validator). Forwards and stack operations are not mandated terminal and may be followed by native statements.

Inline forms
- C# permits multiple statements on one line via `;`. When a Frame statement appears as `...; // ...`, the scanner splits the line so the expansion is inserted before the semicolon, and the remainder stays native.

Notes (C# specifics)
- Surrounding code may include interpolated/verbatim/raw strings; expansion lines are inserted at SOL and do not affect protected regions. Indentation should not break `else if`/`catch`/`finally` flow.

Tests
- Indentation preservation in `if/else`/`try/catch/finally` blocks; mapping anchors.
