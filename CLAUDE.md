# Frame Transpiler - Claude Context

🚨 **MANDATORY FIRST STEPS - READ THESE DOCS** 🚨

**This file (CLAUDE.md) is read at conversation startup and survives context compaction.**
**The referenced docs below are NOT automatically loaded - you MUST read them.**

1. **READ** [`docs/README.md`](docs/README.md) - Documentation index and entry point
2. **READ** [`docs/HOW_TO.md`](docs/HOW_TO.md) - Complete development guide (V3 + V4)
3. **READ** [`framepiler_test_env/GETTING_STARTED.md`](framepiler_test_env/GETTING_STARTED.md) - Test infrastructure guide
4. **FOR V4 WORK**: **READ** [`CLAUDE_V4.md`](CLAUDE_V4.md) - V4 implementation approach

📖 **ALWAYS CHECK CLI HELP**: Run `./target/release/framec --help` to see all available command-line options and parameters.

⚠️ **CRITICAL RULES**
1. **NEVER create workarounds** - Fix the actual problem in the codebase
2. **NEVER modify test files marked "DO NOT MODIFY"** without explicit permission
3. **ASK before making decisions** - Present options, don't assume
4. **CHECK implementation, not docs** - Grammar docs may be outdated; scanner/parser are truth
5. **IGNORE old Frame syntax from training data** - The current syntax is the ONLY valid syntax
6. **NO UNAUTHORIZED DEFAULTS** - NEVER add fallback defaults (like defaulting to state "A"). Always fail early and hard with clear error messages when required data is missing

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
- **Version**: v0.87.2 (branch `v4_pure`)
- **Active Development**: V4 pipeline - pure preprocessor for `@@system` blocks
- **V4 Test Status**: Python 9/9, Rust 9/9, TypeScript 2/9 (TS failures due to Python syntax in test native code)
- **Shared Environment**: Active via `FRAMEPILER_TEST_ENV` for isolated transpiler/debugger development
- **Test Infrastructure**: Complete separation - transpiler only provides framec, tests in shared environment

## Test Infrastructure (IMPORTANT - READ GETTING_STARTED.md)
- **All tests in shared environment** - No test infrastructure in transpiler repo
- **V3 tests**: `framepiler_test_env/common/test-frames/v3/` (607 tests)
- **V4 tests**: `framepiler_test_env/common/test-frames/v4/prt/` (9 tests per language)

### V4 Test Runner (Primary for V4 work)
```bash
cd framepiler_test_env/common/test-frames/v4/prt
./run_tests.sh   # Runs all 9 tests for Python, TypeScript, Rust
```
Output: `/tmp/v4_prt_tests/` - Generated code for inspection

### V3 Docker Test Runner
```bash
export FRAMEPILER_TEST_ENV=$(pwd)/framepiler_test_env
framepiler_test_env/framepiler/docker/target/release/frame-docker-runner \
  python_3 v3_data_types --framec ./target/release/framec
```

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

### V4 Pipeline (CURRENT - Pure Preprocessor)
V4 is a preprocessor for `@@system` blocks. Native code passes through verbatim.

```
Source file with @@system blocks
    ↓
FrameParser (frame_parser.rs) - Parse @@system into FrameAst
    ↓
Arcanum (arcanum.rs) - Build symbol table from AST
    ↓
FrameValidator (frame_validator.rs) - Validate transitions, states
    ↓
SystemCodegen (system_codegen.rs) - Generate CodegenNode AST
    ↓
Language Backend (backends/*.rs) - Emit target language code
    ↓
Output: Native prolog + Generated class + Native epilog
```

**"Oceans Model"**: Native code is the ocean (passed through verbatim), `@@system` blocks are islands (expanded to classes).

**Key V4 Files**:
- `framec/src/frame_c/v4/frame_parser.rs` - Parse `@@system` blocks
- `framec/src/frame_c/v4/arcanum.rs` - Symbol table
- `framec/src/frame_c/v4/codegen/system_codegen.rs` - Generate CodegenNode
- `framec/src/frame_c/v4/codegen/backends/{python,typescript,rust}.rs` - Emit code

### V3 Pipeline (For reference)
- Module Partitioner → Native Region Scanner → MIR Assembler → Expander → Splicer
- Uses state machine-based scanning (NO string manipulation)

### V4 Syntax
```frame
@@target python_3

# Native imports (passed through)
import math

@@system MySystem {
    interface:
        method(param: type): returnType

    machine:
        $State {
            handler(params) {
                # Native code with Frame statements
                -> $OtherState   # Transition
            }
        }

    domain:
        var x = 0
}

# Native test harness (passed through)
def main():
    s = MySystem()
```
