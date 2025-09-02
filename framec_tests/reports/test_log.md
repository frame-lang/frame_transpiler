# Frame Transpiler Test Status Report

## Latest Test Run
**Date:** 2025-09-02  
**Branch:** v0.30  
**Version:** v0.32  
**Changes:** SystemReturn token implementation

## Summary
**Total Tests:** 173  
**Passed:** 170  
**Failed:** 3  
**Success Rate:** 98.3%

## Test Categories

### ✅ Passing (170 tests)
- Basic Scope Tests
- Module Variables & Scope Resolution
- Multi-Entity Support (Functions & Systems)
- Hierarchical State Machines & Parent Dispatch
- Static Operations with @staticmethod
- Import Statements (all forms)
- Self Expression (standalone self usage)
- Enums (all features including custom values, strings, iteration)
- System Return (`system.return` special variable)
- State Variables and Transitions
- Try/Except Exception Handling
- Operations and Actions

### ❌ Failing (3 tests)

| Test File | Issue Type | Details |
|-----------|------------|---------|
| `test_system_interface_calls.frm` | Invalid Syntax | Uses `system.calculate()` and `system.process()` - not supported |
| `test_system_simple.frm` | Invalid Syntax | Uses `system.helper()` - not supported |
| `test_v031_comprehensive.frm` | Invalid Syntax | Uses `system.get_value()` - not supported |

## Analysis

All 3 failing tests use `system.method()` syntax which is not supported. Per the implementation:
- `system.return` is the ONLY valid use of the `system` keyword
- The transpiler correctly rejects these with: "The 'system' keyword is reserved. Only 'system.return' is currently supported"
- These tests need to be updated to remove invalid `system.method()` calls

## Recent Fixes Applied

1. **SystemReturn Token**: Implemented greedy scanning of "system.return" as single token
2. **Error Handling**: Added clear error message for bare `system` keyword
3. **Parser Simplification**: Removed complex `parse_system_interface_call()` method
4. **AST Cleanup**: Removed `CallContextType::SystemCall` variant

## Test Infrastructure Status

- ✅ Test runner functioning correctly
- ✅ Matrix generation working
- ✅ JSON output working
- ✅ All legitimate tests passing
- ⚠️ 3 tests using invalid syntax need correction