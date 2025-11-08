# AST Dump Spec — Frame + Mixed Bodies + Target ASTs

Status: Draft
Date: 2025‑11‑08
Owner: going_native track

## Purpose
Define a stable JSON format to dump the Frame AST augmented with MixedBody segments and lightweight Target AST summaries for native blocks. Used for debugging, tooling, and regression tests.

## Output Artifact
- Sidecar JSON written when `--debug-output`/`--ast-dump` is enabled.
- Filename: `<target_basename>.frame_ast.json` (one per generated file or per compilation unit).

## Top-Level Structure (v1)
```json
{
  "version": 1,
  "sourceFile": "path/to/spec.frm",
  "targetLanguage": "c|cpp|rust|java|typescript|python",
  "module": {
    "imports": [],
    "functions": [ /* see below */ ],
    "systems": [ /* see below */ ]
  }
}
```

## Function Node
```json
{
  "name": "main",
  "params": [],
  "isAsync": false,
  "mixedBody": [ /* MixedBody items if applicable */ ]
}
```

## System Node
```json
{
  "name": "SystemName",
  "interface": [ { "name": "method", "params": ["x:int"], "return": null, "isAsync": false } ],
  "domain": [ { "name": "count", "type": "int", "initializer": 0 } ],
  "actions": [ { "name": "act", "params": [], "mixedBody": [ /* ... */ ] } ],
  "operations": [ { "name": "op", "params": [], "return": null, "mixedBody": [ /* ... */ ] } ],
  "machine": {
    "states": [
      {
        "name": "Start",
        "handlers": [
          { "event": "trigger", "params": [], "mixedBody": [ /* ... */ ] }
        ],
        "enter": { "params": [], "mixedBody": [ /* ... */ ] },
        "exit":  { "params": [], "mixedBody": [ /* ... */ ] }
      }
    ]
  }
}
```

## MixedBody Item
```json
// Native text span
{ "kind": "NativeText", "startLine": 120, "endLine": 123, "text": "const x = 1;\n..." }

// Target AST (if available)
{ "kind": "NativeAst", "startLine": 140, "endLine": 155,
  "target": "typescript|python|c|cpp|java|rust",
  "diagnostics": [ { "message": "…", "targetLine": 3, "column": 12 } ],
  "summary": "optional short description",
  "source": "ast.to_source() shortened or omitted per config"
}

// Frame statement (lowered MIR)
{ "kind": "FrameStmt", "frameLine": 160, "stmt": {
    "type": "Transition|Forward|StackPush|StackPop|Return",
    "state": "Next",        // for Transition
    "args": ["expr1", "expr2"] // raw text, best‑effort only
  }
}
```

Notes:
- `text` for NativeText is included for debugging; allow truncation with a size cap.
- `source` for NativeAst may be omitted for large blocks; always include diagnostics.
- Frame expressions in `args` are stored as raw best‑effort strings from the line text; they are not re‑parsed here.

## Emission Policy
- Dump is produced after parsing/segmentation and before visitor codegen, so it reflects the exact MixedBody the visitor will consume.
- A per‑target flag controls whether full native text/AST is embedded or only diagnostics and summaries (privacy/size).

## CLI
- Extend `framec --debug-output` to optionally include `--ast-dump` (or make AST dump part of the same JSON under a new section).
- For multi‑file/multi‑system outputs, include all systems/functions present in the compilation unit.

## Validation
- Ensure every handler/action/operation that has a MixedBody produces at least one MixedBody item in the dump.
- Unit tests compare against golden JSON with stable field ordering and value normalization.

## Future
- Stable schema versioning and a JSON‑Schema file.
- Optional inclusion of a compact native AST digest (e.g., hash) for change detection without full source embedding.
