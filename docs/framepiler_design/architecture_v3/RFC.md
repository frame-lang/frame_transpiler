# Frame Transpiler V3 Architecture - Request for Comments (RFC)

This document contains RFCs for design decisions, implementation approaches, and architectural choices for the V3 parsing pipeline rebuild.

## RFC Process

**Status Values:**
- `DRAFT` - Initial proposal under discussion
- `REVIEW` - Seeking feedback and technical review
- `ACCEPTED` - Approved for implementation
- `REJECTED` - Not proceeding with this approach
- `IMPLEMENTED` - Completed and verified

**Format:** Each RFC follows standard conventions with sections for motivation, detailed design, alternatives considered, and unresolved questions.

---

## Architecture Review & Resolutions (2025‑11‑09)

This section consolidates concerns raised in the architectural analysis and the concrete resolutions adopted in plan and code. It is normative and supersedes earlier notes.

1) Code Duplication in Scanners
- Concern: Seven scanners duplicate Frame head detection logic.
- Resolution: Introduce a `UnifiedFrameScanner` core (single SOL head detector), with per‑language `LanguageSpecifics` trait for protected‑region handling (strings/comments/raw/preprocessor). Minimizes duplication; improves maintenance.

2) SOL Policy (Start‑of‑Line)
- Concern: SOL disallows indentation.
- Resolution: Clarify as “SOL‑anchored (indentation allowed)”. Scanners accept leading spaces/tabs before Frame statements. Add fixtures proving indented statements are detected.

3) Stage 07 Optionality
- Concern: Two paths diverge behavior.
- Resolution: Stage 07 is runtime‑optional (hermetic core default) but required to implement for all languages. `--validate-native/--strict` enables native parse adapters to validate arg expressions and provide mapped diagnostics via `splice_map`.

4) MIR Scope
- Concern: MIR too small for control flow/expressions.
- Resolution: By policy, V3 keeps logic in native code; MIR models Frame statements only. Extensions require separate RFCs if policy changes.

5) Error Attribution Across Stages
- Concern: Mapping lost across scan→MIR→expand→splice→native parse.
- Resolution: Keep a single `splice_map` from final target spans to origin Frame/native spans; Stage 07 diagnostics remap through it. Trailer printing is gated; mapping composition is always on. Fixtures verify anchors/diagnostics mapping.

6) Multi‑Language Sustainability
- Concern: 7× languages × 8× stages → maintenance/test explosion.
- Resolution: Reduce duplication with `UnifiedFrameScanner`/trait adapters and similar boundaries for outline scanning. Share test patterns via matrix fixtures; prioritize Py/TS strict suites while advancing others for core.

7) Performance vs Correctness
- Concern: Multiple linear passes and allocations.
- Resolution: Favor clear O(n) passes and determinism; add content‑hash caches for incremental builds. Fuse passes only when it doesn’t harm determinism.

8) Language‑Agnostic vs Specific Boundaries
- Concern: Excessive `match lang` in high‑level code.
- Resolution: Push specifics behind traits (protected regions, closers, native policies). Keep high‑level flows language‑agnostic.

9) Native Expression Correctness
- Concern: Without native parsing, malformed arg expressions slip through.
- Resolution: Implement Stage 07 adapters for all languages. In strict mode, expand Frame statements to wrapper calls, splice once, parse with native parser, and map diagnostics back to Frame arg spans via `splice_map`.

10) Testing Strategy
- Concern: Fragmented/duplicate tests.
- Resolution: All behavior tests live in the Python runner. Add per‑phase, per‑language positive and negative fixtures (`v3_prolog`, `v3_imports`, `v3_outline`, `v3_mir`, `v3_mapping`, `v3_validator`, `v3_project`, `v3_facade_*`). Keep crate‑side tests for tiny invariants only.

---

## RFC-001: DPDA-Based Streaming Native Region Scanner (incl. C#)

**Status:** ACCEPTED  
**Date:** 2025-11-09  

### Summary

Implement the Native Region Scanner as a deterministic pushdown streaming scanner (DPDA‑style) with explicit state enums and transition tables. The scanner maintains protected‑region flags, an `at_sol` flag, and a single brace counter for `{…}` in the enclosing Frame body.

### Motivation

The V3 architecture requires a streaming DPDA that maintains `at_sol` + protected‑region flags for accurate boundary detection in native code regions and SOL‑anchored Frame statement detection. The scanner must handle:

- **Python**: single/double/triple quotes, f-strings with brace nesting, `#` comments
- **TypeScript**: single/double quotes, template literals with `${...}` nesting, `//` and `/* */` comments
- **SOL detection**: Frame statement FIRST-set recognition at start-of-line
- **Performance**: O(1) per character with deterministic behavior

### Detailed Design

#### Core DFA Structure

```rust
// Python DFA States
#[derive(Debug, Clone, Copy, PartialEq)]
enum PythonScanState {
    Normal,
    AtSOL,
    InSingleQuote,
    InDoubleQuote,
    InTripleSingle,
    InTripleDouble,
    InFString,
    InFStringBrace { depth: u8 },
    InComment,
}

// Action types for state transitions
#[derive(Debug)]
enum ScanAction {
    Continue,
    FlushNative,           // Start Frame segment 
    ConsumeTriple,         // Consume """ or '''
    FlushAndTransition,    // -> $State detected
    FlushAndForward,       // => $^ detected  
    FlushAndStack,         // $$[+/-] detected
}
```

#### Transition Function Pattern

```rust
impl PythonScanState {
    fn transition(self, ch: char, lookahead: &str) -> (Self, ScanAction) {
        match (self, ch) {
            // Triple quote detection
            (Normal, '\'') if lookahead.starts_with("''") => 
                (InTripleSingle, ScanAction::ConsumeTriple),
            (Normal, '"') if lookahead.starts_with("\"\"") => 
                (InTripleDouble, ScanAction::ConsumeTriple),
            
            // F-string handling
            (Normal, 'f') if lookahead.starts_with("'") || lookahead.starts_with("\"") =>
                (InFString, ScanAction::Continue),
            (InFString, '{') =>
                (InFStringBrace { depth: 1 }, ScanAction::Continue),
            (InFStringBrace { depth }, '{') =>
                (InFStringBrace { depth: depth + 1 }, ScanAction::Continue),
            (InFStringBrace { depth }, '}') if depth > 1 =>
                (InFStringBrace { depth: depth - 1 }, ScanAction::Continue),
            (InFStringBrace { depth: 1 }, '}') =>
                (InFString, ScanAction::Continue),
            
            // SOL and FIRST-set detection
            (Normal, '\n') => (AtSOL, ScanAction::Continue),
            (AtSOL, '-') if lookahead.starts_with("> $") =>
                (Normal, ScanAction::FlushAndTransition),
            (AtSOL, '=') if lookahead.starts_with("> $^") =>
                (Normal, ScanAction::FlushAndForward),
            (AtSOL, '$') if lookahead.starts_with("$[+]") || lookahead.starts_with("$[-]") =>
                (Normal, ScanAction::FlushAndStack),
            (AtSOL, ch) if !ch.is_whitespace() =>
                (Normal, ScanAction::Continue),
            
            // Default transitions
            _ => (self, ScanAction::Continue)
        }
    }
}
```

#### Scanner Interface

```rust
pub trait NativeRegionScanner {
    type State: Copy + Debug;
    
    fn scan_region(&mut self, bytes: &[u8]) -> Vec<Segment>;
    fn reset_state(&mut self);
}

pub struct PythonRegionScanner {
    state: PythonScanState,
    brace_depth: u32,
    current_pos: usize,
    segments: Vec<Segment>,
}
```

#### Decision & Invariants (V3)

- ACCEPTED: Use a deterministic pushdown streaming scanner per target (`NativeRegionScannerPyV3`, `NativeRegionScannerTsV3`).
- One pass yields both the body `close_byte` and `Vec<RegionV3>`; no line slicing; no re‑closing later.
- SOL‑only Frame statement detection (Unicode whitespace allowed); protected‑region aware (strings/templates/comments per target).
- Must‑advance guards; O(n) runtime.
- Rust objects (by stage): `BodyCloser{Py,Ts}V3`, `NativeRegionScanner{Py,Ts}V3`, `FrameStatementParserV3`, `MirAssemblerV3`, `FrameStatementExpander{Py,Ts}V3`, `SplicerV3`, `SourceMapComposerV3`, optional `NativeParseFacade{Py,Ts}V3`, `ValidatorV3`.

#### Segment Output

```rust
#[derive(Debug, Clone)]
pub enum Segment {
    NativeText { 
        span: ByteSpan,
        text: String,
    },
    FrameSegment { 
        span: ByteSpan,
        kind: FrameSegmentKind,
        indent: String,
    },
}

#[derive(Debug, Clone)]
pub enum FrameSegmentKind {
    Transition,    // -> $State(args)
    Forward,       // => $^
    StackPush,     // $$[+] 
    StackPop,      // $$[-]
}
```

### Benefits

1. **Performance**: O(1) per character, no backtracking or stack operations
2. **Deterministic**: Each character advances exactly one state  
3. **Debuggable**: Clear state transitions, easy to trace execution
4. **Composable**: Language-specific DFAs share common interface
5. **Testable**: Pure transition functions enable comprehensive unit tests
6. **Extensible**: Adding new string types or comment styles just requires new states

### Alternatives Considered

#### Closure-Based Scanners
- **Pros**: Functional style, potential for higher-order composition
- **Cons**: Indirection overhead, harder to debug state transitions, less predictable performance

#### Table-Driven Automata  
- **Pros**: Very fast for large state spaces, easy to generate from formal specifications
- **Cons**: Overkill for 10-15 states, harder to maintain, less readable code

#### Regex-Based Scanning

#### C# Notes
- Verbatim/interpolated/raw strings and preprocessor lines require additional states. Raw string openers are `($+)?("{3,})`; closers must match the opener’s quote count, typically on a SOL line. Interpolation brace arity equals the leading `$` count. The scanner maintains `dollar_count`, `quote_count`, and a brace depth within raw/interpolated contexts.
- **Pros**: Concise, well-understood
- **Cons**: Cannot handle nested constructs (f-strings, templates), backtracking performance issues

### Implementation Plan

1. **Phase 1**: Implement Python DFA with basic string/comment/SOL detection
2. **Phase 2**: Add f-string brace nesting and FIRST-set integration  
3. **Phase 3**: Implement TypeScript DFA with template literal support
4. **Phase 4**: Integration testing with existing test suite
5. **Phase 5**: Performance benchmarking and optimization

### Testing Strategy

#### Unit Tests
```rust
#[test]
fn test_python_fstring_nesting() {
    let mut scanner = PythonRegionScanner::new();
    let input = r#"f"outer {inner['key']} text""#;
    let segments = scanner.scan_region(input.as_bytes());
    assert_eq!(segments.len(), 1); // Should be one native segment
}

#[test]
fn test_transition_detection() {
    let mut scanner = PythonRegionScanner::new();
    let input = "    -> $Running\n";
    let segments = scanner.scan_region(input.as_bytes());
    assert!(matches!(segments[0], Segment::FrameSegment { 
        kind: FrameSegmentKind::Transition, .. 
    }));
}
```

#### Integration Tests
- Validate against existing language-specific test suite
- Golden tests for complex nested constructs
- Negative tests for false-positive FIRST-set matches in strings/comments

### Open Questions

1. **Lookahead Size**: How much lookahead is needed for reliable FIRST-set detection? (Currently assuming 4 characters max)

2. **Unicode Handling**: Should the DFA operate on bytes or chars for non-ASCII content? (Recommendation: bytes for delimiters, chars for whitespace)

3. **Error Recovery**: How should the scanner handle malformed constructs like unterminated f-strings?

4. **Memory Usage**: Should we pre-allocate segment vectors or grow them dynamically?

### Unresolved Questions

- Should we optimize for specific patterns (e.g., fast-path when no quotes detected)?
- How should we handle mixed line endings (CR, LF, CRLF) in SOL detection?
- What's the maximum reasonable f-string/template nesting depth we should support?

---

## RFC-002: Native Expression Support in Frame Statement Arguments

**Status:** DRAFT  
**Date:** 2025-11-09  

### Summary

Support full native expression parsing within Frame statement arguments (e.g., `-> $State(complex_expr)`) and implement AST-aware indentation derivation for statement expansion.

### Motivation

## RFC-002: Transition Args (Raw) + Optional AST‑Aware Indentation

**Status:** PARTIALLY ACCEPTED (Core) / DEFERRED (AST features)

### Summary

Keep core argument handling textual and deterministic: extract raw arg text with string‑/nesting‑aware scanning and (optionally) top‑level comma splitting; do not parse native expressions in the core. Expand MIR textually. When Stage 7 native parse facades are enabled, optionally use native AST for indentation/formatting.

### Motivation

Avoid heavyweight parsing in the critical path while allowing quality improvements when native parsers are available.

### Detailed Design

#### Frame Statement Parser (Core)

Use a tiny parser to validate parentheses/brackets/braces and to split args at top‑level commas only. Keep each arg as raw text in MIR; preserve whitespace/newlines.

#### Optional Stage 7 Enhancements

When `NativeParseFacade{Py,Ts}V3` is enabled, parse the spliced body for diagnostics/formatting and allow `IndentationAnalyzer{Py,Ts}` to refine indentation; otherwise, use sibling/line‑based indentation.

#### Object Model

- `FrameStatementParserV3` — target‑agnostic; uses `NativeArgSplitter{Py,Ts}V3` helpers.
- `MirAssemblerV3` — builds MIR using raw arg text.
- `FrameStatementExpander{Py,Ts}V3` — textual expansion + early returns; indentation from sibling lines, optionally AST‑aware via `IndentationAnalyzer{Py,Ts}`.

### Examples (updated nomenclature)

- **Complex expressions**: `-> $State(obj.method(x + y), {"key": value})`
- **Native function calls**: `-> $Running(calculate_timeout(base_ms * factor))`  
- **Target-specific syntax**: TypeScript optional chaining, Python f-strings, etc.

Additionally, indentation derivation should leverage native AST structure when available rather than line-based heuristics.

### Detailed Design

#### Frame Segment Parser Enhancement

```rust
// Enhanced Frame segment parser with native expression support
pub struct FrameSegmentParserV3 {
    target: Target,
    native_parser: Box<dyn NativeExpressionParser>,
}

pub trait NativeExpressionParser {
    fn parse_expression_list(&self, input: &str) -> Result<Vec<ParsedExpr>, ParseError>;
    fn validate_balanced(&self, input: &str) -> Result<(), ParseError>;
}

// Python expression parser implementation
pub struct PythonExpressionParser;

impl NativeExpressionParser for PythonExpressionParser {
    fn parse_expression_list(&self, input: &str) -> Result<Vec<ParsedExpr>, ParseError> {
        // Use rustpython's expression parser for full Python syntax support
        // Handle: literals, calls, subscripts, comprehensions, etc.
    }
}
```

#### MIR Enhancement for Native Expressions

```rust
#[derive(Debug, Clone)]
pub enum MirItem {
    Transition { 
        target: String, 
        args: Vec<NativeExpr>,  // Parsed native expressions
        span: ByteSpan,
    },
    Forward { span: ByteSpan },
    StackPush { span: ByteSpan },
    StackPop { span: ByteSpan },
}

#[derive(Debug, Clone)]
pub struct NativeExpr {
    pub text: String,           // Original source text
    pub parsed: Option<Box<dyn Any>>,  // Target-specific AST node
    pub span: ByteSpan,
}
```

#### AST-Aware Indentation Derivation

```rust
pub trait IndentationAnalyzer {
    fn derive_indent(&self, frame_stmt_span: ByteSpan, context: &NativeAst) -> String;
}

pub struct PythonIndentationAnalyzer;

impl IndentationAnalyzer for PythonIndentationAnalyzer {
    fn derive_indent(&self, frame_stmt_span: ByteSpan, context: &NativeAst) -> String {
        // 1. Find the AST node containing the Frame statement head
        // 2. Determine the expected indentation for that syntactic context
        // 3. Handle special cases: elif/else/except/finally continuation
        // 4. Return computed indent string
    }
}
```

#### Integration with Frame Statement Expansion

```rust
pub struct FrameStatementExpanderPyV3 {
    indentation_analyzer: PythonIndentationAnalyzer,
    runtime_helpers: RuntimeHelpers,
}

impl FrameStatementExpanderPyV3 {
    pub fn expand_transition(&self, 
        mir: &MirItem, 
        native_context: Option<&NativeAst>
    ) -> ExpansionResult {
        match mir {
            MirItem::Transition { target, args, span } => {
                // Use AST-aware indentation if available
                let indent = if let Some(ast) = native_context {
                    self.indentation_analyzer.derive_indent(*span, ast)
                } else {
                    // Fallback to line-based analysis
                    self.derive_indent_from_siblings(*span)
                };
                
                // Generate transition with properly parsed arguments
                let arg_code = args.iter()
                    .map(|expr| &expr.text)  // Use original text
                    .collect::<Vec<_>>()
                    .join(", ");
                    
                format!("{}self._transition_to_{}({})\n{}return", 
                    indent, target, arg_code, indent)
            }
        }
    }
}
```

### Benefits

1. **Full Language Support**: Frame statements can use any valid native expressions
2. **Better Error Reporting**: Native parser provides precise syntax error locations  
3. **AST-Aware Indentation**: Handles complex nested structures correctly
4. **Future-Proof**: Supports language features as they evolve
5. **Consistency**: Same expression parsing as native-only bodies

### Implementation Phases

#### Phase 1: Enhanced Frame Segment Parser
- Integrate native expression parsers (rustpython for Python, SWC for TypeScript)
- Parse transition arguments as full expression lists
- Maintain backward compatibility with simple arguments

#### Phase 2: AST-Aware Indentation  
- Implement IndentationAnalyzer trait per target
- Use native AST structure when available from Stage 7
- Fallback to line-based analysis for compatibility

#### Phase 3: Advanced Expression Features
- Support complex nested expressions
- Handle target-specific syntax (optional chaining, f-strings, etc.)
- Validate expression context (no statements in arguments)

### Testing Strategy

#### Expression Parsing Tests
```rust
#[test]
fn test_complex_python_transition_args() {
    let parser = FrameSegmentParserV3::new(Target::Python);
    let input = r#"-> $Running(
        calculate_delay(base_ms * 1.5),
        {"priority": HIGH, "callback": self.on_complete}
    )"#;
    
    let mir = parser.parse_frame_segment(input).unwrap();
    match mir {
        MirItem::Transition { args, .. } => {
            assert_eq!(args.len(), 2);
            assert!(args[0].text.contains("calculate_delay"));
            assert!(args[1].text.contains("priority"));
        }
        _ => panic!("Expected transition"),
    }
}
```

#### Indentation Tests
```rust
#[test]
fn test_ast_aware_elif_preservation() {
    let expander = FrameStatementExpanderPyV3::new();
    let native_code = r#"
if condition:
    do_something()
    -> $State
elif other_condition:
    do_other()
"#;
    
    // Should preserve elif alignment, not break the chain
    let result = expander.expand_with_ast(native_code);
    assert!(result.contains("elif other_condition:"));
    assert!(!result.contains("if other_condition:"));  // Should not break elif chain
}
```

### Open Questions

1. **Performance Impact**: How much overhead does full expression parsing add?
2. **Error Recovery**: How should we handle malformed expressions in arguments?
3. **AST Caching**: Should we cache parsed expressions for repeated use?
4. **Validation Scope**: Should we validate that arguments match state parameter types?

### Unresolved Questions

- How deep should expression validation go (type checking vs. syntax checking)?
- Should we support multi-line arguments with line continuations?
- How should we handle expressions that reference undefined variables?

---

## RFC-004: Source Map and Debug Output Integration

**Status:** ACCEPTED  
**Date:** 2025-11-09  

### Summary

Compose per‑file source maps from a spliced body and its splice map that attribute every emitted byte span to its origin (Frame statement or native text). Provide deterministic, opt‑in debug outputs (anchors, JSON trailer) to aid bring‑up and tests. Keep the mechanism byte‑based and off by default.

### Detailed Design

#### Data
- `Origin::{ FrameStatement{ frame_line }, NativeText{ start_line, end_line } }`
- `SplicedBodyV3 { text: String, splice_map: Vec<(ByteSpan, Origin)> }`
- `SourceMapV3 { entries: Vec<(ByteSpan /*target*/, Origin)> }`
- `DebugMapPolicyV3 { anchors: bool, trailer_json: bool }`

#### Flow
1) Visitors annotate writes via `builder.map_next(Location)` before emitting glue or native segments.
2) `SplicerV3` collects writes and builds `splice_map`.
3) `SourceMapComposerV3` walks `splice_map` to produce `SourceMapV3`.
4) `DebugMapEmitterV3` (if enabled) injects anchors (comments) near MIR expansions and/or a JSON trailer at EOF.

#### Rules
- MIR expansions (Frame statements) map to their `frame_line` for the entire expansion span.
- Preserved headers after terminal (e.g., `elif:` lines that remain syntactically) map to the same Frame statement origin.
- Native‑only spans map to their native Frame line ranges.
- Byte‑based spans everywhere; byte→line index is used only for diagnostics.

#### Env Flags (Unified)
- `FRAME_MAP_ANCHORS=1` — emit comment anchors near expansions (Py: `# frame:<line>`, TS: `// frame:<line>`)
- `FRAME_MAP_TRAILER=1` — emit JSON trailer with target spans and origins
  - Example trailer:
    ```json
    { "map": [
        { "targetStart": 200, "targetEnd": 230, "frameLine": 123, "origin": "mir" },
        { "targetStart": 231, "targetEnd": 400, "frameStart": 10, "frameEnd": 30, "origin": "native" }
      ],
      "file": "path/to/file.frm" }
    ```
- Default: off in CI; used only in debug and mapping tests.

### Objects
- `SplicerV3`, `SourceMapComposerV3`
- `DebugMapEmitterV3`, `DebugMapPolicyV3`

### Acceptance Criteria
- Linear composition (O(n) in number of writes); disabled by default.
- Mapping is deterministic across runs for the same input.
- Mixed bodies: anchors/trailers correctly reflect MIR/native origins in order.

### Tests
- Golden JSON trailer samples; compare entries order and spans.
- Mixed sequences (Native → MIR → Native) with expected mapping.
- Terminal case: preserved header lines map to MIR frame_line.

### Notes
- Project/multi‑file: compose per‑file maps unchanged; linking does not rewrite spans.

## RFC-003: Cross-Language SOL Pattern Consistency & Unicode Handling

**Status:** ACCEPTED  
**Date:** 2025-11-09  

### Summary

Establish consistent SOL (start-of-line) Frame statement detection patterns across all target languages and define Unicode-aware whitespace handling for robust cross-platform scanning.

### Motivation

The `going_native` documentation reveals important cross-language considerations that V3 must address:

1. **Arrow Conflicts**: Languages like C++, Java, TypeScript use `->` for other purposes (pointer dereference, lambdas)
2. **Forward Conflicts**: Languages like Rust, TypeScript use `=>` in match arms, arrow functions  
3. **Unicode Whitespace**: Need consistent handling of NBSP, em-spaces, and other Unicode whitespace characters
4. **Protected Region Variations**: Each language has different string/comment/template constructs

### Detailed Design

#### Universal SOL Detection Rules

```rust
// Core SOL detection interface (language-agnostic)
pub trait SolDetector {
    fn is_at_sol(&self) -> bool;
    fn advance_whitespace(&mut self);
    fn check_frame_statement_pattern(&self) -> Option<FrameStatementKind>;
}

// Universal Frame statement patterns (require exact tokens)
pub enum FrameStatementPattern {
    Transition,    // "-> $" (require $ to avoid conflicts)
    Forward,       // "=> $^" (require $^ to avoid conflicts)
    StackPush,     // "$$[+]" or "$$+" 
    StackPop,      // "$$[-]" or "$$-"
}
```

#### Unicode-Aware Whitespace

```rust
// Unicode whitespace predicate (consistent across all scanners)
fn is_frame_whitespace(ch: char) -> bool {
    matches!(ch,
        // ASCII whitespace
        ' ' | '\t' | '\n' | '\r' |
        // Unicode whitespace (partial list - see spec)
        '\u{00A0}' |  // NBSP
        '\u{2000}'..='\u{200A}' |  // En quad through hair space
        '\u{202F}' |  // Narrow no-break space
        '\u{205F}' |  // Medium mathematical space
        '\u{3000}'    // Ideographic space
    )
}

// SOL detection after newline
fn advance_to_first_non_whitespace(&mut self) {
    while let Some(ch) = self.current_char() {
        if is_frame_whitespace(ch) {
            self.advance();
        } else {
            break;
        }
    }
}
```

#### Language-Specific Conflict Resolution

```rust
// Python: No major conflicts with Frame patterns
impl SolDetector for PythonRegionScanner {
    fn check_frame_statement_pattern(&self) -> Option<FrameStatementKind> {
        if self.matches_exact("-> $") { Some(Transition) }
        else if self.matches_exact("=> $^") { Some(Forward) }
        else if self.matches_exact("$$[+]") || self.matches_exact("$$+") { Some(StackPush) }
        else if self.matches_exact("$$[-]") || self.matches_exact("$$-") { Some(StackPop) }
        else { None }
    }
}

// TypeScript: Arrow functions use "=>", pointer access not relevant
impl SolDetector for TypeScriptRegionScanner {
    fn check_frame_statement_pattern(&self) -> Option<FrameStatementKind> {
        // Same patterns, but more strict about "=> $^" to avoid arrow function conflicts
        if self.matches_exact("-> $") { Some(Transition) }
        else if self.matches_exact("=> $^") { Some(Forward) }  // Require $^ after =>
        else if self.matches_exact("$$[+]") || self.matches_exact("$$+") { Some(StackPush) }
        else if self.matches_exact("$$[-]") || self.matches_exact("$$-") { Some(StackPop) }
        else { None }
    }
}

// C++: Pointer access "->" requires "-> $" to avoid false positives
impl SolDetector for CppRegionScanner {
    fn check_frame_statement_pattern(&self) -> Option<FrameStatementKind> {
        // More restrictive: require space and $ after -> to avoid ptr->field conflicts
        if self.matches_exact("-> $") { Some(Transition) }  // Require space + $
        else if self.matches_exact("=> $^") { Some(Forward) }
        else if self.matches_exact("$$[+]") || self.matches_exact("$$+") { Some(StackPush) }
        else if self.matches_exact("$$[-]") || self.matches_exact("$$-") { Some(StackPop) }
        else { None }
    }
}
```

#### Protected Region Handling Per Language

```rust
// Language-specific protected region rules
pub trait ProtectedRegionHandler {
    fn enter_string(&mut self, delimiter: char);
    fn handle_escape(&mut self);
    fn enter_comment(&mut self);
    fn enter_template(&mut self);  // For languages with interpolation
}

// Python-specific: f-strings, triple quotes, # comments
impl ProtectedRegionHandler for PythonRegionScanner {
    // Handle f"text {expr {nested}}" with brace nesting
    // Handle single/double/triple quotes
    // Handle # comments to EOL
}

// TypeScript-specific: template literals, // and /* */ comments  
impl ProtectedRegionHandler for TypeScriptRegionScanner {
    // Handle `template ${expr.method()}` with nesting
    // Handle // and /* */ comments
}

// C++: Raw strings, // and /* */ comments, macros
impl ProtectedRegionHandler for CppRegionScanner {
    // Handle R"delimiter(content)delimiter" raw strings
    // Handle #preprocessor lines as protected
}
```

### Language Support Matrix

| Language   | Transition | Forward | String Types | Comments | Special Handling |
|------------|------------|---------|--------------|----------|------------------|
| Python     | `-> $`     | `=> $^` | single/double/triple/f-strings | `#` | f-string brace nesting |
| TypeScript | `-> $`     | `=> $^` | single/double/template | `//` `/* */` | template `${}` nesting |
| C#         | `-> $`     | `=> $^` | normal, verbatim, interpolated, raw (with `$` arity) | `//` `/* */` | interpolation braces, preprocessor lines |
| C          | `-> $`     | `=> $^` | single/double | `//` `/* */` | preprocessor lines |
| C++        | `-> $`     | `=> $^` | single/double/raw | `//` `/* */` | templates, macros |
| Rust       | `-> $`     | `=> $^` | single/double/raw | `//` `/* */` | macro bodies |
| Java       | `-> $`     | `=> $^` | single/double | `//` `/* */` | annotations |

### Benefits

1. **Consistent Behavior**: Frame statements work identically across all targets
2. **Conflict Avoidance**: Strict patterns prevent false positives with native syntax
3. **Unicode Support**: Proper handling of international content and modern editors
4. **Future-Proof**: Pattern-based approach scales to new languages

### Implementation Plan

1. **Phase 1**: Implement universal `SolDetector` trait and Unicode whitespace handling
2. **Phase 2**: Update Python and TypeScript scanners to use consistent patterns  
3. **Phase 3**: Add support for C/C++/Rust/Java with language-specific conflict resolution
4. **Phase 4**: Comprehensive test suite with Unicode edge cases and false-positive prevention

### Testing Strategy

```rust
// Cross-language test suite
#[test]
fn test_unicode_sol_detection() {
    // Test NBSP, em-space, and other Unicode whitespace at SOL
    let input = "\u{00A0}\u{00A0}-> $State\n";  // Leading NBSP
    assert!(scanner.detects_transition_at_sol(input));
}

#[test] 
fn test_language_specific_conflicts() {
    // C++: ptr->field should NOT trigger
    let cpp_input = "ptr->field = value;";
    assert!(!cpp_scanner.detects_transition_at_sol(cpp_input));
    
    // TypeScript: arrow function should NOT trigger  
    let ts_input = "const fn = () => value;";
    assert!(!ts_scanner.detects_forward_at_sol(ts_input));
}
```

### Open Questions

1. **Alternative Syntax**: Should we support alternative Frame statement syntax for conflict-heavy languages?
2. **Validation Level**: How strict should pattern matching be (exact whitespace vs. flexible)?
3. **Error Messages**: How should we report "looks like Frame statement but missing required tokens"?

---

## RFC-004: Source Map and Debug Output Integration

**Status:** DRAFT  
**Date:** 2025-11-09  

### Summary

Integrate the `going_native` source map specification and AST dump capabilities into the V3 pipeline for unified debug output across all target languages.

### Motivation

The `going_native` documentation defines comprehensive debug output specifications that V3 should adopt:

1. **Source Maps**: Line-level mapping from generated code back to Frame source
2. **AST Dumps**: Complete Frame AST with MixedBody details for debugging  
3. **Cross-Language Consistency**: Same debug format across all targets

### Detailed Design

#### Source Map Integration

```rust
// Add to Stage 8: Codegen & Source Maps
pub struct SourceMapComposerV3 {
    mappings: Vec<SourceMapping>,
    source_file: PathBuf,
    target_file: PathBuf,
    target_language: Target,
}

#[derive(Debug, Clone)]
pub struct SourceMapping {
    target_line: usize,
    target_column: Option<usize>,
    source_line: usize, 
    source_column: Option<usize>,
    kind: MappingKind,
    note: Option<String>,
}

#[derive(Debug, Clone)]
pub enum MappingKind {
    NativeSpan,
    MirTransition,
    MirForward, 
    MirStackPush,
    MirStackPop,
    MirReturn,
    Print,
}
```

#### Integration with V3 Pipeline

```rust
// Enhanced Stage 5: Frame Statement Expansion
impl FrameStatementExpanderPyV3 {
    pub fn expand_with_mapping(&mut self, 
        mir: &MirItem,
        source_mapper: &mut SourceMapComposerV3
    ) -> ExpansionResult {
        // Register mapping before expansion
        source_mapper.map_next(mir.source_line());
        
        let expansion = self.expand_transition(mir)?;
        
        // Record the mapping with appropriate kind
        source_mapper.record_mapping(
            self.current_target_line,
            mir.source_line(),
            MappingKind::from_mir_item(mir),
            mir.get_note()
        );
        
        Ok(expansion)
    }
}
```

#### AST Dump Integration

```rust
// Enhanced Stage 4: MIR Assembly  
impl MirAssemblerV3 {
    pub fn assemble_with_debug(&self, 
        segments: Vec<Segment>,
        mir_items: Vec<MirItem>
    ) -> (MixedBody, Option<AstDump>) {
        let mixed_body = self.assemble(segments, mir_items)?;
        
        let ast_dump = if self.debug_enabled {
            Some(self.create_ast_dump(&mixed_body))
        } else {
            None
        };
        
        (mixed_body, ast_dump)
    }
    
    fn create_ast_dump(&self, mixed_body: &MixedBody) -> AstDump {
        AstDump {
            version: 1,
            source_file: self.source_file.clone(),
            target_language: self.target.clone(),
            mixed_body_items: mixed_body.items.iter()
                .map(|item| self.serialize_mixed_body_item(item))
                .collect(),
        }
    }
}
```

#### Debug Output Files

```rust
// Enhanced V3 Pipeline: coordinate debug output
pub struct V3Pipeline {
    debug_enabled: bool,
    output_dir: PathBuf,
}

impl V3Pipeline {
    pub fn compile_with_debug(&mut self, source: &Path) -> Result<CompileOutput, Error> {
        // ... standard pipeline stages ...
        
        if self.debug_enabled {
            self.write_debug_outputs(&source, &mixed_body, &source_map)?;
        }
        
        Ok(output)
    }
    
    fn write_debug_outputs(&self, 
        source: &Path,
        mixed_body: &MixedBody, 
        source_map: &SourceMapComposerV3
    ) -> Result<(), Error> {
        let base_name = source.file_stem().unwrap();
        
        // Write source map: <base>.frame_debug.json
        let source_map_file = self.output_dir.join(format!("{}.frame_debug.json", base_name));
        source_map.write_to_file(&source_map_file)?;
        
        // Write AST dump: <base>.frame_ast.json  
        let ast_dump_file = self.output_dir.join(format!("{}.frame_ast.json", base_name));
        mixed_body.write_ast_dump(&ast_dump_file)?;
        
        Ok(())
    }
}
```

### Integration Points

1. **Stage 2**: Native region scanners record source line info in segments
2. **Stage 4**: MIR assembler creates AST dump when debug enabled  
3. **Stage 5**: Frame statement expanders record mapping entries
4. **Stage 8**: Source map composer writes final debug outputs

### Benefits

1. **Unified Debug Experience**: Same debug output format across all languages
2. **Tool Integration**: IDEs can use source maps for breakpoint mapping
3. **Regression Testing**: AST dumps provide stable comparison artifacts
4. **Performance Debugging**: Source maps enable profiling at Frame level

### Implementation Plan

1. **Phase 1**: Basic source mapping in Python/TypeScript V3 pipeline
2. **Phase 2**: AST dump generation and serialization  
3. **Phase 3**: Integration with `--debug-output` CLI flag
4. **Phase 4**: Tool integration (VS Code extension, etc.)

---

## RFC-005: Frame vs Native Symbol Separation Strategy

**Status:** ACCEPTED (Inherited from Architecture Analysis)  
**Date:** 2025-11-10

### Summary

Establish clear boundaries between Frame symbols (Arcanum) and native symbols (sidecar indices) to maintain architectural separation while enabling cross-domain diagnostics.

### Motivation

The V3 architecture must handle symbols from both Frame (systems, states, actions) and native languages (variables, functions, imports) without creating tight coupling or semantic conflicts. This separation is critical for maintaining clean abstraction boundaries and avoiding architectural complexity.

### Detailed Design

#### Core Principles

1. **Frame symbols remain Frame-owned**: Arcanum holds system/state/action/operation/domain symbols and call chains. We do not merge native symbols into Arcanum.

2. **Native symbols live in sidecar index (optional)**: When target AST is available (native-only bodies), index top-level names and imports for advisory diagnostics only. This index is separate from Arcanum and never blocks codegen.

3. **MixedBody drives emission**: Visitors emit `NativeText` verbatim (or `NativeAst` when available) and expand `MirStatement` using deterministic glue. We do not need a native code printer for glue.

4. **No semantic merging**: Frame and native type systems remain separate. No cross-domain type checking or semantic binding between Frame and native symbols.

#### Symbol Resolution Rules

```rust
// Frame statements inside native bodies reference Frame symbols only
// Arguments are captured as strings; future work may associate parsed target expressions
MirItem::Transition { 
    target: String,        // Frame state name (resolved via Arcanum)
    args: Vec<String>,     // Raw native expressions (not parsed in core)
    span: RegionSpan,
}

// Native symbol index (optional, for diagnostics only)
pub struct NativeSymbolIndex {
    top_level_names: HashMap<String, Span>,
    imports: Vec<ImportDecl>,
    // Advisory only - never used for codegen
}
```

#### Pseudo-symbol Translation (Early Rewrite)

Certain cross-target conveniences are rewritten in an early pass before MIR emission:
- **TypeScript**: `system.return` → `this.returnStack[this.returnStack.length - 1]`  
- **Python**: `system.return` → `self.return_stack[-1]`
- **C#**: `system.return` → `this.ReturnStack[this.ReturnStack.Count - 1]`

Rewrites happen before MixedBody/MIR emission to keep Frame-statement args target-native and avoid Frame-specific leakage in native code.

#### Diagnostics Integration

- Native parser spans map to Frame via `TargetSourceMap`
- Mixed bodies synthesize spans for Frame statements at their Frame lines
- Error messages include both domains where useful (Frame+target)
- Native symbol index provides "undefined native name" warnings without blocking compilation

### Benefits

1. **Clean Separation**: Frame and native concerns remain orthogonal
2. **Maintainable**: No complex cross-domain type systems to maintain
3. **Deterministic**: Symbol resolution is predictable and isolated
4. **Optional Enhancement**: Native symbol indexing is purely advisory
5. **Stable Emission**: No dependence on external printers for Frame statement glue

### Implementation Status

- **Arcanum**: Frame symbol table implemented in V3
- **MixedBody/MIR**: Preserves Frame/native separation
- **Native Symbol Index**: Planned for Stage 7 optional native parse facades
- **Pseudo-symbol Translation**: Deferred pending early rewrite pass design

---

## RFC-006: MIR Design Principles and Terminal Statement Policy

**Status:** ACCEPTED (Implemented in ValidatorV3)  
**Date:** 2025-11-10

### Summary

Establish MIR as a minimal representation focused exclusively on Frame statements, with strict terminal statement validation and ordered preservation of source structure.

### Motivation

The V3 architecture requires a clear, minimal intermediate representation for Frame statements that preserves source order while enforcing Frame semantics. MIR must remain focused and avoid scope creep into general-purpose intermediate representation.

### Detailed Design

#### Core MIR Structure

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MirItemV3 {
    Transition { target: String, args: Vec<String>, span: RegionSpan },
    Forward { span: RegionSpan },
    StackPush { span: RegionSpan },
    StackPop { span: RegionSpan },
}

// MIR is embedded in MixedBody alongside native text
pub enum MixedBodyItem {
    NativeText { text: String, start_line: usize, end_line: usize },
    NativeAst { start_line: usize, end_line: usize, ast: Box<dyn Any> },
    Frame(MirItemV3),
}
```

#### Core Invariants

1. **Item order reflects source order**: MIR preserves the exact sequence of native text and Frame statements as they appear in source
2. **Terminal statements end handlers**: Transition, Forward, and StackPop must be the last statement in handler bodies (enforced by ValidatorV3)
3. **Native logic stays native**: MIR does not model control flow, expressions, or native language constructs
4. **Span preservation**: Every MIR item maintains its original source location for mapping and diagnostics

#### MIR Scope Limitations

**By policy**, V3 keeps business logic in native code; MIR models only Frame state machine statements:

- **No control flow**: No if/else, loops, or conditionals in MIR
- **No expressions**: Native expressions remain as string args, not parsed into MIR
- **No native constructs**: Function calls, variable assignments, etc. stay in native text
- **No type system**: MIR does not model or validate native types

#### Terminal Statement Validation

```rust
// ValidatorV3 enforces terminal-last rule
pub struct TerminalLastRuleV3;

impl ValidationRuleV3 for TerminalLastRuleV3 {
    fn validate(&self, mir: &[MirItemV3]) -> Vec<ValidationIssueV3> {
        // Find terminal statements (Transition, Forward, StackPop)
        // Ensure no MIR statements follow terminal statements
        // Report violations with precise span information
    }
}
```

#### Assembly Strategy

MIR is assembled from segmented regions by `MirAssemblerV3`:
1. **Frame segments** → parse into `MirItemV3` with span preservation
2. **Native segments** → preserve as `MixedBodyItem::NativeText`
3. **Order preservation** → maintain exact source sequence
4. **Validation** → apply terminal-last rule during assembly

#### B2 Emission Strategy (Future)

- Expand `MirItemV3` into target AST nodes (e.g., SWC for TypeScript) rather than string glue
- Print via native code generators for formatting determinism
- Attach synthesized spans mapped to Frame statement lines
- Preserve terminal statement semantics in target code

### Benefits

1. **Minimal Scope**: MIR stays focused on Frame semantics only
2. **Source Fidelity**: Preserves exact order and spans from source
3. **Terminal Safety**: Validates Frame state machine invariants
4. **Emission Flexibility**: Supports both textual and AST-based code generation
5. **Debuggable**: Clear mapping from Frame statements to generated code

### Limitations by Design

- **No native parsing**: Arguments remain as strings in core pipeline
- **No cross-domain validation**: MIR doesn't validate native expressions
- **No optimization**: MIR preserves source structure without transformation
- **Extensions require RFCs**: Any scope expansion needs architectural review

### Implementation Status

- **Core MIR types**: Implemented in `framec/src/frame_c/v3/mir.rs`
- **Terminal validation**: Implemented in `ValidatorV3`
- **Assembly**: Implemented in `MirAssemblerV3`
- **B2 emission**: Future enhancement for AST-based code generation

---

## RFC-007: Enhanced Source Mapping and Debug Output Strategy

**Status:** DRAFT (Extends RFC-004)  
**Date:** 2025-11-10

### Summary

Implement comprehensive source mapping with multiple debug output formats, cross-domain diagnostic mapping, and enhanced debugging capabilities beyond the basic trailer approach.

### Motivation

The current RFC-004 provides basic source mapping, but analysis of legacy documentation reveals more sophisticated mapping requirements for effective debugging, IDE integration, and cross-domain error attribution.

### Detailed Design

#### Enhanced Debug Output Formats

Beyond the current `FRAME_MAP_TRAILER=1` approach, add support for multiple debug modes:

##### TypeScript Enhanced Debug Modes
```bash
# Comment-based mapping trailer
FRAME_TS_MAP_COMMENTS=1

# Generates:
// __frame_map_begin__
// map frame:123 -> ts:210
// map frame:124 -> ts:225  
// __frame_map_end__

# JSON-based mapping with metadata
FRAME_TS_MAP_JSON=1

# Generates:
// __frame_map_json_begin__
// [{"frameLine":123,"tsLine":210,"type":"transition"},{"frameLine":124,"tsLine":225,"type":"native"}]
// __frame_map_json_end__
```

##### Python Enhanced Debug Modes
```bash
# Python comment mapping
FRAME_PY_MAP_COMMENTS=1

# Generates:
# __frame_map_begin__
# map frame:123 -> py:210
# map frame:124 -> py:225
# __frame_map_end__
```

#### Mapping Policy for MixedBody

Detailed mapping rules for different content types:

```rust
#[derive(Debug, Clone)]
pub enum MappingOrigin {
    FrameStatement { frame_line: usize, kind: MirKind },
    NativeText { start_line: usize, end_line: usize },
    PreservedHeader { frame_line: usize }, // Headers after terminal statements
}

impl SourceMapComposerV3 {
    // Map first generated line of segment to segment start_line
    fn map_native_segment(&mut self, target_span: ByteSpan, native_span: RegionSpan);
    
    // Map entire expansion to statement's frame_line
    fn map_frame_statement(&mut self, target_span: ByteSpan, frame_line: usize, kind: MirKind);
    
    // Headers that remain syntactically valid after terminal statements
    fn map_preserved_header(&mut self, target_span: ByteSpan, frame_line: usize);
}
```

#### Cross-Domain Error Attribution

Enhanced error mapping between Frame and native domains:

```rust
pub struct CrossDomainDiagnostic {
    pub frame_location: FrameLocation,
    pub target_location: Option<TargetLocation>,
    pub error_kind: DiagnosticKind,
    pub context: String,
}

pub enum DiagnosticKind {
    FrameValidation,     // Pure Frame errors (state doesn't exist)
    NativeSyntax,        // Native parser errors mapped back to Frame
    CrossDomain,         // Errors spanning both domains
}
```

##### Error Mapping Examples
- **Target parser errors**: Map back via `TargetSourceMap` (frame_start_line + offsets)
- **Native symbol errors**: "Undefined variable 'foo' in Frame statement args (frame line 42, python line 156)"
- **Frame validation errors**: Include original Frame line context with native code snippet

#### Source Map Composition Pipeline

Enhanced Stage 6 integration:

```rust
impl SplicerV3 {
    pub fn splice_with_enhanced_mapping(&self, 
        content: &[u8],
        regions: &[RegionV3],
        expansions: &[String],
        debug_policy: DebugMapPolicyV3,
    ) -> SplicedBodyV3 {
        let mut splice_map = Vec::new();
        let mut composed_text = String::new();
        
        // Build detailed splice map with origin tracking
        for (region, expansion) in regions.iter().zip(expansions) {
            match region {
                RegionV3::NativeText { span } => {
                    let origin = MappingOrigin::NativeText { 
                        start_line: span.start_line, 
                        end_line: span.end_line 
                    };
                    splice_map.push((composed_text.len()..composed_text.len() + expansion.len(), origin));
                }
                RegionV3::FrameSegment { span, kind, frame_line } => {
                    let origin = MappingOrigin::FrameStatement { 
                        frame_line: *frame_line, 
                        kind: kind.clone() 
                    };
                    splice_map.push((composed_text.len()..composed_text.len() + expansion.len(), origin));
                }
            }
            composed_text.push_str(expansion);
        }
        
        // Apply debug output formatting based on policy
        if debug_policy.enhanced_comments {
            self.inject_comment_mapping(&mut composed_text, &splice_map);
        }
        if debug_policy.json_trailer {
            self.inject_json_trailer(&mut composed_text, &splice_map);
        }
        
        SplicedBodyV3 { text: composed_text, splice_map }
    }
}
```

#### Integration with Native Parse Facades (Stage 7)

When native parsing is enabled, enhance diagnostics:

```rust
impl NativeParseFacadeV3 {
    fn validate_with_mapping(&self, 
        spliced_body: &SplicedBodyV3
    ) -> Result<ValidationResult, Vec<CrossDomainDiagnostic>> {
        // Parse spliced native body
        let native_ast = self.parse_spliced_body(&spliced_body.text)?;
        
        // Map any native errors back to Frame spans via splice_map
        let diagnostics = native_ast.errors().into_iter()
            .map(|err| self.map_native_error_to_frame(err, &spliced_body.splice_map))
            .collect();
            
        if diagnostics.is_empty() {
            Ok(ValidationResult::Success)
        } else {
            Err(diagnostics)
        }
    }
}
```

### Benefits

1. **Rich Debug Information**: Multiple output formats for different debugging needs
2. **IDE Integration**: Comment-based maps enable IDE breakpoint mapping
3. **Cross-Domain Attribution**: Precise error mapping between Frame and native code
4. **Flexible Output**: Configurable debug modes via environment variables
5. **Preserved Context**: Headers after terminal statements maintain syntactic validity

### Implementation Plan

1. **Phase 1**: Implement enhanced mapping origins and splice map composition
2. **Phase 2**: Add TypeScript/Python comment-based debug modes
3. **Phase 3**: Integrate with Stage 7 native parse facades for cross-domain diagnostics
4. **Phase 4**: Add JSON trailer format with rich metadata
5. **Phase 5**: IDE integration and tooling support

### Testing Strategy

- **Golden mapping tests**: Compare debug output format consistency across runs
- **Cross-domain error tests**: Verify native errors map correctly to Frame locations  
- **Mixed body mapping**: Test Native→MIR→Native sequences with expected origins
- **Terminal preservation**: Ensure preserved headers map to correct Frame statements

---

## RFC-008: Native Parser Integration Boundaries and Contracts

**Status:** ACCEPTED (Implemented in Stage 7)  
**Date:** 2025-11-10

### Summary

Define clear boundaries between Frame scanning/MIR and native parser responsibilities to avoid semantic coupling while enabling optional enhanced validation and diagnostics.

### Motivation

The V3 architecture must maintain clean separation between Frame semantics and native language processing. Native parsers serve as optional validators and structure providers, but must not interfere with Frame's core compilation pipeline or introduce semantic dependencies.

### Detailed Design

#### Boundary Contract Definitions

1. **Scanner Authority**: Native region scanners ensure Frame statements (`-> $State`, `=> $^`, `$$+/-`) are never passed to target parsers. The scanning phase has complete authority over Frame/native boundary detection.

2. **Mixed Body Priority**: Bodies containing Frame statements use NativeRegionSegmenter + MIR path exclusively. Target parsers are never invoked on mixed content.

3. **Native-Only Bodies**: Target parsers handle pure native code for validation, AST structure, and optional enhanced diagnostics only.

4. **No Semantic Binding**: Target parsers do not resolve external modules, perform symbol binding, or execute semantic analysis beyond syntax validation.

#### Native Parser Responsibilities

```rust
pub trait NativeParseFacadeV3 {
    type Ast;
    type Error;
    
    // Core responsibility: syntax validation only
    fn parse_native_body(&self, text: &str) -> Result<Self::Ast, Vec<Self::Error>>;
    
    // Optional: enhanced diagnostics with Frame context
    fn parse_with_frame_context(&self, 
        text: &str, 
        frame_context: &FrameContext
    ) -> Result<ValidationResult, Vec<CrossDomainError>>;
    
    // Optional: indentation analysis for Frame statement expansion
    fn analyze_indentation(&self, ast: &Self::Ast, frame_stmt_location: ByteSpan) -> String;
}
```

#### Integration Points and Limitations

##### Stage 2: Native Region Scanner Boundary
```rust
// Scanner ensures clean separation - Frame statements never reach parser
impl NativeRegionScannerV3 {
    fn scan(&self, body_bytes: &[u8]) -> Result<ScanResultV3, ScanError> {
        // Frame statements detected at SOL → FrameSegment
        // Everything else → NativeText  
        // Mixed bodies: multiple regions, parser handles NativeText only
        // Native-only bodies: single NativeText region, full parser validation available
    }
}
```

##### Stage 7: Optional Native Parse Facade
```rust
// Native parsing is runtime-optional and hermetic
impl CompilerV3 {
    fn compile_with_native_validation(&self, 
        source: &str,
        enable_native_parsing: bool
    ) -> Result<CompileOutput, CompileError> {
        // Standard pipeline: Scan → Parse → MIR → Expand → Splice
        let spliced_body = self.standard_pipeline(source)?;
        
        if enable_native_parsing {
            // Optional: validate spliced native code and map diagnostics
            let validation = self.native_facade.validate_spliced(&spliced_body)?;
            // Map any errors back to Frame locations via splice_map
        }
        
        Ok(spliced_body)
    }
}
```

#### Native Parser Scope Limitations

**Explicit Non-Responsibilities**:

1. **No External Resolution**: Runtime imports, modules, and external libraries are not resolved during parsing
2. **No Frame Semantics**: Target parsers never handle Frame statements - that's exclusively MIR's responsibility
3. **No Code Generation**: Primary purpose is validation and error mapping, not native code emission
4. **No Symbol Binding**: Variable resolution, type checking, and semantic analysis are out of scope
5. **No Cross-Domain Validation**: No validation of Frame statement arguments as native expressions

#### Contract Enforcement

```rust
// Compile-time contract enforcement
pub struct ParserContractValidator;

impl ParserContractValidator {
    // Ensure Frame statements never reach target parsers
    fn validate_scanner_separation(&self, regions: &[RegionV3]) {
        for region in regions {
            match region {
                RegionV3::FrameSegment { .. } => {
                    // Frame segments must be handled by MIR, not parser
                    assert!(!self.would_reach_parser(region));
                }
                RegionV3::NativeText { .. } => {
                    // Native text may reach parser for validation
                    assert!(!self.contains_frame_statements(region));
                }
            }
        }
    }
}
```

#### Error Handling and Diagnostics

Native parser errors are mapped back to Frame context without semantic interpretation:

```rust
pub struct NativeParserError {
    pub native_location: SourceLocation,     // Location in generated code
    pub frame_location: Option<SourceLocation>, // Mapped back to Frame source
    pub error_kind: NativeErrorKind,
    pub suggestion: Option<String>,
}

pub enum NativeErrorKind {
    SyntaxError,        // Pure syntax issues
    IndentationError,   // Python-specific indentation problems  
    UnterminatedConstruct, // Unclosed strings, comments, etc.
}
```

#### Implementation Status and Integration

##### Current Implementation (V3)
- **Stage 2**: Native region scanners enforce clean Frame/native separation
- **Stage 4**: MIR assembler handles Frame statements exclusively
- **Stage 6**: Splicer produces native-only text for optional parser validation
- **Stage 7**: Native parse facades implemented for major targets (off by default)

##### Integration with Validation
```rust
// ValidatorV3 coordinates Frame and native validation
impl ValidatorV3 {
    fn validate_with_native_parsing(&self, 
        mixed_body: &MixedBody,
        enable_native: bool
    ) -> ValidationResult {
        // Always: Frame validation (terminal-last, state existence, etc.)
        let frame_issues = self.validate_frame_semantics(mixed_body);
        
        // Optional: Native validation if enabled
        let native_issues = if enable_native {
            self.validate_native_syntax(mixed_body)
        } else {
            Vec::new()
        };
        
        ValidationResult::from_issues(frame_issues, native_issues)
    }
}
```

### Benefits

1. **Clean Architecture**: Frame and native concerns remain orthogonal with clear contracts
2. **Optional Enhancement**: Native parsing provides value without blocking core compilation
3. **Maintainable**: Clear responsibilities prevent scope creep and coupling
4. **Extensible**: Additional languages can be supported without architectural changes
5. **Debuggable**: Clear error attribution between Frame and native domains

### Testing Strategy

#### Contract Validation Tests
```rust
#[test]
fn test_frame_statements_never_reach_parser() {
    let mixed_source = r#"{
        native_code();
        -> $NextState
        more_native();
    }"#;
    
    let regions = scanner.scan(mixed_source);
    for region in regions {
        if let RegionV3::NativeText { text } = region {
            // Native text regions must never contain Frame statements
            assert!(!text.contains("-> $"));
            assert!(!text.contains("=> $^"));
            assert!(!text.contains("$$"));
        }
    }
}

#[test] 
fn test_native_only_body_can_use_parser() {
    let native_only = r#"{
        const result = calculateValue(x, y);
        console.log(result);
    }"#;
    
    let regions = scanner.scan(native_only);
    assert_eq!(regions.len(), 1);
    
    // Native-only body can be parsed for validation
    let parse_result = typescript_parser.parse_native_body(&regions[0].text);
    assert!(parse_result.is_ok());
}
```

#### Boundary Enforcement Tests
- **Mixed body separation**: Ensure Frame statements are properly segmented
- **Native-only validation**: Verify pure native bodies can be parsed successfully
- **Error mapping**: Test that native errors map correctly to Frame locations
- **Contract violations**: Negative tests ensuring Frame statements don't reach parsers

### Implementation Notes

Native parsers are implemented as pluggable components with consistent interfaces across languages. The boundary contracts are enforced at compile-time through type system constraints and runtime through validation assertions.

---

## RFC Template

```markdown
## RFC-XXX: Title

**Status:** DRAFT/REVIEW/ACCEPTED/REJECTED/IMPLEMENTED  
**Date:** YYYY-MM-DD  

### Summary
Brief description of the proposal.

### Motivation
Why is this change needed? What problem does it solve?

### Detailed Design
Technical specification with code examples.

### Alternatives Considered
Other approaches that were evaluated.

### Implementation Plan
Phased approach to implementation.

### Testing Strategy
How will this be validated?

### Open Questions
Issues that need resolution.

### Unresolved Questions  
Future considerations and edge cases.
```
### Decision & Implementation Notes (V3)

- SOL definition (both targets): immediately after a newline (or body open) and before any non‑whitespace outside protected regions; recognizes Unicode whitespace at SOL.
- FIRST‑set at SOL (outside protected regions):
  - Transition: `->` WS+ `$` state
  - Parent forward: `=>` WS+ `$^`
  - Stack: `$$[+]` and `$$[-]` (canonical)
- Protected regions: strings/comments/templates per target; no detection inside them.
- Newline handling: LF and CRLF normalize the same for SOL.
- Unicode whitespace at SOL: tabs, ASCII space, NBSP (U+00A0), common Zs (U+2000..U+200B, U+202F, U+205F, U+3000); BOM at body start is skipped.
- Scanner maintains `at_sol`; parser validates heads and rejects trailing tokens.
