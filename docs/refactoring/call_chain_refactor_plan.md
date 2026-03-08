# Call Chain Refactoring Plan for v0.61

## Problem Statement

The current call chain handling in `python_visitor.rs` has several issues:

1. **Code Duplication**: The logic for resolving actions, operations, and method calls is duplicated across:
   - `visit_call_expression_node()` (lines 6364-6522, ~158 lines)
   - `visit_call_expression_node_to_string()` (lines 6526-6670, ~144 lines)  
   - `handle_self_call()` (lines 4400-4423)
   - Additional helper methods

2. **Complexity**: The nested if-else chains make the logic hard to follow and error-prone
3. **Bug-Prone**: The v0.60 double-call bug was a result of this complexity
4. **Maintainability**: Changes need to be made in multiple places

## Solution Design

### 1. Create a Unified Call Resolver

The `CallResolver` struct encapsulates all call resolution logic:

```rust
pub struct CallResolver<'a> {
    arcanium: &'a ScopeManager,
    in_standalone_function: bool,
    in_class_method: bool,
    current_system_actions: &'a HashSet<String>,
    current_system_operations: &'a HashSet<String>,
}
```

Benefits:
- Single source of truth for call resolution
- Testable in isolation
- Clear separation of concerns

### 2. Simplified Resolution Result

```rust
pub struct CallResolution {
    pub prefix: String,        // "self.", "ClassName.", or empty
    pub method_name: String,   // "_action", "operation", "function"
    pub needs_parameters: bool,
}
```

This makes the output of resolution explicit and easy to use.

### 3. Unified Handler Methods

Replace the complex visitor methods with simplified versions:

```rust
// Before: 158 lines of complex logic
fn visit_call_expression_node(&mut self, method_call: &CallExprNode) {
    // Complex nested if-else chains...
}

// After: ~10 lines
fn visit_call_expression_node(&mut self, method_call: &CallExprNode) {
    self.debug_enter("visit_call_expression_node");
    self.handle_call_unified(method_call);
    self.debug_exit("visit_call_expression_node");
}
```

## Implementation Steps

### Phase 1: Add the Refactored Module
✅ (Historical) Created `/framec/src/frame_c/visitors/call_chain_refactor.rs` — removed in 2025-10-30 cleanup after PythonVisitorV2 fully absorbed the refactor.

### Phase 2: Integrate Into PythonVisitor

1. Add module import to `python_visitor.rs`:
```rust
mod call_chain_refactor;
use call_chain_refactor::{CallResolver, CallResolution};
```

2. Replace `visit_call_expression_node()` with simplified version using `handle_call_unified()`

3. Replace `visit_call_expression_node_to_string()` with simplified version using `handle_call_to_string_unified()`

4. Remove or deprecate `handle_self_call()` as it's now handled by the resolver

### Phase 3: Testing

1. Run full test suite to ensure no regressions:
```bash
cd framec_tests
python3 runner/frame_test_runner.py --all --matrix --json --verbose
```

2. Specific tests for call chain scenarios:
   - Action calls: `test_class_simple.frm`
   - Operation calls: `test_operations.frm`
   - Interface calls: `test_interface_methods.frm`
   - Static calls: `test_static_methods.frm`
   - External calls: `test_external_calls.frm`

### Phase 4: Cleanup

1. Remove deprecated methods
2. Update documentation
3. Add unit tests for `CallResolver`

## Benefits

### Before Refactoring
- **Lines of Code**: ~350+ across multiple methods
- **Duplication**: Same logic in 3-4 places
- **Complexity**: Deep nesting, hard to follow
- **Bug Risk**: High (as evidenced by v0.60 bug)

### After Refactoring
- **Lines of Code**: ~150-200 total
- **Duplication**: None (single resolver)
- **Complexity**: Clear separation, easy to follow
- **Bug Risk**: Low (centralized logic, testable)
- **Maintainability**: Changes in one place affect all call sites

## Risk Mitigation

1. **Incremental Integration**: Add new code alongside old, switch gradually
2. **Comprehensive Testing**: Run full test suite after each change
3. **Fallback Plan**: Keep old methods available during transition
4. **Debug Mode**: Add extensive debug output during development

## Success Criteria

1. ✅ All 378 tests still passing
2. ✅ Reduced code size by 40-50%
3. ✅ No code duplication for call resolution
4. ✅ Clear, understandable logic flow
5. ✅ Easy to add new call types in future

## Timeline

- Phase 1: ✅ Complete (refactored module created)
- Phase 2: In Progress (integration)
- Phase 3: Pending (testing)
- Phase 4: Pending (cleanup)

## Notes

This refactoring is a critical improvement that will:
- Prevent bugs like the v0.60 double-call issue
- Make the transpiler more maintainable
- Provide a foundation for future enhancements (e.g., new call types)
- Improve code readability and reduce cognitive load
