# FID & Native Modules — Overview and Guide (Draft)

Status: Draft (actively used), replaces historical FSL references

Purpose
- Define how Frame specs call into target‑native code in a portable way using FID (Frame Interface Definitions) + native modules.
- Explain how this integrates with MixedBody, SOL‑anchored Frame statements, and validation.
- Provide examples for Python and TypeScript.

What Is FID?
- A small, target‑neutral description of external capability surfaces (e.g., networking, filesystem, timers).
- Used by the compiler for:
  - Signature/arity/type validation at the Frame boundary
  - Documentation and IDE support (intellisense, jump‑to‑def in future)
  - Optional codegen hints (naming or import location), without locking into a specific library
- FID does not generate adapter shims by default; we prefer calling native code directly in MixedBody regions.

Relationship to MixedBody
- MixedBody is authoritative inside action/operation/handler bodies: native text/AST is interleaved with Frame statements (MIR: transitions, forward, stack ops, returns).
- FID provides “what” (interface contract); MixedBody provides “how” (actual native invocation). Examples:
  - Python: `resp = await aiohttp.get(url)` inside a MixedBody native span.
  - TypeScript: `const data = await fetch(url).then(r => r.text());` inside a MixedBody native span.

When To Use FID vs. Native Ad‑hoc Calls
- Use FID when the symbol (function/class/method) is meant to be portable and validated across targets.
- Use ad‑hoc native calls when code is intentionally target‑specific and not intended to be validated across targets.

Minimal FID Manifest (Draft)
- Location: per project root (default) or alongside the spec under `fid_manifest.json`.
- Purpose: declare known capability symbols for validation and docs.

Example `fid_manifest.json` (draft):
```json
{
  "version": "0.1",
  "capabilities": [
    {
      "name": "http.get",
      "targets": {
        "python_3": { "module": "aiohttp", "symbol": "get", "async": true },
        "typescript": { "module": "node-fetch", "symbol": "default", "async": true }
      },
      "signature": {
        "params": [ { "name": "url", "type": "string" } ],
        "returns": { "type": "string", "async": true }
      }
    }
  ]
}
```

Validation Behavior (Draft)
- If a MixedBody references a FID symbol, the validator can:
  - Check arity: reported at Frame source lines
  - Flag missing/mismatched async usage (e.g., not awaited when required)
  - Offer advisory diagnostics on unknown or misspelled symbols
- If a symbol is not in FID, it’s treated as ad‑hoc native; no cross‑target checks are applied.

Python Example (MixedBody + FID)
```frame
system Fetcher {
  interface:
    async fetch(url)

  actions:
    run() {
      # native Python body
      import aiohttp
      async def _go():
        async with aiohttp.ClientSession() as s:
          async with s.get(url) as r:
            data = await r.text()
            print("Got:", data[:50])
      -> $Done
    }
  machine:
    $Init { fetch(url) { -> $Running(url) } }
    $Running(url: string) { $>() { -> $Done } }
    $Done { $>() { return } }
}
```

TypeScript Example (MixedBody + FID)
```frame
system FetcherTS {
  interface:
    async fetch(url)

  actions:
    run() {
      // native TS body
      import fetch from 'node-fetch'
      const go = async () => {
        const res = await fetch(url)
        const text = await res.text()
        console.log(text.slice(0, 50))
      }
      -> $Done
    }
  machine:
    $Init { fetch(url) { -> $Running(url) } }
    $Running(url: string) { $>() { -> $Done } }
    $Done { $>() { return } }
}
```

Compiler Integration — Current
- MixedBody is authoritative and validated structurally; native code is emitted deterministically.
- Validation reads `fid_manifest.json` (when present) for advisory checks.
- No automatic stubs/shims are generated; imports are written directly in native text.

Compiler Integration — Roadmap
- Optional per‑target import helpers (avoid duplicated imports across files in multi‑file builds).
- IDE support: hover docs and jump‑to‑def from FID names to manifest entries.
- Source maps: compose native parser spans with MIR expansions across MixedBody.

Authoring Tips
- Keep FID names short and stable; encode dot‑separated namespaces like `http.get`.
- Prefer async markers in FID where the native function is async (validator can ensure `await`).
- Add language‑specific notes in FID if symbol shapes diverge subtly across targets.

FAQ
- Q: Do I have to write FID?
  A: No. FID is optional. Use it when you want cross‑target validation/advice and documentation. Otherwise, write native MixedBody code directly.
- Q: Where do imports go?
  A: In the MixedBody native code for each target. Keep imports idempotent or rely on the emitter to hoist shared imports in multi‑file builds (planned).
- Q: Is FSL still supported?
  A: No. FSL is historical. Use FID + native modules with MixedBody.
