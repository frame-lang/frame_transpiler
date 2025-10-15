# Bug #048: Unreachable Return After Transition Statements

## Metadata
```yaml
bug_number: 048
title: "Unreachable Return After Transition Statements"
status: Won't Fix
priority: High
category: Semantic
discovered_version: v0.81.6
fixed_version: 
reporter: AI System (incorrect analysis)
assignee: Claude
created_date: 2025-10-13
resolved_date: 2025-10-13
```

## Description
An AI system incorrectly reported that the transpiler should allow `return` statements after state transitions (`->` operator). However, this is actually correct Frame language behavior - transitions are terminal statements that end event handler execution.

## Reproduction Steps
1. Create a Frame file with a state transition followed by a return
2. Run transpiler with any target language
3. Observe "Unreachable code: 'return' statement after transition" error

## Test Case
```frame
system EventHandler {
    interface:
        PassAdd(a: int, b: int)
    
    machine:
        $S1 {
            PassAdd(a: int, b: int) {
                -> $S2(a+b)
                return        // This should be rejected
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
The code should be rejected with an error message. In Frame language:
1. Transitions are terminal statements that end event handler execution
2. No code should execute after a transition in the same handler
3. The transpiler should enforce proper Frame semantics

## Actual Behavior
Compilation correctly fails with error:
```
Error: Unreachable code: 'return' statement after transition
  --> test_event_handler.frm:38:17
   |
38 |                 return
   |                 ^^^^^^
```

**This is the correct behavior.**

## Impact
- **Severity**: None - Enforces correct Frame language semantics
- **Scope**: Prevents invalid Frame code patterns
- **Solution**: Remove return statements after transitions as they are unnecessary

## Technical Analysis
The parser's control flow analyzer in `parser.rs` correctly treats state transitions as terminal statements. Frame's semantics are:

1. Transitions end the current event handler execution immediately
2. The Frame runtime automatically generates appropriate returns in target languages
3. No statements should execute after a transition in the same handler

### Root Cause
This was not a bug but a misunderstanding of Frame language semantics by an AI system.

### Affected Files
- None - the current behavior is correct

## Resolution

### Decision: Won't Fix
After careful analysis of Frame language semantics, the original parser behavior is correct:

1. **Transitions are terminal** - they end event handler execution immediately
2. **No code after transitions** - Frame design principle for clear state machine semantics  
3. **Runtime handles returns** - the transpiler generates appropriate returns automatically

### Correct Frame Code
```frame
system EventHandler {
    machine:
        $S1 {
            PassAdd(a: int, b: int) {
                -> $S2(a+b)
                // No return needed - transition ends the handler
            }
        }
}
```

### Why This Enforces Good Design
1. **Clear semantics** - no ambiguity about when handlers end
2. **State machine purity** - transitions are state changes, not code flow
3. **Prevents errors** - catches accidental code after transitions
4. **Consistent behavior** - same rules across all target languages

### Current Implementation
The parser correctly rejects code after transitions:
```rust
// Continue parsing to detect any unreachable code after transition
while !self.check(TokenType::Eof) && !self.check(TokenType::CloseBrace) {
    if self.check(TokenType::Return_) || /* other statements */ {
        self.error_at_current("Unreachable code: 'return' statement after transition");
    }
}
```

### Lessons Learned
- AI systems may misunderstand domain-specific language semantics
- Frame's state machine model differs from procedural programming
- Parser strictness prevents subtle bugs in state machine logic

## Related Issues
None - this maintains Frame's intended behavior

## Work Log
- 2025-10-13: Incorrect bug report filed by AI system
- 2025-10-13: Analysis revealed this is correct Frame behavior
- 2025-10-13: Closed as "Won't Fix" - no change needed

---
*Bug tracking policy version: 1.0*