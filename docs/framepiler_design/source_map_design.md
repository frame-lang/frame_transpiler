# Frame Source Map Design Document

**Version**: 1.0  
**Date**: 2025-01-28  
**Status**: Active Specification  
**Current Implementation**: v0.78.3 (Partial - ~11% coverage)  
**Target Implementation**: v0.79.0 (Complete - >90% coverage)

## Executive Summary

This document defines the complete source mapping strategy for the Frame transpiler, specifying exactly how Frame source lines map to generated Python code to enable debugging in IDEs like VS Code. It addresses the fundamental challenge of mapping a state machine DSL to imperative Python code where significant runtime infrastructure has no direct Frame equivalent.

## Table of Contents

1. [Overview](#overview)
2. [Core Principles](#core-principles)
3. [Mapping Categories](#mapping-categories)
4. [Detailed Mapping Rules](#detailed-mapping-rules)
5. [Implementation Architecture](#implementation-architecture)
6. [Special Cases](#special-cases)
7. [Testing Strategy](#testing-strategy)
8. [Performance Considerations](#performance-considerations)

## Overview

### Purpose of Source Maps

Source maps enable developers to debug Frame code in their natural Frame syntax while the actual execution happens in generated Python. Without accurate source maps:
- Breakpoints cannot be set on Frame lines
- Step debugging shows wrong lines
- Variable inspection happens out of context
- The debugging experience becomes unusable

### Current State (v0.78.3)

- **Coverage**: ~11% of Python lines have mappings
- **What Works**: Function definitions, some statements
- **What's Missing**: System initialization, state dispatchers, most statement bodies, runtime infrastructure

### Target State (v0.79.0)

- **Coverage**: >90% of user-relevant Python lines mapped
- **What Will Work**: All Frame source lines with executable semantics
- **What Won't Be Mapped**: Pure runtime infrastructure with no Frame equivalent

## Core Principles

### 1. User Code Priority
Every line of Frame code written by the user that has executable semantics MUST have a source mapping.

### 2. Meaningful Mappings Only
Runtime infrastructure that has no Frame equivalent should NOT be mapped to avoid confusion.

### 3. One-to-Many Relationships
One Frame line may map to multiple Python lines (e.g., complex statements), but each Python line maps to at most one Frame line.

### 4. Predictable Debugging
When debugging, the flow through Frame source should be predictable and match user expectations.

### 5. No Synthetic Lines
Avoid mapping to "line 0" or other synthetic line numbers that don't exist in the Frame source.

## Mapping Categories

### Category 1: Direct User Code (ALWAYS MAPPED)

Frame code directly written by users maps to its Python equivalent:

```frame
print("Hello")      # Line 5
```
→
```python
print("Hello")      # Maps to Frame line 5
```

### Category 2: Declarative Constructs (MAPPED TO IMPLEMENTATION)

Frame declarations map to their Python implementation:

```frame
system MySystem {   # Line 1
    ...
}
```
→
```python
class MySystem:     # Maps to Frame line 1
    def __init__(self):  # Also maps to Frame line 1
```

### Category 3: Structural Keywords (NEVER MAPPED)

Pure structural elements have no executable semantics:

```frame
machine:           # No mapping
actions:           # No mapping
operations:        # No mapping
domain:            # No mapping
```

### Category 4: Runtime Infrastructure (NEVER MAPPED)

Generated code with no Frame equivalent:

```python
def __kernel(self, __e):        # No mapping
def __router(self, __e):        # No mapping
def __transition(self, next):   # No mapping
class FrameEvent:                # No mapping
class FrameCompartment:          # No mapping
```

## Detailed Mapping Rules

### System Declaration

```frame
system Calculator {    # Line 10
    ...
}
```

**Generates:**
```python
class Calculator:              # Maps to line 10
    def __init__(self):        # Maps to line 10
        # Initialization code   # Maps to line 10
        self.__compartment = ...  # Maps to line 10
```

**Rationale**: The system declaration is the logical "entry point" for the system's initialization.

### Interface Methods

```frame
interface:
    calculate(x, y): int    # Line 15
```

**Generates:**
```python
def calculate(self, x, y):    # Maps to line 15
    self.return_stack.append(None)
    # ... dispatch to state handler
```

### State Declarations

```frame
$Ready {                    # Line 20
    start() {               # Line 21
        print("Starting")   # Line 22
        -> $Running         # Line 23
    }
}
```

**Generates:**
```python
def __testsystem_state_Ready(self, __e, compartment):  # Maps to line 20
    if __e._message == "start":
        return self.__handle_ready_start(__e, compartment)

def __handle_ready_start(self, __e, compartment):      # Maps to line 21
    print("Starting")                                   # Maps to line 22
    next_compartment = FrameCompartment(...)           # Maps to line 23
    self.__transition(next_compartment)                # Maps to line 23
    return
```

### Event Handlers

```frame
$>() {                      # Line 30 (enter handler)
    self.setup()            # Line 31
}
```

**Generates:**
```python
def __handle_ready_enter(self, __e, compartment):  # Maps to line 30
    self.setup()                                    # Maps to line 31
```

### Operations and Actions

```frame
operations:
    calculate(x, y): int {  # Line 40
        return x + y        # Line 41
    }

actions:
    doWork() {              # Line 45
        print("Working")    # Line 46
    }
```

**Generates:**
```python
def calculate(self, x, y):  # Maps to line 40
    return x + y             # Maps to line 41

def doWork(self):           # Maps to line 45
    print("Working")        # Maps to line 46
```

### Domain Variables

```frame
domain:
    var counter = 0         # Line 50
    var name = "Frame"      # Line 51
```

**Generates (in __init__):**
```python
self.counter = 0            # Maps to line 50
self.name = "Frame"         # Maps to line 51
```

### Control Flow

```frame
if x > 0 {                  # Line 60
    print("Positive")       # Line 61
} elif x < 0 {              # Line 62
    print("Negative")       # Line 63
} else {                    # Line 64
    print("Zero")           # Line 65
}
```

**Generates:**
```python
if x > 0:                   # Maps to line 60
    print("Positive")       # Maps to line 61
elif x < 0:                 # Maps to line 62
    print("Negative")       # Maps to line 63
else:                       # Maps to line 64
    print("Zero")           # Maps to line 65
```

### Loops

```frame
for item in items {         # Line 70
    process(item)           # Line 71
}

while condition {           # Line 75
    doWork()               # Line 76
}
```

**Generates:**
```python
for item in items:          # Maps to line 70
    process(item)           # Maps to line 71

while condition:            # Maps to line 75
    doWork()               # Maps to line 76
```

## Implementation Architecture

### CodeBuilder Integration

The `CodeBuilder` struct provides the foundation for source mapping:

```rust
pub struct CodeBuilder {
    output: String,
    current_position: Position,
    mappings: Vec<SourceMapping>,
    pending_mapping: Option<usize>,
    // ...
}

impl CodeBuilder {
    /// Set Frame line for next write
    pub fn map_next(&mut self, frame_line: usize) { }
    
    /// Write with explicit mapping
    pub fn write_mapped(&mut self, s: &str, frame_line: usize) { }
    
    /// Write without mapping (for runtime code)
    pub fn write(&mut self, s: &str) { }
}
```

### Visitor Pattern

Each visitor method must decide whether to map:

```rust
fn visit_call_statement_node(&mut self, node: &CallStmtNode) {
    // User code - MUST map
    self.builder.map_next(node.line);
    let mut output = String::new();
    self.visit_call_expression_node_to_string(&node.call_expr_node, &mut output);
    self.builder.writeln(&output);
}

fn generate_frame_runtime(&mut self) {
    // Runtime code - NO mapping
    self.builder.writeln("class FrameEvent:");  // No map_next() call
    self.builder.indent();
    self.builder.writeln("def __init__(self, message, parameters):");
    // ...
}
```

### AST Requirements

Every user-facing AST node MUST have a `line` field:

```rust
pub struct CallStmtNode {
    pub line: usize,  // Required for mapping
    pub call_expr_node: CallExprNode,
}

pub struct OperationNode {
    pub line: usize,  // Added in v0.78.2
    pub name: String,
    // ...
}
```

## Special Cases

### Multi-line Statements

When a Frame statement spans multiple lines, map to the first line:

```frame
var result = calculate(     # Line 80 - Map here
    x,
    y
)
```

### Generated Parameter Extraction

Event parameters extraction maps to the event handler line:

```frame
start(x, y) {               # Line 90
    // ...
}
```

**Generates:**
```python
def __handle_ready_start(self, __e, compartment):  # Maps to line 90
    x = __e._parameters.get("x")                   # Maps to line 90
    y = __e._parameters.get("y")                   # Maps to line 90
```

### State Transitions

Transition statements generate multiple Python lines, all mapping to the Frame transition:

```frame
-> $NextState               # Line 100
```

**Generates:**
```python
next_compartment = FrameCompartment(...)  # Maps to line 100
self.__transition(next_compartment)       # Maps to line 100
return                                     # Maps to line 100
```

### Empty Blocks

Empty blocks generate `pass` statements that map to the block's opening:

```frame
actions:
    emptyAction() {         # Line 110
    }
```

**Generates:**
```python
def emptyAction(self):      # Maps to line 110
    pass                    # Maps to line 110
```

## Testing Strategy

### Coverage Metrics

Test suite must verify:
1. **Minimum 90% coverage** of user-relevant Python lines
2. **Every Frame executable line** has at least one mapping
3. **No runtime infrastructure** has mappings

### Test Categories

1. **Basic Mapping Tests**
   - Each statement type maps correctly
   - Function/method definitions map to declarations
   
2. **Complex Structure Tests**
   - Nested control flow maintains correct mappings
   - State machines with multiple states
   - Systems with all block types

3. **Edge Case Tests**
   - Empty blocks
   - Multi-line statements
   - Generated code sections

### Validation Script

```python
def validate_source_map(frame_file):
    # Generate debug output
    result = subprocess.run(
        ['framec', '-l', 'python_3', '--debug-output', frame_file],
        capture_output=True, text=True
    )
    data = json.loads(result.stdout)
    
    # Check coverage
    python_lines = data['python'].split('\n')
    mapped_lines = set(m['pythonLine'] for m in data['sourceMap']['mappings'])
    
    # Filter out runtime lines
    relevant_lines = [i for i, line in enumerate(python_lines, 1)
                      if not is_runtime_line(line)]
    
    coverage = len(mapped_lines.intersection(relevant_lines)) / len(relevant_lines)
    assert coverage >= 0.9, f"Coverage {coverage:.1%} below 90% threshold"
```

## Performance Considerations

### Build Time Impact

Source mapping adds minimal overhead:
- **String tracking**: O(n) where n is output size
- **Mapping storage**: O(m) where m is number of Frame lines
- **Typical impact**: <1% of transpilation time

### Runtime Impact

Source maps affect only debug builds:
- **Production**: No impact (maps not included)
- **Debug**: File size increase ~10-20%
- **VS Code**: Parsing happens once at debug start

### Memory Usage

CodeBuilder maintains:
- **Mappings vector**: ~24 bytes per mapping
- **Typical program**: 1000 lines → ~24KB overhead
- **Acceptable for development builds**

## Migration Path

### Phase 1: Core Statements (v0.78.4)
- Add mappings to all statement visitor methods
- Ensure print, assignments, returns are mapped

### Phase 2: System Infrastructure (v0.78.5)
- Map system __init__ to system declaration
- Map state dispatchers to state declarations
- Map interface methods

### Phase 3: Complete Coverage (v0.79.0)
- Map domain variables
- Map all control flow
- Add validation test suite

### Backwards Compatibility

Source maps are optional and backward compatible:
- Old tools ignore mapping data
- `--debug-output` flag controls generation
- No changes to generated Python semantics

## Conclusion

This design provides comprehensive source mapping that enables professional debugging of Frame programs while maintaining clarity about what is user code versus runtime infrastructure. The implementation is straightforward, performant, and provides the foundation for advanced IDE features like step debugging, breakpoints, and variable inspection.

The key insight is that not every Python line needs mapping - only those that correspond to user-written Frame code or its direct implementation. This selective mapping strategy provides the best debugging experience while avoiding confusion from runtime infrastructure appearing in the debugger.