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
- Always check actual test files in `framepiler_test_env/framepiler/fixtures/test-frames/` for examples

## Current State
- **Version**: v0.86.71 (branch `going_native`)
- **Shared Environment**: Active via `FRAMEPILER_TEST_ENV` for isolated transpiler/debugger development
- **Test Status**: TypeScript transpilation 87.5% success (35/40 tests) in shared environment
- **Latest Achievements**: Shared test environment operational, TypeScript import handling improved, Python-style imports properly converted
- **Recent Focus**: Moved from embedded testing to shared environment approach, improved TypeScript transpilation with import collection/conversion

## Quick References
- **Shared environment**: Tests now live in `framepiler_test_env/` (symlinked in project root)
  - 📖 **READ**: `framepiler_test_env/README.md` for shared environment structure
  - 📖 **READ**: `framepiler_test_env/framepiler/README.md` for transpiler team space
  - Test fixtures: `framepiler_test_env/framepiler/fixtures/test-frames/`
  - Set `FRAMEPILER_TEST_ENV=framepiler_test_env` to use shared fixtures
- **Test runner (Rust)**: `FRAMEPILER_TEST_ENV=framepiler_test_env cargo run --bin v3_rs_test_runner -- python v3_core ./target/release/framec`
- **Module separator**: `::` (NOT `.` - dot is for member access)
- **Check before starting**: Read `docs/HOW_TO.md` for complete current processes

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
