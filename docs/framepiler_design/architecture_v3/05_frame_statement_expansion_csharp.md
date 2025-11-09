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
- Frame statements are terminal in handlers and must be last (validator). Comment-only emissions serve as placeholders until full glue is introduced.

Notes (C# specifics)
- Surrounding code may include interpolated/verbatim/raw strings; expansion lines are inserted at SOL and do not affect protected regions. Indentation should not break `else if`/`catch`/`finally` flow.

Tests
- Indentation preservation in `if/else`/`try/catch/finally` blocks; mapping anchors.

