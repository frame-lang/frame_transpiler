# Source Map Spec — Native Backends (C, C++, Rust, Java)

Status: Draft
Date: 2025‑11‑08
Owner: going_native track

## Purpose
- Provide a uniform, language‑agnostic source map for native targets that lets tools map generated lines back to Frame source lines.
- Preserve MixedBody ordering (native spans + Frame statements) and attribute each emitted span to its originating Frame line.

## Terminology
- MixedBody: ordered sequence of items inside a handler/action/operation body
  - NativeText | NativeAst | Frame (MirStatement)
- MirStatement: lowered Frame statement (transition, parent forward, state stack ops, return, prints)

## Output Format (v1)
- Sidecar JSON placed next to generated code when debug is enabled.
- Filename: `<target_basename>.frame_debug.json` (one per generated file)

```json
{
  "version": 1,
  "sourceFile": "path/to/spec.frm",
  "targetFile": "path/to/output.<ext>",
  "targetLanguage": "c|cpp|rust|java|typescript|python",
  "mappings": [
    {
      "targetLine": 42,
      "targetColumn": 1,
      "sourceLine": 137,
      "sourceColumn": 1,
      "kind": "NativeSpan|MirTransition|MirForward|MirStackPush|MirStackPop|MirReturn|Print",
      "note": "optional short text (e.g., state name)"
    }
  ]
}
```

- Granularity: line‑level mappings (columns optional); first line of an emitted span must be mapped. If a MirStatement expands to multiple lines, map all lines if practical; otherwise, map the first line and include a short "note".

## Generation Rules
- Before writing any chunk corresponding to a MixedBody item, register its source line:
  - NativeText/NativeAst: map to the item’s `start_line`.
  - Frame (MirStatement): map to the statement’s `frame_line`.
- Unreachable warnings (after terminal Frame statements) are mapped to the terminal statement’s `frame_line`.
- Strings and helper glue emitted around native text should inherit the same mapping as the enclosing item.

## Visitor Hooks
- All native visitors should expose two minimal hooks:
  - `map_next(frame_line: usize)` → assigns the next write’s starting position to the given Frame line.
  - `record_mapping(target_line, target_col, source_line, source_col, kind, note)` → used internally by `map_next`.
- For code builders that buffer fragments, `map_next` associates the pending line before flush; on flush, exact `targetLine` is resolved.

## MixedBody Algorithm (Reference)
- For each item in MixedBody (in order):
  - NativeText: `map_next(start_line)` then write the raw text; ensure trailing newline for clean line accounting.
  - NativeAst: `map_next(start_line)` then write `ast.to_source()` or `to_code()`.
  - Frame(MirStatement): `map_next(frame_line)` then emit MIR‑glue; set `kind` accordingly.

## Enablement
- CLI flag: `--debug-output` produces the sidecar JSON (existing for Python; extend to C/C++/Rust/Java visitors).
- For multi‑file outputs, each generated file gets its own `.frame_debug.json` next to it.

## Examples
- Transition line mapping:
  - Source: `-> ("msg", 1) $Next` at `sourceLine=120`
  - Emitted: 3 lines of calls; map lines 200..202 to `sourceLine=120`, `kind=MirTransition`, `note="Next"`.
- Native after transition (unreachable): emit code but insert a warning comment; map warning line to the transition’s `sourceLine`.

## Validation
- Add a rule to ensure every MixedBody item creates at least one mapping.
- Unit tests: small handlers verify exact `targetLine ↔ sourceLine` for MIR glue and native spans.

## Future
- Column‑level coverage (segments within a line), and DWARF/CodeView emission for native debugging when feasible.
