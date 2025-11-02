# Bug #055: TypeScript async runtime lacks native socket integration

## Metadata
```yaml
bug_number: 055
title: "TypeScript async runtime lacks native socket integration"
status: Open
priority: High
category: CodeGen
discovered_version: v0.86.25
fixed_version: 
reporter: Codex
assignee: 
created_date: 2025-10-30
resolved_date: 
```

## Description
Frame’s debugger harness now relies on host-native libraries. Python uses `asyncio`, and the TypeScript target should lean on Node’s `net.Socket`. The legacy approach wrapped these helpers behind `native module runtime::socket`; that abstraction no longer exists. Today the TypeScript pipeline still expects the old declaration, so Frame specs that import `net` directly cannot be parsed or emitted without falling back to embedded `#[target: typescript]` blocks.

## Reproduction Steps
1. Run the TypeScript regression suite with the native spec:
   ```bash
   python3 framec_tests/runner/frame_test_runner.py \
       --languages typescript \
       --categories language_specific_typescript --transpile-only \
       --include test_runtime_protocol_native.frm
   ```
2. The transpile step fails because the compiler rejects the inline Node statements (or auto-inserts the legacy runtime helpers).

## Test Case
```frame
@target typescript

import { Socket } from "net";

system RuntimeProtocolTs {
    actions:
        async connect(host, port) {
            const socket = new Socket();
            await new Promise<void>((resolve, reject) => {
                socket.once("connect", () => resolve());
                socket.once("error", (err) => reject(err));
                socket.connect({ host, port });
            });
            this.socket = socket;
            return
        }
}
```

## Expected Behavior
- Frame accepts native TypeScript imports (`import { Socket } from "net"`) without auxiliary pragmas.
- The `.fid` generator discovers the import, produces metadata for Node’s API, and the emitter outputs idiomatic Promise-based code.
- `tsc` compilation succeeds with no manual edits.

## Actual Behavior
- The parser still requires embedded `#[target: typescript]` sections to allow raw TypeScript statements, otherwise it errors out.
- Even with pragmas, the visitor leans on the deprecated `runtime::socket` helpers; Node imports are ignored.
- `.fid` metadata is not generated because our import discovery pipeline skips in-body imports.

## Impact
- **Severity**: High — blocks the debugger harness from running on TypeScript.
- **Scope**: Any Frame spec that needs native Node socket access or other host APIs.
- **Workaround**: None clean; developers must maintain the legacy `native module runtime::socket` declarations by hand.

## Technical Analysis
- Parser: TypeScript body grammar still enforces the legacy Frame-specific statement list and cannot consume arbitrary TypeScript without a pragma wrapper.
- Import discovery: We only collect Frame-level `import` statements. Imports inside target bodies are ignored, so `.fid` generation never runs.
- Visitor: Runtime helpers (`runtime_socket...`) are hardcoded; we do not emit genuine `import { Socket } from "net"` lines or map method calls to Node’s API.

### Root Cause
The native import architecture (Week 6+) never landed for the TypeScript backend. We are still tied to the Frame runtime helpers that mimic Python semantics.

### Affected Areas
- `framec/src/frame_c/parser.rs` — TypeScript body parser needs native statement support.
- `framec/src/frame_c/tools/decl_import.rs` — import discovery must harvest target-body imports and emit `.fid` files (e.g., for `@types/node`).
- `framec/src/frame_c/visitors/typescript_visitor.rs` — visitor should request Node imports and remove references to `runtime::socket`.

## Proposed Fix
1. **Parser upgrade**: allow TypeScript actions to contain native statements without nested pragmas.
2. **FID auto-generation**: detect `import { Socket } from "net"` and feed it into the declaration importer (TypeDoc over `@types/node`). Cache the resulting `.fid` in `.framec/cache/fid/typescript/node_net.fid`.
3. **Visitor update**: emit actual `import { Socket } from "net"` lines, preserve native statements, and drop the legacy runtime helper references.
4. **Regression coverage**: keep `framec_tests/language_specific/typescript/runtime/test_runtime_protocol_native.frm` and `.../declarations/test_runtime_socket_decl.frm` to ensure we never regress.

## Test Coverage
- [ ] Parser unit tests for native TypeScript actions
- [ ] Integration: regenerate `.fid` cache for `@types/node`
- [x] Regression: `framec_tests/language_specific/typescript/runtime/test_runtime_protocol_native.frm`
- [x] Regression: `framec_tests/language_specific/typescript/declarations/test_runtime_socket_decl.frm`

## Related Issues
- Bug #049 — Low TypeScript transpilation success rate.
- Bug #052 — TypeScript actions generate stubs despite proper imports.

## Work Log
- 2025-10-30: Original bug filed (legacy helper gap) — Codex
- 2025-11-02: Native import architecture defined — Docs updated

## Resolution
Pending.

### Fix Summary
_TBD_

### Verification
_TBD_

### Lessons Learned
_TBD_

---
*Bug tracking policy version: 1.0*
