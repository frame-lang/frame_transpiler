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

Outputs
- Python visitor: supports external SourceMapBuilder and can print mapping trailer in debug.
- TypeScript visitor: two debug modes via env vars
  - `FRAME_TS_MAP_COMMENTS=1`: appends comment trailer delimited by `__frame_map_begin__/__end__` with entries `map frame:<N> -> ts:<M>`.
  - `FRAME_TS_MAP_JSON=1`: appends a JSON block inside comments delimited by `__frame_map_json_begin__/__end__`. Each entry has `{ frameLine, tsLine, type }` (type is optional mapping type).

Integration
- Visitors call `builder.map_next(frame_line)` before emitting a native span or MIR expansion. The builder records the first generated position following the call.
- MixedBody mapping policy:
  - NativeText/NativeAst: map at the first generated line of the segment to `start_line`.
  - Frame(MirStatement): map the expansion to the statement’s `frame_line`.

Examples
TS (comments mode):
```ts
// __frame_map_begin__
// map frame:123 -> ts:210
// map frame:124 -> ts:225
// __frame_map_end__
```

TS (JSON mode):
```ts
// __frame_map_json_begin__
// [{"frameLine":123,"tsLine":210,"type":null},{"frameLine":124,"tsLine":225,"type":"Statement"}]
// __frame_map_json_end__
```
