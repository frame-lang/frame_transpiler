# Stage 5f — Frame Statement Expansion (Java)

Purpose
- Minimal expansion for MIR Frame statements in Java bodies: emit comment-only markers at the correct indentation.

Inputs
- `MixedBody` MIR items; indent derived from each Frame-statement line.

Outputs
- Java comment lines (`// ...`) preserving the line’s indentation.

Expansions (Minimal)
- Transition `-> $State(args?)` → `// frame:transition State(args)`
- Forward `=> $^` → `// frame:forward`
- Stack ops `$$+` / `$$-` → `// frame:stack_push` / `// frame:stack_pop`

Indentation
- Use exactly the leading whitespace from the Frame-statement line.

Terminal Semantics
- Frame statements are terminal in handlers and must be last (validator). Comment-only markers are placeholders.

Tests
- Indentation and mapping anchors within `if`/`else`/`try`/`catch`/`finally` structures.

