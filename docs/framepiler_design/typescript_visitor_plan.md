# TypeScript Visitor Implementation Plan

**Created**: 2025-01-13  
**Version**: Frame Transpiler v0.81.6  
**Status**: Planning Phase  

## Executive Summary

This document outlines the complete plan for implementing a TypeScript visitor for the Frame transpiler, identifying all technical challenges, design decisions, and implementation strategies needed to support the full Frame language specification in TypeScript.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Core Challenges](#core-challenges)
3. [TypeScript vs Python Differences](#typescript-vs-python-differences)
4. [Frame Language Feature Support](#frame-language-feature-support)
5. [Implementation Phases](#implementation-phases)
6. [Open Issues & Design Decisions](#open-issues--design-decisions)
7. [Code Generation Patterns](#code-generation-patterns)
8. [Testing Strategy](#testing-strategy)

---

## Architecture Overview

### Reusable Components (No Changes Needed)
- Scanner (`scanner.rs`)
- Parser (`parser.rs`)
- AST (`ast.rs`)
- Symbol Table (`symbol_table.rs`)
- Semantic Analysis (all validation)
- CodeBuilder (`code_builder.rs`)

### New Components Required
- `typescript_visitor.rs` - Main visitor implementation
- `typescript_types.rs` - TypeScript type generation utilities
- `typescript_runtime.ts` - Runtime support library (FrameEvent, FrameCompartment)

### File Structure
```
framec/src/frame_c/visitors/
├── typescript_visitor.rs       // NEW - Main visitor
├── typescript_types.rs          // NEW - Type utilities
└── mod.rs                       // Update to include TypeScript

framec_tests/typescript/
├── runtime/
│   └── frame_runtime.ts        // Runtime support library
├── src/                         // Test .frm files
└── generated/                   // Generated .ts files
```

---

## Core Challenges

### 1. Type System Integration

**Challenge**: Frame's type system must map to TypeScript's static type system.

**Python Approach**: Dynamic typing, no declarations needed
```python
def add(x, y):
    return x + y  # Types inferred at runtime
```

**TypeScript Requirements**: 
```typescript
function add(x: number, y: number): number {
    return x + y;  // Types must be explicit
}
```

**Open Issues**:
- [ ] How to handle Frame's `any` type → TypeScript `any` vs `unknown`
- [ ] Optional types in Frame → TypeScript optional parameters
- [ ] Frame arrays `[]` → TypeScript arrays with proper type parameters
- [ ] Frame maps/dicts → TypeScript `Record<K,V>` or `Map<K,V>`
- [ ] Null/undefined handling differences

### 2. Async/Await Pattern

**Challenge**: TypeScript async differs from Python async.

**Python**:
```python
async def interface_method(self):
    await self._frame_kernel(__e)
```

**TypeScript**:
```typescript
public async interfaceMethod(): Promise<void> {
    await this._frame_kernel(__e);
}
```

**Open Issues**:
- [ ] All async methods must return `Promise<T>`
- [ ] Async state machines need different initialization
- [ ] Event loop integration (Node.js vs browser)
- [ ] Error handling in async contexts

### 3. Class Property Declarations

**Challenge**: TypeScript requires explicit property declarations.

**Python** (properties created on first assignment):
```python
def __init__(self):
    self.__compartment = FrameCompartment(...)  # Created here
    self.domain_var = 0  # Created here
```

**TypeScript** (must declare first):
```typescript
class System {
    private __compartment: FrameCompartment;  // Must declare
    private domain_var: number;                // Must declare
    
    constructor() {
        this.__compartment = new FrameCompartment(...);
        this.domain_var = 0;
    }
}
```

**Open Issues**:
- [ ] Scan domain block to pre-declare all properties
- [ ] Determine visibility (private/public/protected) for each
- [ ] Handle state variables (stored in compartments)

---

## TypeScript vs Python Differences

### Language-Level Differences

| Feature | Python | TypeScript | Decision Needed |
|---------|---------|------------|-----------------|
| **Type Annotations** | Optional | Required | Generate from Frame types or infer |
| **Null Handling** | `None` | `null`/`undefined` | Standardize on one |
| **String Interpolation** | f-strings | Template literals | Direct mapping |
| **Multiple Inheritance** | Supported | Not supported | Use interfaces/mixins |
| **Operator Overloading** | Supported | Limited | May restrict Frame features |
| **Tuples** | Native | Arrays/readonly tuples | Decide representation |
| **Decorators** | `@decorator` | Experimental | Avoid or use carefully |

### Runtime Differences

| Aspect | Python | TypeScript | Impact |
|--------|---------|------------|--------|
| **Module System** | `import module` | `import { } from` | Different code organization |
| **This Binding** | `self` explicit | `this` context-dependent | Arrow functions vs regular |
| **Reflection** | Rich runtime info | Limited | May affect Frame features |
| **Standard Library** | Extensive | Node/Browser APIs | External dependencies |

---

## Frame Language Feature Support

### ✅ Fully Supported Features

These Frame features map cleanly to TypeScript:

1. **Basic State Machines**
   - States → Methods
   - Transitions → Method calls
   - Events → Method parameters

2. **Interface Methods**
   - Direct mapping to public methods
   - Return types fully supported

3. **Actions & Operations**
   - Map to private methods
   - TypeScript supports same patterns

4. **Domain Variables**
   - Map to class properties
   - Full type support

### ⚠️ Features Requiring Special Handling

1. **Hierarchical State Machines (HSM)**
   - **Issue**: TypeScript lacks Python's dynamic attribute lookup
   - **Solution**: Explicit parent chain navigation
   ```typescript
   // Need explicit parent reference handling
   private navigateToParent(compartment: FrameCompartment): void {
       if (compartment.parent) {
           this._frame_router(event, compartment.parent);
       }
   }
   ```

2. **State Parameters**
   - **Issue**: TypeScript needs typed parameter objects
   - **Solution**: Generate interfaces for each state's parameters
   ```typescript
   interface StateAParams {
       count: number;
       name: string;
   }
   ```

3. **State Variables**
   - **Issue**: Dynamic property access harder in TypeScript
   - **Solution**: Use Map or Record types in compartments
   ```typescript
   class FrameCompartment {
       stateVars: Record<string, any>;  // Or use Map<string, any>
   }
   ```

4. **Loop Statements**
   - **Issue**: Frame's `loop` with no condition
   - **Solution**: `while (true)` with explicit breaks

5. **Match Statements**
   - **Issue**: TypeScript switch is less powerful than Python match
   - **Solution**: Generate if-else chains or use discriminated unions

### ❌ Features Needing Design Decisions

1. **Dynamic System Creation**
   - Frame allows: `var sys = SystemName(params)`
   - TypeScript issue: Constructors vs factory functions
   - **Decision needed**: Use `new SystemName()` or factory pattern?

2. **Module-Level Functions**
   - Frame generates Python module-level wrappers
   - TypeScript issue: No true module-level functions in classes
   - **Options**:
     - Static methods on class
     - Separate module exports
     - Singleton pattern

3. **String Operators**
   - Frame's `+` for concatenation
   - Python's `*` for repetition
   - TypeScript lacks `*` operator
   - **Decision needed**: Generate method calls?

4. **Type Inference**
   - Frame often omits types
   - TypeScript needs them
   - **Options**:
     - Default to `any` (loose)
     - Infer from usage (complex)
     - Require type annotations (strict)

---

## Open Issues & Design Decisions

### Critical Decisions Required

#### 1. Runtime Library Architecture
**Options**:
- **A. Single runtime file**: `frame_runtime.ts` with all support classes
- **B. Modular runtime**: Separate files for each class
- **C. NPM package**: `@frame-lang/runtime` package

**Recommendation**: Start with A, migrate to C when stable

#### 2. Module System Strategy
**Options**:
- **A. ES Modules**: Modern, tree-shakeable
- **B. CommonJS**: Node.js compatible
- **C. UMD**: Universal module definition

**Recommendation**: ES Modules with TypeScript compiler handling compatibility

#### 3. Error Handling
**Python approach**: Exceptions with try/catch
**TypeScript options**:
- **A. Exceptions**: Same as Python
- **B. Result types**: `Result<T, E>` pattern
- **C. Error callbacks**: Node.js style

**Recommendation**: Exceptions for consistency with Python

#### 4. Null Safety
**Frame nullable types**: `var x: int?`
**TypeScript options**:
- **A. Union types**: `number | null`
- **B. Optional**: `number | undefined`
- **C. Both**: `number | null | undefined`

**Recommendation**: Union with null (`T | null`) for clarity

#### 5. Collection Types
**Frame collections**: Lists, Maps, Sets
**TypeScript options**:
- **A. Native**: `Array`, `Map`, `Set`
- **B. Immutable.js**: Immutable collections
- **C. Custom**: Frame-specific collection classes

**Recommendation**: Native TypeScript collections

### Implementation-Specific Issues

#### 1. State Method Naming
```typescript
// Option A: Same as Python
private _handle_state_event(): void { }

// Option B: camelCase
private handleStateEvent(): void { }

// Option C: Prefixed camelCase
private frameHandleStateEvent(): void { }
```
**Decision needed**: Naming convention

#### 2. Event Parameter Passing
```typescript
// Option A: Single event object (like Python)
private handleEvent(e: FrameEvent): void { }

// Option B: Destructured parameters
private handleEvent(message: string, params: any): void { }

// Option C: Typed event variants
private handleEvent(e: StartEvent | StopEvent): void { }
```
**Decision needed**: Event representation

#### 3. Compartment Storage
```typescript
// Option A: Class properties
class Compartment {
    stateVars: Record<string, any>;
}

// Option B: Map
class Compartment {
    stateVars: Map<string, any>;
}

// Option C: Typed generics
class Compartment<T extends Record<string, any>> {
    stateVars: T;
}
```
**Decision needed**: State variable storage

---

## Code Generation Patterns

### System Generation Pattern

```typescript
// Generated TypeScript for a Frame system
export class SystemName {
    // Property declarations
    private _compartment: FrameCompartment;
    private _nextCompartment: FrameCompartment | null;
    private returnStack: any[];
    
    // Domain variables
    private domainVar1: number;
    private domainVar2: string;
    
    constructor() {
        // Initialize compartments
        this._compartment = new FrameCompartment('State_Initial');
        this._nextCompartment = null;
        this.returnStack = [null];
        
        // Initialize domain
        this.domainVar1 = 0;
        this.domainVar2 = "";
        
        // Send start event
        this._frame_kernel(new FrameEvent("$>", null));
    }
    
    // Interface methods
    public interfaceMethod(param: number): string {
        this.returnStack.push(null);
        const __e = new FrameEvent("interfaceMethod", {param});
        this._frame_kernel(__e);
        return this.returnStack.pop();
    }
    
    // State handlers
    private _handle_Initial_interfaceMethod(__e: FrameEvent, compartment: FrameCompartment): void {
        // Handler implementation
    }
    
    // State dispatchers
    private State_Initial(__e: FrameEvent, compartment: FrameCompartment): void {
        switch(__e.message) {
            case "interfaceMethod":
                this._handle_Initial_interfaceMethod(__e, compartment);
                break;
        }
    }
    
    // Actions
    private _action_doWork(): void {
        // Action implementation
    }
    
    // Frame runtime
    private _frame_kernel(__e: FrameEvent): void {
        this._frame_router(__e);
        
        while (this._nextCompartment !== null) {
            const nextCompartment = this._nextCompartment;
            this._nextCompartment = null;
            
            // Exit current state
            this._frame_router(new FrameEvent("<$", this._compartment.exitArgs));
            
            // Change state
            this._compartment = nextCompartment;
            
            // Enter new state
            if (nextCompartment.forwardEvent === null) {
                this._frame_router(new FrameEvent("$>", this._compartment.enterArgs));
            } else {
                // Handle forwarded event
                this._frame_router(nextCompartment.forwardEvent);
                nextCompartment.forwardEvent = null;
            }
        }
    }
    
    private _frame_router(__e: FrameEvent, compartment?: FrameCompartment): void {
        const targetCompartment = compartment || this._compartment;
        
        switch(targetCompartment.state) {
            case 'State_Initial':
                this.State_Initial(__e, targetCompartment);
                break;
            // Other states...
        }
    }
    
    private _frame_transition(nextCompartment: FrameCompartment): void {
        this._nextCompartment = nextCompartment;
    }
}
```

### Runtime Support Classes

```typescript
// frame_runtime.ts
export class FrameEvent {
    constructor(
        public message: string,
        public parameters: any
    ) {}
}

export class FrameCompartment {
    constructor(
        public state: string,
        public forwardEvent: FrameEvent | null = null,
        public exitArgs: any = null,
        public enterArgs: any = null,
        public parent: FrameCompartment | null = null,
        public stateVars: Record<string, any> = {},
        public stateArgs: Record<string, any> = {}
    ) {}
}
```

---

## Implementation Phases

### Phase 1: Basic State Machines (Week 1-2)
- [ ] Create `typescript_visitor.rs`
- [ ] Implement basic system generation
- [ ] Simple states and transitions
- [ ] Interface methods
- [ ] Basic actions
- [ ] Runtime support classes

**Milestone**: Generate working TypeScript for simple state machines

### Phase 2: Type System (Week 3)
- [ ] Type inference from Frame annotations
- [ ] Generate TypeScript interfaces
- [ ] Handle nullable types
- [ ] Collection type mappings
- [ ] Generic type support

**Milestone**: Fully typed TypeScript output

### Phase 3: Advanced Features (Week 4-5)
- [ ] Hierarchical state machines
- [ ] State parameters
- [ ] State variables
- [ ] Event forwarding
- [ ] State stack operations

**Milestone**: HSM support complete

### Phase 4: Frame Constructs (Week 6)
- [ ] Loop statements
- [ ] Match statements
- [ ] Try/catch/finally
- [ ] Assert statements
- [ ] Delete statements

**Milestone**: All Frame statements supported

### Phase 5: Module System (Week 7)
- [ ] Import statements
- [ ] Module-level functions
- [ ] Multi-file support
- [ ] Export patterns
- [ ] NPM package structure

**Milestone**: Full module support

### Phase 6: Async Support (Week 8)
- [ ] Async state machines
- [ ] Async interface methods
- [ ] Await in actions
- [ ] Promise handling
- [ ] Error propagation

**Milestone**: Async/await fully integrated

### Phase 7: Testing & Polish (Week 9-10)
- [ ] Port all Python tests to TypeScript
- [ ] Performance optimization
- [ ] Documentation
- [ ] Error messages
- [ ] Source maps

**Milestone**: Production ready

---

## Testing Strategy

### Test Infrastructure

1. **Test Runner**: Similar to `frame_test_runner.py` but for TypeScript
   ```bash
   npm test                           # Run all tests
   npm test -- --pattern "test_*.frm" # Pattern matching
   ```

2. **Test Structure**:
   ```
   framec_tests/typescript/
   ├── src/
   │   ├── positive/     # Should compile and run
   │   └── negative/     # Should fail compilation
   ├── generated/        # Generated TypeScript
   └── test_runner.ts    # Test execution framework
   ```

3. **Validation Approach**:
   - Compile `.frm` → `.ts`
   - TypeScript compile `.ts` → `.js`
   - Run and validate output
   - Compare with Python output

### Test Categories

1. **Type Safety Tests**
   - Verify TypeScript compiler catches type errors
   - Ensure Frame type annotations translate correctly

2. **Runtime Behavior Tests**
   - State machine execution matches Python
   - Event handling identical
   - Return values consistent

3. **Edge Case Tests**
   - Null/undefined handling
   - Empty states
   - Complex transitions
   - Deep hierarchies

4. **Performance Tests**
   - Large state machines
   - Many transitions
   - Deep call stacks

---

## Risk Analysis

### High Risk Items

1. **Type Inference Complexity**
   - Risk: Incorrect type inference leads to runtime errors
   - Mitigation: Default to `any` with strict mode option

2. **Async/Sync Mismatch**
   - Risk: Mixing async and sync incorrectly
   - Mitigation: Clear async detection rules

3. **This Binding Issues**
   - Risk: `this` context lost in callbacks
   - Mitigation: Arrow functions or explicit binding

### Medium Risk Items

1. **Module System Compatibility**
   - Risk: Generated code doesn't work in all environments
   - Mitigation: Configurable module output

2. **Collection Performance**
   - Risk: Map/Set performance differs from Python dict/set
   - Mitigation: Performance testing and optimization

### Low Risk Items

1. **Naming Conflicts**
   - Risk: Generated names conflict with TypeScript keywords
   - Mitigation: Keyword escaping function

2. **Source Map Accuracy**
   - Risk: Debugging experience degraded
   - Mitigation: Reuse Python visitor's mapping logic

---

## Success Criteria

### Must Have
- [ ] All Frame language features supported
- [ ] TypeScript compiler accepts all generated code
- [ ] Runtime behavior matches Python visitor output
- [ ] All existing Frame tests pass when adapted
- [ ] Source maps for debugging

### Should Have
- [ ] Type safety maximized (minimal `any` usage)
- [ ] Idiomatic TypeScript code generated
- [ ] Good error messages for Frame errors
- [ ] Performance comparable to Python
- [ ] Browser and Node.js compatibility

### Nice to Have
- [ ] JSDoc comments from Frame comments
- [ ] TypeScript declaration files (`.d.ts`)
- [ ] Tree-shaking support
- [ ] Optimized for minification
- [ ] React/Vue integration examples

---

## Next Steps

1. **Review and approve this plan**
2. **Make critical design decisions** (marked above)
3. **Create `typescript_visitor.rs`** based on `python_visitor_v2.rs`
4. **Implement Phase 1** (basic state machines)
5. **Validate approach** with simple examples
6. **Iterate through remaining phases**

---

## Appendix A: Design Decision Template

For each open issue, document:

```markdown
### Decision: [Issue Name]

**Options Considered**:
1. Option A: [Description]
   - Pros: ...
   - Cons: ...
2. Option B: [Description]
   - Pros: ...
   - Cons: ...

**Decision**: Option X

**Rationale**: [Why this option was chosen]

**Implementation Notes**: [Any special considerations]
```

## Appendix B: TypeScript Visitor Methods

Key visitor methods to implement (parallel to Python visitor):

```rust
impl TypeScriptVisitor {
    // System level
    fn visit_system_node(&mut self, node: &SystemNode)
    
    // Interface
    fn visit_interface_block_node(&mut self, node: &InterfaceBlockNode)
    fn visit_interface_method_node(&mut self, node: &InterfaceMethodNode)
    
    // Machine
    fn visit_machine_block_node(&mut self, node: &MachineBlockNode)
    fn visit_state_node(&mut self, node: &StateNode)
    fn visit_event_handler_node(&mut self, node: &EventHandlerNode)
    
    // Actions/Operations
    fn visit_actions_block_node(&mut self, node: &ActionsBlockNode)
    fn visit_action_node(&mut self, node: &ActionNode)
    fn visit_operation_node(&mut self, node: &OperationNode)
    
    // Domain
    fn visit_domain_block_node(&mut self, node: &DomainBlockNode)
    
    // Statements
    fn visit_statement(&mut self, stmt: &StatementType)
    fn visit_expression(&mut self, expr: &ExprType)
    
    // Types
    fn generate_type_annotation(&mut self, frame_type: &TypeNode) -> String
}
```

---

## Document History

- 2025-01-13: Initial draft created
- [Future dates for updates]