# Frame V5 Implementation Plan

**Version:** 1.0
**Date:** February 2026
**Status:** Planning
**Prerequisites:** V4 complete

---

## Overview

Frame V5 builds on V4's foundation to add advanced state management features that require careful design consideration.

---

## Feature 1: Push Named State (`push$ $StateName`)

**Status:** NOT IMPLEMENTED

### Description

Push a new compartment for a named state onto the state stack, rather than pushing the current state. This enables patterns like:
- Pre-configured fallback states
- State templates/prototypes
- Multi-state bookmarks

### Syntax

```frame
push$ $<StateName>              // Push new compartment for StateName
push$ $<StateName>(arg1, arg2)  // Push with state parameter initialization
```

### Semantic Requirements

1. **State variable initialization** - The pushed compartment must have its state variables initialized:
   - Default: Use the state's declared initializers
   - With args: Use provided arguments to initialize state parameters

2. **No implicit transition** - `push$ $State` only pushes to the stack; it does not change the current state.

3. **Pop behavior** - When popped with `-> pop$`, the system transitions to the pushed state with full lifecycle (exit current, enter pushed).

### Implementation Dependencies

This feature depends on:
- **V4 Phase 1: State Variables** - Must have `$.varName` fully working
- **State parameters** - Need syntax for state parameter declarations

### Design Questions

1. **State parameters vs state variables**
   - Should `push$ $State(x, y)` initialize state parameters or state variables?
   - How do state parameters differ from state variables in lifecycle?

2. **Compartment creation**
   - When is `$>()` (enter handler) called for the pushed state?
   - Option A: Never (just pre-initialize vars)
   - Option B: Immediately after push
   - Option C: When popped and transitioned to

3. **Type safety**
   - How to validate that push arguments match state parameter types?

### Test Cases

| Test File | Validates |
|-----------|-----------|
| `v5_01_push_named_basic.frm` | `push$ $State` creates new compartment |
| `v5_02_push_named_args.frm` | `push$ $State(args)` initializes parameters |
| `v5_03_push_named_vars.frm` | Pushed state has correct variable values |
| `v5_04_push_pop_roundtrip.frm` | Push named → pop → correct state restored |

### Implementation Tasks

1. Design state parameter syntax (`$State { param x: int = 0 }`)
2. Parse `push$ $StateName` in `frame_statement_parser.rs`
3. Parse `push$ $StateName(args)` with argument list
4. Generate compartment creation with state var/param inits
5. Add validation for argument count/type matching
6. Add E4xx errors for invalid push targets

---

## Feature 2: (Reserved for future features)

---

## Success Criteria

All V5 features must:
1. Pass tests for all PRT languages (Python, Rust, TypeScript)
2. Have clear semantic documentation
3. Not break any V4 functionality
4. Have validation errors for misuse

---

## Timeline

V5 development begins after V4 Phase 7 (Persistence) is complete.
