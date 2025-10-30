# Adding New Target Languages to Frame Transpiler

## Overview

This comprehensive guide provides a step-by-step process for adding new target languages to the Frame transpiler, based on lessons learned from achieving 100% TypeScript transpilation success rate. The Frame transpiler uses a visitor pattern to convert Frame AST to target language code.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Architecture Overview](#architecture-overview)
3. [Implementation Phases](#implementation-phases)
4. [Critical Components](#critical-components)
5. [Testing Strategy](#testing-strategy)
6. [Common Pitfalls](#common-pitfalls)
7. [Infrastructure Considerations](#infrastructure-considerations)
8. [Maintenance Guidelines](#maintenance-guidelines)

## Prerequisites

### Understanding Frame Language
- **Frame syntax**: Study `docs/framelang_design/grammar.md`
- **Frame semantics**: Review Python visitor as canonical reference
- **Test suite**: Familiarize with 422 test cases in `framec_tests/`

### Required Knowledge
- **Rust programming**: Visitor pattern, AST manipulation, error handling
- **Target language**: Deep understanding of syntax, idioms, and runtime features
- **State machines**: Frame's non-recursive transition semantics
- **Testing**: Unit testing, integration testing, test automation

### Tools and Environment
- **Rust toolchain**: Latest stable Rust compiler
- **Target language toolchain**: Compiler/interpreter for validation
- **Test runner**: Python-based test framework in `framec_tests/runner/`

## Architecture Overview

### Core Components

```
Frame Source (.frm)
       ↓
   Scanner/Parser
       ↓
    AST Nodes
       ↓
  Language Visitor  ←── Target-specific implementation
       ↓
  Generated Code
       ↓
  Target Compiler/Runtime
```

### Visitor Pattern Structure

The Frame transpiler uses the visitor pattern where each target language implements:

- **`TargetLanguageVisitor`**: Main visitor struct
- **AST Node Methods**: `visit_*_node()` for each AST node type
- **Code Generation**: Builder pattern for output construction
- **Runtime Support**: Target language runtime classes/functions

## Implementation Phases

### Phase 1: Foundation Setup (Week 1-2)

#### 1.1 Create Visitor Module

Create `framec/src/frame_c/visitors/[language]_visitor.rs`:

```rust
use crate::frame_c::ast::*;
use crate::frame_c::code_builder::CodeBuilder;

pub struct LanguageVisitor {
    builder: CodeBuilder,
    // Language-specific state
}

impl LanguageVisitor {
    pub fn new() -> Self {
        Self {
            builder: CodeBuilder::new(),
        }
    }
    
    pub fn run(&mut self, ast: &FrameModule) -> String {
        self.visit_frame_module_node(ast);
        self.builder.build()
    }
}
```

#### 1.2 Add Target Language Enum

In `framec/src/frame_c/visitors/mod.rs`:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum TargetLanguage {
    Python3,
    TypeScript,
    YourLanguage, // Add here
    // ...
}
```

#### 1.3 Update CLI Interface

In `framec/src/main.rs`, add command-line option:

```rust
match matches.value_of("language") {
    // ... existing cases
    Some("yourlang") => TargetLanguage::YourLanguage,
    // ...
}
```

### Phase 2: Core AST Handling (Week 2-4)

#### 2.1 System and Interface Generation

Start with the most fundamental Frame concepts:

```rust
impl LanguageVisitor {
    fn visit_frame_module_node(&mut self, node: &FrameModule) {
        // Generate imports/headers
        self.generate_imports();
        
        // Generate each system
        for system in &node.systems {
            self.visit_system_node(system);
        }
        
        // Generate runtime support
        self.generate_runtime();
    }
    
    fn visit_system_node(&mut self, node: &SystemNode) {
        // Class/module declaration
        self.builder.writeln(&format!("class {} {{", node.name));
        self.builder.indent();
        
        // Domain variables as class members
        self.generate_domain_variables(node);
        
        // Constructor
        self.generate_constructor(node);
        
        // Interface methods
        self.generate_interface_methods(node);
        
        // State handlers
        self.generate_state_handlers(node);
        
        // Frame runtime methods
        self.generate_frame_runtime();
        
        self.builder.dedent();
        self.builder.writeln("}");
    }
}
```

#### 2.2 Expression System

**Critical**: Implement comprehensive expression handling:

```rust
fn visit_expr_node_to_string(&mut self, node: &ExprNode, output: &mut String) {
    match &node.expr_type {
        ExprType::LiteralExprT { literal_expr_node } => {
            // Handle strings, numbers, booleans
            self.visit_literal_expr_node_to_string(literal_expr_node, output);
        },
        ExprType::VariableExprT { var_node } => {
            // Handle variable references (self.x, local vars)
            self.visit_variable_expr_node_to_string(var_node, output);
        },
        ExprType::CallExprT { call_expr_node } => {
            // Handle function/method calls
            self.visit_call_expr_node_to_string(call_expr_node, output);
        },
        ExprType::BinaryExprT { binary_expr_node } => {
            // Handle operators (+, -, ==, etc.)
            self.visit_binary_expr_node_to_string(binary_expr_node, output);
        },
        ExprType::AwaitExprT { await_expr_node } => {
            // Handle async/await expressions
            output.push_str("await ");
            self.visit_expr_node_to_string(&await_expr_node.expr, output);
        },
        // ... Handle ALL expression types
        _ => {
            panic!("Unhandled expression type: {:?}", node.expr_type);
        }
    }
}
```

**Key Expression Types to Handle:**
- `LiteralExprT`: Strings, numbers, booleans, null
- `VariableExprT`: Variable references (`self.x`, `local_var`)
- `CallExprT`: Function/method calls
- `BinaryExprT`: Operators (`+`, `-`, `==`, `&&`, etc.)
- `UnaryExprT`: Unary operators (`!`, `-`, `+`)
- `AssignmentExprT`: Variable assignments
- `AwaitExprT`: Async expressions (if supported)
- `SystemInstanceExprT`: System instantiation
- `FrameEventPartT`: Event parameter access

### Phase 3: State Machine Runtime (Week 4-6)

#### 3.1 Frame Runtime Implementation

**Critical**: Frame uses a non-recursive kernel for state transitions:

```rust
fn generate_frame_runtime(&mut self) {
    // Kernel method - handles transitions iteratively
    self.builder.writeln("private _frame_kernel(__e: FrameEvent): void {");
    self.builder.indent();
    self.builder.writeln("this._frame_router(__e);");
    self.builder.writeln("while (this._nextCompartment !== null) {");
    self.builder.indent();
    self.builder.writeln("const nextCompartment = this._nextCompartment;");
    self.builder.writeln("this._nextCompartment = null;");
    self.builder.writeln("this._frame_router(new FrameEvent(\"<$\", this._compartment.exitArgs));");
    self.builder.writeln("this._compartment = nextCompartment;");
    self.builder.writeln("if (nextCompartment.forwardEvent === null) {");
    self.builder.indent();
    self.builder.writeln("this._frame_router(new FrameEvent(\"$>\", this._compartment.enterArgs));");
    self.builder.dedent();
    self.builder.writeln("} else {");
    self.builder.indent();
    self.builder.writeln("this._frame_router(nextCompartment.forwardEvent);");
    self.builder.writeln("nextCompartment.forwardEvent = null;");
    self.builder.dedent();
    self.builder.writeln("}");
    self.builder.dedent();
    self.builder.writeln("}");
    self.builder.dedent();
    self.builder.writeln("}");
}
```

#### 3.2 Interface Method Generation

Interface methods create events and use return stack:

```rust
fn generate_interface_method(&mut self, method: &InterfaceMethodNode) {
    let params = self.generate_parameter_list(&method.params);
    let return_type = self.map_return_type(&method.return_type);
    
    self.builder.writeln(&format!("public {}({}): {} {{", method.name, params, return_type));
    self.builder.indent();
    
    // Push to return stack
    self.builder.writeln("this.returnStack.push(null);");
    
    // Create event
    let event_params = self.generate_event_parameters(&method.params);
    self.builder.writeln(&format!(
        "const __e = new FrameEvent(\"{}\", {});", 
        method.name, 
        event_params
    ));
    
    // Call kernel
    self.builder.writeln("this._frame_kernel(__e);");
    
    // Return from stack
    self.builder.writeln("return this.returnStack.pop();");
    
    self.builder.dedent();
    self.builder.writeln("}");
}
```

### Phase 4: Advanced Features (Week 6-8)

#### 4.1 Async/Await Support

If your target language supports async:

```rust
fn generate_event_handler(&mut self, handler: &EventHandlerNode) {
    let handler_name = format!("_handle_{}_{}", state_name, handler.msg);
    
    // Check if handler is async
    if handler.is_async {
        self.builder.writeln(&format!(
            "private async {}(__e: FrameEvent, compartment: FrameCompartment): Promise<void> {{",
            handler_name
        ));
    } else {
        self.builder.writeln(&format!(
            "private {}(__e: FrameEvent, compartment: FrameCompartment): void {{",
            handler_name
        ));
    }
    
    self.builder.indent();
    self.visit_statements_node(&handler.statements);
    self.builder.dedent();
    self.builder.writeln("}");
}
```

#### 4.2 Type System Integration

Map Frame types to target language types:

```rust
fn map_frame_type(&self, frame_type: &str) -> String {
    match frame_type {
        "bool" => "boolean".to_string(),
        "int" => "number".to_string(),
        "float" => "number".to_string(),
        "string" => "string".to_string(),
        _ => "any".to_string(), // Default for unknown types
    }
}
```

### Phase 5: Multifile Support (Week 8-10)

#### 5.1 Module Linker Integration

Update `framec/src/frame_c/modules/linker.rs`:

```rust
fn link_concatenation(&mut self) -> ModuleResult<String> {
    // ... existing code
    
    let module_code = match self.target_language {
        TargetLanguage::Python3 => {
            // ... Python visitor
        },
        TargetLanguage::TypeScript => {
            // ... TypeScript visitor
        },
        TargetLanguage::YourLanguage => {
            use crate::frame_c::visitors::yourlang_visitor::YourLangVisitor;
            let mut visitor = YourLangVisitor::new();
            visitor.run(&module.ast)
        },
        _ => {
            return Err(ModuleError::new(
                ModuleErrorKind::InvalidPath {
                    path: "target_language".to_string(),
                    reason: format!("Multi-file compilation not supported for target language: {:?}", self.target_language),
                },
                String::new(),
            ));
        }
    };
    
    // ... rest of linking logic
}
```

#### 5.2 Language-Aware Infrastructure

Ensure all infrastructure components are language-aware:

```rust
// Comments
match self.target_language {
    TargetLanguage::Python3 => output.push_str("# Comment"),
    TargetLanguage::YourLanguage => output.push_str("// Comment"), // or /* Comment */
    // ...
}

// Import extraction
fn extract_imports(&self, code: &str, imports: &mut HashSet<String>) {
    for line in code.lines() {
        let trimmed = line.trim();
        match self.target_language {
            TargetLanguage::YourLanguage => {
                if trimmed.starts_with("import ") || trimmed.starts_with("using ") {
                    imports.insert(trimmed.to_string());
                }
            },
            // ... other languages
        }
    }
}
```

## Critical Components

### 1. Code Builder Usage

Always use the CodeBuilder for consistent formatting:

```rust
// ✅ CORRECT
self.builder.writeln("function example() {");
self.builder.indent();
self.builder.writeln("// code here");
self.builder.dedent();
self.builder.writeln("}");

// ❌ WRONG - bypasses formatting
output.push_str("function example() {\n    // code here\n}");
```

### 2. Error Handling

Handle all AST node types:

```rust
fn visit_statement_node(&mut self, node: &StatementNode) {
    match &node.stmt_type {
        StatementType::ExpressionStmt { expr_stmt_node } => {
            self.visit_expression_stmt_node(expr_stmt_node);
        },
        StatementType::AssignmentStmt { assignment_stmt_node } => {
            self.visit_assignment_stmt_node(assignment_stmt_node);
        },
        // ... handle ALL statement types
        _ => {
            panic!("Unhandled statement type: {:?}", node.stmt_type);
        }
    }
}
```

**Never** use placeholder comments like `/* TODO: StatementType */` in production!

### 3. Symbol Resolution

Properly handle variable scoping:

```rust
fn visit_variable_node(&mut self, node: &VariableNode, output: &mut String) {
    match &node.scope {
        IdentifierDeclScope::DomainVariable => {
            output.push_str(&format!("this.{}", node.id_node.name.lexeme));
        },
        IdentifierDeclScope::LocalVariable => {
            output.push_str(&node.id_node.name.lexeme);
        },
        IdentifierDeclScope::StateVariable => {
            output.push_str(&format!("this._compartment.stateVars.{}", node.id_node.name.lexeme));
        },
        // ... handle all scopes
    }
}
```

## Testing Strategy

### Phase 1: Basic Functionality Tests

Start with the simplest tests:

```bash
# Test basic system generation
./target/release/framec -l yourlang framec_tests/common/tests/systems/test_simple_system.frm

# Test interface methods
./target/release/framec -l yourlang framec_tests/common/tests/core/test_interface_simple.frm
```

### Phase 2: Expression Tests

Test all expression types systematically:

```bash
# Test literals
./target/release/framec -l yourlang framec_tests/common/tests/data_types/test_literals.frm

# Test operators
./target/release/framec -l yourlang framec_tests/common/tests/operators/test_binary_operators.frm

# Test function calls
./target/release/framec -l yourlang framec_tests/common/tests/core/test_function_calls.frm
```

### Phase 3: State Machine Tests

Test Frame's core semantics:

```bash
# Test transitions
./target/release/framec -l yourlang framec_tests/common/tests/core/test_transitions.frm

# Test enter/exit handlers
./target/release/framec -l yourlang framec_tests/common/tests/core/test_enter_exit.frm

# Test async handlers (if supported)
./target/release/framec -l yourlang framec_tests/common/tests/core/test_async_handlers.frm
```

### Phase 4: Full Test Suite

Run the complete test suite:

```bash
cd framec_tests
python3 runner/frame_test_runner.py --languages yourlang --framec ../target/release/framec
```

**Target**: Achieve 95%+ success rate like TypeScript.

### Validation Strategy

For each test:

1. **Transpile**: Generate target language code
2. **Compile**: Verify syntax with target language compiler
3. **Execute**: Run generated code and verify output
4. **Compare**: Match behavior with Python reference implementation

## Common Pitfalls

### 1. Expression Handling Gaps

**Problem**: Missing expression types cause `TODO` placeholders:

```rust
// ❌ CAUSES: let data = /* TODO: AwaitExprT */;
ExprType::AwaitExprT { .. } => {
    // Missing implementation!
}

// ✅ SOLUTION: Complete implementation
ExprType::AwaitExprT { await_expr_node } => {
    output.push_str("await ");
    self.visit_expr_node_to_string(&await_expr_node.expr, output);
}
```

### 2. Runtime Class Mixing

**Problem**: Multifile compilation mixes languages:

```typescript
// ❌ WRONG: Python runtime in TypeScript
class FrameEvent:
    def __init__(self, message, parameters):
```

**Solution**: Language-aware module linker (see Phase 5).

### 3. Async Function Signatures

**Problem**: Async expressions without async functions:

```typescript
// ❌ CAUSES: "await expressions are only allowed within async functions"
private handleEvent(__e: FrameEvent): void {
    let data = await fetchData();  // ERROR!
}

// ✅ SOLUTION: Check is_async flag
private async handleEvent(__e: FrameEvent): Promise<void> {
    let data = await fetchData();  // OK!
}
```

### 4. Boolean Literal Translation

**Problem**: Python booleans in target language:

```javascript
// ❌ WRONG: Python boolean literals
if (True) { return False; }

// ✅ CORRECT: Target language booleans
if (true) { return false; }
```

### 5. Import/Export Confusion

**Problem**: Class exports treated as imports:

```typescript
// ❌ WRONG: Class export in imports section
// Consolidated imports
export class Calculator {

// ✅ CORRECT: Imports only
// Consolidated imports
import { FrameEvent } from './runtime';
```

## Infrastructure Considerations

### Build System Integration

Update `Cargo.toml` dependencies if needed:

```toml
[dependencies]
# Add target language specific dependencies
serde_json = "1.0"  # For JSON handling
regex = "1.0"       # For text processing
```

### CLI Integration

Update help text in `main.rs`:

```rust
.help("Target language: python_3, typescript, yourlang, graphviz")
```

### Documentation

Create target language guide:

```
docs/framelang_design/target_language_translation_guides/frame_to_yourlang.md
```

Follow the TypeScript guide structure for consistency.

### Runtime Support

Many target languages need runtime files:

```
framec_tests/yourlang/runtime/
├── frame_runtime.yourlang    # FrameEvent, FrameCompartment classes
├── README.md                 # Setup instructions
└── package.yourlang          # Package definition if needed
```

## Maintenance Guidelines

### Version Compatibility

Ensure version synchronization:

```rust
// In visitor constructor
pub fn new() -> Self {
    Self {
        builder: CodeBuilder::new(),
        framec_version: "Emitted from framec_v0.83.3",  // Keep in sync!
    }
}
```

### Test Suite Maintenance

Add new test categories as needed:

```
framec_tests/common/tests/yourlang_specific/
├── test_yourlang_idioms.frm
├── test_yourlang_types.frm
└── test_yourlang_features.frm
```

### Error Messages

Provide helpful error messages:

```rust
fn visit_unsupported_node(&mut self, node_type: &str) {
    panic!(
        "YourLanguage visitor: {} not yet implemented. \
        This is a known limitation. \
        See docs/framelang_design/target_language_translation_guides/frame_to_yourlang.md",
        node_type
    );
}
```

### Performance Considerations

Target language compilation speed matters:

```rust
// Use efficient string building
let mut output = String::with_capacity(1024);  // Pre-allocate

// Avoid excessive string copying
self.builder.write(&format!("code"));  // vs push_str when possible
```

## Implementation Checklist

### Foundation ✅
- [ ] Create visitor module
- [ ] Add target language enum
- [ ] Update CLI interface
- [ ] Basic code generation structure

### Core Features ✅
- [ ] System/class generation
- [ ] Domain variables
- [ ] Interface methods
- [ ] Constructor generation
- [ ] Return stack handling

### Expression System ✅
- [ ] Literal expressions (strings, numbers, booleans)
- [ ] Variable expressions (self.x, local_var)
- [ ] Binary expressions (operators)
- [ ] Unary expressions
- [ ] Call expressions
- [ ] Assignment expressions
- [ ] Await expressions (if applicable)
- [ ] All other expression types

### State Machine Runtime ✅
- [ ] Non-recursive kernel implementation
- [ ] State router/dispatcher
- [ ] Event handler generation
- [ ] Transition mechanism
- [ ] Enter/exit event handling
- [ ] Event forwarding support

### Language-Specific Features ✅
- [ ] Type system mapping
- [ ] Async/await support (if applicable)
- [ ] Error handling patterns
- [ ] Import/export system
- [ ] Comment style
- [ ] Naming conventions

### Multifile Support ✅
- [ ] Module linker integration
- [ ] Import consolidation
- [ ] Language-aware infrastructure
- [ ] Cross-module symbol resolution

### Testing ✅
- [ ] Basic functionality tests (>50% pass rate)
- [ ] Expression tests (>80% pass rate)
- [ ] State machine tests (>90% pass rate)
- [ ] Full test suite (>95% pass rate)
- [ ] Manual validation of generated code

### Documentation ✅
- [ ] Target language translation guide
- [ ] Runtime setup instructions
- [ ] Example implementations
- [ ] Troubleshooting guide

### Production Ready ✅
- [ ] Error handling for all AST nodes
- [ ] No TODO placeholders in generated code
- [ ] Version synchronization
- [ ] Performance optimization
- [ ] Code quality review

## Success Metrics

Based on TypeScript achievements:

- **Transpilation**: 100% (422/422 tests generate code)
- **Compilation**: >95% (generated code compiles successfully)
- **Execution**: >95% (compiled code runs correctly)
- **Categories**: All test categories >90% success rate

## Conclusion

Adding a new target language to Frame requires systematic implementation across multiple phases. The key to success is:

1. **Comprehensive expression handling** - Handle ALL AST node types
2. **Language-aware infrastructure** - Don't mix language syntax
3. **Proper state machine semantics** - Use non-recursive kernel
4. **Thorough testing** - Test every component incrementally
5. **Documentation** - Provide clear guidance for users

Following this guide and learning from the TypeScript implementation experience should enable you to achieve similar 100% success rates for new target languages.

The Frame transpiler architecture is well-designed for extensibility - most of the hard work is in the visitor implementation and ensuring proper integration with the existing infrastructure.

## Related Documentation

- `frame_to_typescript.md` - TypeScript implementation reference
- `docs/framelang_design/grammar.md` - Frame language specification  
- `docs/framepiler_design/architecture.md` - Transpiler architecture
- `framec_tests/runner/frame_test_runner.py` - Test framework usage