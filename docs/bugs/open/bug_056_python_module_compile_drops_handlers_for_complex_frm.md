# Bug #056: Python module compile drops handlers/actions for complex FRM

## Metadata


## Description
Compiling a moderately complex module FRM (adapter/runtime harness) yields a skeleton Python module with only the class boilerplate and no interface handlers or  methods. This breaks unit tests that expect  and basic handler emission.

## Reproduction Steps
1. Use the provided FRM copied from the rebuild harness:
   - 
2. Run the script:
   - framec 0.86.37
/var/folders/kl/kn9ds0z918n0bhzr9w40xlt00000gn/T/tmp.KssabBFzX9/runtime_protocol.py
BUG_REPRODUCED: handlers/actions missing
/var/folders/kl/kn9ds0z918n0bhzr9w40xlt00000gn/T/tmp.KssabBFzX9/runtime_protocol.py
     1	from frame_runtime_py import FrameEvent, FrameCompartment
     2	
     3	class RuntimeProtocol:
     4	    def __init__(self):
     5	        self._compartment = FrameCompartment("__RuntimeProtocol_state_A")
     6	    def _frame_transition(self, next_compartment: FrameCompartment):
     7	        self._compartment = next_compartment
     8	    def _frame_router(self, __e: FrameEvent, c: FrameCompartment=None):
     9	        pass
    10	    def _frame_stack_push(self):
    11	        pass
    12	    def _frame_stack_pop(self):
    13	        pass
3. Observe output prints  and shows the generated  with no handlers/actions.

## Validation Assets
- FRM: 
- Script: framec 0.86.37
/var/folders/kl/kn9ds0z918n0bhzr9w40xlt00000gn/T/tmp.L2Zo8EffF0/runtime_protocol.py
BUG_REPRODUCED: handlers/actions missing
/var/folders/kl/kn9ds0z918n0bhzr9w40xlt00000gn/T/tmp.L2Zo8EffF0/runtime_protocol.py
     1	from frame_runtime_py import FrameEvent, FrameCompartment
     2	
     3	class RuntimeProtocol:
     4	    def __init__(self):
     5	        self._compartment = FrameCompartment("__RuntimeProtocol_state_A")
     6	    def _frame_transition(self, next_compartment: FrameCompartment):
     7	        self._compartment = next_compartment
     8	    def _frame_router(self, __e: FrameEvent, c: FrameCompartment=None):
     9	        pass
    10	    def _frame_stack_push(self):
    11	        pass
    12	    def _frame_stack_pop(self):
    13	        pass

## Expected vs Actual
- Expected: Generated Python module includes interface handlers and  methods per actions section.
- Actual: Only skeleton methods are emitted.

## Impact
- Severity: High — blocks Frame-only generation for runtime validation; unit tests cannot execute without handlers.

## Technical Analysis
- The FRM includes async actions, event-loop helpers, and multiple actions. Generator may be failing to emit handlers when certain async/try/except patterns combine with domain declarations.
- Minimal reproduction still pending; current repro uses the harness FRM. A smaller FRM can be derived if needed.

## Proposed Solution
- Ensure codegen emits interface handlers and action methods regardless of action content complexity.
- Add tests to assert presence of emitted  methods and interface handlers for module FRMs.

## Work Log
- 2025-11-15: Initial report with /tmp repro — Codex

---
*Bug tracking policy version: 1.1*
