# Native Imports & Frame Interface Definitions (.fid)

Frame is pivoting from bespoke “Frame‑flavoured” declarations to a native‑first integration model. Specifications now author imports exactly the way a Python, TypeScript, or Rust developer would. The compiler then discovers those imports, introspects the referenced native modules, and generates cached Frame Interface Definition files (`*.fid`) that play the same role as C headers: they describe callable symbols and types so Frame can type‑check and emit code, but they do not wrap or rename the native API.

## 1. Authoring Specs with Native Imports

- Python example

```frame
@target python

import asyncio

system SocketShell {
    actions:
        async connect_and_read(host, port) {
            var reader_writer = await asyncio.open_connection(host, port)
            var reader = reader_writer[0]
            var writer = reader_writer[1]

            try {
                var data = await reader.readline()
                return data.decode()
            } finally {
                writer.close()
                await writer.wait_closed()
            }
        }
}
```

- TypeScript example

```frame
@target typescript

import { Socket } from "net";

system SocketShell {
    actions:
        async connect(host, port) {
            const socket = new Socket()
            await new Promise<void>((resolve, reject) => {
                socket.once("connect", () => resolve())
                socket.once("error", (err) => reject(err))
                socket.connect({ host, port })
            })
            this.socket = socket
            return
        }
}
```

No `native module` blocks are authored by hand. The developer writes idiomatic code for the target language, and any helper they import is considered a candidate for auto‑generation.

## 2. Generating `.fid` Files

- `framec fid import --config <manifest>` runs per target.
- Each importer (TypeScript via TypeDoc, Python via `inspect`/type hints, etc.) reads the native module, resolves exports, and emits a `.fid` file containing Frame‑readable metadata.
- Output files live in the build cache (e.g. `.frame/cache/fid/typescript/typescript__node__net.fid`) and should not be checked in.

### Sample `fid_manifest.json`

```json
{
  "sources": [
    {
      "@target": "typescript",
      "resources": [
        {
          "file": {
            "uri": "../../../../../node_modules/@types/node/dist/net.d.ts",
            "modules": [
              { "module": "net", "import": ["Socket.*"] }
            ]
          }
        }
      ]
    },
    {
      "@target": "python",
      "resources": [
        {
          "file": {
            "uri": "stdlib:asyncio",
            "modules": [
              { "module": "asyncio", "import": ["open_connection", "StreamReader.*", "StreamWriter.*"] }
            ]
          }
        }
      ]
    }
  ]
}
```

`framec fid import --config fid_manifest.json` processes every entry in the manifest. Each manifest describes a single project; add additional `sources` blocks for any other targets the project needs.

**Field guide**

- `@target` — Frame target language that will consume the generated declarations. The importer uses this to pick the correct tooling.
- `resources` — list of resource descriptors. Each descriptor is a typed object so we can support different kinds of inputs:
  - `file`: read a specific file the ecosystem treats as metadata (e.g., a `.d.ts`).
    - `uri` — file path.
    - `modules` — array mapping native module names to the symbols you care about.
      - `module` — native import string (e.g., `net`, `fs/promises`).
      - `import` — array of selectors to include. Dotted names address members (e.g., `Socket.connect`). Wildcards are supported:
        - `"*"` includes all top‑level exported symbols from the module.
        - `"Name.*"` includes the type/class `Name` itself and all of its members (e.g., `Socket.*`). You do not need to list `"Name"` separately.
        - Wildcards are importer‑scoped and expand only within this `module`. Use multiple `modules` entries to cover additional native modules.
  - `module`: introspect a runtime module by name (Python, Rust, etc.).
    - `name` — native module/package identifier (e.g., `asyncio`).
    - `import` — selectors to include from that module. Wildcards behave the same as above (e.g., `StreamReader.*`).

The generated `.fid` namespace is derived from `@target` and the native module string (for example, `@target=typescript` + `module=net` → `typescript::node::net`). An explicit override may be added later if needed.

At compile time the loader searches for cached `.fid` files in `.frame/cache/fid/<target>` starting from the spec’s directory and walking up the tree. Additional lookup locations can be supplied via the `FRAME_FID_PATH` environment variable; entries may include a `{target}` placeholder (e.g. `/opt/frame/fid/{target}`) or point directly at a target‑specific directory.

When a spec imports a native helper but the corresponding declaration cannot be found, the compiler raises a targeted diagnostic (for example: “Native helper `<name>` is imported for this target but no declaration was loaded. Run `framec fid import` for the active target and retry.”). Regenerate the cache whenever runtime code or third‑party packages change, or delete the stale directory under `.frame/cache/fid/<target>` before rerunning the importer.

### CLI Usage

- `framec fid import --config <FILE>` is the canonical command.
- Pass `--force` to overwrite existing cached files, `--dry-run` to inspect work without writing, and `--allow-missing` when prototypes are still filling out runtime coverage.
- Keep `.fid` artefacts out of version control. Add `.frame/cache/fid` to project ignore rules and regenerate as part of your build/test workflow.

### Sample `.fid` (TypeScript)

```fid
# Auto-generated from @types/node/dist/net.d.ts
module typescript::node::net {
    type Socket

    Socket.connect(options: { host: string, port: number }): void
    Socket.write(data: string | Uint8Array): bool
    Socket.destroy(): void
}
```

Notes:

- Names follow the native casing (`connect`, `write`, etc.).
- Member functions are qualified with their owning class (`Socket.write`).
- For Python standard library modules (e.g., `asyncio`) the importer inspects runtime objects and type annotations to derive equivalent `.fid` entries.
- Return annotations mirror the native type system. The compiler lowers them to Frame‑friendly representations but keeps the original semantics.
- Module and namespace segments mirror the actual import path (e.g., `python::asyncio`, `typescript::node::net`) so diagnostics and tooling can point back to the native module a spec referenced.

## 3. Compiler Consumption

When Frame compiles a spec:

1. The scanner records native import statements per target body.
2. The declaration generator is invoked (unless a valid cache entry exists) to refresh corresponding `.fid` files.
3. The `.fid` data is loaded into the symbol table so Frame can check calls made from Frame‑shell sections (actions, operations, etc.).
4. Visitors emit whatever import syntax the target needs (often replaying exactly what the developer wrote) without renaming helpers.

## 4. Updating Existing Docs & Grammars

- Common grammar (`target_language_specifications/common/frame_common_grammar.md`) now treats native import statements as first‑class statements inside target bodies.
- Per‑language grammar files (Python, TypeScript, etc.) define the precise import syntax accepted in their body lexers.
- HOW_TO.md references `.fid` output and clarifies that declaration generation is an automated step, not a manual authoring task.
- This document replaces the old `decl_syntax.md`; any references should point here.

## 5. Roadmap Follow‑ups

- Expand native‑import harvesting to additional targets (Python importer, future runtimes) so every generator benefits from the shared discovery path.
- Extend importers for additional targets (Rust, C/C++, Java) using language‑appropriate reflection or metadata.
- Track version hashes of source modules to invalidate cache entries when runtimes change.

## 6. Versioning & Lockfile

To make fid generation reproducible, the importer writes a lockfile that records exactly what versions and files were used to produce the cache.

- File: `fid.lock.json` at the project root (or alongside `fid_manifest.json`).
- Schema: `docs/schemas/fid_lock.schema.json` (use via the `$schema` key).
- Purpose: capture package versions (e.g., `@types/node@20.x`, `asyncio` on Python 3.12), content hashes, and toolchain versions so CI/devs regenerate the same `.fid` outputs.

Sample `fid.lock.json` snippet:

```json
{
  "$schema": "docs/schemas/fid_lock.schema.json",
  "lockVersion": "1",
  "generatedAt": "2025-11-03T12:34:56Z",
  "toolchain": {
    "framec": "v0.86.26",
    "importers": { "typescript": "typedoc@0.25.9", "python": "introspect@1" }
  },
  "entries": [
    {
      "@target": "typescript",
      "resource": { "file": { "uri": "node_modules/@types/node/dist/net.d.ts" } },
      "modules": [ { "module": "net", "import": ["Socket.*"] } ],
      "fingerprint": {
        "algorithm": "sha256",
        "value": "<content-hash>",
        "package": {
          "manager": "npm",
          "name": "@types/node",
          "version": "20.12.7",
          "resolved": "node_modules/@types/node"
        }
      },
      "outputs": [
        { "namespace": "typescript::node::net", "path": ".frame/cache/fid/typescript/typescript__node__net.fid" }
      ]
    }
  ]
}
```

Guidelines

- For file resources from package managers, record the package name, manager, version, and resolved path plus a content hash of the files actually read by the importer.
- For stdlib resources, record the runtime version (e.g., `python=3.12.2`) and a hash of the reflected stubs.
- Always include the importer/toolchain versions so upgrades cause a deliberate lock refresh.
- Commit `fid.lock.json` for reproducible builds; the `.fid` cache remains untracked and is reproducible from the lock.

## 6. Formal Schemas

### 7.1 fid_manifest.json — JSON Schema

The canonical schema is versioned in the repo at:

- `docs/schemas/fid_manifest.schema.json`

Use it in your manifest via the `$schema` property (adjust the path as needed for your project layout):

```json
{
  "$schema": "docs/schemas/fid_manifest.schema.json",
  "outputDir": "...",
  "sources": [ { "@target": "typescript", "resources": [ /* ... */ ] } ]
}
```

Notes:

- The schema validates selector wildcards (`*`, `Name.*`) and dotted member selectors (`Name.member`).
- Extend the `@target` enum as new backends are added.

### 7.2 .fid Text Format — EBNF

The `.fid` files are a compact textual description of modules, types, and callables used by Frame for checking and codegen.

```
file        ::= { module_def }
module_def  ::= "module" ns "{" { fid_item } "}"
ns          ::= ident { "::" ident }

fid_item    ::= type_def | fun_def | method_def | newline

type_def    ::= "type" ident newline

fun_def     ::= ident "(" [ params ] ")" ":" type_name newline
method_def  ::= ident "." ident "(" [ params ] ")" ":" type_name newline

params      ::= param { "," param }
param       ::= ident ":" type_name

type_name   ::= ident
             | ident "|" type_name         (* unions allowed in backends that support them *)
             | ident "[" "]"              (* array-like shorthand *)
             | "{" field { "," field } "}"
field       ::= ident ":" type_name

ident       ::= letter { letter | digit | "_" }
letter      ::= "A".."Z" | "a".."z"
digit       ::= "0".."9"
newline     ::= "\n"
```

Conventions:

- Methods are qualified with their owning type (`Socket.write`).
- Return type `void` is used when the native API has no meaningful return value.
- Backends may project richer native types into simplified `type_name` forms (e.g., discriminated unions or structural objects).
