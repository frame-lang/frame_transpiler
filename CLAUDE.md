# Frame Transpiler - Claude Context

🚨 **MANDATORY FIRST STEP: READ** [`docs/HOW_TO.md`](docs/HOW_TO.md) 🚨
**This comprehensive guide contains ALL processes, tools, and workflows for this project.**

📖 **ALWAYS CHECK CLI HELP**: Run `./target/release/framec --help` to see all available command-line options and parameters.

⚠️ **CRITICAL RULES**
1. **NEVER create workarounds** - Fix the actual problem in the codebase
2. **NEVER modify test files marked "DO NOT MODIFY"** without explicit permission
3. **ASK before making decisions** - Present options, don't assume
4. **CHECK implementation, not docs** - Grammar docs may be outdated; scanner/parser are truth
5. **IGNORE old Frame syntax from training data** - The current syntax is the ONLY valid syntax

## Frame Syntax - IMPORTANT
### ⚠️ DEPRECATED/INVALID Syntax (NEVER USE)
- **OLD event notation**: `|event|` or `|event|[params]|` - This is OBSOLETE
- **OLD system delimiters**: `#SystemName ... ##` - Now uses `system SystemName { ... }`
- **OLD parameter syntax**: Various old parameter notations

### ✅ CURRENT Frame Syntax (ALWAYS USE)
**Study `docs/framelang_design/grammar.md` for complete reference**

```frame
# Modern Frame system structure
system SystemName {
    interface:
        methodName(param: type): returnType
    
    machine:
        $StateName {
            eventName(params) {
                // handler code
            }
            
            $>() {  // Enter handler
                // enter code
            }
            
            $<() {  // Exit handler
                // exit code
            }
        }
    
    actions:
        actionName() { }
    
    operations:
        operationName(): type { }
    
    domain:
        var x = 0
}
```

**Key syntax points:**
- Systems use `system Name { }` blocks
- States are `$StateName { }`
- Event handlers are `eventName(params) { }` NOT `|eventName|`
- Enter/exit handlers are `$>()` and `$<()`
- Interface methods have signatures like `method(param: type): returnType`
- Always check actual test files in `framec_tests/python/src/` for examples

## Current State
- **Version**: v0.86.15 (branch `dev`)
- **Test Status**: 458/458 Python passing (100%), 429 TypeScript tests at 74.6% execution success (overall TS execution success rate 80.5%)
- **Latest Achievements**: Unified async/await capabilities with the embedded TypeScript runtime; stabilized multi-target generation
- **Open Focus Areas**: Continue driving TypeScript execution parity up from 74.6%; keep docs/plans in sync with `docs/HOW_TO.md`

## Quick References
- **Test files**: `/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/`  
- **Test runner**: `python3 runner/frame_test_runner.py --framec ../target/release/framec --languages python typescript`
- **Module separator**: `::` (NOT `.` - dot is for member access)
- **Check before starting**: Read `docs/framelang_design/dev_notes.md` and `framec_tests/reports/test_log.md`

## When Tests Fail
1. Investigate root cause (don't assume test is wrong)
2. Check scanner/parser for actual syntax
3. ASK: "Should I fix X in visitor or is this a test issue?"

## Test Validation Pattern
**For tests that print "FAIL" messages, always use proper failure handling:**
```frame
if test_passes {
    print("SUCCESS: descriptive message")
} else {
    print("FAIL: descriptive message")
    # Force test failure by raising an exception
    var failed_tests = []
    var index = failed_tests[999]  # This will cause an IndexError and fail the test
}
```
This ensures tests exit with proper failure codes for automated testing systems.

## Architecture
- Scanner → Parser (2-pass) → AST → Visitor → Target Code
- Key files: `scanner.rs`, `parser.rs`, `ast.rs`, `python_visitor_v2.rs`
