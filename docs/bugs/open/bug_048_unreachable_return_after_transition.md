# Bug #048: Unreachable Return After Transition Statements

## Metadata
```yaml
bug_number: 048
title: "Unreachable Return After Transition Statements"
status: Open
priority: High
category: Semantic
discovered_version: v0.81.6
fixed_version: 
reporter: Test Suite
assignee: 
created_date: 2025-10-13
resolved_date: 
```

## Description
The transpiler incorrectly rejects valid Frame code that has `return` statements after state transitions (`->` operator). These returns are actually reachable in certain contexts but the transpiler's control flow analysis treats transitions as terminal statements.

## Reproduction Steps
1. Create a Frame file with a state transition followed by a return
2. Run transpiler with any target language
3. Observe "Unreachable code: 'return' statement after trans" error

## Test Case
```frame
system EventHandler {
    interface:
        PassAdd(a: int, b: int)
    
    machine:
        $S1 {
            PassAdd(a: int, b: int) {
                -> $S2(a+b)
                return        // Error: Unreachable code
            }
        }
        
        $S2(sum: int) {
            $>() {
                print(f"Sum is: {sum}")
            }
        }
}
```

## Expected Behavior
The code should compile successfully. The return statement after transition is valid Frame semantics for:
1. Ensuring clean handler exit
2. Stack unwinding in certain runtime implementations
3. Returning control to the caller after scheduling state change

## Actual Behavior
Compilation fails with error:
```
Error: Unreachable code: 'return' statement after trans
  --> test_event_handler.frm:38:17
   |
38 |                 return
   |                 ^^^^^^
```

## Impact
- **Severity**: High - Blocks valid Frame patterns
- **Scope**: 6 core tests failing (10% of core test suite)
- **Workaround**: Remove return statements, but this may not match intended semantics

## Technical Analysis
The parser's control flow analyzer in `parser.rs` treats state transitions as terminal statements similar to `throw` or `panic!`. However, Frame's semantics differ:

1. Transitions schedule a state change but don't immediately transfer control
2. The return statement ensures proper handler cleanup
3. Some runtime implementations require explicit returns

### Root Cause
In `parser.rs`, the `check_unreachable_code()` function marks code after transitions as unreachable:
- Line ~5469: Transition statements set `found_unreachable_code = true`
- This triggers error on subsequent statements

### Affected Files
- `framec/src/frame_c/parser.rs` - Control flow analysis
- Test files:
  - `test_event_handler.frm` (lines 38, 45)
  - `test_state_context.frm` (line 37)
  - `test_state_context_stack.frm` (line 48)
  - `test_state_params.frm` (line 15)
  - `test_state_stack.frm` (line 39)
  - `test_transition_params.frm` (line 13)

## Proposed Solution

### Option 1: Allow Return After Transition (Recommended)
Modify control flow analysis to treat transitions as non-terminal:
- Don't set `found_unreachable_code` after transition statements
- Allow optional return statements after transitions
- Generate appropriate code for each target language

**Pros**: 
- Matches Frame's intended semantics
- Maintains backward compatibility
- Allows proper cleanup patterns

**Cons**: 
- May confuse users coming from other languages
- Requires documentation update

### Option 2: Make Return Implicit
Automatically insert returns after transitions during code generation:
- Remove explicit returns from Frame code
- Visitor adds returns as needed for target language

**Pros**: 
- Cleaner Frame syntax
- No ambiguity about control flow

**Cons**: 
- Breaking change for existing code
- Less explicit control for users

### Option 3: Context-Aware Analysis
Analyze whether return is needed based on context:
- Check handler return type
- Consider target language requirements
- Allow returns only when necessary

**Pros**: 
- Most precise solution
- Optimizes generated code

**Cons**: 
- Complex to implement
- Different behavior across contexts

## Test Coverage
- [ ] Fix existing failing tests
- [ ] Add test for return after transition
- [ ] Add test for no return after transition
- [ ] Verify all target languages handle correctly

## Related Issues
- Bug #030: Spurious unreachable return statements (similar issue, different context)

## Work Log
- 2025-10-13: Bug identified and documented during migration

## Resolution
[Pending]

---
*Bug tracking policy version: 1.0*