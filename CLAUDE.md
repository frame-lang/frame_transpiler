# Frame Transpiler - Claude Context

🚨 **MANDATORY FIRST STEPS** 🚨
1. **READ** [`docs/HOW_TO.md`](docs/HOW_TO.md) - Complete development guide
2. **READ** [`framepiler_test_env/GETTING_STARTED.md`](framepiler_test_env/GETTING_STARTED.md) - Test infrastructure guide

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
- Always check actual test files in `framepiler_test_env/common/test-frames/v3/` for examples

## Current State
- **Version**: v0.86.72 (branch `going_native`)
- **Shared Environment**: Active via `FRAMEPILER_TEST_ENV` for isolated transpiler/debugger development
- **Test Infrastructure**: Complete separation - transpiler only provides framec, tests in shared environment
- **Test Status**: 100% success for all PRT languages (Python, Rust, TypeScript)
- **Latest Achievements**: All test infrastructure moved to shared environment, 100% PRT test success
- **Recent Focus**: Completed Stage 4 of V3 migration - full architectural separation

## Test Infrastructure (IMPORTANT - READ GETTING_STARTED.md)
- **All tests moved to shared environment** - No test infrastructure in transpiler
- **Docker Test Runner**: Pure Rust binary at `framepiler_test_env/framepiler/docker/target/release/frame-docker-runner`
- **Test location**: `framepiler_test_env/common/test-frames/v3/` (607 tests)
- **Quick test run**:
  ```bash
  export FRAMEPILER_TEST_ENV=$(pwd)/framepiler_test_env
  framepiler_test_env/framepiler/docker/target/release/frame-docker-runner \
    python_3 v3_data_types --framec ./target/release/framec
  ```
- **No scripts needed** - The Docker runner is a self-contained Rust binary
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
