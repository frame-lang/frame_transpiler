# Frame Transpiler AST Line Number Implementation Plan

## Executive Summary

The Frame transpiler AST currently has **108 nodes missing line number information** out of 122 total nodes. Only 14 nodes currently track line numbers for debugging support. This analysis provides a prioritized implementation plan to add line number tracking to the most critical nodes for debugging.

## Current State

### Nodes WITH Line Numbers (14)
- ✅ `AssignmentExprNode`
- ✅ `ClassNode`
- ✅ `DispatchNode` 
- ✅ `EventHandlerNode`
- ✅ `FunctionNode`
- ✅ `IdentifierNode`
- ✅ `ImportNode`
- ✅ `MessageNode`
- ✅ `MethodNode`
- ✅ `ParentDispatchStmtNode`
- ✅ `StateNode`
- ✅ `SystemNode`
- ✅ `TerminatorExpr`
- ✅ `TypeAliasNode`

## Implementation Priority

### 🔴 Phase 1: HIGH PRIORITY (23 nodes)
**Essential for step-through debugging - implement first**

#### Control Flow Statements
- `IfStmtNode` - Conditional branches
- `ForStmtNode` - For loops
- `WhileStmtNode` - While loops  
- `MatchStmtNode` - Pattern matching
- `TryStmtNode` - Exception handling
- `WithStmtNode` - Context managers

#### Function Calls
- `CallStmtNode` - Function call statements
- `CallExprNode` - Function call expressions
- `ActionCallStmtNode` - Action call statements
- `ActionCallExprNode` - Action call expressions
- `InterfaceMethodCallExprNode` - Interface method calls
- `OperationCallExprNode` - Operation calls

#### Assignments
- `AssignmentStmtNode` - Assignment statements
- `VariableStmtNode` - Variable statements

#### Control Statements
- `ReturnStmtNode` - Return statements
- `ReturnAssignStmtNode` - Return assignment statements
- `BreakStmtNode` - Break statements
- `ContinueStmtNode` - Continue statements
- `RaiseStmtNode` - Exception raising
- `DelStmtNode` - Delete statements
- `AssertStmtNode` - Assertion statements

#### State Machine Operations
- `TransitionExprNode` - State transitions
- `TransitionStatementNode` - Transition statements

### 🟡 Phase 2: MEDIUM PRIORITY (16 nodes)
**Helpful for expression debugging**

#### Expression Evaluation
- `BinaryExprNode` - Binary operations
- `UnaryExprNode` - Unary operations
- `LiteralExprNode` - Literal values
- `AwaitExprNode` - Async await
- `YieldExprNode` - Generator yield
- `YieldFromExprNode` - Generator yield from
- `LambdaExprNode` - Lambda expressions
- `GeneratorExprNode` - Generator expressions

#### Loop Variants
- `LoopStmtNode` - Generic loop statements
- `LoopInStmtNode` - Iterator loops
- `LoopForStmtNode` - C-style for loops
- `LoopInfiniteStmtNode` - Infinite loops

#### System Expressions
- `SystemInstanceStmtNode` - System instantiation statements
- `SystemInstanceExprNode` - System instantiation expressions
- `SystemTypeStmtNode` - System type statements
- `SystemTypeExprNode` - System type expressions

### 🟠 Phase 3: LOW PRIORITY (16 nodes)
**Less critical but useful for completeness**

#### Container Statements
- `BlockStmtNode` - Code blocks
- `ExprListStmtNode` - Expression list statements
- `ListStmtNode` - List statements
- `BinaryStmtNode` - Binary statements

#### Literal Expressions
- `CallChainStmtNode` - Call chain statements
- `CallChainExprNode` - Call chain expressions
- `CallChainLiteralExprNode` - Call chain literals
- `EnumeratorStmtNode` - Enumerator statements
- `EnumeratorExprNode` - Enumerator expressions
- `SelfExprNode` - Self expressions

#### Collection Operations
- `StarExprNode` - Star expressions (unpacking)
- `UnpackExprNode` - Unpacking expressions
- `DictUnpackExprNode` - Dictionary unpacking

#### Call Helpers
- `CallExprListNode` - Call expression lists
- `ExprListNode` - Expression lists
- `OperationRefExprNode` - Operation references

### ⚪ Phase 4: STRUCTURAL ONLY (52 nodes)
**Generally don't need line numbers - consider only for specific use cases**

These nodes are primarily structural and don't represent executable code that would be stepped through in a debugger. They include declarations, type information, pattern matching components, and runtime metadata.

## Implementation Details

### Required Changes per Node

For each node that needs line number tracking:

1. **Add field to struct**:
   ```rust
   pub struct ExampleStmtNode {
       // existing fields...
       pub line: usize,  // ADD THIS
   }
   ```

2. **Update constructor**:
   ```rust
   impl ExampleStmtNode {
       pub fn new(/* existing params */, line: usize) -> ExampleStmtNode {
           ExampleStmtNode {
               // existing field assignments...
               line,  // ADD THIS
           }
       }
   }
   ```

3. **Update parser calls**:
   - Find all locations where the node is constructed
   - Pass the current line number from the parser
   - Parser typically has `self.current_token.line` or similar

4. **Optional: Add getter method**:
   ```rust
   impl ExampleStmtNode {
       pub fn get_line(&self) -> usize {
           self.line
       }
   }
   ```

### Parser Integration

The parser (parser.rs) will need updates to:
1. Track current line numbers during parsing
2. Pass line numbers to node constructors
3. Ensure line numbers are propagated correctly through recursive parsing

### Visitor Pattern Updates

Code generation visitors may need minor updates to access line number information, but this is typically optional unless generating debug information or source maps.

## Benefits

Implementing line number tracking will enable:

1. **Better Error Messages**: Show exact line numbers in compilation errors
2. **Debugging Support**: Enable debugger step-through functionality
3. **Source Maps**: Generate accurate source maps for transpiled code
4. **IDE Integration**: Better IDE support with precise location information
5. **Testing**: More accurate test failure reporting

## Effort Estimation

- **Phase 1 (23 nodes)**: ~2-3 days - Critical functionality
- **Phase 2 (16 nodes)**: ~1-2 days - Enhanced debugging  
- **Phase 3 (16 nodes)**: ~1 day - Completeness
- **Phase 4 (52 nodes)**: ~2-3 days - Only if needed

**Total effort**: 6-9 days for complete implementation

## Recommendation

Start with **Phase 1 (HIGH PRIORITY)** nodes as they provide the most debugging value for the least effort. This covers all critical control flow, function calls, and state machine operations that a developer would want to step through in a debugger.