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