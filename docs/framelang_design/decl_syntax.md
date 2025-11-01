# Native Declaration Syntax Proposal

## 1. Problem Statement
Bug #055 (TypeScript async sockets) highlighted that Frame specs currently inline target-specific code whenever they need to talk to native runtimes (e.g. Node's `net.Socket`). Python specs import helpers from `frame_runtime_py`, TypeScript specs drop `[target: typescript]` blocks, and each visitor/runtime must coordinate manually. We want a language feature that lets a spec declare "this helper exists" without embedding target-specific code.

## 2. Requirements
- **Ambient declarations:** describe APIs implemented by per-target runtimes (e.g. `frame_runtime_py`, `frame_runtime_ts`).
- **Cross-target parity:** declarations are language-agnostic; runtimes implement them.
- **Async aware:** declarations cover `async` functions and return types.
- **Error visibility:** optional metadata for expected exceptions/Promise rejections.
- **Type placeholders:** allow opaque handles (e.g. `SocketHandle`) even if Frame doesn’t map them to concrete types.
- **Validation:** compiler should error if a target lacks an implementation.

## 3. Existing Approaches to Reference
| Language | Mechanism | Notes |
| --- | --- | --- |
| TypeScript | `declare` keyword, ambient modules | Runtime provides the symbol; compiler trusts signature. |
| C / Rust | `extern` functions | Linker provides implementation. |
| WebIDL / gRPC | Interface definition languages | Generate cross-language bindings. |
| OCaml Modules | Interface (.mli) vs implementation (.ml) | Explicit signatures separate from bodies. |

## 4. Proposed Frame Syntax (Strawman)
```frame
native module runtime/socket {
    type SocketHandle

    async connect(host: string, port: int) -> SocketHandle
    async read_line(handle: SocketHandle) -> string
    async write_line(handle: SocketHandle, data: string)
    close(handle: SocketHandle)
}
```

Usage inside spec:
```frame
var handle = runtime/socket.connect(self.host, self.port)
var line = await runtime/socket.read_line(handle)
```

Implementation expectations:
- `frame_runtime_py.runtime.socket.connect` exported in Python runtime package.
- `frame_runtime_ts` exports `FrameSocketClient.connect` (or similar) and the visitor maps module names to runtime exports.

## 5. Compiler/Visitor Changes
1. **Parser:** new `native module` syntax parsing, storing function signatures.
2. **Symbol Table:** track native modules/functions so visitor can emit proper imports.
3. **Visitors:**
   - Python: emit `from frame_runtime_py import socket as runtime_socket` (or equivalent).
   - TypeScript: emit `import { FrameSocketClient as runtime_socket } from './frame_runtime_ts'`.
4. **Diagnostics:** error if declaration is used but runtime doesn’t export matching symbol.
5. **Optional metadata:** allow future extensions for error types, target-specific renames.

## 6. Migration Strategy
1. Introduce declaration syntax (no runtime changes yet).
2. Update runtimes to export the declared modules (e.g., rename `FrameSocketClient` exports to match module naming).
3. Refactor specs (starting with `runtime_protocol_ts`) to use the declaration instead of `[target: ...]` blocks.
4. Add compiler validation ensuring declared module exists for the active target.

## 7. Open Questions
1. **Binding style:** Should declarations describe module-level functions, injected objects, or both?
2. **Type mapping:** Do we need built-in `SocketHandle` semantics or can they stay opaque?
3. **Error handling:** Should declarations describe exceptions/Promise rejections? If yes, how does Frame surface them?
4. **Resolution order:** Should Frame allow overriding declarations per target (e.g., `@target typescript` provides alternative signature)?

## 8. Next Steps
- Gather feedback on syntax (e.g. `native module` vs `extern module`).
- Decide how the compiler maps module names to runtime exports (naming convention vs explicit annotations).
- Prototype parser + visitor changes for a small declaration (Socket client) and iterate.

## 7. Open Questions (Resolved for Active Targets)
1. **Binding style**: start with module-level functions (e.g. `runtime/socket.connect`). Works for Python imports, TypeScript named exports, and gives LLVM/C/C++ a clear symbol name.
2. **Type mapping**: treat declared types as opaque handles. Python/TypeScript can use objects; LLVM/C/C++ can pass pointers; Java can wrap long handles; Rust can use newtypes.
3. **Error handling**: leave errors implicit initially. Runtimes throw exceptions/rejections; Frame surfaces them according to target semantics. Metadata for errors can be added later if needed.
4. **Resolution order**: one shared declaration per module. Each runtime implements it; per-target overrides can be designed later if we hit diverging semantics.

## 9. Target Implementation Notes
| Target | Mapping Strategy | Error Surface | Handle Representation |
| --- | --- | --- | --- |
| Python | `frame_runtime_py.socket` module functions | Python exceptions | Real objects (asyncio stream wrappers) |
| TypeScript | `frame_runtime_ts` named exports (e.g. `FrameSocketClient`) | Promise rejection | Class instance / object |
| LLVM (future) | Runtime shim functions (FFI) | Return codes / panic | `i8*` opaque pointer |
| C | `extern` functions in runtime header | Return status + errno helper | `void*` handle |
| C++ | `extern "C"` functions or namespace helpers | Exception or return status | `void*` / RAII wrapper |
| Rust | `extern "C"` + safe wrapper module | `Result` mapped to status code | `*mut c_void` newtype |
| Java | Static methods (e.g. `FrameRuntime.Socket`) | Checked exceptions | Long token / Socket object |

Implementations translate the declaration to the idioms of each runtime but maintain a shared contract.
