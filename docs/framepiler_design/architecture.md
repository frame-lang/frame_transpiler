# Frame Transpiler Architecture (Authoritative)

Status: Authoritative design reference for the current and in‑flight architecture (MixedBody/MIR — “B2”).  
Last updated: 2025‑11‑06

This document is the single source of truth for the Frame compiler pipeline, terminology, and component responsibilities. Older sections below are retained as legacy context; the “Authoritative Architecture” sections supersede them.

## Authoritative Architecture

### Pipeline (high‑level)

```
CompilationUnit (.frm file)
  → ModulePartitioner (file/module structure & native regions)
  → Parser (AST build; 2‑pass today)
  → NativeRegionSegmenter (per native region inside a body; top‑level classification)
  → Semantic analysis (current: in Parser pass 2; planned: dedicated SemanticAnalyzer)
  → Visitors (code generation)
```

### End‑to‑End Pipeline (detailed)

```
               +----------------------+
               |  Frame Source (.frm) |
               +-----------+----------+
                           |
                           v
         +-----------------+-----------------+
         |     Module Partitioner             |
         |  (file/module partitions & ranges) |
         +-----------------+-----------------+
                           |
                           v
        +------------------+------------------+
        |   Scanner (Frame Common)            |
        | - Tokens (Frame)                    |
        | - Target regions (aka NativeRegion) |
        +------------------+------------------+
                           |
                           v
      +--------------------+--------------------+
      |      Parser (Pass 1: Symbols)          |
      |      Arcanum (symbol tables)           |
      +--------------------+--------------------+
                           |
                           v
      +--------------------+--------------------+
      |      Parser (Pass 2: AST)              |
      |  - Frame AST                            |
      |  - Target body parse (SWC/RustPython)   |
      |  - NativeRegionSegmenter (body slices)  |
      +--------------------+--------------------+
                           |
                           v
      +--------------------+--------------------+
      |    MixedBody / MIR Assembly            |
      |  [NativeText | Frame(MirDirective)]     |
      +--------------------+--------------------+
                           |
                           v
      +--------------------+--------------------+
      | Semantic Analyzer (planned, out-of-pass)|
      |  Resolution, validation, typing rules   |
      +--------------------+--------------------+
                           |
                           v
      +--------------------+--------------------+
      |   Visitors (TS/Py/LLVM/GraphViz)       |
      |   Code emission + source maps          |
      +--------------------+--------------------+
                           |
                           v
      +--------------------+--------------------+
      | Toolchains / Runtime Integration       |
      | node/tsc · python3 · clang/llvm        |
      +--------------------+--------------------+
                           |
                           v
      +--------------------+--------------------+
      | Test Runner + CI (frame_test_runner)   |
      +----------------------------------------+
```

## Stage‑by‑Stage Details

1) Inputs & Configuration
- Inputs: `.frm` sources, CLI flags (`-l target`), optional FID cache (`.frame/cache/fid/{target}`); env overrides (`FRAME_FID_PATH`).
- Invariants: all sources in a build agree on target; `@target` prolog matches CLI.

2) Module Partitioner
- Produces ordered partitions (Prolog, Imports, FrameOutline, Body). Body partitions carry brace‑balanced ranges. See stages/module_partitioner.md

3) Scanner (Frame Common) & Target Discovery
- Emits Frame tokens and `TargetRegion` (aka `NativeRegion`) entries for native bodies with `TargetSourceMap` anchors.

4) Parser Pass 1 (Symbols)
- Builds `Arcanum` scope stack for systems/classes/functions/actions/operations.

5) Parser Pass 2 (AST)
- Builds `FrameModule` AST; for bodies:
  - Native‑only → target parser (SWC/rustpython) → `ParsedTargetBlock`.
  - Mixed/undecidable → `NativeRegionSegmenter` → `segmented_body`.
 - More: stages/target_parsers.md

6) NativeRegionSegmenter (per target)
- Classifies native body lines into `BodySegment::{Native,Directive}` using a brace/string/comment/template‑aware state machine. See stages/native_region_segmenter.md

7) MixedBody / MIR Assembly
- Converts to `[MixedBodyItem::{NativeText,Frame(MirDirective)}]` for stable codegen and source mapping. See stages/mixed_body_mir.md

Parsers & Mixed AST Linkage
- Target parsers (SWC for TypeScript; rustpython for Python) validate native‑only bodies and supply structured spans, but do not own Frame semantics or codegen.
- Mixed bodies skip target parsing and instead use the NativeRegionSegmenter + MixedBody/MIR. Visitors emit native spans verbatim and expand directives with deterministic glue.
- Frame symbols are maintained by Arcanum; native symbol information (when collected) is kept in a sidecar index for diagnostics only. No merging into Arcanum.
- Further details: stages/mixed_asts_and_symbols.md

### TypeScript Body Boundary Detection

Some TypeScript bodies include template literals with nested `${…}` that can confuse simple token‑based brace counting. For reliable body boundary detection we use a textual, template‑aware closer that scans bytes and tracks strings/comments/template nesting. See stages/ts_textual_body_closer.md for the algorithm and tested behaviors. Current usage: applied to operations; staged rollout for actions/handlers (guarded by backtick detection).

8) Semantic Analyzer (planned)
- Out‑of‑pass analyzer for resolution and validation; removes semantic checks from parser pass 2.

9) Visitors
- Target code emission; B2 path expands MIR into target AST (e.g., SWC) instead of string glue.

10) Diagnostics & Source Maps
- Compose Frame/native spans across partitions and mixed bodies. See stages/diagnostics_and_source_maps.md

11) Toolchains & Runtime
- Node/tsc (TS), Python 3 (Py), LLVM FFI→IR transition.

12) Test Runner & CI
- Full suite validation after each stage change.

### Terminology: Partition vs Segment

- Partition (Frame‑outer context): a contiguous region where Frame is the host grammar and native code appears only as embedded islands. Partitions are produced by the ModulePartitioner.
  - Examples: PrologPartition (@target), NativeImportPartition (contiguous native imports), FrameOutlinePartition (systems/blocks/headers), BodyPartition (per member body: FrameBody or NativeBody).

- Segment (Native‑outer context): a classification inside a native region where the host is the target language and Frame appears only as embedded control directives. Segments are produced by the NativeRegionSegmenter.
  - BodySegment::Native { text, start_line, end_line }
  - BodySegment::Directive { kind: Transition | Forward | StackPush | StackPop, frame_line }

In short: we partition Frame blocks; we segment native blocks.

### Components (current mapping)

- ModulePartitioner (file/module pass)
  - Implementation: `framec/src/frame_c/scanner_outline.rs`
  - Role: Identify ModuleUnit(s) in a CompilationUnit and produce ordered partitions:
    - PrologPartition (e.g., `@target`)
    - NativeImportPartition(s)
    - FrameOutlinePartition (systems/blocks/headers, body ranges)
    - BodyPartition (per member body)
  - Notes: An implicit ModuleUnit exists even without explicit `module {}`; it still contains multiple partitions.

- Parser (AST build; two‑pass today)
  - Implementation: `framec/src/frame_c/parser.rs`
  - Pass 1 builds symbol tables (Arcanum). Pass 2 performs semantic parsing/validation and constructs the AST.
  - Attaches body metadata: Frame bodies remain Frame; native bodies will carry `ActionBody::Segmented(segments)` after segmentation.

- NativeRegionSegmenter (per target native region)
  - Implementation (TS scaffold): `framec/src/frame_c/native_region_segmenter/typescript.rs`
  - Role: For each NativeBody partition, classify top‑level lines into Native vs Directive segments (brace/string/template/comment aware). No reordering and no full native parsing.

- Semantic analysis
  - Current: performed during Parser pass 2.
  - Planned: a dedicated `SemanticAnalyzer` that walks the AST after a single parse (see “Evolution” below).

- Visitors (code generation)
  - Implementations: `framec/src/frame_c/visitors/*`
  - Role: Emit Native segments verbatim; emit glue for Directive segments (transition, forward, stack operations).

- Diagnostics & Source Maps
  - Implementations: `framec/src/frame_c/source_map.rs`, `framec/src/frame_c/source_mapping.rs`
  - Policy: Partitions choose diagnostic domain (Frame vs native). Segments carry dual locations (frame_line for directives; start/end target lines for native).

### MixedBody + MIR (Authoritative)

This section defines the MIR used by the Frame compiler to represent embedded Frame control semantics inside target‑native bodies.

Definition
- MIR (Minimal Intermediate Representation) is a compact, target‑agnostic model for Frame‑only control semantics that may appear inside native target code blocks. It is intentionally small and stable to enable deterministic code generation and mapping.

Scope
- MIR only models Frame semantics that cross native boundaries:
  - State transition: `-> $State(args)`
  - Parent forward: `=> $^`
  - State stack operations: `$$[+]`, `$$[-]`
  - Return terminator: `return [expr]` (handler/operation/action context)
- Everything else remains as native text (verbatim) and is not represented in MIR.

Data Model
- MixedBodyItem (ordered):
  - `NativeText { target, text, start_line, end_line }`
  - `Frame(MirDirective)`
- MirDirective:
  - `Transition { state: String, args: Vec<String> }`
  - `ParentForward`
  - `StackPush`
  - `StackPop`
  - `Return(Option<String>)`

### Directive Detection Invariants (All Targets)

- SOL-anchored: A line is classified as a Frame directive only if the directive tokens begin at the first non-whitespace column. No mid-line detection.
- Full token patterns:
  - Transition: `-> $State` (require `$` after optional whitespace following `->`).
  - Parent forward: `=> $^` (require `$^` after optional whitespace following `=>`).
  - Stack ops: `$$[+]`, `$$[-]`.
- String/comment safe: Per-target segmenters must ignore occurrences inside string literals, template literals, and comments using a small, explicit DFA. No heuristics.
- Whitespace: Detection relies on a Unicode-aware space predicate (includes tabs, NBSP, U+2000–U+200A, U+202F, U+205F, U+3000). This ensures SOL and leading/trailing whitespace handling is robust across editors.
- EOLs: The pipeline tolerates LF and CRLF. Standalone CR is normalized by the reader if encountered.
- BOM: If a UTF-8 BOM is present, it is stripped before scanning so SOL detection is unaffected.

### Per-Target Segmenters (Summary)

- Python: Handles single/double/triple-quoted strings and `#` comments; SOL detection applies; triple-quoted literals are respected inside native bodies.
- TypeScript: Handles single/double quotes, `//`, `/* */`, and backtick template literals with nested `${…}`; SOL detection applies; braces inside template expressions do not affect top-level detection.
- C/C++: Recognizes `//`, `/* */`, and raw strings; SOL detection requires `-> $` to avoid `ptr->field` false positives.
- Rust/Java/Kotlin/C#: Apply the same SOL + pattern rules; treat native arrow (`=>`) or lambdas as non-directives unless followed by `$^` at SOL.

These invariants are enforced in segmenters and mirrored in the FrameCommon scanner to treat space-like characters as whitespace. Negative fixtures exist for common false positives.

Semantics
- Ordering: MixedBodyItem order equals source order.
- Termination: Transition/Forward/StackPop are handler‑terminal; any following statements are considered unreachable (validator enforces).
- Purity: MIR nodes carry only what is needed to generate correct target glue; evaluation of arguments (when present in the future) is deferred to target codegen.

Construction
- Native‑only body: parser attaches native target AST; MixedBody contains a single `NativeText` spanning the body.
- Mixed body: NativeRegionSegmenter produces `BodySegment::{Native,Directive}`; parser converts segments to MixedBody by mapping directives to MirDirective and aggregating adjacent native lines into `NativeText` slices.

Mapping to Codegen
- Visitors iterate MixedBody and:
  - Emit `NativeText` verbatim.
  - Expand `MirDirective` to target‑native glue (custom visitor logic keeps emission deterministic).
- Source maps:
  - `NativeText`: use recorded start/end frame lines to map target lines back to Frame.
  - `MirDirective`: synthesize mapping anchored at the directive’s frame line.

Examples
```
// Frame (TypeScript target body)
{
  const x = 1;
  -> $Running
}

// MixedBody
[
  NativeText { text: "  const x = 1;\n", start_line: L+1, end_line: L+1, target: typescript },
  Frame(Transition { state: "Running", args: [] })
]
```

Constraints & Invariants
- No reformatting of native text in MixedBody assembly.
- MIR carries names and (later) argument string forms only; it does not own type information (that remains in Arcanum / analyzer).
- MixedBody must be present or derivable for all bodies that contain directives inside native code.

Extensibility
- New Frame control semantics can be added as new MirDirective variants with a corresponding visitor emission rule.

Further Reading
- Stage doc: stages/mixed_body_mir.md (assembly rules and validation).

### Evolution (planned and in progress)

- Parse‑once + dedicated SemanticAnalyzer
  - Move all semantic checks (call‑chain, start‑state/enter‑param validation, scope resolution) out of Parser pass 2 into a separate analyzer pass that walks the AST with Arcanum.
  - Remove the second Parser pass once the analyzer is in place.

- Naming cleanup (in code)
  - `TargetRegion` → `NativeRegion`
  - `interleaver` modules → `native_region_segmenter`
  - `FrameStmtKind` → `DirectiveKind`
  - `ActionBody::Interleaved` → `ActionBody::Segmented`

- MixedBody/B2 implementation
  - Populate `mixed_body` for all target bodies that are native or segmented.
  - Translate `MirDirective` into target-native AST (SWC for TypeScript), then delegate emission to native printers.
  - Compose Frame/native source maps for precise diagnostics across mixed content.

---

## Legacy Architecture (for context)

The following sections describe the pre‑Option‑A pipeline and historical details. They are preserved for reference but are not authoritative for ongoing work.

## Overview

The Frame transpiler (v0.82) converts Frame language source files (.frm) to target languages (Python, TypeScript, GraphViz, etc.).

**Current Status**: 100% test success rate (397/397 tests passing)

## Compilation Pipeline

```
Frame Source (.frm) 
    ↓
Scanner (Tokenizer) → framec/src/frame_c/scanner.rs
    ↓  
Parser → framec/src/frame_c/parser.rs
    ↓
AST (FrameModule) → framec/src/frame_c/ast.rs
    ↓
Visitors (Code Generation) → framec/src/frame_c/visitors/
    ↓
Target Code (Python, TypeScript, GraphViz, etc.)
```

## v0.31 Modular AST Structure

```
FrameModule (Top-Level)
├── Module (metadata/attributes)
├── Imports[] (v0.31: native import statements)
├── Functions[] (peer entities)
└── Systems[] (peer entities)
    └── SystemNode
        ├── Module (system-specific metadata)
        ├── Interface Block
        ├── Machine Block  
        ├── Actions Block
        ├── Operations Block (v0.31: static validation)
        └── Domain Block
```

## Key Components

### Scanner (scanner.rs)
- Token recognition in `scan_token()` method
- New tokens added to `TokenType` enum
- Use `peek()` and `peek_next()` for lookahead
- **v0.31**: Added Import, From, As tokens for native import support
- **v0.31**: Added None_ token, removed Null and Nil tokens

### Parser (parser.rs)
- Event handler parsing in `event_handler()` method
- Terminator parsing handles `return`, `=>`, `@:>`
- Use `TerminatorType` enum for different terminators
- **v0.30**: Multi-entity parsing with smart fallback to syntactic mode
- **v0.31**: Import statement parsing with dotted module names
- **v0.31**: Static method validation (prevents self usage in @staticmethod)
- **v0.31**: System.return parsing as special variable for interface returns
- **v0.31**: Default return value parsing (`: type = value`) for all contexts
- **v0.31**: Scope context checking prevents ActionCallExprNode in function scope

### AST (ast.rs)
- All syntax tree node definitions
- `TerminatorType` enum defines terminator semantics
- **v0.30**: FrameModule container with peer Functions[] and Systems[]
- **v0.31**: ImportNode and ImportType for native imports
- **v0.31**: Self expression support (standalone and dotted)
- **v0.31**: return_init_expr_opt for default return values in all contexts

### Symbol Table (symbol_table.rs)
- **v0.30**: System-scoped state resolution
- Arcanum provides system.get_state() pattern
- Proper isolation between multiple systems
- **v0.31**: LEGB scope resolution with legb_lookup() method
- **v0.31**: ScopeContext enum tracks parsing context (Global/Function/System)
- **v0.31**: is_symbol_accessible() enforces scope isolation rules

### Visitors (visitors/)
- Each target language has its own visitor
- All visitors must handle new `TerminatorType::DispatchToParentState`
- Python visitor is primary reference implementation
- **v0.30**: Fixed FrameCompartment generation bug
- **v0.31**: Import statement code generation
- **v0.31**: Operations only static when @staticmethod attributed
- **v0.31**: System.return generates as return_stack mechanism
- **v0.31**: Event handler default values override interface defaults
- **v0.82**: TypeScript visitor added with full state machine support

## v0.31 Language Features

### Native Import Statements (NEW in v0.31)
- Simple imports: `import math`
- Aliased imports: `import numpy as np`
- From imports: `from typing import List, Dict`
- Wildcard imports: `from collections import *`
- Dotted module names: `import os.path`

### Self Expression Enhancement (v0.31)
- Standalone self: `jsonpickle.encode(self)`
- Dotted access: `self.variable`, `self.method()`
- Static method validation prevents self usage

### Null Value Standardization (v0.31)
- `None` is the only null keyword
- `null` and `nil` removed from language
- All target languages map None appropriately

### Static Method Validation (v0.31)
- Parse-time validation for @staticmethod operations
- Clear error messages for invalid self usage
- Operations are instance methods by default

### System Return Semantics (v0.31)
- Interface methods can specify default return values: `getValue(): int = 42`
- Event handlers can override defaults: `getValue(): int = 99 {`
- `system.return` special variable sets interface return values from anywhere
- Actions can set system.return while returning different values to caller
- Operations cannot use system.return (enforced at parse time)
- Return stack mechanism preserves values through call chains

## v0.30 Multi-Entity Features

### Smart Parsing Fallback
When semantic parsing fails on complex multi-entity files, the transpiler automatically falls back to syntactic parsing mode, allowing code generation to continue.

### System-Scoped State Resolution
Multiple systems in the same file maintain proper isolation through system-scoped symbol table operations.

### FrameCompartment Generation
Fixed bug where Python visitor generated system-specific compartment classes instead of the standard FrameCompartment class.

### Call Chain Scope Processing (v0.30 Critical Fix)
Resolved critical bug in Python visitor where external object method calls (`obj.method()`) incorrectly generated `obj.self.method()` while internal operation calls (`self.method()`) lost required `self.` prefixes. The fix implements conditional flag setting in call chain processing to properly distinguish between external and internal call contexts.

## Build System

### Main Build
```bash
cargo build
```

### Test Build (Disabled)
The framec_tests crate is temporarily disabled in Cargo.toml to allow main transpiler builds to succeed.

## Testing

Test files are located in:
- **Frame source (.frm)**: `framec_tests/python/src/`
- **Generated Python**: Next to source files in `src/`

### Validation Protocol
1. **Generate**: Run framec to generate code
2. **Execute**: Run the generated target code
3. **Validate**: Verify output matches expected behavior
4. **Report**: Document verified functionality

## Architecture Improvements (v0.30)

### System-Scoped API Restructuring
Moved from singleton pattern to proper system-scoped operations:
- `system_symbol.get_state()` instead of `arcanum.get_state_in_system()`
- Eliminated infinite loops and parser hangs
- Proper encapsulation of system-specific operations

### Modular Architecture
Replaced SystemNode-centric design with proper FrameModule container:
- Functions and systems are peer entities within modules
- No artificial parent-child relationships
- Clean separation between module structure and entity content
