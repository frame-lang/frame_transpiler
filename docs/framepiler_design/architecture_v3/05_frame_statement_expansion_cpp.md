# Stage 5d — Frame Statement Expansion (C++)

Purpose
- Minimal expansion for MIR Frame statements in C++ bodies: emit comment-only markers at the correct indentation.

Inputs
- `MixedBody` MIR items; indent derived from each Frame-statement line.

Outputs
- C++ comment lines (`// ...`) preserving the line’s indentation.

Expansions (Minimal)
- Transition `-> $State(args?)` → `// frame:transition State(args)`
- Forward `=> $^` → `// frame:forward`
- Stack ops `$$+` / `$$-` → `// frame:stack_push` / `// frame:stack_pop`

Indentation
- Use exactly the leading whitespace from the Frame-statement line.

Terminal Semantics
- All Frame statements are terminal in handlers; validator enforces terminal-last. Comment-only markers are placeholders until full codegen.

Tests
- Indentation and comment text golden files; verify splice mapping attributes to Frame origin.

