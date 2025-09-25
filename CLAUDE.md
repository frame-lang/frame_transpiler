# Frame Transpiler - Claude Context

⚠️ **CRITICAL RULES**
1. **NEVER create workarounds** - Fix the actual problem in the codebase
2. **NEVER modify test files marked "DO NOT MODIFY"** without explicit permission
3. **ASK before making decisions** - Present options, don't assume
4. **CHECK implementation, not docs** - Grammar docs may be outdated; scanner/parser are truth

## Current State
- **Version**: v0.76.1, Branch: v0.30, **99.5% tests passing** (365/367)
- **PythonVisitorV2**: Default with CodeBuilder architecture
- **Recent Achievements**: FSL removed, native Python operations, critical bug fixes

## Quick References
- **Test files**: `/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/`  
- **Test runner**: `python3 framec_tests/runner/frame_test_runner.py --all --framec ./target/release/framec`
- **Module separator**: `::` (NOT `.` - dot is for member access)
- **Check before starting**: Read `docs/framelang_design/dev_notes.md` and `framec_tests/reports/test_log.md`

## When Tests Fail
1. Investigate root cause (don't assume test is wrong)
2. Check scanner/parser for actual syntax
3. ASK: "Should I fix X in visitor or is this a test issue?"

## Architecture
- Scanner → Parser (2-pass) → AST → Visitor → Target Code
- Key files: `scanner.rs`, `parser.rs`, `ast.rs`, `python_visitor_v2.rs`