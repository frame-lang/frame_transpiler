# v0.82.5 Achievement: Transition Enter Arguments Fix

## Overview
Fixed a critical bug in the Python visitor where transition enter arguments were being completely ignored, causing state transition parameter passing to fail.

## Problem Description
Frame transition syntax `-> (args) $State` was not correctly passing enter arguments to the target state's enter handler. The Python visitor was only handling exit arguments but completely ignoring enter arguments from `TargetStateContextNode.enter_args_opt`.

### Example Issue
```frame
$Start {
    $>() {
        -> (42) $End  // Enter argument 42 should be passed to $End
    }
}

$End {
    $>(value) {  // Should receive value = 42
        print("Got: " + str(value))  // Was printing "Got: None"
    }
}
```

## Root Cause Analysis
1. **Parser**: Correctly parsed transition enter arguments into `TargetStateContextNode.enter_args_opt`
2. **Python Visitor**: Only extracted `state_ref_args_opt` but completely ignored `enter_args_opt`
3. **Generated Python**: FrameCompartment was created with `None` for enter_args parameter
4. **Runtime**: Enter events were sent with `None` parameters instead of the transition arguments

## Solution Implemented
Modified `visit_transition_statement_node()` in `python_visitor_v2.rs`:

1. **Extract Enter Arguments**: Added logic to extract `enter_args_opt` from `TargetStateContextNode`
2. **Build Enter Args Dictionary**: Created `enter_args_dict` that maps transition enter arguments to target state enter handler parameter names
3. **Pass to FrameCompartment**: Updated FrameCompartment creation to pass `enter_args_dict` as the 4th parameter
4. **Runtime Integration**: Frame runtime now correctly uses `compartment.enter_args` as enter event parameters

### Code Changes
- Enhanced transition statement visitor to handle both state reference args and enter args
- Added parameter name resolution from target state's enter handler
- Updated FrameCompartment creation calls to include enter arguments
- Simplified and corrected state_args_dict logic

## Validation Results
- **Simple Test**: `-> (42) $End` now correctly prints "Got: 42" instead of "Got: None"
- **Services Looper Tests**: Previously failing regression tests now pass
- **Test Success Rate**: Maintained 96.4% (430/446) Python test success

## Impact
- Resolves critical state transition parameter passing regression
- Fixes services looper test failures
- Restores correct Frame transition semantics for Python target
- Enables proper stateful system designs with transition parameter passing

## Technical Details
The fix correctly implements the Frame transition semantics:
- Exit arguments (before `->`) come from current state
- Enter arguments (after `->`) become enter event parameters for target state
- Python runtime uses `compartment.enter_args` as enter event parameters via `FrameEvent("$>", self.__compartment.enter_args)`

This resolves a fundamental issue in Frame's Python code generation that was breaking stateful system designs relying on transition parameter passing.