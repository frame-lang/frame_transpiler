# Frame Target Language Runtime Libraries

**Document Version**: 1.0  
**Date**: 2025-01-22  
**Status**: Design Specification  

## Overview

This document defines the architecture and implementation strategy for Frame target language runtime libraries. These libraries provide Frame-semantic implementations of language features that differ across target platforms, enabling consistent Frame behavior while generating clean, maintainable target code.

## 🎯 Architecture Goals

### Primary Objectives
1. **Semantic Consistency**: Identical Frame behavior across all target languages
2. **Clean Code Generation**: Simple function calls vs complex inline implementations
3. **Performance**: Language-optimized implementations using native patterns
4. **Maintainability**: Centralized bug fixes and feature additions

### Non-Goals
- Replace native language features where Frame semantics match
- Provide general-purpose utility libraries beyond Frame needs
- Support dynamic runtime loading or modification

## 🏗️ Design Principles

### 1. Selective Usage
```
Python Target: NO runtime needed
- Native Python semantics match Frame semantics
- Direct code generation continues unchanged
- Example: set1 == set2 → set1 == set2

TypeScript Target: Runtime for semantic gaps
- Where JavaScript/TypeScript differs from Frame/Python
- Example: set1 == set2 → FrameRuntime.equals(set1, set2)
```

### 2. Embedded Runtime
- Runtime code embedded directly in generated files
- No external dependencies for users
- Standalone compilation of generated code
- Version-locked to transpiler version

### 3. Language-Native Implementation
- Each runtime written in target language
- Uses target language best practices
- Optimized for target platform performance
- No cross-language abstraction overhead

## 📁 File Organization

```
docs/framelang_design/runtime_libraries/
├── typescript/
│   ├── frame_runtime.ts          # Core runtime functions
│   ├── frame_collections.ts      # Collection operations
│   ├── frame_strings.ts          # String operations
│   └── frame_async.ts            # Async/await patterns
├── rust/
│   ├── frame_runtime.rs
│   ├── frame_collections.rs
│   └── frame_strings.rs
├── java/
│   ├── FrameRuntime.java
│   ├── FrameCollections.java
│   └── FrameStrings.java
└── design/
    ├── api_specification.md      # Cross-language API contract
    ├── semantic_requirements.md  # Frame behavior definitions
    └── testing_strategy.md       # Validation approach
```

## 🔧 Implementation Strategy

### Phase 1: TypeScript Runtime Library

#### Core Functions Required
```typescript
class FrameRuntime {
    // Equality operations
    static equals(left: any, right: any): boolean
    static notEquals(left: any, right: any): boolean
    
    // Collection creation
    static createSet(items: any[]): Set<any>
    static createDict(pairs?: [any, any][]): Map<any, any>
    static createList(items?: any[]): any[]
    
    // Type operations
    static getType(obj: any): string
    static isinstance(obj: any, type: string): boolean
    
    // Range operations
    static range(stop: number): number[]
    static range(start: number, stop: number): number[]
    static range(start: number, stop: number, step: number): number[]
}
```

#### String Operations
```typescript
class FrameString {
    static format(template: string, ...args: any[]): string
    static startswith(str: string, prefix: string): boolean
    static endswith(str: string, suffix: string): boolean
    static join(separator: string, items: any[]): string
    static split(str: string, separator?: string): string[]
    static strip(str: string, chars?: string): string
    static upper(str: string): string
    static lower(str: string): string
}
```

#### Collection Operations
```typescript
class FrameCollections {
    // Dictionary operations
    static dictFromKeys(keys: any[], defaultValue?: any): Map<any, any>
    static dictUpdate(target: Map<any, any>, source: Map<any, any>): void
    static dictSetDefault(dict: Map<any, any>, key: any, defaultValue: any): any
    
    // Set operations
    static setUnion(a: Set<any>, b: Set<any>): Set<any>
    static setIntersection(a: Set<any>, b: Set<any>): Set<any>
    static setDifference(a: Set<any>, b: Set<any>): Set<any>
    
    // List operations
    static listExtend(target: any[], source: any[]): void
    static listInsert(list: any[], index: number, item: any): void
    static listRemove(list: any[], item: any): boolean
}
```

### Phase 2: Visitor Integration

#### Code Generation Changes
```rust
// Current TypeScript visitor (inline approach):
fn visit_binary_expr(&mut self, node: &BinaryExprNode) {
    match node.operator {
        OperatorType::EqualEqual => {
            // Generate 10+ lines of inline equality logic
            output.push_str("((l, r) => { /* complex logic */ })");
        }
    }
}

// Updated TypeScript visitor (runtime approach):
fn visit_binary_expr(&mut self, node: &BinaryExprNode) {
    match node.operator {
        OperatorType::EqualEqual => {
            output.push_str("FrameRuntime.equals(");
            self.visit_expr(&node.left);
            output.push_str(", ");
            self.visit_expr(&node.right);
            output.push_str(")");
        }
    }
}
```

#### Runtime Embedding Strategy
```rust
impl TypeScriptVisitor {
    fn generate_file_header(&mut self) -> String {
        format!(
            "// Generated by Frame v{}\n\n{}\n\n{}",
            self.version,
            self.get_typescript_runtime_code(),
            self.get_user_imports()
        )
    }
    
    fn get_typescript_runtime_code(&self) -> &'static str {
        include_str!("../runtime/typescript/frame_runtime.ts")
    }
}
```

### Phase 3: Testing and Validation

#### Runtime Unit Tests
```typescript
// Runtime function tests
describe('FrameRuntime.equals', () => {
    test('Set equality', () => {
        const set1 = new Set([1, 2, 3]);
        const set2 = new Set([3, 2, 1]);
        expect(FrameRuntime.equals(set1, set2)).toBe(true);
    });
    
    test('Primitive equality', () => {
        expect(FrameRuntime.equals(5, 5)).toBe(true);
        expect(FrameRuntime.equals(5, "5")).toBe(false);
    });
});
```

#### Cross-Language Compatibility Tests
```frame
# Test frame file that validates consistent behavior
fn test_set_operations() {
    var set1 = {1, 2, 3}
    var set2 = {3, 2, 1}
    
    assert set1 == set2        # Must work identically in all languages
    assert set1.len() == 3     # Must return same value
    
    var combined = set1 | set2 # Must produce same result
    assert combined == {1, 2, 3}
}
```

## 📊 Success Metrics

### Performance Targets
- **Code Size**: 50% reduction vs current inline approach
- **Runtime Performance**: No more than 5% overhead vs native operations
- **Compilation Speed**: No measurable impact on transpilation time

### Quality Targets  
- **Test Coverage**: 95%+ for all runtime functions
- **Cross-Language Consistency**: 100% identical behavior for covered operations
- **Bug Resolution**: Fix once in runtime vs N times in visitors

### Success Rate Targets
- **TypeScript**: 85%+ execution success (from current 75.8%)
- **Rust**: 70%+ execution success (establish baseline)
- **Java**: 70%+ execution success (establish baseline)

## 🔄 Migration Strategy

### Step 1: Implement Core Runtime (Week 1-2)
1. Create TypeScript runtime with 10 most critical functions
2. Update TypeScript visitor for equality, collections, type operations
3. Validate against current test suite
4. Target: Match current 75.8% success rate

### Step 2: Expand Runtime Coverage (Week 3-4)
1. Add string operations, advanced collections, edge cases
2. Update visitor to use expanded runtime
3. Fix failing tests using runtime improvements
4. Target: Achieve 85%+ success rate

### Step 3: Rust Runtime (Week 5-6)
1. Implement Rust runtime based on validated TypeScript patterns
2. Create or update Rust visitor for runtime usage
3. Establish Rust baseline success rate
4. Target: 70%+ Rust execution success

### Step 4: Documentation and Optimization (Week 7-8)
1. Complete API documentation for all runtimes
2. Performance optimization and benchmarking
3. Cross-language behavior validation
4. Prepare for additional target languages

## 🚨 Risk Mitigation

### Code Maintenance Risks
- **Risk**: Bug fixes needed in multiple runtimes
- **Mitigation**: Comprehensive test suite, automated cross-language validation
- **Contingency**: Shared test definitions that validate all runtimes

### Performance Risks
- **Risk**: Runtime overhead impacts performance
- **Mitigation**: Benchmark against current inline approach, optimize hot paths
- **Contingency**: Selective runtime usage - inline for performance-critical operations

### Compatibility Risks  
- **Risk**: Runtime behavior diverges between languages
- **Mitigation**: Formal API specification, shared test suite
- **Contingency**: Automated compatibility testing in CI pipeline

## 🎯 Future Extensions

### Additional Target Languages
- **C++**: High-performance runtime for systems programming
- **Go**: Cloud-native applications runtime
- **Java**: Enterprise applications runtime
- **C#**: .NET ecosystem runtime

### Advanced Features
- **Async/Await**: Cross-language async pattern support
- **Memory Management**: Consistent resource handling
- **Error Handling**: Unified exception/error patterns
- **Debugging Support**: Source maps and runtime debugging

### Optimization Opportunities
- **Dead Code Elimination**: Only include used runtime functions
- **Inline Optimization**: Selective inlining for performance-critical paths
- **Platform Specialization**: OS/architecture-specific optimizations

## 📚 References

- **Current Implementation**: `framec/src/frame_c/visitors/typescript_visitor.rs`
- **Existing Capability Modules**: `framec_tests/common/capability_modules/`
- **Cross-Language Analysis**: `docs/framelang_design/cross_language_universality_analysis.md`
- **Python Visitor**: `framec/src/frame_c/visitors/python_visitor_v2.rs` (reference implementation)

---

**Next Steps**: 
1. Review and approve this design specification
2. Create TypeScript runtime implementation 
3. Update TypeScript visitor for runtime integration
4. Begin implementation of Phase 1 objectives