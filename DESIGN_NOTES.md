# Frame Language Design Notes

## @:> (DispatchToParentState) Operator

### Decision: Block Terminator with Implicit Return

**Date**: 2025-01-16  
**Context**: Implementation of @:> operator to replace :> for event forwarding

### Design Decision

The `@:>` operator is implemented as a **block terminator** rather than a statement. This means:

1. **Parser behavior**: `@:>` must appear as the last token before the closing `}` of an event handler
2. **Semantic behavior**: `@:>` forwards the event to the parent state and **terminates** execution of the current handler
3. **Codegen requirement**: All target language visitors must generate an implicit `return` statement after the dispatch code to ensure no subsequent statements are executed

### Rationale

- **Prevents confusing control flow**: If `@:>` were a statement, subsequent statements could execute after the parent processes the event, leading to unpredictable behavior especially if the parent triggers a state transition
- **Maintains clean HSM semantics**: Event forwarding should be a terminating operation - once forwarded, the event is "consumed" by the parent
- **Consistent with Frame's design**: Similar to how transitions terminate event handlers

### Implementation Notes

- `TokenType::DispatchToParentState` is recognized by scanner for `@:>` syntax
- `TerminatorType::DispatchToParentState` enum variant handles semantic meaning
- All visitor files updated to handle the new terminator type
- Code generators must emit implicit return/break/exit logic after dispatch

### Future Considerations

This design choice was made to avoid the complexity of allowing post-dispatch statement execution. If needed in the future, a separate "forward and continue" operator could be added with different semantics.

---

## Return Statements as Regular Statements

### Decision: Support Return in All Statement Contexts

**Date**: 2025-01-16  
**Context**: Fixed if/elif/else parsing failure in event handlers

### Problem

Event handlers failed to parse if/elif/else chains containing return statements:
```
[line 23] Error at 'return' : Expected '}'.
[line 24] Error at 'elif' : Expected '}'
```

Frame v0.20 aimed for conventional syntax but `return` was only supported as event handler terminators, not as regular statements within conditional blocks.

### Root Cause

Frame's parser had an architectural limitation where `return` statements were only parsed as terminators for methods/event handlers, not as regular statements within blocks:

1. Event handler flow: `statements()` → terminator → `}`
2. When `return` appeared inside if blocks, parser expected immediate `}` and failed on `elif`
3. Action methods worked because they had consistent statement parsing

### Design Decision

**Implemented `return` as a regular statement** by adding:

1. **AST Layer**: `StatementType::ReturnStmt { return_stmt_node }`
2. **Parser Layer**: Return statement parsing in `statement()` method  
3. **Visitor Layer**: `visit_return_stmt_node()` for all target languages

### Implementation Details

```rust
// AST (ast.rs:1689, 3780-3794)
ReturnStmt {
    return_stmt_node: ReturnStmtNode,
}

pub struct ReturnStmtNode {
    pub expr_t_opt: Option<ExprType>,
}

// Parser (parser.rs:4652-4667)  
if self.match_token(&[TokenType::Return_]) {
    let expr_t_opt = self.equality().ok().flatten();
    let return_stmt_node = ReturnStmtNode::new(expr_t_opt);
    return Ok(Some(StatementType::ReturnStmt { return_stmt_node }));
}

// Visitor (python_visitor.rs:4695-4704)
fn visit_return_stmt_node(&mut self, return_stmt_node: &ReturnStmtNode) {
    if let Some(expr_t) = &return_stmt_node.expr_t_opt {
        self.add_code(&format!("return {}", output));
    } else {
        self.add_code("return");
    }
}
```

### Rationale

- **Conventional Syntax**: Aligns with Frame v0.20 goal of conventional programming patterns
- **Consistency**: Event handlers and action methods now behave identically  
- **Extensibility**: Provides foundation for other statement types
- **Backward Compatibility**: Event handler terminators continue to work unchanged

### Validation

Created comprehensive test suite:
- `test_simple_elif.frm` - Basic if/elif without returns ✅
- `test_elif_with_return.frm` - Returns in conditional blocks ✅  
- `test_enums.frm` - Complex enum patterns with if/elif/return ✅

Generated code quality verified across Python target.

### Known Issues

**Dead Code Generation** (Low Priority):
- Event handlers still generate default `return` terminator even when all paths return
- Example: `if/elif/else` with returns followed by unreachable `return`
- Status: Functional correctness fine, optimization opportunity for future

### Alternative Considered

**Enhanced Terminator Parsing**: Could have made terminator parser handle if/elif chains, but regular statements approach was chosen for:
- Better alignment with conventional syntax goals
- Consistency with existing action method behavior
- Cleaner separation of concerns
- Easier future extensibility

### Impact

- ✅ Enables conventional conditional logic in event handlers
- ✅ Resolves parser limitation blocking Frame v0.20 adoption
- ✅ Maintains full backward compatibility
- ✅ Clean, readable generated code in all target languages