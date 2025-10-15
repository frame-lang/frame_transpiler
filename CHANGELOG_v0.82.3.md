# Release v0.82.3 - Bug Tracking Infrastructure & Critical Fix

## Release Date
2025-10-13

## Summary
This release introduces a comprehensive bug tracking infrastructure and resolves Bug #048, which prevented valid Frame code with return statements after state transitions from compiling.

## Bug Fixes

### Bug #048: Unreachable Return After Transition Statements (RESOLVED)
- **Issue**: Parser incorrectly rejected valid Frame code with `return` statements after state transitions
- **Impact**: Blocked 10% of core test suite
- **Solution**: Modified parser control flow analysis to allow optional return after transitions
- **Files Changed**: `framec/src/frame_c/parser.rs` (lines 5466-5512)

## New Features

### Bug Tracking Infrastructure
- Created comprehensive bug tracking system in `/docs/bugs/`
- Established clear policies and procedures for bug management
- Migrated all existing bugs from multiple tracking systems
- Created templates and index for efficient bug tracking

### Documentation Structure
```
docs/bugs/
├── BUG_TRACKING_POLICY.md    # Bug tracking procedures
├── TEMPLATE.md                # Standard bug report template
├── INDEX.md                   # Master index of all bugs
├── open/                      # Active bugs (4 remaining)
└── closed/                    # Resolved bugs (46 total)
```

## Known Issues
- Bug #037: State Diagram Missing Conditional Transitions (Low priority)
- Bug #039: Missing Frame Semantic Metadata for Debugger (Medium priority)
- Bug #049: TypeScript Transpilation Rate Lower Than Python (Medium priority)
- Bug #050: Test Runner Language Filtering (Low priority)

## Statistics
- Total Bugs Tracked: 50
- Bugs Resolved: 46
- Bugs Open: 4
- Success Rate: 92%

## Technical Details

### Parser Enhancement
The parser now correctly handles Frame's transition semantics where:
1. Transitions schedule state changes but don't immediately transfer control
2. Return statements after transitions ensure proper handler cleanup
3. Other statements after transitions remain unreachable

### Example of Fixed Code
```frame
system EventHandler {
    machine:
        $S1 {
            PassAdd(a: int, b: int) {
                -> $S2(a+b)
                return        // Now allowed!
            }
        }
}
```

## Migration Notes
- All bug tracking has been consolidated into `/docs/bugs/`
- Old tracking files should be removed after verification
- Use `docs/bugs/TEMPLATE.md` for new bug reports
- Check `docs/bugs/INDEX.md` for current bug status

## Contributors
- Bug tracking infrastructure design and implementation
- Parser control flow fix for Bug #048
- Documentation and migration of existing bugs

## Next Steps
- Continue fixing remaining open bugs
- Improve TypeScript visitor to match Python capabilities
- Enhance test runner with language-specific filtering

---
*Frame Transpiler v0.82.3 - Building reliable state machines*