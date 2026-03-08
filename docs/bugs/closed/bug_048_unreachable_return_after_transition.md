# Bug #048: Unreachable Return After Transition Statements — Closed

Status: Closed (WontFix under V3 semantics)
Date: 2025-11-12

Summary
- V3 enforces “Transition must be last statement in its containing block” (E400). Expanders emit a native return immediately after a Transition; native code following a Transition is invalid by design.

Rationale
- The prior request assumed returns after transitions were reachable. In V3, transitions are terminal within their block and returns are injected by expanders. This produces deterministic behavior across languages and simplifies validation.

Action
- Close as WontFix. Tests and docs reflect the V3 rule; no further work.

