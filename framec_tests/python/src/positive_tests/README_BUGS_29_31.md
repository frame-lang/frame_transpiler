# Test Cases for Bugs #29 and #31

## Overview
These test cases demonstrate two related transpiler bugs in Frame v0.80.4:
- **Bug #29**: Missing interface method event routing in some states
- **Bug #31**: Spurious interface method calls in wrong event handlers

## Test Files

### 1. `test_bug29_31_exact_repro.frm`
- **Purpose**: Exact reproduction of the bugs from VS Code extension testing
- **Status**: FAILS - Both bugs present
- **Details**: This is the exact file that triggered the bug discovery

### 2. `test_bug29_missing_routing.frm`
- **Purpose**: Demonstrate Bug #29 (missing handlers/routing)
- **Status**: PASSES - Bug doesn't manifest in this structure
- **Details**: Similar structure but simpler, shows the bug is complexity-triggered

### 3. `test_bug31_spurious_calls.frm`
- **Purpose**: Demonstrate Bug #31 (spurious calls)
- **Status**: PASSES - Bug doesn't manifest in this structure  
- **Details**: Focused test for spurious calls

### 4. `test_bugs_29_31_combined.frm`
- **Purpose**: Show both bugs are related
- **Status**: PASSES - Bugs don't manifest
- **Details**: Demonstrates the connection between missing handlers and spurious calls

### 5. `test_bugs_29_31_minimal_works.frm`
- **Purpose**: Prove simple files work correctly
- **Status**: PASSES - Works as expected
- **Details**: Minimal test showing the transpiler works for simple cases

## How to Verify the Bugs

```bash
# Test the exact reproduction case
framec -l python_3 test_bug29_31_exact_repro.frm > test_bug29_31_exact_repro.py

# Check Bug #29 - Missing handlers (should exist but don't)
grep "def __handle_running_getCurrentState" test_bug29_31_exact_repro.py
grep "def __handle_paused_getCurrentState" test_bug29_31_exact_repro.py
# Result: No output (handlers missing)

# Check Bug #29 - Missing routing (should exist but doesn't)
grep -A10 "def __minimaldebugprotocol_state_Running" test_bug29_31_exact_repro.py | grep getCurrentState
# Result: No output (routing missing)

# Check Bug #31 - Spurious calls (shouldn't exist but do)
grep -n "getCurrentState()" test_bug29_31_exact_repro.py | grep -v "def "
# Result: Lines 190 and 226 have spurious calls
```

## Bug Analysis

The bugs appear to be triggered by:
1. **File complexity**: Simple test cases work, complex ones fail
2. **Specific state/handler combinations**: Running and Paused states with getCurrentState
3. **Interface method ordering**: May be related to how interface methods are processed

### Connection Between Bugs
The same `getCurrentState` method that:
1. Should have its own handler (missing - Bug #29)
2. Appears as a spurious call in `canExecuteCommand` (Bug #31)

This suggests the transpiler is attempting to process `getCurrentState` but placing it in the wrong location.

## Current Status
- **Version Affected**: v0.80.4 (and earlier)
- **Severity**: HIGH - Breaks interface contract
- **Impact**: Interface methods silently fail in certain states

## Files for Transpiler Team
The key file for debugging is `test_bug29_31_exact_repro.frm` as it reliably reproduces both bugs.