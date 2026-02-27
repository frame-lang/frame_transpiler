# Stage 5g — Frame Statement Expansion (Rust)

Purpose
- Expand MIR Frame statements in Rust bodies. The module compile path emits runnable stubs by default and uses an enum for state identifiers (no fallback).

Inputs
- `MixedBody` MIR items; indent derived from each Frame-statement line.

Outputs
- Module compile: runnable code with a lightweight `FrameCompartment` and transitions targeting `StateId::<State>`.
- Facade/exec‑smoke: wrapper calls that produce standardized markers (no runtime dependency), used in minimal exec tests.

Expansions (Module compile)
- Transition `-> $State(args?)` → constructs `FrameCompartment { state: StateId::<State>, ..Default::default() }`, calls transition shim, then returns.
- Forward `=> $^` → non‑terminal router call.
- Stack ops `$$+` / `$$-` → push/pop shims.

Indentation
- Use exactly the leading whitespace from the Frame‑statement line.

Terminal Semantics
- Transitions are terminal within their containing block; forwards/stack ops are non‑terminal. The validator enforces the rule; expansions reflect it by emitting an immediate return after transitions.

Inline forms
- Support `;`‑separated single‑line forms; expansion precedes the semicolon; trailing native text (or `//` comment) remains native.

Tests
- Indentation/mapping anchors in nested `if`/`match` blocks.
- Enum state ID default: fixtures assert presence of `enum StateId` and `state: StateId`.
