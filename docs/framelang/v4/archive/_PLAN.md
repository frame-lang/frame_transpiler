> **⚠️ DEPRECATED - DO NOT READ UNLESS INSTRUCTED ⚠️**
>
> This document is archived and may contain outdated or incorrect information about Frame syntax.
> For current Frame V4 syntax, see `frame_v4_lang_reference.md` in the parent directory.

---

# Frame V4 Execution Plan

**Status:** Active Development
**Last Updated:** 2026-02-13
**Prerequisites:** V4 is already default with working PRT backends

---

## Current State Assessment

### What's Complete

| Component | Status | Notes |
|-----------|--------|-------|
| V4 as default | DONE | No `FRAME_USE_V4` needed |
| Frame Parser | DONE | 55KB, handles all syntax |
| Arcanum Symbol Table | DONE | 28KB, scope resolution working |
| Python Backend | DONE | Full featured, tested |
| Rust Backend | DONE | Full featured, tested |
| TypeScript Backend | DONE | Full featured, tested |
| Native Code Preservation | DONE | Oceans model working |
| Splicer | DONE | Combines native + generated |

### What's Partially Complete

| Component | Status | Gap |
|-----------|--------|-----|
| Validation | 14 codes | 25+ more planned |
| Source Maps | Design only | Not implemented |
| C#/Java/C/C++ Backends | Stubs | Low priority |

### Current Error Codes (14 total)

**Structural (E1xx):**
- E111: Duplicate system parameter
- E113: System blocks out of order
- E114: Duplicate section blocks
- E115: Multiple `fn main` functions

**Semantic (E4xx):**
- E400: Transition must be last statement
- E401: Frame statements not allowed in actions/operations
- E402: Unknown state in transition
- E403: Invalid parent forwards
- E404: Handler body must be inside state
- E405: State parameter arity mismatch
- E406: Invalid interface method calls
- E416: Start params must match start state
- E417: Enter handler params mismatch
- E418: Domain param has no matching variable

---

## Phase 1: Validation Expansion (Priority: HIGH)

**Goal:** Implement 25+ additional error codes

**Timeline:** 6 weeks

### Week 1: Foundation & Infrastructure

Tasks:
1. Create `validation/` module structure with `ValidationPass` trait
2. Implement `ValidationRunner` to orchestrate passes
3. Implement `ValidationReport` with output formats (human, JSON, IDE)
4. Migrate existing 14 codes to new system
5. Add CLI flags: `--warn-as-error`, `--suppress=E4XX`

Files to modify:
- `v4/frame_validator.rs` → refactor into modular passes
- `v4/pipeline/config.rs` → add validation config options
- `framec/src/frame_c/cli.rs` → add CLI flags

### Week 2: Structural Checks (E41x)

New error codes:
- E410: `unreachable-state` - No incoming transitions
- E411: `dead-end-state` - No outgoing transitions
- E412: `orphan-state` - Isolated state
- E413: `missing-start-state` - No start state
- E414: `empty-machine` - Machine has no states

Tasks:
1. Implement `TransitionGraph` builder from Arcanum
2. Implement `StructuralPass` with reachability analysis
3. Add tests in `test-frames/v4/validation/structural/`

### Week 3: Event Handling Checks (E42x)

New error codes:
- E420: `unhandled-interface-event` - Interface method not handled
- E421: `unhandled-event-in-state` - No handler, no parent
- E422: `dead-handler` - Handler never called
- E423: `interface-event-mismatch` - Signature mismatch
- E424: `shadowed-handler` - Child shadows parent (info)

Tasks:
1. Implement `EventPass` with interface/handler matching
2. Wire to Arcanum for cross-referencing
3. Add tests

### Week 4: Transition Checks (E43x)

New error codes:
- E430: `self-transition-no-effect` - No enter/exit handlers
- E431: (exists) Transition after transition
- E432: `conditional-transition-incomplete` - Some branches missing
- E433: `transition-in-enter-handler` - Potential infinite loop
- E434: `transition-target-params` - Missing required params

Tasks:
1. Implement `TransitionPass` with control flow analysis
2. Add basic branch analysis for conditionals
3. Add tests

### Week 5: HSM Checks (E44x)

New error codes:
- E440: `hsm-cycle` - Parent chain has cycle
- E441: `hsm-depth-exceeded` - Too deep (>10 levels)
- E442: `forward-no-parent` - `=> $^` without parent
- E443: `forward-unhandled` - Parent doesn't handle
- E444: `orphan-parent` - Parent doesn't exist

Tasks:
1. Implement `HsmPass` with parent chain resolution
2. Add cycle detection
3. Add tests

### Week 6: Stack & Domain Checks (E45x, E46x)

New error codes:
- E450: `pop-without-push` - Stack underflow
- E451: `push-without-pop` - Stack leak
- E452: `stack-in-enter-exit` - Dangerous pattern
- E460: `undefined-domain-var` - Reference to unknown var
- E461: `unused-domain-var` - Declared but unused

Tasks:
1. Implement `StackPass` with interprocedural analysis
2. Implement `DomainPass` for variable tracking
3. Add tests

---

## Phase 2: Source Maps (Priority: MEDIUM)

**Goal:** Enable Frame source debugging in VS Code

**Timeline:** 2 weeks

### Design Decisions

1. **Format:** Use JavaScript source map v3 format (industry standard)
2. **Granularity:** Map at statement level (not expression)
3. **Coverage:** Frame constructs only (native code maps to itself)

### Week 1: Implementation

Tasks:
1. Create `codegen/source_map.rs` module
2. Track spans during splicer operation
3. Generate source map JSON
4. Emit `.map` files alongside generated code

Data structure:
```rust
pub struct SourceMapEntry {
    pub generated_line: u32,
    pub generated_col: u32,
    pub source_line: u32,
    pub source_col: u32,
    pub name: Option<String>,
}

pub struct SourceMap {
    pub file: String,
    pub source_root: String,
    pub sources: Vec<String>,
    pub mappings: Vec<SourceMapEntry>,
}
```

### Week 2: Integration

Tasks:
1. Wire to CLI with `--source-map` flag
2. Test with VS Code debugger
3. Document usage

---

## Phase 3: Testing & Verification (Priority: HIGH)

**Goal:** Comprehensive test coverage for V4

**Timeline:** Ongoing

### Test Categories

1. **Validation Tests** (`test-frames/v4/validation/`)
   - Each error code gets 3+ test cases
   - Positive (error should fire) and negative (should pass)

2. **Backend Tests** (`test-frames/v4/prt/`)
   - Parallel tests for Python, Rust, TypeScript
   - Same Frame source → verify all backends produce runnable code

3. **Regression Tests** (`test-frames/v4/regression/`)
   - Fixed bugs should never reoccur

### Test Infrastructure

Location: `framepiler_test_env/common/test-frames/v4/`

Structure:
```
v4/
├── validation/
│   ├── structural/
│   ├── events/
│   ├── transitions/
│   ├── hsm/
│   └── stack_domain/
├── prt/
│   ├── basic_transition.frm
│   ├── interface_methods.frm
│   ├── enter_exit.frm
│   ├── hsm_forward.frm
│   └── stack_ops.frm
└── regression/
```

---

## Phase 4: Documentation (Priority: MEDIUM)

**Goal:** User-facing documentation for V4

### Documents to Create/Update

1. **Migration Guide** - V3 to V4 differences
2. **Error Reference** - All error codes with examples
3. **Language Reference** - Complete Frame syntax
4. **IDE Integration** - VS Code setup with source maps

---

## Success Criteria

| Metric | Target |
|--------|--------|
| Error codes | 40+ (from 14) |
| Test coverage | Each error code has 3+ tests |
| False positive rate | <1% on existing passing tests |
| Performance | Validation adds <10% to transpile time |
| PRT tests | 100% pass rate |
| Source maps | Work in VS Code debugger |

---

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| False positives annoy users | Conservative severity; easy `--suppress` |
| Control flow analysis complex | Start simple; iterate |
| Performance regression | Lazy evaluation; skip irrelevant passes |
| Breaking existing workflows | Warnings don't block; new errors are warnings first |

---

## Deferred (V5+)

The following are explicitly out of scope for V4:

1. **Native code analysis** - V5 will add optional cross-environment symbol resolution
2. **Type checking across boundaries** - V5 scope
3. **Multi-file projects** - V5 scope
4. **C#/Java/C/C++ backends** - Low priority, complete when needed

Future work will be documented in V5 planning documents.

---

## Getting Started

To begin work:

```bash
# Build transpiler
cargo build --release -p framec

# Run existing tests
cd framepiler_test_env
./framepiler/docker/target/release/frame-docker-runner python_3 v4_*

# Run validation-only
./target/release/framec test.frm --validate-only
```

---

*Document created: 2026-02-13*
