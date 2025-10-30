# Bug #055: TypeScript async runtime lacks socket helpers for Frame debugger

## Metadata
```yaml
bug_number: 055
title: "TypeScript async runtime lacks socket helpers for Frame debugger"
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
The new Frame rebuild harness uses an asyncio-driven runtime specification (`runtime_protocol.frm`) to own the debugger socket protocol. The Python target works correctly, but the TypeScript target fails to compile because the generated code depends on helpers (newline-delimited socket reads/writes, UTF-8 decoding, structured `try/except` translation) that the TypeScript runtime/emitter does not implement.

## Reproduction Steps
1. From the VS Code editor repository, transpile the async runtime spec to TypeScript:
   ```bash
   /Users/marktruluck/projects/frame_transpiler/target/release/framec -l typescript rebuild/runtime_protocol.frm > rebuild/RuntimeProtocol.ts
   ```
2. Compile the generated file:
   ```bash
   npx tsc -p rebuild/tsconfig.json
   ```
3. Observe multiple `TS1005`/`TS1434` errors and missing helper references (e.g., `reader.readline()`, `.decode()`, `.encode()`).

## Test Case
```frame
import asyncio
import json
import os
import sys

system RuntimeProtocol {
    interface:
        run()

    machine:
        $Idle {
            async run() {
                await self.runtimeMain()
                -> $Terminated
            }
        }
        # ... see rebuild/runtime_protocol.frm for full spec
}
```

## Expected Behavior
- `framec -l typescript` should emit Promise-based helpers that mirror the asyncio implementation.
- The generated TypeScript should compile under `npx tsc -p rebuild/tsconfig.json` without manual edits.

## Actual Behavior
- The output references Python stream APIs verbatim (`await this.reader.readline()`, `.decode`, `.encode`) and emits malformed try/catch blocks.
- TypeScript compilation fails with syntax errors (`TS1005`, `TS1434`) and missing helper implementations.

```
rebuild/RuntimeProtocol.ts(2731,96): error TS1005: '}' expected.
rebuild/RuntimeProtocol.ts(2774,65): error TS1434: Unexpected keyword or identifier.
rebuild/RuntimeProtocol.ts(2856,47): error TS1127: Invalid character.
```

## Impact
- **Severity**: High — blocks cross-language debugger harness modernization.
- **Scope**: Any Frame spec that relies on async socket handling for TypeScript.
- **Workaround**: None practical; must avoid async sockets or patch generated TS manually.

## Technical Analysis
- Python output succeeds because the asyncio runtime already exists.
- TypeScript lacks equivalent runtime helpers for:
  - Newline-delimited socket reads (`StreamReader.readline` analogue).
  - Buffer/string encoding helpers (`decode`, `encode`).
  - Proper translation of bare `except:` clauses into structured `try/catch`.
- The generator inserts placeholder variables (`var output: any; // TODO`) and leaves Python idioms intact, leading to invalid TS.

### Root Cause
Async socket support has not been implemented in the TypeScript runtime library or emitter; the transpiler copies Python semantics directly without language-specific lowering.

### Affected Files
- `src/typescript/codegen/runtime_writer.rs` (needs async helper support)
- `src/typescript/codegen/action_writer.rs` (needs try/catch translation tweaks)
- TypeScript runtime library (`frame_async.ts`) missing socket utilities

## Proposed Solution

### Option 1: Implement async socket helpers in TypeScript runtime
- Add a small adapter around Node's `net.Socket` that exposes `readLine`, `writeUtf8`, and Promise-based close semantics.
- Teach the emitter to map Python calls (`reader.readline`, `.decode`) to these helpers.
- Update try/except lowering to hyphenate bare `except:` into `catch (err)` blocks.
- Pros: Keeps Frame spec portable; minimal changes to Frame source.
- Cons: Requires coordinated runtime + emitter update.

### Option 2: Introduce Frame-level abstractions for socket I/O
- Add intrinsic actions (`frameRuntimeSocketReadLine`, `frameRuntimeSocketWrite`) to the FSL and call those from the spec.
- Map the intrinsics per language (Python uses asyncio, TypeScript uses Promises).
- Pros: Cleaner separation of transport API from Frame specs.
- Cons: Larger change to Frame language/runtime surface area.

## Test Coverage
- [ ] Unit test added
- [ ] Integration test added
- [ ] Regression test added
- [x] Manual testing completed

## Related Issues
- Bug #049 — TypeScript transpilation rate lower than Python (potentially linked runtime gaps)
- Bug #052 — TypeScript actions generate stubs despite proper imports

## Work Log
- 2025-10-30: Initial report while porting rebuild debugger runtime — Codex

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
