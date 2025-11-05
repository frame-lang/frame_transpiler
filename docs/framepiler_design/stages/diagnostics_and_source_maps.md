# Diagnostics & Source Maps

Goals
- Provide precise diagnostics in both Frame and target domains with stable line mapping across partitions and mixed bodies.

Sources of Spans
- Frame tokens/statements → direct Frame spans.
- Target parser spans → `TargetSourceMap` back to frame lines.
- MIR expansions → synthesized spans (directive frame lines).

Composition
- For each emission site, choose domain (Frame or target) and attach span metadata.
- When printing native AST (B2), leverage native printers’ source map output and compose with `TargetSourceMap`.

Examples
- Transition in native body: MIR(Transition) synthesized at the directive’s frame line.
- Native block verbatim: start/end frame lines from `NativeText` item guide mapping.

Validation
- Golden mapping tests: report errors in native code and verify mapped frame lines.
- AST dump includes MixedBody items with line spans.

