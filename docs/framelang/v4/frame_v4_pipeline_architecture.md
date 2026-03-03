# Frame V4 Pipeline Architecture

**Version:** 1.0
**Date:** March 2026
**Status:** Proposed — awaiting review
**Scope:** Complete rewrite of the compilation pipeline

---

## 1. Design Principles

### 1.1 Classical Compiler Architecture

The pipeline follows the standard compiler phases, each with a single responsibility:

| Phase | Input | Output | Responsibility |
|-------|-------|--------|----------------|
| Segmenter | raw bytes | SourceMap | Separate native ocean from Frame islands |
| Lexer | Frame region bytes | TokenStream | Convert bytes to tokens |
| Parser | TokenStream | FrameAst | Build abstract syntax tree from tokens |
| Arcanum | FrameAst | SymbolTable | Build symbol table, resolve references |
| Validator | FrameAst + Arcanum | Diagnostics | Validate semantics |
| Codegen | FrameAst + Arcanum | CodegenNode | Transform to language-agnostic IR |
| Emitter | CodegenNode | String | Emit target language source |

**Each phase is a pure function of its inputs.** No phase mutates the output of a previous phase. No phase reaches back to raw source bytes except the Segmenter and Lexer.

### 1.2 Single Responsibility

- **Only the Segmenter touches raw source bytes for native/Frame boundary detection.** The current architecture has three places that perform segmentation: `skip_native_preamble()` in the parser, `skip_pragmas_keep_native()` in compiler.rs, and `NativeRegionScanner` in codegen. The new architecture consolidates all segmentation into one stage.

- **Only the Lexer converts bytes to tokens.** The current parser operates on raw `Vec<u8>` with byte-peeking (`peek_string`, `peek_char`). The new architecture introduces a proper token stream.

- **Codegen never scans source bytes.** The current `generate_system()` receives raw `source: &[u8]` and uses `NativeRegionScanner` to split handler bodies. In the new architecture, the AST is complete after parsing — codegen transforms AST nodes to IR nodes without touching source bytes.

### 1.3 No Raw Bytes After Lexing

After the Lexer phase, no downstream stage should work with `&[u8]` or byte offsets. All text content (native code, identifiers, type annotations) is represented as `String` in the token stream and AST. Source spans are carried for error reporting but never used for content extraction after parsing.

### 1.4 The Oceans Model Is a Segmenter Concern

The "oceans model" (native code passes through, @@system blocks expand) is implemented by the Segmenter and the final Output Assembly. The Lexer, Parser, Arcanum, Validator, and Codegen know nothing about native prolog/epilog — they only see Frame constructs.

### 1.5 Fail Early, Fail Hard

Every phase either succeeds completely or produces clear diagnostics. No fallbacks, no default behaviors, no silent recovery. If the Segmenter can't find a closing brace, it reports the error with location. If the Lexer encounters an unexpected byte, it reports it. If the Parser sees an unexpected token, it reports it.

---

## 2. Pipeline Overview

```
Source File (raw bytes)
    │
    ▼
┌─────────────────────────────────────────────────────────┐
│  Stage 0: Source Segmenter                              │
│  Input:  raw bytes + target language                    │
│  Output: SourceMap                                      │
│  - Identifies native regions, pragmas, @@system blocks  │
│  - Language-aware (uses SyntaxSkipper for strings/comments) │
└─────────────────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────────────────┐
│  Stage 1: Frame Lexer                                   │
│  Input:  @@system block bytes                           │
│  Output: TokenStream                                    │
│  - Two modes: structural (Frame syntax) and             │
│    native-aware (handler/action/operation bodies)       │
│  - Frame constructs → typed tokens                      │
│  - Native code → NativeCode tokens (opaque strings)     │
└─────────────────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────────────────┐
│  Stage 2: Frame Parser                                  │
│  Input:  TokenStream                                    │
│  Output: FrameAst                                       │
│  - Recursive descent over tokens (not bytes)            │
│  - Builds complete AST including handler body statements │
│  - No raw byte access                                   │
└─────────────────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────────────────┐
│  Stage 3: Arcanum (Symbol Table)                        │
│  Input:  FrameAst                                       │
│  Output: Arcanum                                        │
│  - Catalogs systems, states, events, variables          │
│  - Resolves HSM parent-child relationships              │
│  - Computes codegen configuration                       │
└─────────────────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────────────────┐
│  Stage 4: Validator                                     │
│  Input:  FrameAst + Arcanum                             │
│  Output: Vec<Diagnostic>                                │
│  - Structural validation (duplicates, missing refs)     │
│  - HSM graph validation (cycles, orphan parents)        │
│  - Statement validation (transitions in wrong context)  │
└─────────────────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────────────────┐
│  Stage 5: Codegen                                       │
│  Input:  FrameAst + Arcanum                             │
│  Output: CodegenNode tree                               │
│  - Pure AST-to-IR transformation                        │
│  - No source byte access                                │
│  - No scanning or segmentation                          │
└─────────────────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────────────────┐
│  Stage 6: Backend Emitter                               │
│  Input:  CodegenNode tree                               │
│  Output: String (generated code for @@system block)     │
│  - Language-specific syntax emission                    │
│  - Indentation management                               │
└─────────────────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────────────────┐
│  Stage 7: Output Assembly                               │
│  Input:  SourceMap + generated code per system          │
│  Output: Final source file                              │
│  - Reassembles native prolog + generated + native epilog│
│  - Expands @@SystemName() tagged instantiations         │
│  - Pure string concatenation from SourceMap segments    │
└─────────────────────────────────────────────────────────┘
```

---

## 3. Stage 0: Source Segmenter

### 3.1 Purpose

The Segmenter is the **only stage that implements the oceans model**. It scans raw source bytes and produces a `SourceMap` — an ordered list of segments that partitions the entire source file into typed regions.

### 3.2 Interface

```rust
pub fn segment(source: &[u8], lang: TargetLanguage) -> Result<SourceMap, SegmentError>;
```

### 3.3 SourceMap

```rust
pub struct SourceMap {
    /// Ordered, non-overlapping segments covering the entire source
    pub segments: Vec<Segment>,
    /// Original source bytes (owned, for content extraction by Lexer)
    pub source: Vec<u8>,
    /// Target language (from @@target pragma)
    pub target: TargetLanguage,
}

pub enum Segment {
    /// Native code region — passes through verbatim to output
    Native {
        span: Span,
    },
    /// Pragma line (@@target, @@persist, @@run-expect, etc.)
    Pragma {
        kind: PragmaKind,
        span: Span,
        /// Parsed pragma value (e.g., "python_3" for @@target)
        value: Option<String>,
    },
    /// @@system block — contents will be lexed, parsed, and compiled
    System {
        /// Span of the entire block including `@@system Name { ... }`
        outer_span: Span,
        /// Span of just the system body (between { and })
        body_span: Span,
        /// System name extracted during segmentation
        name: String,
    },
}

pub enum PragmaKind {
    Target,
    Persist,
    PersistWithLibrary(String),
    Codegen,
    RunExpect,
    SkipIf,
    Timeout,
}
```

### 3.4 Algorithm

1. Initialize cursor at byte 0, state = `StartOfLine`.
2. At each line:
   a. Skip whitespace.
   b. If line starts with `@@`:
      - `@@target` → emit `Pragma::Target`, extract language value
      - `@@persist` → emit `Pragma::Persist` (with optional library)
      - `@@system` → extract system name, find matching `}` using `BodyCloser`, emit `System` segment
      - Other `@@` pragmas → emit `Pragma` with appropriate kind
   c. Otherwise: accumulate into current `Native` segment.
3. Use language-specific `SyntaxSkipper` to avoid false `@@` detection inside strings and comments.
4. Use `BodyCloser` to find matching close brace for `@@system` blocks (handles nested braces, strings, comments).

### 3.5 Reuse

The Segmenter consolidates and replaces:
- `PragmaScanner` (existing — good foundation, needs adaptation)
- `BodyCloser` trait and all language implementations (existing — reuse directly)
- `SyntaxSkipper` trait (existing — reuse directly)
- `skip_native_preamble()` in frame_parser.rs (eliminated)
- `skip_pragmas()` in frame_parser.rs (eliminated)
- `skip_pragmas_keep_native()` in compiler.rs (eliminated)
- `skip_pragmas_simple()` in compiler.rs (eliminated)
- `extract_native_code()` in compiler.rs (eliminated)

### 3.6 Errors

```rust
pub enum SegmentError {
    /// Unterminated @@system block (no matching close brace)
    UnterminatedSystem { name: String, open_brace: Span },
    /// Unterminated string literal found during scanning
    UnterminatedString { span: Span },
    /// Unterminated comment found during scanning
    UnterminatedComment { span: Span },
    /// No @@system block found in source
    NoSystemBlock,
    /// Invalid @@target value
    InvalidTarget { value: String, span: Span },
}
```

---

## 4. Stage 1: Frame Lexer

### 4.1 Purpose

The Lexer converts Frame source bytes into a stream of typed tokens. It operates **only on @@system block bytes** (provided by the Segmenter's `System.body_span`). The Lexer never sees native prolog/epilog.

### 4.2 Interface

```rust
pub fn lex(source: &[u8], body_span: Span, lang: TargetLanguage) -> Result<TokenStream, LexError>;
```

### 4.3 Two Lexing Modes

The Lexer operates in two modes, controlled by the Parser via a callback or mode-switching protocol:

**Structural Mode** (default): Tokenizes Frame language syntax — keywords, identifiers, operators, delimiters, type annotations.

**Native-Aware Mode**: Activated when the lexer enters a handler body, action body, or operation body. In this mode, the lexer:
- Recognizes Frame constructs: `-> $State`, `=> $^`, `push$`, `pop$`, `$.varName`, `@@.param`, `@@:return`, `return <expr>`
- Passes everything else through as `NativeCode(String)` tokens
- Uses `SyntaxSkipper` to avoid false Frame construct detection inside native strings/comments

The mode switch happens when the parser requests it (typically at the `{` of a handler/action/operation body).

### 4.4 Token Types

```rust
pub enum Token {
    // ===== Frame Structural Keywords =====
    Interface,           // "interface"
    Machine,             // "machine"
    Actions,             // "actions"
    Operations,          // "operations"
    Domain,              // "domain"
    Var,                 // "var"

    // ===== Frame Statements =====
    Return,              // "return" (Frame return sugar)
    If,                  // "if"
    Else,                // "else"

    // ===== State Syntax =====
    // Note: `$` is not a standalone token. The lexer recognizes `$` as the
    // start of composite Frame tokens and emits the appropriate variant:
    StateRef(String),     // "$StateName" — state reference in transitions (-> $Foo)
    EnterHandler,         // "$>" — enter event handler
    ExitHandler,          // "<$" — exit event handler
    StateVarRef(String),  // "$.varName" — state variable reference (read/write)
    StateVarDecl(String), // "$.varName" in state body declaration ($.x: int = 0)
    ParentRef,            // "$^" — parent state reference (HSM forward: => $^)

    // ===== Transition & Control =====
    Arrow,               // "->"
    FatArrow,            // "=>"
    PushState,           // "push$"
    PopState,            // "pop$"

    // ===== Context Syntax =====
    ContextParam(String),    // "@@.paramName"
    ContextReturn,           // "@@:return"
    ContextEvent,            // "@@:event"
    ContextData(String),     // "@@:data[key]"
    ContextParams(String),   // "@@:params[key]"

    // ===== Delimiters =====
    LBrace,              // "{"
    RBrace,              // "}"
    LParen,              // "("
    RParen,              // ")"
    LBracket,            // "["
    RBracket,            // "]"
    Comma,               // ","
    Colon,               // ":" — param/type separator (a: int), return type (: str)
    SectionColon,        // ":" at end of section keyword (interface:, machine:, etc.)
    Equals,              // "="
    Dot,                 // "."

    // ===== Identifiers & Literals =====
    Ident(String),       // alphanumeric identifier
    IntLit(i64),         // integer literal
    FloatLit(f64),       // float literal
    StringLit(String),   // string literal
    BoolLit(bool),       // true/false

    // ===== Native Code (only in native-aware mode) =====
    NativeCode(String),  // opaque native code chunk

    // ===== Meta =====
    Newline,             // significant newline (if needed for grammar)
    Eof,                 // end of token stream
}

pub struct Spanned {
    pub token: Token,
    pub span: Span,
}

pub type TokenStream = Vec<Spanned>;
```

#### Signature Tokenization Example

The signature `foo(a:int, b:str) : str = "default" {}` tokenizes as:

```
Ident("foo") LParen Ident("a") Colon Ident("int") Comma
Ident("b") Colon Ident("str") RParen Colon Ident("str")
Equals StringLit("default") LBrace RBrace
```

This covers interface methods, event handlers, actions, and operations — all use the same `name(params) : returnType = defaultValue { body }` pattern (with optional parts).

### 4.5 Structural Mode Rules

In structural mode, the Lexer recognizes:

| Pattern | Token |
|---------|-------|
| `interface` followed by `:` | `Interface`, `SectionColon` |
| `machine` followed by `:` | `Machine`, `SectionColon` |
| `actions` followed by `:` | `Actions`, `SectionColon` |
| `operations` followed by `:` | `Operations`, `SectionColon` |
| `domain` followed by `:` | `Domain`, `SectionColon` |
| `var` | `Var` |
| `$` followed by identifier | `StateRef(name)` |
| `$>` | `EnterHandler` |
| `<$` | `ExitHandler` |
| `$.` followed by identifier | `StateVarRef(name)` |
| `$^` | `ParentRef` |
| `->` | `Arrow` |
| `=>` | `FatArrow` |
| `push$` | `PushState` |
| `pop$` | `PopState` |
| alphabetic identifier | `Ident(name)` |
| numeric literal | `IntLit` or `FloatLit` |
| `(`, `)`, `{`, `}`, etc. | Corresponding delimiter |

### 4.6 Native-Aware Mode Rules

In native-aware mode, the Lexer scans the handler/action/operation body byte-by-byte:

1. **Start-of-line Frame detection**: At the beginning of each line (after indentation), check for Frame statement patterns:
   - `->` → `Arrow` (start of transition)
   - `=>` → `FatArrow` (start of forward)
   - `push$` → `PushState`
   - `pop$` → `PopState`
   - `` ` `` (backtick) followed by Frame statement → backtick prefix variant
   - `return` → `Return` (Frame return sugar)

2. **Mid-line Frame detection**: Within a line, recognize:
   - `$.varName` → `StateVarRef(name)`
   - `$.varName` followed by `=` → `StateVarRef(name)`, `Equals` (assignment)
   - `@@.param` → `ContextParam(name)`
   - `@@:return` → `ContextReturn`
   - `@@:event` → `ContextEvent`
   - `@@:data[key]` → `ContextData(key)`
   - `@@:params[key]` → `ContextParams(key)`

3. **Native pass-through**: Everything between Frame constructs is accumulated into `NativeCode(String)` tokens.

4. **String/comment skipping**: Uses `SyntaxSkipper` to skip over native strings and comments, preventing false Frame construct detection.

### 4.7 Reuse

The native-aware mode logic consolidates and replaces:
- `NativeRegionScanner` trait and all 7 language implementations (the detection logic moves into the Lexer; `SyntaxSkipper` is reused)
- `match_frame_statement_at_sol()` in unified.rs (logic moves to Lexer)
- `scan_native_regions()` in unified.rs (logic moves to Lexer)
- `parse_handler_body_with_scanner()` in frame_parser.rs (eliminated)

The `SyntaxSkipper` trait and all language implementations are **reused unchanged** — they provide the language-specific string/comment awareness that the Lexer needs.

### 4.8 Errors

```rust
pub enum LexError {
    /// Unexpected byte in Frame structural context
    UnexpectedByte { byte: u8, span: Span },
    /// Unterminated string literal in native code
    UnterminatedString { span: Span },
    /// Unterminated comment in native code
    UnterminatedComment { span: Span },
    /// Invalid Frame construct syntax
    InvalidFrameConstruct { text: String, span: Span },
}
```

---

## 5. Stage 2: Frame Parser

### 5.1 Purpose

The Parser consumes a `TokenStream` and builds a `FrameAst`. It is a recursive descent parser that matches token patterns — it never touches raw bytes.

### 5.2 Interface

```rust
pub fn parse(tokens: &TokenStream) -> Result<SystemAst, Vec<ParseError>>;
```

Note: The parser receives tokens for a single `@@system` block. Multi-system files produce multiple `TokenStream`s (one per system), each parsed independently. The pipeline orchestrator handles multi-system assembly.

### 5.3 Grammar (Token-Based)

```
system       := Ident LBrace section* RBrace
section      := interface_section
             | machine_section
             | actions_section
             | operations_section
             | domain_section

interface_section := Interface SectionColon interface_method*
interface_method  := Ident LParen param_list RParen (Colon type)? (Equals native_expr)?

machine_section := Machine SectionColon state*
state           := StateRef(name) (FatArrow StateRef(parent))?
                   (LParen param_list RParen)?
                   LBrace state_var* handler* (FatArrow ParentRef)? RBrace
state_var       := StateVarRef(name) (Colon type)? Equals native_expr
handler         := event_name (LParen param_list RParen)? (Colon type)? LBrace handler_body RBrace
enter_handler   := EnterHandler (LParen param_list RParen)? LBrace handler_body RBrace
exit_handler    := ExitHandler (LParen param_list RParen)? LBrace handler_body RBrace

handler_body    := (frame_stmt | NativeCode)*
frame_stmt      := transition | forward | push | pop | state_var_access
                 | state_var_assign | context_access | return_sugar
transition      := Arrow StateRef(target)
                 | Arrow FatArrow StateRef(target)
                 | Arrow PopState
                 | LParen native_args RParen Arrow (LParen native_args RParen)? StateRef(target)
forward         := FatArrow ParentRef
push            := PushState
pop             := PopState
state_var_access := StateVarRef(name)
state_var_assign := StateVarRef(name) Equals native_expr
context_access  := ContextParam | ContextReturn | ContextEvent | ContextData | ContextParams
return_sugar    := Return native_expr?

actions_section   := Actions SectionColon action*
action            := Ident LParen param_list RParen LBrace NativeCode RBrace

operations_section := Operations SectionColon operation*
operation          := Ident LParen param_list RParen Colon type LBrace NativeCode RBrace

domain_section    := Domain SectionColon domain_var*
domain_var        := Var Ident (Colon type)? Equals native_expr

param_list        := (param (Comma param)*)?
param             := Ident (Colon type)? (Equals native_expr)?
type              := Ident
native_expr       := NativeCode | literal | Ident
```

### 5.4 Parser Responsibilities

1. **Consume tokens sequentially** — `expect(Token)`, `peek()`, `advance()`
2. **Build complete AST** — including all handler body statements. After parsing, the AST contains every Frame statement and every native code chunk. No further scanning is needed.
3. **Record spans** — every AST node carries a `Span` for error reporting.
4. **Mode switching** — when entering a handler/action/operation body (at the `{` token), signal the Lexer to switch to native-aware mode. When exiting (at the matching `}`), switch back to structural mode.

### 5.5 AST Changes from Current

The existing `FrameAst` and its component structs (`SystemAst`, `StateAst`, `HandlerAst`, etc.) are largely correct. Key changes:

1. **`HandlerBody` becomes fully populated**: Currently, `HandlerBody` stores a span and the parser re-scans during codegen. In the new architecture, `HandlerBody.statements` contains a complete interleaving of `Statement::NativeCode(String)` and Frame statement variants. No spans referencing raw source.

2. **`ActionBody` and `OperationBody` contain native code as String**: Currently they store a `Span` that codegen extracts from source bytes. In the new architecture, the lexer produces `NativeCode` tokens that the parser stores directly as `String` content.

3. **Domain variable initializers are String, not Span**: Same principle — the Lexer extracts the native expression text, the parser stores it as a String.

4. **No `source: &[u8]` parameter in codegen**: `generate_system()` currently takes `source: &[u8]` to extract native code. That parameter is eliminated — all text content is in the AST.

5. **Add `Statement::NativeCode(String)`**: A new variant in the `Statement` enum for native code chunks within handler bodies.

```rust
pub enum Statement {
    // Existing Frame statement variants...
    Transition(TransitionAst),
    TransitionForward(TransitionForwardAst),
    Forward(ForwardAst),
    StackPush(StackPushAst),
    StackPop(StackPopAst),
    Return(ReturnAst),
    StateVarAccess(StateVarAccessAst),
    StateVarAssign(StateVarAssignAst),
    ContextAccess(ContextAccessAst),

    // NEW: native code chunk within handler body
    NativeCode(String),
}
```

### 5.6 Error Recovery

The parser collects errors and attempts to recover at synchronization points (section keywords, state declarations, handler declarations). This enables reporting multiple errors per compilation rather than stopping at the first.

```rust
pub struct ParseError {
    pub code: String,        // E0xx
    pub message: String,
    pub span: Span,
    pub severity: Severity,
}

pub enum Severity {
    Error,
    Warning,
}
```

---

## 6. Stage 3: Arcanum (Symbol Table)

### 6.1 Purpose

The Arcanum walks the `FrameAst` and builds a comprehensive symbol table. It resolves forward references (transitions to states declared later), computes HSM relationships, and determines which codegen features are needed.

### 6.2 Interface

```rust
pub fn build_arcanum(ast: &SystemAst) -> Result<Arcanum, Vec<ArcanumError>>;
```

### 6.3 No Changes from Current

The Arcanum's design is sound. It receives a `FrameAst` and produces a symbol table. No changes needed to the Arcanum's architecture — only implementation adjustments to work with the updated AST types (String content instead of Span references).

---

## 7. Stage 4: Validator

### 7.1 Purpose

The Validator checks semantic correctness against the AST and Arcanum.

### 7.2 Interface

```rust
pub fn validate(ast: &SystemAst, arcanum: &Arcanum) -> Vec<Diagnostic>;
```

### 7.3 No Changes from Current

The Validator's design is sound. The validation rules (E402 unknown transition target, E403 duplicate state, HSM cycle detection, etc.) remain the same. The Validator continues to receive AST + Arcanum and produce diagnostics.

---

## 8. Stage 5: Codegen

### 8.1 Purpose

The Codegen transforms `SystemAst` + `Arcanum` into a `CodegenNode` tree. This is a pure AST-to-IR transformation.

### 8.2 Interface

```rust
pub fn generate_system(system: &SystemAst, arcanum: &Arcanum, lang: TargetLanguage) -> CodegenNode;
```

Note: **No `source: &[u8]` parameter.** The current `generate_system()` takes raw source bytes to extract native code at span locations. In the new architecture, all native code is already in the AST as String content.

### 8.3 Key Change: No NativeRegionScanner in Codegen

Currently, codegen calls `NativeRegionScanner` to scan handler body bytes for Frame constructs. This responsibility moves entirely to the Lexer (Stage 1). By the time codegen runs:

- `HandlerBody.statements` is a complete list of `Statement` nodes — both Frame statements and `NativeCode(String)` chunks
- `ActionBody` contains native code as String
- `OperationBody` contains native code as String
- Domain variable initializers are String

Codegen simply walks these nodes and maps them to `CodegenNode` equivalents:
- `Statement::Transition(...)` → `CodegenNode::Transition { ... }`
- `Statement::NativeCode(s)` → `CodegenNode::NativeBlock { code: s }`
- etc.

### 8.4 CodegenNode — No Changes

The `CodegenNode` enum is well-designed and does not need changes. It already has all the variants needed: structural (Class, Method, Constructor), statements (VarDecl, Assignment, If, Match), expressions, Frame-specific (Transition, Forward, StackPush, StackPop), and native code preservation (NativeBlock).

---

## 9. Stage 6: Backend Emitter

### 9.1 No Changes

The `LanguageBackend` trait, `EmitContext`, `ClassSyntax`, and all language-specific backends (Python, TypeScript, Rust, C, C++, Java, C#) remain unchanged. They receive `CodegenNode` trees and emit target language strings.

---

## 10. Stage 7: Output Assembly

### 10.1 Purpose

The Output Assembler takes the `SourceMap` from Stage 0 and the generated code from Stages 5-6, and produces the final output file.

### 10.2 Interface

```rust
pub fn assemble(
    source_map: &SourceMap,
    generated_systems: &[(String, String)],  // (system_name, generated_code)
    lang: TargetLanguage,
) -> Result<String, AssemblyError>;
```

### 10.3 Algorithm

1. Walk the `SourceMap.segments` in order:
   - `Segment::Native` → extract text from source bytes at span, append to output
   - `Segment::Pragma` → skip (pragmas are consumed by earlier stages)
   - `Segment::System` → look up system name in `generated_systems`, append generated code (including runtime classes: FrameEvent, Compartment, FrameContext)
2. Post-process: expand `@@SystemName()` tagged instantiations in native regions.
3. Return final assembled output.

### 10.4 Tagged Instantiation Expansion

The `expand_tagged_instantiations()` logic moves here — it operates on native code segments (not Frame code), expanding `@@SystemName(args)` to the appropriate native constructor. This uses `SyntaxSkipper` to avoid expansion inside strings/comments.

### 10.5 Reuse

Replaces:
- The prolog/epilog extraction logic in `compile_ast_based()` (compiler.rs lines 540-631)
- The multi-system assembly logic in `compile_ast_based()` (compiler.rs lines 635-740)
- `expand_tagged_instantiations()` (moves here from compiler.rs)

---

## 11. Pipeline Orchestrator

### 11.1 Purpose

The orchestrator replaces the current `compile_ast_based()` function. It invokes each stage in sequence and threads outputs between them.

### 11.2 Interface

```rust
pub fn compile(source: &[u8], config: &PipelineConfig) -> Result<CompileResult, CompileError>;
```

### 11.3 Algorithm

```rust
pub fn compile(source: &[u8], config: &PipelineConfig) -> Result<CompileResult, CompileError> {
    // Stage 0: Segment
    let source_map = segment(source, config.target)?;

    let mut generated_systems = Vec::new();

    for system_segment in source_map.systems() {
        // Stage 1: Lex
        let tokens = lex(&source_map.source, system_segment.body_span, config.target)?;

        // Stage 2: Parse
        let system_ast = parse(&tokens)?;

        // Stage 3: Arcanum
        let arcanum = build_arcanum(&system_ast)?;

        // Stage 4: Validate
        let diagnostics = validate(&system_ast, &arcanum);
        if diagnostics.has_errors() {
            return Err(diagnostics.into());
        }

        // Stage 5: Codegen
        let codegen_node = generate_system(&system_ast, &arcanum, config.target);

        // Stage 6: Emit
        let backend = get_backend(config.target);
        let mut ctx = EmitContext::new();
        let code = backend.emit(&codegen_node, &mut ctx);

        generated_systems.push((system_ast.name.clone(), code));
    }

    // Stage 7: Assemble
    let output = assemble(&source_map, &generated_systems, config.target)?;

    Ok(CompileResult {
        code: output,
        errors: vec![],
        warnings: vec![],
        source_map: None,
    })
}
```

---

## 12. Lexer-Parser Communication Protocol

### 12.1 The Mode Problem

The Lexer needs to know when it's inside a handler body (native-aware mode) vs. when it's in Frame structural syntax. But the Parser is the one that understands the grammar and knows when a handler body starts.

### 12.2 Solution: Pull-Based Lexer

Instead of the Lexer producing all tokens up front, the Parser **pulls** tokens from the Lexer on demand. The Parser controls mode switching:

```rust
pub struct Lexer<'a> {
    source: &'a [u8],
    cursor: usize,
    mode: LexerMode,
    lang: TargetLanguage,
    skipper: Box<dyn SyntaxSkipper>,
}

pub enum LexerMode {
    /// Structural: tokenize Frame keywords, identifiers, operators
    Structural,
    /// NativeAware: detect Frame constructs, pass native code through
    NativeAware,
}

impl<'a> Lexer<'a> {
    /// Get the next token
    pub fn next_token(&mut self) -> Result<Spanned, LexError>;

    /// Switch to native-aware mode (called by parser at handler body open brace)
    pub fn enter_native_mode(&mut self);

    /// Switch to structural mode (called by parser at handler body close brace)
    pub fn enter_structural_mode(&mut self);

    /// Peek at the next token without consuming
    pub fn peek(&mut self) -> Result<&Spanned, LexError>;
}
```

The Parser calls `lexer.enter_native_mode()` when it encounters the `{` of a handler/action/operation body, and `lexer.enter_structural_mode()` when it encounters the matching `}`.

### 12.3 Alternative: Eager Lexing with Markers

An alternative approach is for the Lexer to eagerly produce all tokens, using grammar-aware heuristics to detect body boundaries (e.g., `{` after a handler signature pattern triggers native-aware mode). This is simpler to implement but less robust.

**Recommendation:** Pull-based lexer. It's the standard approach in production compilers and gives the parser full control.

---

## 13. Reuse Map

### 13.1 Components Reused Unchanged

| Component | Location | Used By |
|-----------|----------|---------|
| `SyntaxSkipper` trait | native_region_scanner/unified.rs | Segmenter, Lexer |
| `BodyCloser` trait | native_region_scanner/body_closer/ | Segmenter |
| All 7 language `SyntaxSkipper` impls | native_region_scanner/{python,typescript,...}.rs | Segmenter, Lexer |
| All 7 language `BodyCloser` impls | native_region_scanner/body_closer/{python,...}.rs | Segmenter |
| `CodegenNode` enum | codegen/ast.rs | Codegen, Backend |
| `LanguageBackend` trait | codegen/backend.rs | Backend |
| All 7 backend impls | codegen/backends/*.rs | Backend |
| `EmitContext` | codegen/backend.rs | Backend |
| `ClassSyntax` | codegen/backend.rs | Backend |
| `Arcanum` builder | arcanum.rs | Arcanum |
| `FrameValidator` | frame_validator.rs | Validator |
| Runtime generation | codegen/system_codegen.rs (partial) | Codegen |

### 13.2 Components Rewritten

| Component | Current | New |
|-----------|---------|-----|
| `FrameParser` | Hand-rolled byte parser (frame_parser.rs) | Token-consuming parser (new) |
| `compile_ast_based()` | compiler.rs | Pipeline Orchestrator (new) |
| Source segmentation | Split across parser + compiler.rs | Source Segmenter (new, based on PragmaScanner) |
| Handler body scanning | NativeRegionScanner called in codegen | Lexer native-aware mode (new, using same detection logic) |

### 13.3 Components Eliminated

| Component | Reason |
|-----------|--------|
| `skip_native_preamble()` | Replaced by Segmenter |
| `skip_pragmas()` in parser | Replaced by Segmenter |
| `skip_pragmas_simple()` | Replaced by Segmenter |
| `skip_pragmas_keep_native()` | Replaced by Segmenter |
| `extract_native_code()` | Replaced by Segmenter |
| `count_systems()` (full-text search) | Replaced by Segmenter |
| `parse_handler_body_with_scanner()` | Replaced by Lexer native-aware mode + Parser |
| `source: &[u8]` parameter in `generate_system()` | All content now in AST |
| `NativeRegionScanner` trait | Detection logic absorbed by Lexer |
| `RegionScannerTrait` in pipeline/traits.rs | No longer needed |
| `get_region_scanner()` factory | No longer needed |

---

## 14. Data Flow Summary

```
raw bytes
    │
    ├── Segmenter ──► SourceMap (segments: Native, Pragma, System)
    │                     │
    │                     ├── Native spans (for Output Assembly)
    │                     └── System body bytes (for Lexer)
    │
    ├── Lexer ──────► TokenStream (per system)
    │                     │
    │                     ├── Structural tokens (keywords, idents, delims)
    │                     └── NativeCode tokens (opaque strings)
    │
    ├── Parser ─────► SystemAst (per system)
    │                     │
    │                     ├── Handler bodies with interleaved Statement nodes
    │                     ├── Action/Operation bodies as String
    │                     └── Domain initializers as String
    │
    ├── Arcanum ────► Arcanum (symbol table)
    │
    ├── Validator ──► Vec<Diagnostic>
    │
    ├── Codegen ────► CodegenNode tree
    │                     │
    │                     ├── Class, Constructor, Method nodes
    │                     ├── Frame-specific nodes (Transition, Forward, etc.)
    │                     └── NativeBlock nodes (from Statement::NativeCode)
    │
    ├── Backend ────► String (generated class code per system)
    │
    └── Assembler ──► String (final output file)
                          │
                          ├── native prolog (from SourceMap)
                          ├── generated code (from Backend)
                          └── native epilog (from SourceMap)
```

---

## 15. Error Handling

### 15.1 Error Types Per Stage

| Stage | Error Type | Examples |
|-------|-----------|----------|
| Segmenter | `SegmentError` | Unterminated @@system, no system found |
| Lexer | `LexError` | Unexpected byte, unterminated string in native code |
| Parser | `ParseError` | Unexpected token, missing required section |
| Arcanum | `ArcanumError` | Circular HSM reference |
| Validator | `Diagnostic` | Unknown transition target, duplicate state |
| Codegen | (infallible) | Codegen should not fail given valid AST |
| Backend | (infallible) | Backend should not fail given valid CodegenNode |
| Assembler | `AssemblyError` | Missing system in generated map, undefined @@SystemName() |

### 15.2 Unified Diagnostics

All errors convert to a common `Diagnostic` type for reporting:

```rust
pub struct Diagnostic {
    pub code: String,       // "E001", "E402", etc.
    pub message: String,
    pub span: Option<Span>,
    pub severity: Severity,
    pub stage: PipelineStage,
}

pub enum PipelineStage {
    Segmenter,
    Lexer,
    Parser,
    Arcanum,
    Validator,
    Codegen,
    Assembler,
}
```

### 15.3 Error Collection vs. Early Exit

- **Segmenter, Lexer**: Early exit on first error (can't meaningfully continue)
- **Parser**: Collect errors, attempt recovery at synchronization points (section boundaries, state boundaries)
- **Arcanum**: Collect all issues
- **Validator**: Collect all diagnostics (errors + warnings)
- **Codegen, Backend, Assembler**: These stages receive validated input and should not produce errors

---

## 16. File Structure

```
framec/src/frame_c/v4/
├── pipeline/
│   ├── mod.rs                    # Pipeline orchestrator (compile function)
│   ├── config.rs                 # PipelineConfig (existing, minor updates)
│   └── source_map.rs             # SourceMap, Segment types
│
├── segmenter/
│   ├── mod.rs                    # segment() entry point
│   └── syntax_skippers/          # Reused SyntaxSkipper + BodyCloser impls
│       ├── mod.rs                # SyntaxSkipper, BodyCloser traits
│       ├── python.rs
│       ├── typescript.rs
│       ├── rust.rs
│       ├── c.rs
│       ├── cpp.rs
│       ├── java.rs
│       └── csharp.rs
│
├── lexer/
│   ├── mod.rs                    # Lexer struct + next_token()
│   ├── token.rs                  # Token enum, Spanned
│   ├── structural.rs             # Structural mode lexing rules
│   └── native_aware.rs           # Native-aware mode (Frame construct detection)
│
├── parser/
│   ├── mod.rs                    # Parser struct + parse()
│   ├── system.rs                 # System, interface, machine parsing
│   ├── state.rs                  # State, handler parsing
│   ├── expression.rs             # Expression and type parsing
│   └── error.rs                  # ParseError, recovery
│
├── frame_ast.rs                  # FrameAst types (updated, no Span-based content)
├── arcanum.rs                    # Symbol table builder (minor updates)
├── frame_validator.rs            # Validator (minor updates)
│
├── codegen/
│   ├── mod.rs                    # Codegen entry point, generate_system()
│   ├── ast.rs                    # CodegenNode (unchanged)
│   ├── system_codegen.rs         # AST → CodegenNode (updated: no source bytes)
│   ├── backend.rs                # LanguageBackend trait (unchanged)
│   └── backends/
│       ├── python.rs             # (unchanged)
│       ├── typescript.rs         # (unchanged)
│       ├── rust_backend.rs       # (unchanged)
│       ├── c_backend.rs          # (unchanged)
│       ├── cpp_backend.rs        # (unchanged)
│       ├── java_backend.rs       # (unchanged)
│       └── csharp_backend.rs     # (unchanged)
│
└── assembler/
    └── mod.rs                    # Output assembly + tagged instantiation expansion
```

---

## 17. TargetLanguage Unification

### 17.1 Current Problem

There are currently two `TargetLanguage` enums:
- `frame_ast::TargetLanguage` — used by parser and AST
- `visitors::TargetLanguage` — used by CLI, pipeline config, backends

This creates conversion boilerplate throughout the pipeline.

### 17.2 Solution

Define a single `TargetLanguage` enum in a shared location (e.g., `framec/src/frame_c/v4/target.rs`) and use it everywhere. Eliminate the duplicate enum and all conversion code.

```rust
/// Target language for code generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetLanguage {
    Python3,
    TypeScript,
    Rust,
    C,
    Cpp,
    Java,
    CSharp,
}
```

---

## 18. Testing Strategy

### 18.1 Stage-Level Unit Tests

Each stage gets independent unit tests:

| Stage | Test approach |
|-------|--------------|
| Segmenter | Given source bytes, verify SourceMap segments |
| Lexer (structural) | Given Frame syntax bytes, verify token sequence |
| Lexer (native-aware) | Given handler body bytes, verify Frame/Native token interleaving |
| Parser | Given token stream, verify AST structure |
| Arcanum | Given AST, verify symbol table contents |
| Validator | Given AST + Arcanum, verify diagnostics |
| Codegen | Given AST + Arcanum, verify CodegenNode tree |
| Assembler | Given SourceMap + generated code, verify output |

### 18.2 Integration Tests

The existing 539 PRTC tests (Python 144, TypeScript 126, Rust 130, C 139) serve as end-to-end integration tests. Every test that passes before the rewrite must pass after.

### 18.3 Regression Strategy

The rewrite can proceed stage-by-stage. At each stage, the full test suite is run:
1. Implement Segmenter → existing tests pass (Segmenter feeds into existing parser)
2. Implement Lexer → existing tests pass (Lexer feeds into existing parser, then new parser)
3. Implement Parser → existing tests pass (new parser produces compatible AST)
4. Update Codegen (remove source bytes) → existing tests pass
5. Implement Assembler → existing tests pass
6. Remove dead code → existing tests pass

---

## 19. Relationship to Existing Architecture Doc

This document **supersedes** the pipeline sections (1-6) of `frame_v4_architecture.md`. The compartment architecture (Section 0), persistence codegen (Section 9), and runtime dispatch architecture (Section 8) from that document remain accurate and are not affected by this pipeline rewrite.

Once implemented, `frame_v4_architecture.md` should be updated to reference this document for pipeline details, or the relevant sections should be replaced.
