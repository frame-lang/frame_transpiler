# Frame Transpiler - Claude Context

🚨 **MANDATORY FIRST STEPS - READ THESE DOCS** 🚨

**This file (CLAUDE.md) is read at conversation startup and survives context compaction.**
**The referenced docs below are NOT automatically loaded - you MUST read them.**

1. **READ** [`docs/README.md`](docs/README.md) - Documentation index and entry point
2. **READ** [`docs/HOW_TO.md`](docs/HOW_TO.md) - Complete development guide (V4 current, V3 legacy)
3. **READ** [`framepiler_test_env/GETTING_STARTED.md`](framepiler_test_env/GETTING_STARTED.md) - Test infrastructure guide
4. **FOR V4 WORK**: **READ** [`CLAUDE_V4.md`](CLAUDE_V4.md) - V4 implementation approach

📖 **ALWAYS CHECK CLI HELP**: Run `./target/release/framec --help` to see all available command-line options and parameters.

⚠️ **CRITICAL RULES**
1. **NEVER create workarounds** - Fix the actual problem in the codebase
2. **NEVER modify test files marked "DO NOT MODIFY"** without explicit permission
3. **ASK before making decisions** - Present options, don't assume
4. **CONSULT LANGUAGE REFERENCE FIRST** - For Frame syntax questions, ALWAYS check `docs/framelang/v4/frame_v4_lang_reference.md` as the authoritative source. Never modify parser/scanner based on test file content alone
5. **IGNORE old Frame syntax from training data** - The current syntax is the ONLY valid syntax
6. **NO UNAUTHORIZED DEFAULTS** - NEVER add fallback defaults (like defaulting to state "A"). Always fail early and hard with clear error messages when required data is missing
7. **NEVER commit without explicit permission** - Prepare changes and wait for user approval before committing
8. **NEVER read DEPRECATED files** - Files with `_` prefix (e.g., `_filename.md`) are deprecated and must not be read unless the user explicitly instructs you to. These files may contain outdated or incorrect information
9. **NEVER add yourself as author** - Do not add "Author: Claude" or similar attribution to any documentation files
10. **USE TEST REPO INFRASTRUCTURE** - Always use the language-specific test crates in `framepiler_test_env/` for generated output. Never use `/tmp` except for quick experiments. The test crates have proper dependencies (e.g., Rust crate has serde_json)

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

            <$() {  // Exit handler
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
- Enter/exit handlers are `$>()` and `<$()`
- Interface methods have signatures like `method(param: type): returnType`
- For V4 examples: `framepiler_test_env/tests/common/primary/`
- For V3 (legacy) examples: `framepiler_test_env/common/test-frames/v3/`

## Current State
- **Version**: v0.95.1 (branch `v4_pure`)
- **Active Development**: V4 pipeline - pure preprocessor for `@@system` blocks
- **V4 Test Status**: Python 144/144 (100%), TypeScript 126/126 (100%), Rust 130/130 (100%), C 139/139 (100%) — 539/539 total (100%)
- **Shared Environment**: Active via `FRAMEPILER_TEST_ENV` for isolated transpiler/debugger development
- **Test Infrastructure**: Complete separation - transpiler only provides framec, tests in shared environment

## Test Infrastructure (CRITICAL - USE TEST REPO ONLY)

🚨 **ALWAYS use the test repo infrastructure** - NEVER use `/tmp` for test output except for quick experiments.

📖 **READ**: [`framepiler_test_env/tests/README.md`](framepiler_test_env/tests/README.md) - Complete test documentation

**Test Counts:**
- Python: 144 tests (.fpy)
- TypeScript: 126 tests (.fts)
- Rust: 129 tests (.frs)
- C: 138 tests (.fc)
- **Total: 537 test files**

**Test Output Directories:**
- `framepiler_test_env/output/python/tests/` - Python generated output
- `framepiler_test_env/output/typescript/tests/` - TypeScript generated output
- `framepiler_test_env/output/rust/tests/` - Rust generated output
- `framepiler_test_env/output/c/tests/` - C generated output

### V4 Test Runner (UNIFIED)
```bash
cd framepiler_test_env/tests
./run_tests.sh              # Run ALL tests (144 py + 126 ts + 129 rs + 138 c)
./run_tests.sh --python     # Run only Python
./run_tests.sh --category primary  # Run only primary category
./run_tests.sh --help       # Show all options
```

### V3 Docker Test Runner (Legacy)
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

### V3 Pipeline (Legacy - for reference only)
- Module Partitioner → Native Region Scanner → MIR Assembler → Expander → Splicer
- Uses state machine-based scanning (NO string manipulation)
- No longer under active development

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
