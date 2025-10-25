# AI Planning and Communication Document

**Last Updated**: October 24, 2025  
**Current Version**: v0.86.20  
**Purpose**: Communication channel between AI sessions working on the Frame transpiler

## Current Status

### Test Statistics
- **Python Execution**: 100.0% (458/458 tests passing) 🎉
- **TypeScript Execution (overall)**: 82.5% (354/429 passing; full regression sweep still pending post-runtime overhaul)
- **TypeScript data_types suite**: Transpile 100% (66/66) • Execution 100% (66/66) after runtime + visitor shims
- **Known TS Failures**: Remaining gaps now outside the data_types suite (multifile orchestration, debugger-oriented scenarios, legacy edge cases)
- **Total Runtime Failures**: 0 (Python) • 71 (TypeScript execution gaps tracked across suites)

### Recent Achievements (v0.86.x)
- ✅ Unified FrameDict runtime usage across dict comprehensions, union, and constructor paths (v0.86.19)
- ✅ Added comprehensive TypeScript string/list/dict runtime shims (format/count/rfind/OrderedDict/defaultdict/etc.), bringing `data_types` execution to 100% (v0.86.20)
- ✅ Unified async/await behavior via embedded TypeScript runtime support
- ✅ Maintained 100% Python execution coverage while expanding async capabilities
- ✅ Stabilized multi-target compilation pipeline post TypeScript visitor fixes

## Known Issues

### TypeScript Execution Gap
**Status**: Open — 82.5% TypeScript execution success (354/429)
**Focus**: Address remaining multifile orchestration and capability-driven scenarios needed for debugger controller support
**Quality**: Python production ready; TypeScript execution approaching debugger-ready but still needs targeted fixes

### Recent Resolutions (v0.85.x)
- **Legacy Interface Callbacks**: Updated tests to use `system.interfaceMethod()` syntax
- **Source Mapping Classification (Bug #35)**: AST-based mapping types now stable
- **Method Resolution Validation**: Prefix semantics enforced with conflict detection

## Active Development Areas

### 1. TypeScript Execution Stabilization (Priority: HIGH)
Target the remaining ~70 TypeScript execution failures (multifile orchestration + capability APIs). Track uplift toward ≥90% execution success while preserving Python parity and debugger readiness.

### 2. Source Mapping (Status: COMPLETE)
Source mapping is functionally complete with ~50-70% coverage of user code. All critical constructs are mapped.

### 3. Multi-File Support (Status: STABLE)
Multi-file module system is working correctly with proper import/export mechanisms.

### 4. Interface Method Calls (Status: COMPLETE v0.81.2)
System interface method calls now fully supported with `system.interfaceMethod()` syntax:
- Scanner tokenizes `system.methodName` patterns
- Parser validates interface method existence (in second pass)
- Visitor generates correct Python code (`self.methodName`)
- Comprehensive error messages for invalid usage patterns

## Architecture Notes

### PythonVisitorV2 (Default)
- Uses CodeBuilder architecture for automatic line tracking
- Handles all Python 3 features including async/await
- Source mapping integrated throughout

### Call Chain Resolution
The visitor distinguishes between:
- **Local calls**: Need `self.` prefix for operations/actions
- **Qualified calls**: System.method() should NOT have `self.` prefix
- **Static methods**: Must use @staticmethod decorator

### Key Code Locations
- Parser: `framec/src/frame_c/parser.rs`
- Python Visitor: `framec/src/frame_c/visitors/python_visitor_v2.rs`
- AST Definitions: `framec/src/frame_c/ast.rs`
- Test Runner: `framec_tests/runner/frame_test_runner.py`

## Testing Infrastructure

### Running Tests
```bash
# Full test suite
python3 framec_tests/runner/frame_test_runner.py --all --framec ./target/release/framec

# Specific pattern
python3 framec_tests/runner/frame_test_runner.py "test_static*.frm" --framec ./target/release/framec

# With verbose output
python3 framec_tests/runner/frame_test_runner.py --all --verbose --framec ./target/release/framec
```

### Test Categories
- **Positive Tests**: All Python execution suites green; TypeScript positives continue to surface async/multifile gaps
- **Negative Tests**: Validation suites passing (expected errors consistently caught)
- **Build Exclusions**: Legacy backtick specs, legacy negative suites, and known long-tail parser edge cases

## Next AI Session Guidance

### If Working on Parser Bug:
1. Look at `parser.rs` around symbol table construction
2. Focus on state machine transitions between system and module parsing
3. Test with minimal reproduction: system followed by function

### If Adding New Features:
1. Update grammar.md first
2. Add AST nodes if needed
3. Implement in PythonVisitorV2
4. Add comprehensive tests
5. Update AI documentation

### If Fixing Bugs:
1. Check open_bugs.md for known issues
2. Use test runner to verify fixes
3. Update CHANGELOG.md
4. Bump version appropriately (patch for fixes)

## Communication Protocol

When starting a new session:
1. Read this document first
2. Check git status and recent commits
3. Run test suite to verify current state
4. Update this document before ending session

## Source Map Validation System (NEW v0.79.0)

### Validation Tools
- **`/tools/source_map_validator.py`**: Core analysis for individual files
- **`/tools/source_map_test_integration.py`**: Batch validation for test suites
- **`/tools/test_framework_integration.py`**: VS Code extension interface
- **`/tools/source_map_config.json`**: Quality standards configuration

### VS Code Extension AI Integration
**CRITICAL**: Extension AI should use transpiler validation tools as source of truth:

```typescript
// Extension AI should call transpiler tools directly
const validation = await exec('python3 tools/test_framework_integration.py --file frameFile.frm');
const quality = JSON.parse(validation.stdout);

// AI interprets results for users
if (quality.classification === 'EXCELLENT') {
    return "Perfect debugging experience - set breakpoints anywhere";
} else if (quality.duplicates > 5) {
    return "Some duplicate mappings detected - stepping may be choppy in complex expressions";
}
```

### Quality Status (v0.79.0)
- **Overall Assessment**: GOOD (76.8% test files pass validation)
- **Executable Coverage**: 100% for main functions
- **Duplicate Mappings**: 683 total (mostly acceptable minor patterns)
- **Bug #27**: Active but classified as minor (functional debugging)

## Version Management

Current: v0.86.16 - **AUTOMATED SYSTEM**
- Major.Minor.Patch
- Patch: Bug fixes only
- Minor: New features, improvements
- Major: Breaking changes (only project owner decides)

**NEW**: Single source of truth system:
1. Update `[workspace.package].version` in the root `Cargo.toml`.
2. Run `./scripts/sync-versions.sh` to refresh `version.toml`.
3. Version strings in the compiler and emitted code come from `CARGO_PKG_VERSION` automatically.
4. Keep changelog entries and this document aligned with the new version.

## v0.86.16 Update Snapshot (October 22, 2025)

### Highlights
- **Python**: 458/458 execution tests passing (100% sustained)
- **TypeScript**: 82.5% execution success (354/429) with emphasis on multifile orchestration and capability coverage
- **Roadmap Alignment**: Testing architecture roadmap extended to support debugger controller validation

### Key Technical Themes
1. **Debugger Controller Readiness**: Establish runtime and testing requirements for TypeScript debugger integration
2. **Regression Guardrails**: Maintain 100% Python coverage while scaling TypeScript execution scenarios
3. **Targeted Capability Work**: Focus on process spawning, networking, and async workflows needed by the controller

### Next Priorities
- Close remaining 75 TypeScript execution failures and raise success rate >90%
- Implement missing capability APIs required by debugger workflows (spawn, TCP, filesystem)
- Preserve deterministic test runner behavior as controller-focused suites expand
