# AI Planning and Communication Document

**Last Updated**: October 28, 2025  
**Current Version**: v0.86.25  
**Purpose**: Communication channel between AI sessions working on the Frame transpiler

## Current Status

### Test Statistics
- **Python Execution**: 100.0% (462/462 tests passing) 🎉
- **TypeScript Execution**: 100.0% (433/433 tests passing) 🎉
- **LLVM Smoke Suite**: 10/10 (`language_specific_llvm`) ✅
- **Negative Suite**: 14/14 (includes new nested-function regression guard)
- **Aggregate**: 905 specs (common + language-specific + LLVM smoke) executed successfully across all active targets

### Recent Achievements (v0.86.25)
- ✅ LLVM runtime exposes `frame_runtime_compartment_set_forward_event`, enabling queued parent/enter/exit forwarding work.
- ✅ Builder refactor hoists compartment pointers so transitions reuse kernel-controlled handles, eliminating stale pointers before queue wiring lands.
- ✅ Added `basic/test_action_locals.frm` to stretch action locals, typed counters, and inferred strings in the LLVM suite; smoke coverage now spans nine fixtures.
- ✅ Documentation (README, HOW_TO, project status, planning) updated for v0.86.25, highlighting the native backend focus and Mac-first rollout expectations.

## Known Issues

### Frame Standard Library Cohesion
**Status**: Open — capability modules exist for networking/process/timers, but we still need a documented, language-agnostic FSL surface (import paths, naming, optional facilities) to keep future targets consistent.
**Focus**: Finalize module naming, document required behaviours per capability, and ensure both Python/TypeScript visitors emit deterministic imports.

### Advanced Syntax Backlog
**Status**: Deferred — walrus operator lowering, generator expressions (`yield`, `yield from`), richer exception propagation, and extended dunder coverage remain unimplemented even though current specs avoid them.
**Focus**: Stage implementation plans so we can enable the remaining v0.56 syntax without regressing existing parity.

### Roadmap Documentation Drift
**Status**: Monitoring — with Python/TypeScript execution gaps closed and LLVM smoke now automated, roadmap milestones must be re-scoped around FSL expansion, new targets (C++/Rust), and debugger tooling rather than “get TypeScript to 90%”.

## Active Development Areas

### 1. Frame Standard Library Alignment (Priority: HIGH)
- Define the Frame Standard Library (FSL) API surface (network/process/filesystem/timers).
- Implement or stub FSL modules per target while keeping the FrameRuntime focused on language semantics.
- Migrate language-specific tests to consume FSL modules instead of raw platform APIs.
- Document contribution guidance so new capabilities land in the FSL rather than ad-hoc visitor/runtime code.

### 2. Advanced Syntax Enablement
- Walrus operator lowering into assignment + conditionals for TypeScript visitor.
- Generator (`yield`, `yield from`) semantics and runtime helpers across targets.
- Raise/exception pipeline: emit proper `throw` expressions for `raise error` and preserve exception values.
- Remaining dunder coverage: in-place ops (`__iadd__`, …), ordering comparisons, `__getitem__/__setitem__`, hashing.
- Complex numbers, numeric underscore literals, and rich numeric formatting parity across targets (Python already supports; TypeScript runtime groundwork partially landed).

### 3. Debugger & Tooling Roadmap
- Update debugger-controller requirements now that both runtimes execute cleanly.
- Ensure source-map validation tooling stays compatible with new async output.
- Coordinate documentation updates (HOW_TO, roadmap, capability guides) each time we touch runtime semantics.

### 4. LLVM Backend Expansion
- Parent dispatch now re-invokes parent handlers via direct branching; forwarded-event/enter-exit wiring for the queue remains TODO (current kernel short-circuits after handler execution).
- Core LLVM backend split into `builder.rs`, `context.rs`, `utils.rs`, `value.rs`, and `visitor.rs` so new targets can share infrastructure without bloated modules.
- Domain value helpers now live in `value.rs` with dead variants removed; keep an eye on coercion paths before widening type coverage.
- Add automated coverage for domain mutations, transitions, and action locals beyond the initial basic suite.
- Monitor GitHub Actions LLVM smoke job (now integrated) and extend the suite as runtime features land.

#### LLVM Backend Status
| Area | Status | Notes |
| --- | --- | --- |
| Module decomposition (`builder/context/utils/value/visitor`) | ✅ Complete | `framec/src/frame_c/llvm/mod.rs` now exposes the visitor directly and compiler call sites use it; the legacy visitors shim is gone. |
| Domain value helpers cleanup | ✅ Complete | `DomainFieldInit::None`, `coerce_value_for_field`, and redundant init helpers removed; warning noise eliminated ahead of new typing work. |
| Runtime enter/exit queue semantics | 🔄 In progress | Parent dispatch now enqueues the current message via `frame_runtime_compartment_set_forward_event` and routes through the kernel queue; enter/exit forwarding remains pending. |
| LLVM coverage for hierarchy + mutations | 🔄 In progress | `basic/test_parent_hierarchy.frm` exercises multi-level parents; `basic/test_action_locals.frm` now pokes typed/untyped domain mutations via action locals. Still need queue-forward stress once runtime wiring lands. |

## Type System Direction
- Frame stays **gradually typed**: typed declarations enforce static guarantees while untyped declarations remain dynamic for rapid prototyping.
- Mixed typed/untyped domain variables are supported; typed slots must receive compatible literals or expressions, while untyped slots accept any runtime value (mirrors TypeScript's `any` mixed with explicit types).
- This approach is common in gradually typed ecosystems, but we must add backend safeguards (LLVM defaults to `CString` today) so strongly typed targets reject mismatched assignments instead of silently coercing.
- Next steps: extend `ValueKind` beyond the current four primitives, design a runtime-agnostic coercion story, and document MacOS-specific interoperability expectations before widening target support.

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
- **Positive Tests**: Python, TypeScript, and LLVM smoke suites all green (900 total specs)
- **Negative Tests**: Validation suites passing (expected errors consistently caught)
- **Build Exclusions**: Legacy backtick specs, legacy negative suites, and known long-tail parser edge cases

## Next AI Session Guidance

### If Working on Parser Bug:
1. Look at `parser.rs` around symbol table construction
2. Focus on state machine transitions between system and module parsing
3. Test with minimal reproduction: system followed by function

### If Adding New Features:
1. Define the portable API in the Frame Standard Library (FSL) and update documentation.
2. Implement the feature in each target's FSL module (even if some start as TODOs).
3. Update grammar.md only if the language surface changes.
4. Add visitor/runtime glue as needed, keeping FrameRuntime focused on semantics.
5. Add comprehensive tests and update documentation.

### If Fixing Bugs:
1. Check open_bugs.md for known issues
2. Use test runner to verify fixes (include `--languages llvm --categories language_specific_llvm` when touching the native backend)
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

Current: v0.86.22 - **AUTOMATED SYSTEM**
- Major.Minor.Patch
- Patch: Bug fixes only
- Minor: New features, improvements
- Major: Breaking changes (only project owner decides)

**Reminder**: Single source of truth system:
1. Update `[workspace.package].version` in the root `Cargo.toml`.
2. Run `./scripts/sync-versions.sh` to refresh `version.toml`.
3. Version strings in the compiler and emitted code come from `CARGO_PKG_VERSION` automatically.
4. Keep changelog entries and this document aligned with the new version.

## v0.86.22 Update Snapshot (October 28, 2025)

### Highlights
- **Python**: 462/462 execution tests passing (language-specific fixtures align with action helpers)
- **TypeScript**: 433/433 execution tests passing (async runtime parity + capability shims in place)
- **LLVM Smoke**: 5/5 (unified runner compiles/links `language_specific/llvm` fixtures against `frame_runtime_llvm`)
- **Negative Coverage**: Added nested-function regression guard to enforce parser constraints
- **Roadmap Alignment**: Documentation/roadmap refocused on FSL definition, advanced syntax enablement, and debugger tooling

### Key Technical Themes
1. **Cross-Language Runtime Parity**: Async detection + kernel updates keep Python and TypeScript output structurally aligned.
2. **Capability Standardization**: Move network/process/timer helpers into documented FSL modules for future targets.
3. **Forward-Looking Syntax Work**: Walrus, generators, and richer dunder support remain staged as next feature tranche.
4. **Native Backend Validation**: LLVM backend executes smoke fixtures end-to-end via shared runtime crate and automated linking.

### Next Priorities
- Publish a draft FSL spec outlining required modules and behaviors per target
- Prototype walrus lowering and generator support without regressing existing suites
- Keep roadmap/docs synchronized whenever runtime semantics evolve
- Expand LLVM coverage (enter/exit handlers, richer dispatch) once runtime kernel dispatch matures; integrate suite into CI when stable
