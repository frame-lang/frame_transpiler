# Critical AST Nodes Missing Line Numbers - Quick Reference

## 🔴 IMMEDIATE ACTION REQUIRED (Top 10)

These nodes represent the most common executable statements that debuggers need to step through:

1. **`IfStmtNode`** - Conditional statements (`if`/`elif`/`else`)
2. **`CallStmtNode`** - Function call statements 
3. **`CallExprNode`** - Function call expressions
4. **`AssignmentStmtNode`** - Variable assignments
5. **`ForStmtNode`** - For loop statements
6. **`WhileStmtNode`** - While loop statements
7. **`ReturnStmtNode`** - Return statements
8. **`TryStmtNode`** - Exception handling blocks
9. **`BreakStmtNode`** - Loop break statements
10. **`ContinueStmtNode`** - Loop continue statements

## 🟡 SECONDARY PRIORITY (Next 10)

Important for Frame-specific debugging:

11. **`ActionCallStmtNode`** - Frame action calls
12. **`TransitionExprNode`** - State machine transitions
13. **`MatchStmtNode`** - Pattern matching statements
14. **`WithStmtNode`** - Context manager statements
15. **`RaiseStmtNode`** - Exception raising
16. **`DelStmtNode`** - Delete statements
17. **`AssertStmtNode`** - Assertion statements
18. **`BinaryExprNode`** - Binary operations (`+`, `-`, `*`, etc.)
19. **`UnaryExprNode`** - Unary operations (`not`, `-`, etc.)
20. **`AwaitExprNode`** - Async await expressions

## Implementation Pattern

For each node, add:

```rust
pub struct NodeName {
    // existing fields...
    pub line: usize,  // ADD THIS
}

impl NodeName {
    pub fn new(/* existing params */, line: usize) -> NodeName {
        NodeName {
            // existing assignments...
            line,  // ADD THIS
        }
    }
}
```

Then update parser calls to pass `self.current_token.line` or equivalent.

## Total Impact

- **Current coverage**: 14/122 nodes (11.5%) have line numbers
- **After Phase 1**: 37/122 nodes (30.3%) - covers all critical debugging scenarios
- **After Phase 2**: 53/122 nodes (43.4%) - comprehensive expression debugging
- **Full implementation**: 122/122 nodes (100%) - complete line number coverage