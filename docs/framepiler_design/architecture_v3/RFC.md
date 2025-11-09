# Frame Transpiler V3 Architecture - Request for Comments (RFC)

This document contains RFCs for design decisions, implementation approaches, and architectural choices for the V3 parsing pipeline rebuild.

## RFC Process

**Status Values:**
- `DRAFT` - Initial proposal under discussion
- `REVIEW` - Seeking feedback and technical review
- `ACCEPTED` - Approved for implementation
- `REJECTED` - Not proceeding with this approach
- `IMPLEMENTED` - Completed and verified

**Format:**
Each RFC follows standard conventions with sections for motivation, detailed design, alternatives considered, and unresolved questions.

---

## RFC-001: DPDA-Based Streaming Native Region Scanner

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

Support full native expression parsing within Frame statement arguments (e.g., `-> $State(complex_expr)`) and implement AST-aware indentation derivation for directive expansion.

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
        // 1. Find the AST node containing the directive
        // 2. Determine the expected indentation for that syntactic context
        // 3. Handle special cases: elif/else/except/finally continuation
        // 4. Return computed indent string
    }
}
```

#### Integration with Directive Expansion

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
