# Native Imports & Frame Interface Definitions (.fid)

Frame is pivoting from bespoke “Frame-flavoured” declarations to a native-first integration model. Specifications now author imports exactly the way a Python, TypeScript, or Rust developer would. The compiler then discovers those imports, introspects the referenced native modules, and generates cached **Frame Interface Definition** files (`*.fid`) that play the same role as C headers: they describe callable symbols and types so Frame can type-check and emit code, but they do not wrap or rename the native API.

## 1. Authoring Specs with Native Imports

- **Python example**

```frame
@target python

import asyncio

system SocketShell {
    actions:
        async connect_and_read(host, port) {
            var reader_writer = await asyncio.open_connection(host, port)
            var reader = reader_writer[0]
            var writer = reader_writer[1]

            try:
                var data = await reader.readline()
                return data.decode()
            finally:
                writer.close()
                await writer.wait_closed()
        }
}
```

- **TypeScript example**

```frame
@target typescript

import { Socket } from 'net';

system SocketShell {
    actions:
        async connectAndRead(host, port) {
            const socket = new Socket();
            const chunk = await new Promise<Buffer>((resolve, reject) => {
                socket.once('data', resolve);
                socket.once('error', reject);
                socket.connect({ host, port });
            });
            socket.destroy();
            return chunk.toString('utf8');
        }
}
```

No `native module` blocks are authored by hand. The developer writes idiomatic code for the target language, and any helper they import is considered a candidate for auto-generation.

## 2. Generating `.fid` Files

- `framec decl --config ...` (command name retained for continuity) runs per target.
- Each adapter (TypeScript via TypeDoc, Python via `inspect`/type hints, etc.) reads the native module, resolves exports, and emits a `.fid` file containing Frame-readable metadata.
- Output files live in the build cache (e.g. `.framec/cache/fid/typescript/runtime_socket.fid`) and should **not** be checked in.

### Sample `.fid` (TypeScript)

```fid
# Auto-generated from @types/node/dist/net.d.ts
module typescript::node::net {
    type Socket

    Socket.connect(port: number, host?: string): Socket
    Socket.once(event: string, listener: (...args: any[]) => void): Socket
    Socket.destroy(): void
}
```

Notes:

- Names follow the native casing (`connect`, `once`, etc.).
- Member functions are qualified with their owning class (`Socket.once`).
- For Python standard library modules (e.g., `asyncio`) the importer inspects runtime objects and type annotations to derive equivalent `.fid` entries.
- Return annotations mirror the native type system. The compiler lowers them to Frame-friendly representations but keeps the original semantics.

## 3. Compiler Consumption

When Frame compiles a spec:

1. The scanner records native import statements per target body.
2. The declaration generator is invoked (unless a valid cache entry exists) to refresh corresponding `.fid` files.
3. The `.fid` data is loaded into the symbol table so Frame can check calls made from Frame-shell sections (actions, operations, etc.).
4. Visitors emit whatever import syntax the target needs (often replaying exactly what the developer wrote) without renaming helpers.

## 4. Updating Existing Docs & Grammars

- **Common grammar (`target_language_specifications/common/frame_common_grammar.md`)** now treats native import statements as first-class statements inside target bodies.
- **Per-language grammar files** (Python, TypeScript, etc.) define the precise import syntax accepted in their body lexers.
- **HOW_TO.md** references `.fid` output and clarifies that declaration generation is an automated step, not a manual authoring task.
- This document replaces the old `decl_syntax.md`; any references should point here.

## 5. Roadmap Follow-ups

- Automate discovery: link scanned import paths to the declaration generator automatically (currently done via explicit configs).
- Extend adapters for additional targets (Rust, C/C++, Java) using language-appropriate reflection or metadata.
- Track version hashes of source modules to invalidate cache entries when runtimes change.
