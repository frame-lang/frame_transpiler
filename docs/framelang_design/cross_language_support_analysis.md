# Cross-Language Support Analysis

**Document Version**: 1.0  
**Date**: 2025-10-30  
**Status**: Design Analysis  
**Related Issues**: Bug #055 - TypeScript async runtime lacks socket helpers

## Overview

This document analyzes the current cross-language support challenges in Frame and proposes a target-specific syntax approach to improve language compatibility while maintaining Frame's state machine semantics.

**Philosophical Shift**: Frame is evolving from "universal syntax" to "universal state machine patterns with target-specific implementation." This preserves Frame's core value—structured state machine development—while allowing each target language to use its native idioms for implementation details.

<!-- Comment (AI): Consider stating up front that we're pivoting from "universal syntax" toward "universal state structure" so reviewers grasp the philosophical shift before diving into mechanics. -->
<!-- Response: Agreed - added philosophical shift statement to overview for clarity -->

## 🎯 Current State Analysis

### Frame Language Philosophy Evolution

Frame has evolved from a "write-once-run-everywhere" transpiler to a "write-Frame-compile-to-idiomatic-target" system:

- **Python Target**: 95%+ pass-through syntax, Frame ≈ Python semantics
- **TypeScript Target**: Significant semantic gaps requiring runtime helpers
- **LLVM Target**: Completely different paradigm requiring custom runtime

### Current Cross-Language Issues

1. **Semantic Gaps**: Features that work differently across languages
   - Set equality: `set1 == set2` (Python native vs TypeScript requires helpers)
   - Async operations: `await reader.readline()` (Python asyncio vs Node.js streams)
   - Exception handling: `try/except` vs `try/catch` patterns

2. **Runtime Maintenance Burden**: 
   - Each semantic gap requires runtime helper implementation
   - Visitors must be updated for every runtime change (N languages × M features)
   - Tight coupling between runtime libraries and code generation

3. **Performance Overhead**:
   - Runtime abstraction layers reduce performance
   - Generated code is less idiomatic
   - Debugging generated code is harder

**Concrete Example**: Bug #055 demonstrates the cost—adding TypeScript async socket support requires implementing `FrameAsyncSocket.readline()`, `FrameBuffer.encode()`, and `FrameBuffer.decode()` helpers, plus updating the TypeScript visitor to use them. Each new async feature multiplies this N×M maintenance burden.

<!-- Comment (AI): Maybe cite a representative example (e.g., async sockets) so stakeholders see the concrete cost of keeping the status quo. -->
<!-- Response: Added Bug #055 as concrete example of runtime maintenance burden -->

## 🚨 Specific Problem: Bug #055

### Issue Description
The Frame debugger's `runtime_protocol.frm` uses async socket operations that work in Python but fail in TypeScript due to missing runtime helpers:

```frame
# Works in Python, fails in TypeScript
import asyncio
var line = await reader.readline()
var encoded = text.encode('utf-8')
```

### Current Solutions Considered

1. **Runtime Helpers Approach**: Add TypeScript implementations for socket operations
   - **Problem**: Doesn't scale - every new async feature needs N runtime implementations

2. **Intrinsic Functions Approach**: Built-in language features for common operations
   - **Problem**: Ensuring identical semantics across very different runtime models

## 🎯 Proposed Solution: Pragma-Based Target-Specific Syntax

### Core Concept

Accept that Frame programs will work differently per target language and expand syntax to support native language constructs using pragma declarations.

### File-Level Target Declaration Approach

```frame
@target typescript

import * as net from 'net'
import * as util from 'util'

system RuntimeProtocol {
    actions:
        async setupSocket() {
            // Native TypeScript code
            this.socket = net.connect(this.port, this.host)
            this.reader = new util.promisify(this.socket.read.bind(this.socket))
        }
        
        async readLine(): string {
            return await this.reader()
        }
}
```

Equivalent Python version:
```frame
@target python

import asyncio
import socket

system RuntimeProtocol {
    actions:
        async setupSocket(self):
            # Native Python code
            self.reader, self.writer = await asyncio.open_connection(self.host, self.port)
        
        async readLine(self) -> str:
            line = await self.reader.readline()
            return line.decode('utf-8')
}
```

**LLVM Native Implementation Example:**
```frame
@target llvm

system RuntimeProtocol {
    actions:
        async connect() {
            // Embedded Rust helpers compiled to LLVM IR
            %socket = call i32 @frame_socket_connect(i32 %port, i8* %host)
            %reader = call %FrameAsyncReader* @frame_reader_new(i32 %socket)
        }
        
        async readLine(): string {
            %line_ptr = call i8* @frame_reader_readline(%FrameAsyncReader* %reader)
            ret i8* %line_ptr
        }
}
```

For LLVM, "native" means embedded Rust runtime helpers that compile to LLVM IR, providing Frame async semantics through optimized system calls and memory management.

<!-- Comment (AI): For LLVM it'd help to show what "native" looks like—are we embedding Rust helpers or handwritten IR? Without that, it's hard to gauge the impact on the native backend roadmap. -->
<!-- Recommendation: prototype two strategies (inline Rust FFI shim vs. raw IR snippets) and pick one upfront; document which toolchain (e.g., `cc`, `clang`, `rustc`) will own the compiled artifacts so the visitor/runtime bindings stay predictable. -->
<!-- Response: Added LLVM example showing embedded Rust helpers compiled to LLVM IR -->

## 🏗️ Multi-Pass Parser Architecture

### Current Architecture (2-Pass)
```
Pass 1: Build symbol table (is_building_symbol_table = true)
Pass 2: Parse + semantic analysis (is_building_symbol_table = false)
```

### Proposed Architecture (3+ Pass)

#### **Pass 1: Target Declaration & Syntax Discovery**
- Parse `@target language` declaration at file start
- Identify Frame syntax regions vs target-specific syntax regions
- Store raw tokens for target-specific sections

```rust
struct TargetDiscoveryPass {
    target_language: Option<TargetLanguage>,
    syntax_regions: Vec<SyntaxRegion>,
}

enum SyntaxRegion {
    CommonFrame { start: Position, end: Position },
    TargetSpecific { 
        target: TargetLanguage, 
        start: Position, 
        end: Position,
        raw_tokens: Vec<Token> 
    },
}
```

#### **Pass 2: Frame Common Syntax**
- Parse Frame-universal constructs (systems, states, transitions)
- Build symbol table for Frame constructs
- Leave target-specific syntax as unparsed token streams

#### **Pass 3: Target-Specific Syntax Integration**
- Parse target-specific syntax into native AST nodes
- Integrate with Frame AST structure
- Validate target syntax correctness

```rust
impl Parser {
    fn parse_target_specific_block(&mut self, tokens: &[Token], target: TargetLanguage) -> Box<dyn AstNode> {
        match target {
            TargetLanguage::Python => self.parse_python_syntax(tokens),
            TargetLanguage::TypeScript => self.parse_typescript_syntax(tokens),
            TargetLanguage::Llvm => self.parse_llvm_syntax(tokens),
        }
    }
}
```

### AST Node Extensions

```rust
enum ActionBody {
    Frame(FrameActionBody),           // Traditional Frame actions
    TargetSpecific {
        target: TargetLanguage,
        native_ast: Box<dyn TargetAst>,  // Native language AST
    },
}

trait TargetAst {
    fn to_code(&self) -> String;
    fn get_dependencies(&self) -> Vec<String>;
    fn validate(&self) -> Result<(), ParseError>;
}
```

## 🔧 Scanner Architecture

### Single Scanner with Target-Aware Tokenization

The approach uses **one scanner** that adapts its tokenization strategy based on the target language declaration. This avoids the complexity of multiple scanners while supporting target-specific syntax.

### Scanner State Management

```rust
pub struct Scanner {
    // Existing Frame scanner fields
    source: Vec<char>,
    current: usize,
    line: usize,
    
    // New target-aware fields
    target_language: Option<TargetLanguage>,
    scanning_mode: ScanningMode,
    target_specific_regions: Vec<TargetRegion>,
}

enum ScanningMode {
    TargetDiscovery,           // Scanning for @target declaration
    FrameCommon,               // Standard Frame tokenization
    TargetSpecific(TargetLanguage), // Language-specific tokenization
}

struct TargetRegion {
    start_position: usize,
    end_position: Option<usize>,
    raw_content: String,
    target: TargetLanguage,
    source_map: TargetSourceMap,  // Maps target lines to Frame lines
}

pub struct TargetSourceMap {
    frame_start_line: usize,
    target_line_offsets: Vec<usize>,  // Target line N maps to frame_start_line + offset[N]
}
```

**Diagnostics Strategy**: Target-specific parsing errors report both native language line numbers (for developer familiarity) and Frame source locations (for IDE integration). Example:
```
Error: TypeScript syntax error at line 3 in action body
  --> runtime_protocol.frm:15:8 (Frame source)
  --> action connect(), line 3:12 (TypeScript)
   |
 3 |     this.socket = net.connect(port host)  // Missing comma
   |                                    ^^^^
```

<!-- Comment (AI): Worth clarifying how diagnostics will surface—if a target block fails to parse, do we report native-language line numbers? Tooling folks will want that detail. -->
<!-- Recommendation: capture per-target offsets in `TargetSourceMap` and surface dual-line info in errors (Frame + native); commit to wiring this through the existing diagnostic pipeline before enabling pragmatic mode. -->
<!-- Response: Added diagnostics strategy with dual line number reporting and source mapping -->
```

### Token Type Extensions

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Existing Frame tokens
    Identifier,
    String,
    Number,
    // ... existing tokens
    
    // New target declaration tokens
    AtSymbol,              // @
    TargetKeyword,         // target
    
    // Target-specific token container
    TargetSpecificBlock {
        target: TargetLanguage,
        raw_tokens: String,   // Unparsed target-specific content
        start_line: usize,
        end_line: usize,
    },
}
```

### Scanning Strategy

#### 1. Target Discovery Phase
```rust
impl Scanner {
    fn scan_target_declaration(&mut self) -> Result<Option<TargetLanguage>, ScanError> {
        // Look for @target at start of file
        if self.current == 0 && self.match_char('@') {
            if self.scan_identifier() == "target" {
                let target_name = self.scan_identifier();
                return Ok(Some(TargetLanguage::from_string(&target_name)?));
            }
        }
        Ok(None)
    }
}
```

#### 2. Adaptive Tokenization
```rust
impl Scanner {
    fn scan_token(&mut self) -> Token {
        match self.scanning_mode {
            ScanningMode::TargetDiscovery => {
                self.scan_for_target_declaration()
            }
            ScanningMode::FrameCommon => {
                self.scan_frame_token()
            }
            ScanningMode::TargetSpecific(target) => {
                self.scan_target_specific_content(target)
            }
        }
    }
    
    fn scan_target_specific_content(&mut self, target: TargetLanguage) -> Token {
        // Don't tokenize - just capture raw content until Frame syntax resumes
        let start = self.current;
        
        // Scan until we hit Frame constructs like 'system', 'actions', etc.
        while !self.at_frame_keyword() && !self.is_at_end() {
            self.advance();
        }
        
        let content = self.source[start..self.current].iter().collect();
        Token::new(
            TokenType::TargetSpecificBlock {
                target,
                raw_tokens: content,
                start_line: self.line,
                end_line: self.line,
            },
            self.line,
        )
    }
    
    fn at_frame_keyword(&self) -> bool {
        // Check if current position is at a Frame keyword
        matches!(self.peek_identifier(), Some("system" | "actions" | "machine" | "interface" | "domain"))
    }
}
```

### Why Single Scanner Works

#### **Advantages:**
1. **State continuity** - maintains line numbers, positions across mode switches
2. **Simpler architecture** - one tokenization pipeline
3. **Shared lexical rules** - Frame keywords work consistently
4. **Error handling** - unified error reporting

#### **Mode Switching Strategy:**
```rust
impl Scanner {
    fn switch_scanning_mode(&mut self, new_mode: ScanningMode) {
        match (&self.scanning_mode, &new_mode) {
            (ScanningMode::TargetDiscovery, ScanningMode::FrameCommon) => {
                // Transition after @target found
                self.scanning_mode = new_mode;
            }
            (ScanningMode::FrameCommon, ScanningMode::TargetSpecific(target)) => {
                // Entering action body or import section
                self.start_target_region(*target);
                self.scanning_mode = new_mode;
            }
            (ScanningMode::TargetSpecific(_), ScanningMode::FrameCommon) => {
                // Exiting action body, returning to Frame syntax
                self.end_target_region();
                self.scanning_mode = new_mode;
            }
            _ => {} // Other transitions not allowed
        }
    }
}
```

**Boundary Detection Strategy**: The scanner uses a **keyword-based approach** to transition back to Frame syntax. When in `TargetSpecific` mode, the scanner looks ahead for Frame keywords (`system`, `actions`, `machine`, `interface`, `domain`) or Frame-specific syntax (state transitions `-> $State`). 

**Nested Construct Handling**: Braces, parentheses, and brackets within target-specific blocks are counted to ensure balanced nesting, but don't trigger mode switches. Only Frame keywords at the appropriate nesting level trigger the return to `FrameCommon` mode.

```rust
fn detect_frame_boundary(&self) -> bool {
    self.brace_depth == 0 && 
    self.at_frame_keyword() &&
    !self.in_string_literal()
}
```

<!-- Comment (AI): Might call out how we detect the boundary back to Frame syntax (keywords vs braces) to assure readers nested constructs won't confuse the scanner. -->
<!-- Recommendation: spell out the exit heuristic (e.g., zero brace depth + upcoming Frame keyword) and add regression tests covering nested braces/strings so we catch scanner drift early. -->
<!-- Response: Added boundary detection strategy using keywords with balanced nesting support -->
```

### Scanner Flow Example

**Input File:**
```frame
@target typescript

import * as net from 'net'

system RuntimeProtocol {
    actions:
        async connect() {
            this.socket = net.connect(port, host)
        }
}
```

**Scanning Process:**
1. **Target Discovery**: Scans `@target typescript`, sets target language
2. **Target-Specific**: Captures `import * as net from 'net'` as raw content
3. **Frame Common**: Parses `system RuntimeProtocol {`
4. **Frame Common**: Parses `actions:`
5. **Target-Specific**: Captures action body as raw TypeScript content
6. **Frame Common**: Parses closing `}`

### Integration with Existing Scanner

The current Frame scanner in `scanner.rs` can be extended incrementally:

```rust
// Add to existing Scanner struct
impl Scanner {
    // New method - called before existing scan() method
    pub fn scan_with_target_support(&mut self) -> Vec<Token> {
        // Phase 1: Discover target language
        if let Some(target) = self.scan_target_declaration()? {
            self.target_language = Some(target);
        }
        
        // Phase 2: Scan with target awareness
        self.scanning_mode = ScanningMode::FrameCommon;
        self.scan_all_tokens()
    }
}
```

**Benefits of this approach:**
- **Minimal changes** to existing scanner
- **Backward compatibility** - files without @target work unchanged
- **Incremental implementation** - can add target-specific features gradually

## 🔧 Implementation Strategy

### Runtime & FSL Direction

### Native Declaration Syntax (Draft)
- See `docs/framelang_design/decl_syntax.md` for the proposal covering ambient module declarations, opaque handle types, and runtime implementation guidance.
- This integrates with the runtime/FSL approach above; Python, TypeScript, LLVM, C/C++, Rust, and Java runtimes implement the declared module contract rather than embedding target-specific logic in specs.
- Treat the existing per-target runtimes (`frame_runtime_py`, `frame_runtime_ts`, `runtime/llvm`) as the canonical home for Frame semantics (kernel loop, state stack, forwarded events). No behavioural change required—just keep them lean and target-native.
- Define a target-neutral Frame Standard Library surface (FSL) and provide per-target implementations instead of mirroring Python helper modules. Pragmas/native blocks should call into these FSL shims rather than re-implementing platform APIs inline.
- Document capability matrices per target (async, try/catch, state stack, etc.) so visitors can reject unsupported constructs during code generation.

### Phase 1: Target Declaration Parsing (Week 1)
1. Extend scanner to recognize target declaration tokens (`@`, `target`)
2. Add target discovery pass to parser
3. Store target language in compilation context

### Phase 2: Target-Specific Token Storage (Week 2)
1. Identify action bodies and import statements as target-specific regions
2. Store raw tokens instead of parsing
3. Maintain Frame structure parsing for systems/states/transitions

### Phase 3: Native Syntax Integration (Week 3-4)
1. Implement target-specific parsers for action bodies
2. Create native AST node types for each target
3. Integrate with existing visitor architecture

### Phase 4: Bug #055 Resolution (Week 5)
1. Create TypeScript-specific version of `runtime_protocol.frm`
2. Use native Node.js async socket operations
3. Validate compilation and execution

## 📊 Benefits Analysis

### Immediate Benefits
1. **Solves Bug #055** - Native TypeScript async socket code
2. **Reduces transpiler complexity** - Less semantic translation needed
3. **Improves performance** - Native idioms, no runtime overhead
4. **Better debugging** - Generated code looks hand-written

### Long-term Benefits
1. **Scalability** - Each target uses optimal patterns
2. **Ecosystem access** - Full target language libraries available
3. **Developer familiarity** - Language experts write in their language
4. **Maintenance** - No N×M runtime coordination needed

### Trade-offs
1. **Code duplication** - Same logic written multiple ways
2. **Testing complexity** - Must validate all target variants  
3. **Frame fragmentation** - Loses universal syntax appeal
4. **Learning curve** - Developers need target language knowledge

**Note**: For LLVM and other performance-critical targets, we already re-implement significant logic for optimization. The native backend approach formalizes this pattern and provides better performance than runtime abstraction layers. The benefits of native optimization outweigh duplication concerns for systems programming use cases.

<!-- Comment (AI): For LLVM specifically, we already re-implement lots of logic for performance; maybe highlight that the native backend benefits outweigh duplication concerns. -->
<!-- Recommendation: note in the plan that LLVM pragmas should target performance-critical paths and reuse existing runtime abstractions when possible, keeping duplication bounded. -->
<!-- Response: Added note that LLVM already does target-specific optimization, this formalizes the pattern -->

## 🎯 Design Principles

### 1. Frame Structure Preservation
State machines, transitions, and system architecture remain universal Frame constructs.

### 2. Target-Specific Implementation
Action bodies, imports, and low-level operations use native target syntax.

### 3. Incremental Adoption
Start with action bodies only, expand to other constructs as needed.

### 4. Validation at Parse Time
Catch target syntax errors during compilation, not runtime.

### 5. First-Class Diagnostics
Error reporting for target-specific code must provide both Frame source locations and native language line numbers from day one, not retrofitted later.

### 6. Robust Boundary Detection
Scanner transitions between Frame and target syntax must handle nested constructs (braces, strings, comments) reliably with comprehensive regression testing.

### 7. Clear Toolchain Ownership
Each target must define which tools own compilation artifacts (e.g., `rustc` for LLVM Rust helpers, `tsc` for TypeScript, `python` for Python) to maintain predictable build processes.

### 8. Performance-Justified Duplication
Target-specific implementations should target performance-critical paths and reuse existing runtime abstractions when possible. Document performance justification for each target block.

### 9. Bounded Target Usage
Establish and enforce limits on target-specific code percentage per system to prevent excessive fragmentation while allowing necessary optimization.

## 🔄 Migration Strategy

### Existing Frame Code
Current Frame code continues to work unchanged. Pragma is optional.

### New Target-Specific Code
```frame
#[target: typescript]
// TypeScript-specific implementation

#[target: python]  
// Python-specific implementation
```

### Shared Logic
Frame state machine logic remains universal and shared.

## 🚀 Success Metrics

### Technical Metrics
- **Bug #055**: TypeScript async socket operations compile and execute correctly
- **Performance**: Generated code performance matches hand-written target code
- **Maintainability**: Reduced visitor update frequency for new features

### Quality Metrics
- **Developer Experience**: Faster development using native language features
- **Code Quality**: Generated code is idiomatic and debuggable
- **Test Coverage**: All target variants validated in CI

## 🔮 Future Extensions

### Additional Pragma Types
```frame
#[target: rust]
#[target: go]
#[target: java]
#[target: csharp]
```

### Advanced Syntax Regions
```frame
#[imports: typescript]
import * as fs from 'fs'

#[imports: python]
import os
import sys

system FileProcessor {
    // Shared Frame logic
}
```

### Conditional Compilation
```frame
system NetworkHandler {
    #[target: typescript]
    actions:
        async connect() {
            // Node.js implementation
        }
    
    #[target: python]
    actions:
        async connect(self):
            # Python implementation
}
```

## 📋 Best Practices for Target-Specific Syntax

### 1. **Minimize Target-Specific Code**
- Keep shared business logic in Frame syntax (states, transitions, domain variables)
- Use target-specific syntax only for platform integration (I/O, networking, file systems)

### 2. **Isolate Target Blocks**  
- Prefer small, focused target-specific action bodies
- Avoid target-specific entire systems or large state machines

### 3. **Maintain Semantic Equivalence**
- Target variants should produce identical behavior from Frame's perspective
- Document any unavoidable semantic differences

### 4. **Version Control Strategy**
- Keep target variants in same repository for consistency
- Use branching/tagging to coordinate target-specific changes

### Example of Good Practice:
```frame
@target typescript

system FileProcessor {
    // Shared Frame logic
    machine:
        $Idle {
            processFile(filename) {
                -> $Reading(filename)
            }
        }
        
        $Reading {
            $>() {
                // Target-specific I/O only
                this.readFileAsync()
                -> $Processing
            }
        }
    
    actions:
        // Small, focused target-specific implementation
        readFileAsync() {
            this.content = fs.readFileSync(this.filename, 'utf8')
        }
}
```

<!-- Comment (AI): Maybe add best practices (e.g., keep shared logic in Frame, isolate pragmas) so teams don't fork entire systems per target by accident. -->
<!-- Recommendation: include a "Target Pragmas Best Practices" section (Frame shared logic first, target blocks limited to integration edges) and pair it with linting/rule checks once tooling lands. -->
<!-- Response: Added comprehensive best practices section to prevent system fragmentation -->
```

## 📚 Related Work

### Similar Approaches in Other Languages
- **Rust**: `#[cfg(target_os = "windows")]` conditional compilation
- **C/C++**: `#ifdef` preprocessor directives  
- **Kotlin Multiplatform**: `expect/actual` declarations
- **Xamarin**: Platform-specific implementations

### Frame-Specific Precedents
- Current Python pass-through syntax approach
- Existing visitor-specific code generation patterns
- Target language runtime library architecture

## 🤔 Open Questions

1. **Scope**: Should pragmas apply to entire files or specific code blocks?
2. **Validation**: How strict should target syntax validation be?
3. **IDE Support**: How to provide syntax highlighting and completion?
4. **Testing**: How to ensure equivalent behavior across target variants?
5. **Documentation**: How to maintain examples for multiple targets?

## 📋 Next Steps

1. **Design Review**: Validate this approach with Frame maintainers
2. **Prototype**: Implement basic pragma parsing for Bug #055
3. **Evaluation**: Test with TypeScript async socket operations
4. **Expansion**: Plan broader target-specific syntax support
5. **Documentation**: Update Frame language specification

## 🎯 Conclusion

The pragma-based target-specific syntax approach represents a fundamental shift from "universal syntax" to "universal state machine patterns with target-specific implementation." This aligns with Frame's current reality where different targets require different approaches while preserving Frame's core value proposition of structured state machine development.

For Bug #055 specifically, this approach provides an immediate solution without the complexity and maintenance burden of expanding runtime helper libraries across all targets.

---

## 🔍 Frame Semantics Analysis

### Core Frame Universal Semantics

Based on analysis of the Python grammar specification, these constructs form the **universal Frame language** that should be identical across all target languages:

#### **System Architecture (Universal)**
```bnf
system ::= "system" identifier system_params? "{" system_blocks "}"
system_blocks ::= interface_block? machine_block? actions_block? operations_block? domain_block?
interface_block ::= "interface:" interface_method*
machine_block ::= "machine:" state*
actions_block ::= "actions:" action*
operations_block ::= "operations:" operation*
domain_block ::= "domain:" domain_var*
```

#### **State Machine Semantics (Universal)**
```bnf
state ::= "$" identifier state_params? "{" event_handler* state_var* "}"
event_handler ::= identifier "(" parameter_list? ")" "{" statements* terminator? "}"
transition ::= "->" "$" identifier ("(" expr_list ")")?
parent_dispatch ::= "=>" "$^"
```

#### **Frame Control Flow (Universal)**
```bnf
frame_statement ::= assignment | transition | parent_dispatch | domain_access | action_call
assignment ::= identifier "=" expr
domain_access ::= "self." identifier
action_call ::= identifier "(" expr_list? ")"
```

### Python-Specific Semantics

These constructs are **Python-specific** and would need target-language alternatives:

#### **Python Import System**
```bnf
python_import ::= "import" dotted_name ("as" identifier)?
                | "from" dotted_name "import" (identifier | "*")
dotted_name ::= identifier ("." identifier)*
```

#### **Python Expression Syntax**
```bnf
python_expressions ::= list_comprehension | dict_comprehension | set_comprehension
                     | slice_notation | string_formatting | unpacking_operator
list_comprehension ::= "[" expr "for" identifier "in" expr ("if" expr)? "]"
slice_notation ::= expr "[" slice_start? ":" slice_end? (":" slice_step)? "]"
unpacking_operator ::= "*" expr | "**" expr
```

#### **Python Native Function Calls**
```bnf
python_builtins ::= "print" | "len" | "str" | "int" | "max" | "min" | "range"
python_methods ::= string_methods | list_methods | dict_methods
string_methods ::= ".find" | ".split" | ".join" | ".strip" | ".replace"
list_methods ::= ".append" | ".extend" | ".pop" | ".sort" | ".reverse"
```

#### **Python Data Types**
```bnf
python_literals ::= list_literal | dict_literal | set_literal | tuple_literal
list_literal ::= "[" expr_list? "]"
dict_literal ::= "{" (expr ":" expr)* "}"
set_literal ::= "{" expr_list "}"
tuple_literal ::= "(" expr ("," expr)* ")"
```

#### **Python Control Structures**
```bnf
python_control ::= for_loop | while_loop | if_statement | with_statement
for_loop ::= "for" identifier "in" expr ":" statements
if_statement ::= "if" expr ":" statements ("elif" expr ":" statements)* ("else" ":" statements)?
with_statement ::= "with" expr "as" identifier ":" statements
```

### AST Architecture Implications

#### **Universal Frame AST Nodes**
```rust
// These nodes should be identical across all targets
enum FrameUniversalNode {
    System(SystemNode),
    State(StateNode),
    EventHandler(EventHandlerNode),
    Transition(TransitionNode),
    ActionCall(ActionCallNode),
    DomainAccess(DomainAccessNode),
    InterfaceMethod(InterfaceMethodNode),
}

struct SystemNode {
    name: String,
    interface: Option<InterfaceBlock>,
    machine: Option<MachineBlock>,
    actions: Option<ActionsBlock>,
    operations: Option<OperationsBlock>,
    domain: Option<DomainBlock>,
}

struct StateNode {
    name: String,
    parameters: Option<ParameterList>,
    event_handlers: Vec<EventHandlerNode>,
    state_variables: Vec<StateVarNode>,
}
```

#### **Target-Specific AST Nodes**
```rust
// These nodes vary by target language
enum TargetSpecificNode {
    Python(PythonNode),
    TypeScript(TypeScriptNode),
    Llvm(LlvmNode),
}

enum PythonNode {
    ImportStatement(PythonImportNode),
    ListComprehension(ListCompNode),
    StringMethod(StringMethodNode),
    WithStatement(WithStmtNode),
    ForLoop(ForLoopNode),
    DictLiteral(DictLiteralNode),
}

enum TypeScriptNode {
    ImportStatement(TsImportNode),        // import * as name from 'module'
    ArrayMethod(ArrayMethodNode),         // .push(), .slice()
    ObjectLiteral(ObjectLiteralNode),     // { key: value }
    AsyncAwait(AsyncAwaitNode),           // Promise-based async
    TypeAnnotation(TypeAnnotationNode),   // : Type syntax
}
```

### Parsing Architecture Implications

#### **Two-Phase Parsing Strategy**
```rust
// Phase 1: Parse Frame Universal Constructs
struct FrameParser {
    fn parse_system(&mut self) -> SystemNode;
    fn parse_state(&mut self) -> StateNode;
    fn parse_transition(&mut self) -> TransitionNode;
    fn parse_action_signature(&mut self) -> ActionSignature;
}

// Phase 2: Parse Target-Specific Content
trait TargetParser {
    fn parse_import_block(&mut self, tokens: &[Token]) -> ImportNode;
    fn parse_action_body(&mut self, tokens: &[Token]) -> Vec<StatementNode>;
    fn parse_expression(&mut self, tokens: &[Token]) -> ExpressionNode;
}

struct PythonParser;
impl TargetParser for PythonParser {
    fn parse_action_body(&mut self, tokens: &[Token]) -> Vec<StatementNode> {
        // Parse Python-specific syntax: list comprehensions, for loops, etc.
    }
}

struct TypeScriptParser;
impl TargetParser for TypeScriptParser {
    fn parse_action_body(&mut self, tokens: &[Token]) -> Vec<StatementNode> {
        // Parse TypeScript-specific syntax: arrow functions, type annotations, etc.
    }
}
```

### Scanner Architecture Implications

#### **Mode-Based Scanning**
```rust
enum ScanningContext {
    FrameUniversal,    // System, state, transition syntax
    TargetSpecific {   // Import statements, action bodies
        target: TargetLanguage,
        nesting_depth: usize,
    },
}

impl Scanner {
    fn scan_in_context(&mut self, context: ScanningContext) -> Vec<Token> {
        match context {
            ScanningContext::FrameUniversal => {
                // Tokenize Frame keywords: system, machine, actions, ->, =>, $State
            }
            ScanningContext::TargetSpecific { target, .. } => {
                // Capture raw tokens until Frame boundary detected
                self.capture_until_frame_boundary()
            }
        }
    }
}
```

### Key Insights for Implementation

1. **Clear Separation**: ~70% of Frame syntax is universal (system architecture), ~30% is target-specific (expressions, imports, control flow)

2. **Action Bodies are the Primary Target-Specific Region**: Most Python-specific syntax appears in action implementations, not system structure

3. **Import Statements Need Target Translation**: 
   - Python: `import math` / `from collections import defaultdict`
   - TypeScript: `import * as math from 'math'` / `import { defaultdict } from 'collections'`

4. **Expression Complexity Varies by Target**:
   - Python: List comprehensions, slice notation, unpacking
   - TypeScript: Type annotations, object destructuring, Promise chains

5. **Control Flow Syntax Differences**:
   - Python: `for x in list:` / `if condition:`
   - TypeScript: `for (const x of list)` / `if (condition)`

This analysis confirms that the **@target approach is viable** - Frame's core state machine semantics remain universal while allowing target-specific implementations for imports, expressions, and control flow within action bodies.

## 📚 Related Documents

- **Implementation Plan**: [Cross-Language Support Plan](../plans/cross_language_support_plan.md) - Structured implementation plan based on this analysis
- **Bug Report**: [Bug #055](../bugs/open/bug_055_async_typescript_socket_runtime.md) - Original issue driving this analysis
- **Frame Runtime**: [Frame Runtime Specification](frame_runtime.md) - Abstract runtime requirements
- **Python Grammar**: [Python Grammar Specification](target_language_specifications/python/python_grammar.md) - Source for this analysis

---

**Document Status**: Draft for review  
**Next Review**: TBD  
**Implementation Target**: TBD
