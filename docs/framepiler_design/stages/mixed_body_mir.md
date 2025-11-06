# MixedBody & MIR (Minimal Intermediate Representation)

Purpose
- Represent native body content and embedded Frame directives in a single, ordered structure for stable codegen and mapping.

Core Types
- `MirStatement`:
  - `Transition { state: String, args: Vec<String> }`
  - `ParentForward`
  - `StackPush`
  - `StackPop`
  - `Return(Option<String>)`
- `MixedBodyItem`:
  - `NativeText { target, text, start_line, end_line }`
  - `NativeAst { target, start_line, end_line, ast }`
  - `Frame(MirStatement)`

Assembly
- Built from `BodySegment` output where available; for native‑only (target‑parsed) bodies, synthesize a single `NativeText` item.

B2 Emission Strategy
- Expand `MirStatement` into target AST (e.g., SWC nodes for TypeScript) rather than string glue.
- Print via native code generators to ensure formatting determinism; attach synthesized spans mapped to Frame directive lines.

Invariants
- Item order reflects original source order.
- Frame statements must be terminal for handler bodies that transition/forward/pop (validator planned).

Validation
- Golden tests for directive expansions; AST dump includes MixedBody with spans.
