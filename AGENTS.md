# AGENTS.md — Working Rules for this Repo

Scope: Entire repository.

Non‑negotiables
- Do NOT commit or push without explicit user approval. Always stage changes for review first.
- Always validate each meaningful change by running the full TypeScript transpile‑only suite, and Python when relevant.
- Never introduce temporary hacks that compromise determinism in scanning/lexing/parsing.

Architecture alignment
- MixedBody/MIR (B2) is the authoritative model: bodies are sequences of native items and Frame statements.
- Frame directives are strictly SOL‑anchored (start of line), ignored in strings/comments/templates.
- Per‑target segmenters must be brace/string/comment aware; avoid regex for language syntax.

Testing policy
- Prefer hermetic tests and compile‑only validation where runtime dependencies exist.
- Add negative fixtures alongside positive ones for policy changes (e.g., inline target directives).

Docs
- Keep `docs/framepiler_design/architecture.md` as the detailed, authoritative reference.
- Reflect syntax and policy decisions promptly in per‑language grammar docs.

Operations
- If a change requires external tooling (npm/typedoc/etc.), document it and gate it behind CLI flags; avoid adding network dependencies to the build.

