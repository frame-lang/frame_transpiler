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
- E403: Parent forward without declared parent — "Cannot forward to parent: no parent available" (applies to module demos with `machine:`; single‑body demos are exempt).

Native Policy (Python)
- E500: `var` in Python body — "'var' is not allowed in Python native bodies".
- E501: Brace‑style control in Python body — "brace‑style control is not allowed in Python native bodies".

Native Policy (TypeScript)
- E600: Disallowed equality operator — "use '==='/'!==' instead of '=='/'!='".

Attribution & Rendering
- Frame‑origin errors (segmentation, MIR, validation) report Frame spans via byte→(line,col) mapping.
- Native‑origin errors (native parse/policy) report spliced spans remapped through `splice_map` to original Frame/native origins.
- Message format: `[CODE] message — file:line:col` with a short cause hint where applicable.
