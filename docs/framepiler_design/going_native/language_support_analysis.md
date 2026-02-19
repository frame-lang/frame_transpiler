# Going Native: Language Support Analysis (Python, TypeScript/JS, C, C++, Rust, Java)

Status: Draft for “going_native” branch
Last updated: 2025‑11‑05

## Roadmap Links
- C Target Plan — Going Native: `docs/framepiler_design/going_native/c_backend_plan.md`
- C Body Grammar (target‑native mixed bodies): `docs/framelang_design/target_language_specifications/c/c_body_grammar.md`
- C++ Target Plan — Going Native: `docs/framepiler_design/going_native/cpp_backend_plan.md`
- C++ Body Grammar: `docs/framelang_design/target_language_specifications/cpp/cpp_body_grammar.md`
- Java Target Plan — Going Native: `docs/framepiler_design/going_native/java_backend_plan.md`
- Java Body Grammar: `docs/framelang_design/target_language_specifications/java/java_body_grammar.md`
- Rust Target Plan — Going Native: `docs/framepiler_design/going_native/rust_backend_plan.md`
- Rust Body Grammar: `docs/framelang_design/target_language_specifications/rust/rust_body_grammar.md`
 - Source Map Spec: `docs/framepiler_design/going_native/source_map_spec.md`
 - AST Dump Spec: `docs/framepiler_design/going_native/ast_dump_spec.md`

## Runtime ABI and Shims (Cross‑Target)
- Single C ABI: reuse `frame_runtime_llvm` exported `extern "C"` symbols for events, compartments, kernel loop, stack ops, and prints to guarantee identical semantics across native targets.
- Per‑language shims:
  - Rust: safe wrapper module/crate with newtypes for opaque pointers, `CString` conversions, and panic‑free calls returning `Result`.
  - C++: header‑only RAII helpers (unique_ptr‑style deleters) and `std::string` conversion helpers; noexcept wrappers around C calls.
  - Java: JNI class that holds native pointers as `long`/`ByteBuffer`, implements `AutoCloseable`, forwards calls to the C ABI.
- ABI requirements:
  - Stable signatures (no varargs), opaque pointers for handles, copy‑in/copy‑out string policy, status codes (no unwinding across the boundary), and an `abi_version()` function for early mismatch detection.
- Packaging:
  - Ship `include/frame_runtime_c.h` (canonical header), optional `frame_runtime_cpp.hpp`, optional `frame_runtime_rs`, and `frame_runtime_jni` (class + native lib) with examples for macOS/Linux/Windows.
- Build/linking:
  - Prefer dynamic libs; provide guidance for rpath/loader_path/ORIGIN and Windows import libs; sample `clang/clang++/rustc/javac` commands.
- Testing/CI:
  - ABI conformance tests on each OS; per‑language smoke runs (enter/exit, parent forward, stack multi‑pop, typed params); leak checks via ASan/Valgrind.
- Risks/mitigations:
  - String/handle leaks (hide alloc/free in shims), ABI drift (versioning + tests), platform linking (packaged examples).

## Implementation Difficulty and Priorities
- Easiest → Hardest: C → C++ → Rust → Java.
- Java is hardest due to:
  - JNI layer requirement, cross‑platform packaging of native libs, `java.library.path` management.
  - GC vs native lifetime rules (pointers as `long`/`ByteBuffer`, ensuring deterministic frees).
  - String/array bridging cost and pinning semantics, exception mapping (no throw across ABI).
  - Tooling friction in the runner (JDK + JNI toolchain).

## Goals
- Make Frame maximally accommodative of target‑native syntax inside the Frame structure (modules/systems/classes/functions/actions/operations/handlers).
- Treat native code as the host inside member bodies, and embed Frame semantics via precise inline Frame statements (MIR) at well‑defined, deterministic boundaries.
- Preserve excellent diagnostics (dual mapping), deterministic codegen, and predictable runtime behavior.

## Architecture Recap (authoritative for going_native)
- Partition (Frame‑outer): Module outline and structural ranges
  - ModulePartitions: Prolog (@target), NativeImport regions, FrameOutline, Body ranges (per action/operation/handler)
  - OutlineScanner computes exact body start/end (brace/indent aware per target)
- Segment (Native‑outer): Body regions become ordered items
  - [NativeAst | NativeText | FrameStmt(MIR)] with target‑native spans
  - SOL‑anchored FrameStmt detection, full token patterns:
    - Transition: `-> $State(…exprs…)`
    - Parent forward: `=> $^`
    - State stack: `push$`, `pop$`
- MixedBody authoritative: all actions/operations/handlers are MixedBody; no legacy “Frame statements” parsing inside handlers
- MIR expansion (B2): Visitors expand FrameStmt into target‑native AST or minimal text recipes with mapping
- Source maps: compose native parser spans + MIR expansions to precise frame/target mappings
- Pseudo‑symbol desugar (early): e.g., `system.return` → target‑native expression per runtime

## Cross‑Cutting Concerns
- SOL anchoring: Only recognize FrameStmt at first non‑whitespace column; ignore in strings/comments/heredocs/templates
- Whitespace/EOL/BOM: Unicode whitespace treated as whitespace; support LF/CRLF and normalize BOM
- Comments: Use target‑native comments inside bodies
- Imports: Preserve native import declarations (module‑level and inside bodies if target permits); linker consolidates only where safe (TS multi‑file)
- Async/concurrency: Respect target’s model; ensure MIR recipes return/control‑flow correctly after transitions
- Exceptions/errors: Keep target semantics in native code; Frame runtime glue uses well‑defined error paths (e.g., try/except blocks generated where needed)
- Symbol model: Keep Frame’s Arcanum frame‑centric; maintain native symbol index sidecar per body for diagnostics; do not merge into Arcanum

---

## Python

### Lexing/Parsing
- Indentation‑sensitive; OutlineScanner must compute body ranges using indent/dedent stacks.
- Strings: single/double/triple quotes; f‑strings; raw strings; bytes. Segmenter ignores Frame‑statement‑like tokens inside these.
- Comments: `#` single line; ignore Frame tokens in comments.

### Embedding in Frame
- Functions/actions/operations/handlers: MixedBody always. Use rustpython for native‑only bodies; segmentation for mixed.
- Classes: Python classes can be defined in native bodies (actions or dedicated native modules). Frame classes remain Frame constructs; native classes are just emitted code.
- Modules/packages: `import`/`from … import …` are preserved; linker must avoid duplicating or moving them incorrectly. Prefer module‑level imports above member bodies.
- Systems: Represented as Python classes with a runtime kernel; MIR recipes for transitions/forward/stack ops expand to deterministic methods that set/return `return_stack[-1]` and adjust compartments.

### Signatures
- Full Python function syntax inside native bodies is supported (pos‑only `/`, kw‑only `*`, `*args`, `**kwargs`, annotations, overload stubs). For Frame action/operation headers, either:
  - Option A (now): keep portable subset; place full Python signatures in native helpers called from actions
  - Option B (later/optional): allow target‑native headers and normalize to internal form for codegen; more work (semantics across targets unclear)

### Async
- Native `async def` + `await` are permitted in native bodies. Guards: warn on `await` in non‑async handlers.

### Tricky Issues
- Boundary correctness: ensure the body end excludes dedent that closes the Frame block; avoid bleeding native code past action boundary.
- f‑string braces within nested expressions; ensure Segmenter ignores `${…}` in f‑strings properly (we currently do for TS templates; Python f‑strings differ).
- Decorators and top‑level definitions inside bodies: Python allows defs inside defs; preserve without reordering.

### Tests to Add
- Large native‑only action with: decorators, dataclasses, descriptors, context managers, pattern matching, `*args/**kwargs`, f‑strings, comprehensions, `match/case` guards.
- Mixed bodies with multiple FrameStmt interleavings; returns after transitions.

---

## TypeScript / JavaScript

### Lexing/Parsing
- Braces, semicolons optional; OutlineScanner tracks brace depth. Segmenter supports backtick templates with nested `${…}`; ignores Frame tokens inside comments and strings.

### Embedding in Frame
- Functions/actions/operations/handlers: MixedBody; SWC for native‑only, segmentation for mixed; MIR expansions to SWC AST (B2) for determinism.
- Classes: TS classes can live inside native bodies or as separate modules; Frame classes remain Frame constructs with generated TS classes.
- Modules/imports: ESM/CJS variations; linker should consolidate runtime imports once, preserve relative imports between generated files. Multi‑file support requires consistent output dir and module path mapping.
- Systems: Represented as TS classes with runtime kernel; MIR expansions to set `returnStack[stack.length-1]`, adjust compartments, etc.

### Signatures
- TS type annotations, generics, union/intersection, optional params, default params are allowed in native bodies. For Frame headers, keep portable subset unless we adopt “native headers”.

### Async
- `async/await` and Promises supported; warn if used in non‑async handlers where runtime isn’t async.

### Tricky Issues
- Arrow `=>` conflicts: we require `=> $^` to avoid collisions.
- Module resolution (multi‑file): ensure imports resolve relative to generated outputs; consolidate shared runtime import.
- Target libs (DOM/node): tests need lib assumptions clear (node vs browser).

### Tests to Add
- Mixed bodies with complex templates `${{ a: () => ..}}`, nested braces, multi‑Frame‑statement bodies, unreachable code after transitions.
- Multi‑file linking with shared runtime and relative imports.

---

## C

### Lexing/Parsing
- Preprocessor, macros, `#include`; OutlineScanner must skip preprocessor lines and treat them as native. Strings, char literals; comments `//`, `/* */`.

### Embedding in Frame
- Functions/actions/operations/handlers: MixedBody with segmentation; no widely used C AST in Rust with comment retention—start with tolerant segmentation + pattern checks; optionally integrate tree‑sitter‑c for AST slices.
- Modules/headers: C uses headers and linkers; “native imports” become `#include` zones; avoid reordering. For declarations, prefer a separate native module block or fid_manifest mapping.
- Systems: Emit runtime kernel C wrappers calling the Frame runtime ABI (if LLVM backend is primary, prefer that path for C family).

### Signatures
- Allow full C signatures inside native helpers; Frame headers remain portable or use a constrained mapping to C types.

### Tricky Issues
- Macros/conditional compilation: segmentation must ignore Frame tokens inside `#if` branches; body end detection should not be confused by preprocessor directives.
- No exceptions: error handling through return codes; MIR glue should not presume exception semantics.

### Tests to Add
- Bodies with macros, preprocessor branches, raw string macros, multi‑line macros, nested comments.

---

## C++

### Lexing/Parsing
- Templates, namespaces, exceptions, lambdas, raw strings `R"(…)"`.
- `->` is pointer dereference; avoid false positives by requiring `-> $` for Frame transitions.

### Embedding in Frame
- MixedBody; AST via tree‑sitter‑cpp or cling‑based parser (future). Start with robust segmentation + boundary.
- Modules: headers/inline; preserve `#include` and `using` statements.
- Systems: C++ class wrapping the kernel; RAII semantics for compartments are helpful but must be deterministic.

### Signatures
- Full C++ syntax inside native helpers; Frame headers portable initially.

### Tricky Issues
- Templates and `>>` parsing ambiguities; lambdas with `=>`-like tokens in other languages don’t exist, but C++ uses `[](){}`.
- Exceptions vs noexcept; ensure MIR glue doesn’t throw in unexpected contexts.

### Tests to Add
- Templates with nested angle brackets, `std::variant`, lambdas, raw strings containing `-> $` tokens.

---

## Rust

### Lexing/Parsing
- Ownership/borrowing/lifetimes; raw strings r#"…"#; macros; attributes `#[…]`.
- `=>` appears in match arms; require `=> $^` to avoid conflict.

### Embedding in Frame
- MixedBody; rust parser (syn) for AST slices is heavy; begin with segmentation + outline; MIR expansions generate idiomatic code into impl blocks.
- Modules/crates: `use` and module paths preserved; linker must not reorder.
- Systems: Struct + impl with kernel integration; transitions produce compartment state changes; lifetimes kept internal to native segments; MIR glue minimal and deterministic.

### Signatures
- Full Rust function syntax inside native bodies; Frame headers portable for now.

### Tricky Issues
- Borrow checker/lifetimes around generated MIR code; expansions must avoid holding references across transitions.
- Macros: segmentation must ignore Frame tokens inside macro bodies.

### Tests to Add
- Matches with many arms (`=>`), raw strings with braces, macro invocations containing Frame‑statement‑like tokens.

---

## Java

### Lexing/Parsing
- Packages/imports; annotations; lambdas `->`; try‑with‑resources; generics.
- `->` conflicts with Frame transition; require `-> $` to avoid collision.

### Embedding in Frame
- MixedBody; a Java parser for AST slices is heavy; start with segmentation and outline.
- Modules/packages: preserve package/import lines at top; linker must leave ordering intact.
- Systems: Java class with kernel fields/methods; MIR expansions generate appropriate method calls and return statements.

### Signatures
- Full Java signatures inside native bodies; Frame headers portable for now.

### Tricky Issues
- Checked exceptions; MIR glue must either declare or handle them.
- Generics + type erasure; no major effect on MIR glue but affects native bodies.

### Tests to Add
- Lambdas with `->`, try‑with‑resources, nested classes, annotations with curly args containing `-> $`.

---

## C#

### Lexing/Parsing
- C# has `//` and `/* */` comments; verbatim/interpolated strings: `@"…"`, `$"…{expr}…"`, raw string literals (`"""…"""` in C# 11). Lambdas use `=>` (like TS/Java); require `=> $^` for Frame parent forward to avoid conflicts.
- OutlineScanner: brace depth for bodies; segmenter must ignore Frame tokens inside strings and comments; handle `$"…{ … }…"` interpolation blocks.

### Embedding in Frame
- Functions/actions/operations/handlers: MixedBody; start with segmentation + RegionGraph boundaries; later integrate a C# parser (Roslyn via subprocess or a Rust binding) for AST slices if needed.
- Classes/namespaces: preserve `using` directives and namespace declarations at top; linker must not reorder. For single‑file generation, keep using directives near top of output.
- Systems: C# class with kernel fields/methods; MIR expansions generate calls and set return values deterministically.

### Signatures
- Full C# method signatures inside native bodies (generics, params arrays, optional params, ref/out) are allowed. For Frame headers, keep portable subset for now; if we support target‑native headers, map `params` (varargs) to arrays internally; `ref/out` remain native‑only and not surfaced at Frame boundary initially.

### Async/await
- C# async/await mirrors TS/Py patterns; async handlers allowed with `async Task` and `await`. Ensure MIR transitions/forward terminate the logical event consistently even within async flows.

### Tricky Issues
- Interpolated strings `$"…{expr}…"`: segmenter must ignore `->`, `=>`, `$$` inside strings; interpolation expressions can contain braces and nested strings.
- `=>` lambdas vs `=> $^` Frame: SOL anchoring + `$^` requirement avoids ambiguity.
- `using` statement vs `using` directives: keep both intact and in order; do not interleave with Frame glue.

### Tests to Add
- Bodies with `$"…{ expr with braces }…"`, raw strings `"""…"""`, nested lambdas, and Frame‑statement‑like tokens inside strings/comments.
- MixedBody with transitions/forward/stack ops interleaved.
- Async handler with `await` and transitions.

---

## Tricky Cross‑Language Topics
- Return semantics after transitions: insert early `return` consistently post‑expansion; warn on unreachable native code
- Async vs sync: ensure the runtime for a language supports async where needed (e.g., Python asyncio vs TS Promises vs Java futures)
- Multi‑file linking (TS/Java): unify how consolidated imports are emitted; single shared runtime import to avoid redeclarations
- Preprocessors/macros (C/C++): scanning must ignore Frame tokens inside `#if` blocks and macros
- Formatting stability: native printers or consistent builders to avoid diffs across environments (SWC codegen for TS; Python prints via builder; Java/Rust TBD)

## Open Questions
1. Boundaries: Are we comfortable making MixedBody authoritative for all actions/operations/handlers (dropping legacy per‑token parsing inside handlers)?
2. Signatures at Frame boundary: Do you want target‑native signatures in Frame headers now (e.g., Python’s `/` and `*`, TS generics), or keep them inside native helpers for a while?
3. Java/C++/Rust priority: Which of these should we target next after Python/TS? (Effort: Java < C++ < Rust, given AST availability and printing.)
4. Error policy: For transitions that imply return, should we emit warnings or errors for native code that follows (e.g., “unreachable after transition”)?
5. C/C++ macro strategy: Is segmentation‑only acceptable initially, or should we integrate a parser (tree‑sitter) for AST validation sooner?
6. Source maps: Do you want per‑language source map artifacts standardized (e.g., v3 sourcemaps for TS, JSON mapping for Python/Java/Rust)?
7. Runtime imports (TS/Java): Should the linker always own shared runtime imports (preferred) or allow per‑module imports when users explicitly opt in?

## Next Steps
- Harden Python/TS body end‑range logic in OutlineScanner and Parser (exclude closing brace/dedent lines precisely)
- Make MixedBody path authoritative for member bodies; keep native parser validation optional
- Replace TS MIR glue with SWC AST recipes (B2) and maintain mapping
- Extend test suites per language with edge cases listed above

---

## Appendix: Fixtures & Acceptance Checklists

### TypeScript Fixtures
- Islands directory: `framec_tests/language_specific/typescript/islands/`
  - 10_template_literals.frm
  - 16_comments_with_frame_statement_tokens.frm
  - 17_template_literals_nested.frm
  - 22_typescript_island_mega_syntax.frm (comprehensive, no Node deps)
  - 23_indented_transition_and_unreachable.frm
  - 24_multiple_directives_interleaved.frm
  - 25_directives_in_block_comments.frm
  - 26_directives_in_strings_escaped.frm
  - 18_whitespace_tabs_spaces.frm, 19_nbsp_indentation.frm, 20_arrow_fn_not_segmented.frm, 21_directive_tokens_in_strings.frm

Note: A deeper nested template case with backticks inside backticks is deferred until the TS textual body-closer helper is unified.

TypeScript Segmenter Acceptance Checklist
- [x] Template literals with nested `${}` do not confuse Frame-statement detection
- [x] Comments containing Frame‑statement‑like tokens are ignored
- [x] Mixed native+Frame lines are split only at SOL Frame statements
- [x] Brace depth and string states do not leak across lines

### Python Fixtures
- Comprehensive island: `framec_tests/language_specific/python/test_python_island_mega_syntax.frm`

Python Segmenter Acceptance Checklist
- [x] Triple-quoted strings (''' and """) handled without false positives
- [x] f-strings ignore Frame-statement-like tokens inside expressions
- [x] Comments starting with `#` suppress detection on the line
- [x] Indent/dedent boundaries produce correct body ranges

### Cross‑Language Segmenter Checklist (apply per target)
- [ ] Strings (all forms for the language)
- [ ] Comments (single-line, block, doc comments)
- [ ] Raw/multiline strings (e.g., Python triple quotes, Rust raw, Swift multi-line)
- [ ] Interpolation (C#, Kotlin, Swift, JS templates)
- [ ] Preprocessor/macros/annotations handling (C/C++/Rust/Java/C#)
- [ ] SOL Frame-statement tokens recognized only at first non-whitespace column
- [ ] No detection inside strings/comments/templates/macros
- [ ] Brace/indent depth tracking stable across lines

---

## Planned Targets (Docs‑only, no code yet)

These sections capture what the segmenters must support and propose a comprehensive “mega” fixture outline for each language. We will not add code/tests until TS/Python MixedBody work is complete and validated.

### Java
- Strings/Comments: " ", character literals with escapes; // and /* */ comments; Javadoc.
- Annotations: @Annotation on classes/methods/params; ignore Frame-statement tokens inside annotations.
- Generics/Bounds: List<Map<String, List<Integer>>>; wildcards; method refs.
- Interop: package/import; nested classes/interfaces; enums.
- Acceptance checklist (apply Cross‑Language list with Java specifics):
  - [ ] Multi-line block comments containing Frame-statement-like tokens do not segment
  - [ ] Annotations before methods/classes do not segment
  - [ ] String/char escapes do not break SOL detection
- Proposed mega fixture (not added):
  - operations: body with imports, class-with-generic methods, streams/lamdas, try-with-resources, annotations, and SOL Frame statements on their own lines.

### Rust
- Strings: normal, byte, raw strings r#"…"# with variable hashes; format! macros.
- Comments: //, /* */ and doc comments ///, //!; macros and attributes #[derive(...)] etc.
- Macros: ensure we do not segment inside macro invocations or attribute payloads.
- Lifetimes/generics/where-clauses; modules/use; nested blocks.
- Acceptance notes:
  - [ ] Raw strings with # counts do not confuse scanning
  - [ ] Macro bodies do not segment even if they contain Frame-statement-like tokens
- Proposed mega fixture (not added):
  - operations: body with struct/enum impl blocks, generics, iterator adapters, format!/println!, attributes, and SOL Frame statements.

### C / C++
- Strings: normal and raw (C++11 R"(…)"); comments // and /* */; preprocessor lines starting with #.
- Templates (C++), namespaces, lambdas, initializer lists; macro definitions/expansions.
- Strategy: segmenter must treat preprocessor lines as single-line native and ignore contents for Frame detection.
- Acceptance notes:
  - [ ] No detection inside preprocessor, macros, or raw strings
  - [ ] Template angle brackets do not affect brace depth
- Proposed mega fixture (not added):
  - operations: body with macros (#define), templates, namespaces, try/catch (C++), and SOL Frame statements.

### Go
- Strings: " ", raw backtick strings; comments // and /* */; package/import; short var :=.
- Defer/panic/recover; interfaces; goroutines/channels (syntax only, no runtime needed in tests).
- Acceptance notes:
  - [ ] Backtick raw strings may contain Frame-statement-like tokens without segmentation
  - [ ] Short var declarations at SOL do not clash with Frame‑statement detection
- Proposed mega fixture (not added):
  - operations: body with multiple funcs, methods on types, interface impl patterns, and SOL Frame statements.

### Swift / Kotlin
- Swift: multiline triple-quote strings, interpolation \(…); // and /* */; attributes @.
- Kotlin: multiline strings """…""", interpolation ${…}; annotations @; generics/inline functions.
- Acceptance notes:
  - [ ] Multiline strings with interpolation are ignored by detector
  - [ ] Annotations/attributes do not trigger segmentation
- Proposed mega fixtures (not added):
  - Swift: body using extensions, protocols, generics, optionals, and SOL Frame statements.
  - Kotlin: body with data classes, sealed classes, coroutines syntax (no runtime), and SOL Frame statements.
