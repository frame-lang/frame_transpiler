# Frame v0.31 Test Matrix

**Generated**: 2025-08-30 17:33  
**Total Tests**: 153  
**Current Branch**: v0.31  

## Summary Statistics

| Metric | Count | Percentage |
|--------|-------|------------|
| **Total Tests** | 153 | 100% |
| **Transpilation Success** | 151 | 98.7% |
| **Execution Success** | 146 | 95.4% |
| **Complete Success** | 146 | 95.4% |

## v0.31 Features

✅ **IMPORT STATEMENTS**: Native import support without backticks
✅ **SELF EXPRESSION**: Standalone self usage (e.g., `jsonpickle.encode(self)`)
✅ **STATIC METHOD VALIDATION**: Parse-time validation for @staticmethod
✅ **OPERATIONS DEFAULT**: Operations are instance methods by default

## Failed Tests

| Test File | Transpile | Execute | Error |
|-----------|-----------|---------|-------|
| test_import_statements.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/test_import_statements.py", line 6, in <module>
    import numpy as np
ModuleNotFoundErr |
| test_legb_scope_resolution.frm | ❌ | ❌ | DEBUG: Starting first pass - building symbol table
DEBUG: Created syntactic parser with is_building_symbol_table=true
DEBUG: Building symbol table for function: main
DEBUG: Entering function scope for |
| test_single_system_transitions.frm | ✅ | ❌ | Timeout during execution |
| test_static_self_error.frm | ❌ | ❌ | DEBUG: Starting first pass - building symbol table
DEBUG: Created syntactic parser with is_building_symbol_table=true
DEBUG: First pass parsing succeeded
Framec failed with an error:
First pass parsin |
| test_system_isolation.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/test_system_isolation.py", line 209, in <module>
    main()
    ~~~~^^
  File "/Users/ma |
| test_traffic_light_persist.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/test_traffic_light_persist.py", line 5, in <module>
    import jsonpickle
ModuleNotFound |
| test_your_example.frm | ✅ | ❌ | Timeout during execution |

## Test Details

| Test File | Transpile | Execute | Status |
|-----------|-----------|---------|--------|
| test_all_blocks_comprehensive.frm | ✅ | ✅ | ✅ PASS |
| test_basic_scope.frm | ✅ | ✅ | ✅ PASS |
| test_basic_scope_working.frm | ✅ | ✅ | ✅ PASS |
| test_blocks_simple.frm | ✅ | ✅ | ✅ PASS |
| test_builtin_access.frm | ✅ | ✅ | ✅ PASS |
| test_call_chain_debug.frm | ✅ | ✅ | ✅ PASS |
| test_call_chain_scope.frm | ✅ | ✅ | ✅ PASS |
| test_comprehensive_scope_validation.frm | ✅ | ✅ | ✅ PASS |
| test_comprehensive_v0_20_features.frm | ✅ | ✅ | ✅ PASS |
| test_controlled_hsm_loop.frm | ✅ | ✅ | ✅ PASS |
| test_controlled_hsm_loop_verbose.frm | ✅ | ✅ | ✅ PASS |
| test_correct_transition.frm | ✅ | ✅ | ✅ PASS |
| test_debug.frm | ✅ | ✅ | ✅ PASS |
| test_domain_assignment.frm | ✅ | ✅ | ✅ PASS |
| test_domain_simple.frm | ✅ | ✅ | ✅ PASS |
| test_domain_type_debug.frm | ✅ | ✅ | ✅ PASS |
| test_elif_with_return.frm | ✅ | ✅ | ✅ PASS |
| test_empty_params.frm | ✅ | ✅ | ✅ PASS |
| test_enum_basic.frm | ✅ | ✅ | ✅ PASS |
| test_enums.frm | ✅ | ✅ | ✅ PASS |
| test_enums_doc_calendar.frm | ✅ | ✅ | ✅ PASS |
| test_enums_doc_fruitsystem.frm | ✅ | ✅ | ✅ PASS |
| test_enums_doc_function.frm | ✅ | ✅ | ✅ PASS |
| test_enums_doc_grocery_demo.frm | ✅ | ✅ | ✅ PASS |
| test_enums_doc_grocery_full.frm | ✅ | ✅ | ✅ PASS |
| test_enums_doc_values.frm | ✅ | ✅ | ✅ PASS |
| test_enums_simple.frm | ✅ | ✅ | ✅ PASS |
| test_enums_terminator.frm | ✅ | ✅ | ✅ PASS |
| test_explicit_self_syntax.frm | ✅ | ✅ | ✅ PASS |
| test_explicit_self_system_comprehensive.frm | ✅ | ✅ | ✅ PASS |
| test_first_plus_simple.frm | ✅ | ✅ | ✅ PASS |
| test_first_system_only.frm | ✅ | ✅ | ✅ PASS |
| test_force_syntactic.frm | ✅ | ✅ | ✅ PASS |
| test_forward_event.frm | ✅ | ✅ | ✅ PASS |
| test_function_call.frm | ✅ | ✅ | ✅ PASS |
| test_function_isolation.frm | ✅ | ✅ | ✅ PASS |
| test_function_scope_isolation.frm | ✅ | ✅ | ✅ PASS |
| test_functions_basic.frm | ✅ | ✅ | ✅ PASS |
| test_functions_event_handler.frm | ✅ | ✅ | ✅ PASS |
| test_functions_simple.frm | ✅ | ✅ | ✅ PASS |
| test_functions_with_system.frm | ✅ | ✅ | ✅ PASS |
| test_history.frm | ✅ | ✅ | ✅ PASS |
| test_if_elif_returns.frm | ✅ | ✅ | ✅ PASS |
| test_if_simple.frm | ✅ | ✅ | ✅ PASS |
| test_if_with_simple_stmt.frm | ✅ | ✅ | ✅ PASS |
| test_import_simple.frm | ✅ | ✅ | ✅ PASS |
| test_import_statements.frm | ✅ | ❌ | ❌ FAIL |
| test_instantiation_debug.frm | ✅ | ✅ | ✅ PASS |
| test_instantiation_fix.frm | ✅ | ✅ | ✅ PASS |
| test_interface_type_annotation.frm | ✅ | ✅ | ✅ PASS |
| test_just_return_assign.frm | ✅ | ✅ | ✅ PASS |
| test_just_transition.frm | ✅ | ✅ | ✅ PASS |
| test_just_transition_v2.frm | ✅ | ✅ | ✅ PASS |
| test_keyword.frm | ✅ | ✅ | ✅ PASS |
| test_legb_basic.frm | ✅ | ✅ | ✅ PASS |
| test_legb_scope_resolution.frm | ❌ | ❌ | ❌ FAIL |
| test_method_call_simple.frm | ✅ | ✅ | ✅ PASS |
| test_minimal_call.frm | ✅ | ✅ | ✅ PASS |
| test_minimal_scope.frm | ✅ | ✅ | ✅ PASS |
| test_minimal_syntax.frm | ✅ | ✅ | ✅ PASS |
| test_minimal_transition.frm | ✅ | ✅ | ✅ PASS |
| test_minimal_transition_single.frm | ✅ | ✅ | ✅ PASS |
| test_minimal_two_systems.frm | ✅ | ✅ | ✅ PASS |
| test_mixed_entities.frm | ✅ | ✅ | ✅ PASS |
| test_mixed_function_system.frm | ✅ | ✅ | ✅ PASS |
| test_mixed_returns.frm | ✅ | ✅ | ✅ PASS |
| test_mixed_system_states.frm | ✅ | ✅ | ✅ PASS |
| test_module_function_calls.frm | ✅ | ✅ | ✅ PASS |
| test_module_scope_basic.frm | ✅ | ✅ | ✅ PASS |
| test_module_var_access.frm | ✅ | ✅ | ✅ PASS |
| test_module_var_simple.frm | ✅ | ✅ | ✅ PASS |
| test_multi_entity_demo.frm | ✅ | ✅ | ✅ PASS |
| test_multi_entity_scopes.frm | ✅ | ✅ | ✅ PASS |
| test_multi_entity_simple.frm | ✅ | ✅ | ✅ PASS |
| test_multi_systems_with_interface.frm | ✅ | ✅ | ✅ PASS |
| test_multi_systems_with_main.frm | ✅ | ✅ | ✅ PASS |
| test_multi_systems_with_transitions.frm | ✅ | ✅ | ✅ PASS |
| test_multiple_systems_valid.frm | ✅ | ✅ | ✅ PASS |
| test_native_print.frm | ✅ | ✅ | ✅ PASS |
| test_operations.frm | ✅ | ✅ | ✅ PASS |
| test_operations_call_bug.frm | ✅ | ✅ | ✅ PASS |
| test_operations_simple.frm | ✅ | ✅ | ✅ PASS |
| test_operations_single_entity.frm | ✅ | ✅ | ✅ PASS |
| test_parent_dispatch.frm | ✅ | ✅ | ✅ PASS |
| test_parent_dispatch_complete.frm | ✅ | ✅ | ✅ PASS |
| test_parent_transition_detection.frm | ✅ | ✅ | ✅ PASS |
| test_python_style.frm | ✅ | ✅ | ✅ PASS |
| test_return_assign.frm | ✅ | ✅ | ✅ PASS |
| test_return_assign_actions.frm | ✅ | ✅ | ✅ PASS |
| test_return_assign_event_handler.frm | ✅ | ✅ | ✅ PASS |
| test_scope_operations.frm | ✅ | ✅ | ✅ PASS |
| test_seat_booking_simple.frm | ✅ | ✅ | ✅ PASS |
| test_seat_booking_simple_working.frm | ✅ | ✅ | ✅ PASS |
| test_seat_booking_workflow.frm | ✅ | ✅ | ✅ PASS |
| test_self_call.frm | ✅ | ✅ | ✅ PASS |
| test_self_call_debug.frm | ✅ | ✅ | ✅ PASS |
| test_self_domain_vars.frm | ✅ | ✅ | ✅ PASS |
| test_simple_call.frm | ✅ | ✅ | ✅ PASS |
| test_simple_call_chain_debug.frm | ✅ | ✅ | ✅ PASS |
| test_simple_condition.frm | ✅ | ✅ | ✅ PASS |
| test_simple_elif.frm | ✅ | ✅ | ✅ PASS |
| test_simple_hsm_loop.frm | ✅ | ✅ | ✅ PASS |
| test_simple_multi.frm | ✅ | ✅ | ✅ PASS |
| test_simple_operation.frm | ✅ | ✅ | ✅ PASS |
| test_simple_print.frm | ✅ | ✅ | ✅ PASS |
| test_simple_scope.frm | ✅ | ✅ | ✅ PASS |
| test_simple_seat.frm | ✅ | ✅ | ✅ PASS |
| test_simple_system.frm | ✅ | ✅ | ✅ PASS |
| test_simple_validation.frm | ✅ | ✅ | ✅ PASS |
| test_single_lifecycle.frm | ✅ | ✅ | ✅ PASS |
| test_single_system.frm | ✅ | ✅ | ✅ PASS |
| test_single_system_only.frm | ✅ | ✅ | ✅ PASS |
| test_single_system_transitions.frm | ✅ | ❌ | ❌ FAIL |
| test_single_transition.frm | ✅ | ✅ | ✅ PASS |
| test_state_parameters.frm | ✅ | ✅ | ✅ PASS |
| test_state_var.frm | ✅ | ✅ | ✅ PASS |
| test_state_vars_complex.frm | ✅ | ✅ | ✅ PASS |
| test_state_vars_simple.frm | ✅ | ✅ | ✅ PASS |
| test_state_vars_transition.frm | ✅ | ✅ | ✅ PASS |
| test_states_simple.frm | ✅ | ✅ | ✅ PASS |
| test_static_calls.frm | ✅ | ✅ | ✅ PASS |
| test_static_operations.frm | ✅ | ✅ | ✅ PASS |
| test_static_self_error.frm | ❌ | ❌ | ❌ FAIL |
| test_system_interface_calls.frm | ✅ | ✅ | ✅ PASS |
| test_system_isolation.frm | ✅ | ❌ | ❌ FAIL |
| test_system_no_function.frm | ✅ | ✅ | ✅ PASS |
| test_system_only_operations.frm | ✅ | ✅ | ✅ PASS |
| test_system_operation_calls.frm | ✅ | ✅ | ✅ PASS |
| test_system_scope_isolation.frm | ✅ | ✅ | ✅ PASS |
| test_system_simple.frm | ✅ | ✅ | ✅ PASS |
| test_traffic_light_persist.frm | ✅ | ❌ | ❌ FAIL |
| test_traffic_light_simple.frm | ✅ | ✅ | ✅ PASS |
| test_transition_return.frm | ✅ | ✅ | ✅ PASS |
| test_transition_with_return.frm | ✅ | ✅ | ✅ PASS |
| test_two_systems_no_function.frm | ✅ | ✅ | ✅ PASS |
| test_two_systems_print.frm | ✅ | ✅ | ✅ PASS |
| test_type_annotation_fix.frm | ✅ | ✅ | ✅ PASS |
| test_type_fix.frm | ✅ | ✅ | ✅ PASS |
| test_underscore_actions.frm | ✅ | ✅ | ✅ PASS |
| test_v030_edge_cases.frm | ✅ | ✅ | ✅ PASS |
| test_v030_functions_only.frm | ✅ | ✅ | ✅ PASS |
| test_v030_hierarchical_systems.frm | ✅ | ✅ | ✅ PASS |
| test_v030_lifecycle_demo.frm | ✅ | ✅ | ✅ PASS |
| test_v030_mixed_entities.frm | ✅ | ✅ | ✅ PASS |
| test_v030_multi_system_basic.frm | ✅ | ✅ | ✅ PASS |
| test_v030_simple_lifecycle.frm | ✅ | ✅ | ✅ PASS |
| test_v030_system_lifecycle.frm | ✅ | ✅ | ✅ PASS |
| test_v030_system_lifecycle_simple.frm | ✅ | ✅ | ✅ PASS |
| test_v030_system_with_functions.frm | ✅ | ✅ | ✅ PASS |
| test_v030_three_systems.frm | ✅ | ✅ | ✅ PASS |
| test_v031_comprehensive.frm | ✅ | ✅ | ✅ PASS |
| test_validation_comprehensive.frm | ✅ | ✅ | ✅ PASS |
| test_your_example.frm | ✅ | ❌ | ❌ FAIL |
