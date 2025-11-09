# Stage 5g — Frame Statement Expansion (Rust)

Purpose
- Minimal expansion for MIR Frame statements in Rust bodies: emit comment-only markers at the correct indentation.

Inputs
- `MixedBody` MIR items; indent derived from each Frame-statement line.

Outputs
- Rust comment lines (`// ...`) preserving the line’s indentation.

Expansions (Minimal)
- Transition `-> $State(args?)` → `// frame:transition State(args)`
- Forward `=> $^` → `// frame:forward`
- Stack ops `$$+` / `$$-` → `// frame:stack_push` / `// frame:stack_pop`

Indentation
- Use exactly the leading whitespace from the Frame-statement line.

Terminal Semantics
- Terminal-last rule enforced by validator; comments serve as placeholders.

Tests
- Indentation and mapping anchors in nested `if`/`match` blocks.

