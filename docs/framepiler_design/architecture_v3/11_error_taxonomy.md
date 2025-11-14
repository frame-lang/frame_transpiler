# Error Taxonomy (V3)

Purpose
- Enumerate canonical error classes across stages with message shapes and attribution rules.

Partitioning / Body Closers
- E100: Unterminated string (single/double) — "unterminated string starting here"; span: start quote → EOF.
- E101: Unterminated triple quote — "unterminated triple‑quoted string starting here"; span: start triple → EOF.
- E102: Unterminated template/backtick — "unterminated template literal starting here".
- E103: Unterminated body — "unterminated body starting at { index }".
- E104: Stray closing brace — "unexpected '}'".
- E105: Missing/invalid prolog — "expected @target <lang> at start of file".
- E106: Unterminated comment — "unterminated comment starting here".

Import Scanners
- E110: Unterminated or malformed import/include directive — e.g., "unterminated include directive", "unterminated import/export statement", "unterminated using directive", "unterminated package/import statement", "unterminated use/extern statement".

Outline / Headers
- E111: Missing '{' after module artifact header — applies to function headers in interface/actions/operations sections.
 - E112: Missing '{' after state header in machine: — applies to `$State` headers inside the `machine:` section.

Segmentation
- E200: Invalid Frame-statement token at SOL — "invalid Frame statement at start of line"; span: statement slice.

Frame Segment Parser
- E300: Transition missing `$State` — "expected $State after '->'".
- E301: Unbalanced parentheses in transition args — "unbalanced '(' in transition arguments".
- E302: Trailing tokens after Frame statement — "unexpected trailing tokens after Frame statement" (only when no inline separator is present; `;` or comment start ends the Frame segment for eligible languages).

MIR Assembly / Validation
- E400: Transition not terminal within its block — "Transition must be last statement in its containing block".
- E401: Frame statement not allowed in actions/ops — "Frame statements are not allowed in native bodies (actions/ops)".
- E402: Unknown state in transition — "unknown state '$Name'".
- E403: Parent forward without declared parent — "Cannot forward to parent: no parent available". Triggered when a handler’s enclosing state does not declare a parent in `machine:`. Applies to module demos; single‑body demos are exempt.
 - E404: Handler outside state block — "handler body must be inside a state block". Triggered when a handler header appears in `machine:` but is not nested within a `$State { … }` block.
 - E405: State parameter arity mismatch (advisory, Stage 10B) — Emitted when a transition supplies a different number of `state_args` than the target state’s declared parameter list in the outline header. Flag‑gated via FRAME_VALIDATE_NATIVE_POLICY.

Native Policy (Python)
- E500: `var` in Python body — "'var' is not allowed in Python native bodies".
- E501: Brace‑style control in Python body — "brace‑style control is not allowed in Python native bodies".

Native Policy (TypeScript)
- E600: Disallowed equality operator — "use '==='/'!==' instead of '=='/'!='".

Attribution & Rendering
- Frame‑origin errors (segmentation, MIR, validation) report Frame spans via byte→(line,col) mapping.
- Native‑origin errors (native parse/policy) report spliced spans remapped through `splice_map` to original Frame/native origins.
- Message format: `[CODE] message — file:line:col` with a short cause hint where applicable.
