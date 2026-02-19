# Frame V4 Implementation Plan

**Version:** 1.2
**Date:** February 2026
**Status:** Active Development
**Approach:** Test-Driven (PRT: Python, Rust, TypeScript)

---

## Test Audit Summary

**Existing:** 9 core tests + 365 migrated tests
**Required:** 24 tests per codegen spec
**Gap:** 8 tests missing, 3 need dedicated versions

---

## Phase 0: Baseline Validation

**Status:** COMPLETE

**Goal:** Verify existing tests pass for all PRT languages

**Results:**
- Python: 9/9 tests pass
- TypeScript: 7/7 tests pass
- Rust: 9/9 tests pass (created 2026-02-15)

**Rust Implementation Notes:**
- Added match-based event dispatch for interface methods
- Added match-based dispatch for enter/exit handlers
- Added V4 syntax support to native region scanner (`push$`, `-> pop$`)
- Fixed handler return types and tail expressions
- Fixed state stack boxing/unboxing for `Any` type

**Acceptance:** PASSED - All core tests pass for all 3 languages

---

## Phase 1: State Variables (`$.varName`)

**Status:** NOT IMPLEMENTED

### 1.1 Write Tests First

| Test File | Validates |
|-----------|-----------|
| `10_state_var_basic.frm` | `$.var` declaration, init, read, write |
| `11_state_var_reentry.frm` | `$.var` reinitialized when state re-entered |
| `12_state_var_push_pop.frm` | `$.var` preserved across push$/pop$ |

**Test template:**
```frame
@@target python_3

@@system StateVarTest {
    interface:
        test(): bool

    machine:
        $Start {
            $.counter: int = 0

            test(): bool {
                $.counter = $.counter + 1
                return $.counter == 1
            }
        }

    domain:
        var result = False
}

if __name__ == '__main__':
    t = StateVarTest()
    if t.test():
        print("PASS")
    else:
        print("FAIL")
        raise AssertionError()
```

### 1.2 Implementation Tasks

1. Add `StateVarAst` to `frame_ast.rs`
2. Parse `$.varName: type = init` in `frame_parser.rs`
3. Add `$.varName` to `native_region_scanner.rs`
4. Expand to `self.__compartment.state_vars["varName"]` in splicer
5. Generate state var init in compartment creation
6. Add E420 validation (duplicate state var)

### 1.3 Verify

```bash
frame-docker-runner python_3 10_state_var_basic
frame-docker-runner python_3 11_state_var_reentry
frame-docker-runner python_3 12_state_var_push_pop
# Repeat for typescript and rust
```

---

## Phase 2: System Return

**Status:** PARTIALLY IMPLEMENTED (validation only)

### 2.1 Write Tests First

| Test File | Validates |
|-----------|-----------|
| `13_system_return_basic.frm` | `system.return = value` sets interface return |
| `14_system_return_default.frm` | Default return when handler doesn't set |
| `15_system_return_chain.frm` | Last writer wins across transitions |
| `16_system_return_reentrant.frm` | Nested interface calls maintain separate returns |

### 2.2 Implementation Tasks

1. Parse default returns: `method(): type = default`
2. Generate return stack field
3. Generate push (with default) in interface methods
4. Generate pop and return in interface methods
5. Expand `system.return = expr` in splicer
6. Expand `return expr` sugar in handlers (not actions)

### 2.3 Verify

All 4 tests pass for Python, TypeScript, Rust

---

## Phase 3: Extended Transitions

**Status:** PARTIAL

### 3.1 Write Tests First

| Test File | Validates |
|-----------|-----------|
| `17_transition_enter_args.frm` | `-> (args) $State` passes to enter handler |
| `18_transition_exit_args.frm` | `(args) -> $State` passes to exit handler |
| `19_event_forwarding.frm` | `-> => $State` forwards current event |
| `20_transition_pop.frm` | `-> pop$` transitions with lifecycle |

### 3.2 Implementation Tasks

1. Parse `-> pop$` in `frame_statement_parser.rs`
2. Add `TransitionTarget::Pop` to AST
3. Generate pop transition in splicer
4. Test enter/exit arg passing (may already work)
5. Test event forwarding (may already work)

### 3.3 Verify

All 4 tests pass for PRT

---

## Phase 4: @@codegen Directive

**Status:** NOT IMPLEMENTED

### Purpose

The `@@codegen` directive allows users to explicitly request FrameEvent class generation. This is the ONLY user-controllable codegen option. State stack generation is internal compiler logic and not user-configurable.

### Syntax

```frame
@@target python_3

@@codegen {
    frame_event: on
}

@@system MySystem { ... }
```

### 4.1 Write Tests First

| Test File | Validates |
|-----------|-----------|
| `26_codegen_frame_event.frm` | `frame_event: on` generates FrameEvent class |

### 4.2 Implementation Tasks

1. Parse `@@codegen { frame_event: on|off }` after `@@target`
2. Add `codegen_config` field to `FrameAst`
3. Generate FrameEvent class when `frame_event: on` or auto-enabled
4. Auto-enable `frame_event` when spec requires it:
   - Enter/exit args on transitions
   - Event forwarding (`-> =>`)
   - `system.return` usage
   - Interface methods with return values
5. Generate W401 warning when auto-enable overrides explicit `off`

### 4.3 Verify

Test passes for PRT

---

## Phase 5: Static Operations

**Status:** NOT IMPLEMENTED

### 5.1 Write Tests First

| Test File | Validates |
|-----------|-----------|
| `22_static_operations.frm` | `static op()` has no self/this |

### 5.2 Implementation Tasks

1. Parse `static` keyword in operations
2. Validate no instance access
3. Generate `@staticmethod` / `static`

### 5.3 Verify

Test passes for PRT

---

## Phase 6: Persistence

**Status:** NOT IMPLEMENTED

### 6.1 Write Tests First

| Test File | Validates |
|-----------|-----------|
| `23_persist_basic.frm` | `@@persist` generates save/restore |
| `24_persist_roundtrip.frm` | Save → restore preserves state + domain |

### 6.2 Implementation Tasks

1. Generate `_save()` method
2. Generate `_restore(data)` class method
3. Handle state stack serialization
4. Handle field filtering (`domain=[...]`, `exclude=[...]`)

### 6.3 Verify

Tests pass for PRT

---

## Phase 7: Service Pattern

**Status:** NOT TESTED

### 7.1 Write Tests First

| Test File | Validates |
|-----------|-----------|
| `25_service_pattern.frm` | Enter handler chains don't stack overflow |

### 7.2 Implementation Tasks

Kernel already handles this. Just need test validation.

### 7.3 Verify

Test passes for PRT

---

## Test Summary

| # | Test File | Phase | Feature |
|---|-----------|-------|---------|
| 10 | `10_state_var_basic.frm` | 1 | State variables |
| 11 | `11_state_var_reentry.frm` | 1 | State var reentry |
| 12 | `12_state_var_push_pop.frm` | 1 | State var + stack |
| 13 | `13_system_return_basic.frm` | 2 | system.return |
| 14 | `14_system_return_default.frm` | 2 | Default returns |
| 15 | `15_system_return_chain.frm` | 2 | Return chaining |
| 16 | `16_system_return_reentrant.frm` | 2 | Return reentrancy |
| 17 | `17_transition_enter_args.frm` | 3 | Enter args |
| 18 | `18_transition_exit_args.frm` | 3 | Exit args |
| 19 | `19_event_forwarding.frm` | 3 | `-> =>` |
| 20 | `20_transition_pop.frm` | 3 | `-> pop$` |
| 21 | `21_codegen_auto_enable.frm` | 4 | @@codegen |
| 22 | `22_static_operations.frm` | 5 | Static ops |
| 23 | `23_persist_basic.frm` | 6 | @@persist |
| 24 | `24_persist_roundtrip.frm` | 6 | Persistence |
| 25 | `25_service_pattern.frm` | 7 | Enter chains |

---

## Validation Commands

**Single test:**
```bash
frame-docker-runner python_3 10_state_var_basic --framec ./target/release/framec
```

**All tests:**
```bash
for lang in python_3 typescript rust; do
    frame-docker-runner $lang --all --framec ./target/release/framec
done
```

---

## Success Criteria

| Phase | Tests | Must Pass |
|-------|-------|-----------|
| 0 | 01-09 | All PRT |
| 1 | 10-12 | All PRT |
| 2 | 13-16 | All PRT |
| 3 | 17-20 | All PRT |
| 4 | 21 | All PRT |
| 5 | 22 | All PRT |
| 6 | 23-24 | All PRT |
| 7 | 25 | All PRT |

**Final:** 25 tests × 3 languages = 75 test passes
