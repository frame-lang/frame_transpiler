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
- Transitions are terminal within their containing block; forwards/stack ops are not mandated terminal and may be followed by native statements. Validator enforces only transition-as-terminal.

Inline forms
- Support `;` separated single-line forms; expansion precedes the semicolon; trailing native text (or `//` comment) remains native.

Tests
- Indentation and comment text golden files; verify splice mapping attributes to Frame origin.
