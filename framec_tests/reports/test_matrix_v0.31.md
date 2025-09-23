# Frame v0.31 Test Matrix

**Generated**: 2025-09-22 22:13  
**Total Tests**: 379  
**Current Branch**: v0.31  

## Summary Statistics

| Metric | Count | Percentage |
|--------|-------|------------|
| **Total Tests** | 379 | 100% |
| **Transpilation Success** | 376 | 99.2% |
| **Execution Success** | 290 | 76.5% |
| **Complete Success** | 293 | 77.3% |

## v0.31 Features

✅ **IMPORT STATEMENTS**: Native import support without backticks
✅ **SELF EXPRESSION**: Standalone self usage (e.g., `jsonpickle.encode(self)`)
✅ **STATIC METHOD VALIDATION**: Parse-time validation for @staticmethod
✅ **OPERATIONS DEFAULT**: Operations are instance methods by default

## Failed Tests

| Test File | Transpile | Execute | Error |
|-----------|-----------|---------|-------|
| test_circular_a.frm | ❌ | ❌ | Framec failed with an error:
Multi-file compilation failed: Error: Circular dependency detected: unknown → unknown

 |
| test_circular_b.frm | ❌ | ❌ | Framec failed with an error:
Multi-file compilation failed: Error: Circular dependency detected: unknown → unknown

 |
| test_circular_main.frm | ❌ | ❌ | Framec failed with an error:
Multi-file compilation failed: Error: Circular dependency detected: /Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/negative_tests/multifile/circular_import/./test_circular_a.frm → /Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/negative_tests/multifile/circular_import/./test_circular_b.frm → /Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/negative_tests/multifile/circular_import/./test_circular_a.frm → / |
| test_all_8_collection_patterns.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_all_8_collection_patterns.py", line 65, in <module>
    main()
    ~ |
| test_async_stress.frm | ✅ | ❌ | /Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_async_stress.py:182: RuntimeWarning: coroutine 'AsyncDataPipeline.__kernel' was never awaited
  self.__kernel(_ |
| test_async_stress_fixed.frm | ✅ | ❌ | /Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_async_stress_fixed.py:150: RuntimeWarning: coroutine 'AsyncDataPipeline.__kernel' was never awaited
  self.__ke |
| test_async_with_proper.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_async_with_proper.py", line 276, in <module>
    main()
    ~~~~^^
  |
| test_async_with_real.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_async_with_real.py", line 208, in <module>
    main()
    ~~~~^^
  F |
| test_class_basic.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_class_basic.py", line 40, in <module>
    main()
    ~~~~^^
  File " |
| test_class_simple.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_class_simple.py", line 28, in <module>
    main()
    ~~~~^^
  File  |
| test_comprehensive_scope_validation.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_comprehensive_scope_validation.py", line 251, in <module>
    main() |
| test_comprehensive_v0_20_features.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_comprehensive_v0_20_features.py", line 258, in <module>
    main()
  |
| test_controlled_hsm_loop.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_controlled_hsm_loop.py", line 117, in <module>
    main()
    ~~~~^^ |
| test_controlled_hsm_loop_verbose.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_controlled_hsm_loop_verbose.py", line 129, in <module>
    main()
   |
| test_debug_simple.frm | ✅ | ❌ |   File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_debug_simple.py", line 29
    __multi_var__:a,b = divmod_custom(17, 5)
                   ^
SyntaxError |
| test_debug_system.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_debug_system.py", line 398, in <module>
    main()
    ~~~~^^
  File |
| test_dict_from_sequences.frm | ✅ | ❌ |   File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_dict_from_sequences.py", line 40
    copy2 = {**original: None}
                       ^
SyntaxError: i |
| test_dict_in_system.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_dict_in_system.py", line 118, in <module>
    main()
    ~~~~^^
  Fi |
| test_dict_literal.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_dict_literal.py", line 146, in <module>
    main()
    ~~~~^^
  File |
| test_dict_merge_complete.frm | ✅ | ❌ |   File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_dict_merge_complete.py", line 38
    merged = {**dict1: None, **dict2: None, **dict3: None}
            |
| test_dict_unpacking.frm | ✅ | ❌ |   File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_dict_unpacking.py", line 26
    combined = {**base: None}
                      ^
SyntaxError: invalid  |
| test_dict_update.frm | ✅ | ❌ |   File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_dict_update.py", line 39
    merged = {**d1: None, **d2: None, **d3: None}
                  ^
SyntaxEr |
| test_domain_assignment.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_domain_assignment.py", line 101, in <module>
    main()
    ~~~~^^
  |
| test_domain_type_debug.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_domain_type_debug.py", line 98, in <module>
    main()
    ~~~~^^
   |
| test_enum_compliance.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_enum_compliance.py", line 93, in <module>
    main()
    ~~~~^^
  Fi |
| test_enum_custom_values.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_enum_custom_values.py", line 198, in <module>
    main()
    ~~~~^^
 |
| test_enum_iteration.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_enum_iteration.py", line 206, in <module>
    main()
    ~~~~^^
  Fi |
| test_enum_module_scope.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_enum_module_scope.py", line 132, in <module>
    main()
    ~~~~^^
  |
| test_enum_string_values.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_enum_string_values.py", line 117, in <module>
    main()
    ~~~~^^
 |
| test_enums.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_enums.py", line 125, in <module>
    main()
    ~~~~^^
  File "/User |
| test_enums_doc_function.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_enums_doc_function.py", line 136, in <module>
    main()
    ~~~~^^
 |
| test_enums_doc_grocery_demo.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_enums_doc_grocery_demo.py", line 125, in <module>
    main()
    ~~~ |
| test_enums_doc_grocery_full.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_enums_doc_grocery_full.py", line 125, in <module>
    main()
    ~~~ |
| test_event_handlers_poc.frm | ✅ | ❌ | /Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_event_handlers_poc.py:78: RuntimeWarning: coroutine 'MixedHandlerDemo.__kernel' was never awaited
  self.__kern |
| test_explicit_self_syntax.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_explicit_self_syntax.py", line 104, in <module>
    main()
    ~~~~^ |
| test_explicit_self_system_comprehensive.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_explicit_self_system_comprehensive.py", line 230, in <module>
    ma |
| test_forward_event.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_forward_event.py", line 113, in <module>
    main()
    ~~~~^^
  Fil |
| test_fsl_list_operations.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_fsl_list_operations.py", line 46, in <module>
    main()
    ~~~~^^
 |
| test_fsl_list_operations_extended.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_fsl_list_operations_extended.py", line 58, in <module>
    main()
   |
| test_fsl_string_operations.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_fsl_string_operations.py", line 43, in <module>
    main()
    ~~~~^ |
| test_function_refs_complete.frm | ✅ | ❌ |   File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_function_refs_complete.py", line 62
    result = operations[0].@indexed_call(10, 5)
                    |
| test_functions_basic.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_functions_basic.py", line 122, in <module>
    main()
    ~~~~^^
  F |
| test_functions_event_handler.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_functions_event_handler.py", line 112, in <module>
    main()
    ~~ |
| test_functions_with_system.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_functions_with_system.py", line 112, in <module>
    main()
    ~~~~ |
| test_generators_basic.frm | ✅ | ❌ |   File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_generators_basic.py", line 32
    def test_generator_expressions():
    ^^^
IndentationError: expected  |
| test_handlers_simple.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_handlers_simple.py", line 156, in <module>
    main()
    ~~~~^^
  F |
| test_hierarchy.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_hierarchy.py", line 31, in <module>
    main()
    ~~~~^^
  File "/U |
| test_if_elif_returns.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_if_elif_returns.py", line 169, in <module>
    main()
    ~~~~^^
  F |
| test_interface_type_annotation.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_interface_type_annotation.py", line 98, in <module>
    main()
    ~ |
| test_lambda_complete_fixed.frm | ✅ | ❌ |   File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_lambda_complete_fixed.py", line 48
    result1 = ops["add"].@indexed_call(5, 3)
                        |
| test_lambda_simple_collections.frm | ✅ | ❌ |   File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_lambda_simple_collections.py", line 24
    print("Add result: " + str(ops["add"].@indexed_call(5, 3)))
 |
| test_lambda_working.frm | ✅ | ❌ |   File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_lambda_working.py", line 27
    print("3. Dict lambda: multiply(6, 7) = " + str(ops["multiply"].@indexe |
| test_list_features.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_list_features.py", line 60, in <module>
    main()
    ~~~~^^
  File |
| test_list_native_methods.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_list_native_methods.py", line 67, in <module>
    main()
    ~~~~^^
 |
| test_mixed_returns.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_mixed_returns.py", line 132, in <module>
    main()
    ~~~~^^
  Fil |
| test_module_scope_comprehensive.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_module_scope_comprehensive.py", line 369, in <module>
    main()
    |
| test_module_scope_variables.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_module_scope_variables.py", line 121, in <module>
    main()
    ~~~ |
| test_multi_entity_demo.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_multi_entity_demo.py", line 401, in <module>
    main()
    ~~~~^^
  |
| test_multi_entity_scopes.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_multi_entity_scopes.py", line 255, in <module>
    main()
    ~~~~^^ |
| test_negative_indexing.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_negative_indexing.py", line 44, in <module>
    main()
    ~~~~^^
   |
| test_none_keyword.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_none_keyword.py", line 128, in <module>
    main()
    ~~~~^^
  File |
| test_parent_dispatch.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_parent_dispatch.py", line 215, in <module>
    main()
    ~~~~^^
  F |
| test_parent_dispatch_complete.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_parent_dispatch_complete.py", line 236, in <module>
    main()
    ~ |
| test_parent_transition_detection.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_parent_transition_detection.py", line 156, in <module>
    main()
   |
| test_return_assign_actions.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_return_assign_actions.py", line 113, in <module>
    main()
    ~~~~ |
| test_return_assign_event_handler.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_return_assign_event_handler.py", line 133, in <module>
    main()
   |
| test_self_domain_vars.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_self_domain_vars.py", line 104, in <module>
    main()
    ~~~~^^
   |
| test_simple_hierarchy.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_simple_hierarchy.py", line 28, in <module>
    main()
    ~~~~^^
  F |
| test_simple_hsm_loop.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_simple_hsm_loop.py", line 113, in <module>
    main()
    ~~~~^^
  F |
| test_simple_seat.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_simple_seat.py", line 115, in <module>
    main()
    ~~~~^^
  File  |
| test_single_system_transitions.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_single_system_transitions.py", line 134, in <module>
    main()
     |
| test_state_parameters.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_state_parameters.py", line 164, in <module>
    main()
    ~~~~^^
   |
| test_state_vars_complex.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_state_vars_complex.py", line 186, in <module>
    main()
    ~~~~^^
 |
| test_state_vars_transition.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_state_vars_transition.py", line 123, in <module>
    main()
    ~~~~ |
| test_static_comprehensive_v062.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_static_comprehensive_v062.py", line 232, in <module>
    main()
     |
| test_system_isolation.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_system_isolation.py", line 206, in <module>
    main()
    ~~~~^^
   |
| test_system_scope_isolation.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_system_scope_isolation.py", line 336, in <module>
    main()
    ~~~ |
| test_type_annotation_fix.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_type_annotation_fix.py", line 99, in <module>
    main()
    ~~~~^^
 |
| test_v030_mixed_entities.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_v030_mixed_entities.py", line 332, in <module>
    main()
    ~~~~^^ |
| test_v030_system_lifecycle.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_v030_system_lifecycle.py", line 455, in <module>
    main()
    ~~~~ |
| test_v031_comprehensive.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_v031_comprehensive.py", line 131, in <module>
    main()
    ~~~~^^
 |
| test_v039_features.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_v039_features.py", line 199, in <module>
    main()
    ~~~~^^
  Fil |
| test_v053_collection_fixes.frm | ✅ | ❌ |   File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_v053_collection_fixes.py", line 60
    __multi_var__:x,y,z = (1, 2, 3)
                   ^
SyntaxError |
| test_v053_multi_var.frm | ✅ | ❌ |   File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_v053_multi_var.py", line 21
    __multi_var__:x,y,z = (1, 2, 3)
                   ^
SyntaxError: inval |
| test_v054_star_expressions.frm | ✅ | ❌ |   File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_v054_star_expressions.py", line 21
    __multi_var__:first,*rest = [1, 2, 3, 4, 5]
                     |
| test_validation_comprehensive.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_validation_comprehensive.py", line 288, in <module>
    main()
    ~ |
| test_validation_with_main.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_validation_with_main.py", line 119, in <module>
    main()
    ~~~~^ |
| test_with_statement.frm | ✅ | ❌ |   File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_with_statement.py", line 196
    async with aiohttp.ClientSession() as session:
    ^^^^^^^^^^^^^^^^^^^ |
| test_with_statement_basic.frm | ✅ | ❌ | Traceback (most recent call last):
  File "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/positive_tests/test_with_statement_basic.py", line 216, in <module>
    main()
    ~~~~^ |

## Test Details

| Test File | Transpile | Execute | Status |
|-----------|-----------|---------|--------|
| test_circular_a.frm | ❌ | ❌ | ❌ FAIL |
| test_circular_b.frm | ❌ | ❌ | ❌ FAIL |
| test_circular_main.frm | ❌ | ❌ | ❌ FAIL |
| test_multifile_calculator.frm | ✅ | ✅ | ✅ PASS |
| test_multifile_math.frm | ✅ | ✅ | ✅ PASS |
| test_multifile_utils.frm | ✅ | ✅ | ✅ PASS |
| test_multifile_complex.frm | ✅ | ✅ | ✅ PASS |
| test_multifile_formatters.frm | ✅ | ✅ | ✅ PASS |
| test_multifile_math.frm | ✅ | ✅ | ✅ PASS |
| test_multifile_strings.frm | ✅ | ✅ | ✅ PASS |
| test_multifile_calculator.frm | ✅ | ✅ | ✅ PASS |
| test_multifile_large.frm | ✅ | ✅ | ✅ PASS |
| test_multifile_main.frm | ✅ | ✅ | ✅ PASS |
| test_multifile_utils.frm | ✅ | ✅ | ✅ PASS |
| test_perf_module1.frm | ✅ | ✅ | ✅ PASS |
| test_perf_module2.frm | ✅ | ✅ | ✅ PASS |
| test_perf_module3.frm | ✅ | ✅ | ✅ PASS |
| test_perf_module4.frm | ✅ | ✅ | ✅ PASS |
| test_perf_module5.frm | ✅ | ✅ | ✅ PASS |
| test_multifile_performance.frm | ✅ | ✅ | ✅ PASS |
| test_perf_module1.frm | ✅ | ✅ | ✅ PASS |
| test_perf_module2.frm | ✅ | ✅ | ✅ PASS |
| test_perf_module3.frm | ✅ | ✅ | ✅ PASS |
| test_perf_module4.frm | ✅ | ✅ | ✅ PASS |
| test_perf_module5.frm | ✅ | ✅ | ✅ PASS |
| test_access_modifiers_v048.frm | ✅ | ✅ | ✅ PASS |
| test_all_8_collection_patterns.frm | ✅ | ❌ | ❌ FAIL |
| test_all_blocks_comprehensive.frm | ✅ | ✅ | ✅ PASS |
| test_all_constructors.frm | ✅ | ✅ | ✅ PASS |
| test_assert_v047.frm | ✅ | ✅ | ✅ PASS |
| test_async_basic.frm | ✅ | ✅ | ✅ PASS |
| test_async_debug.frm | ✅ | ✅ | ✅ PASS |
| test_async_generators.frm | ✅ | ✅ | ✅ PASS |
| test_async_handler.frm | ✅ | ✅ | ✅ PASS |
| test_async_interface.frm | ✅ | ✅ | ✅ PASS |
| test_async_minimal.frm | ✅ | ✅ | ✅ PASS |
| test_async_simple.frm | ✅ | ✅ | ✅ PASS |
| test_async_stress.frm | ✅ | ❌ | ❌ FAIL |
| test_async_stress_fixed.frm | ✅ | ❌ | ❌ FAIL |
| test_async_stress_simple.frm | ✅ | ✅ | ✅ PASS |
| test_async_validate.frm | ✅ | ✅ | ✅ PASS |
| test_async_with_proper.frm | ✅ | ❌ | ❌ FAIL |
| test_async_with_real.frm | ✅ | ❌ | ❌ FAIL |
| test_backtick_removal.frm | ✅ | ✅ | ✅ PASS |
| test_basic_scope.frm | ✅ | ✅ | ✅ PASS |
| test_basic_scope_working.frm | ✅ | ✅ | ✅ PASS |
| test_bitwise_xor.frm | ✅ | ✅ | ✅ PASS |
| test_blocks_simple.frm | ✅ | ✅ | ✅ PASS |
| test_builtin_access.frm | ✅ | ✅ | ✅ PASS |
| test_c_style_comments.frm | ✅ | ✅ | ✅ PASS |
| test_c_style_comments_simple.frm | ✅ | ✅ | ✅ PASS |
| test_call_chain_debug.frm | ✅ | ✅ | ✅ PASS |
| test_call_chain_scope.frm | ✅ | ✅ | ✅ PASS |
| test_chaining.frm | ✅ | ✅ | ✅ PASS |
| test_class_basic.frm | ✅ | ❌ | ❌ FAIL |
| test_class_simple.frm | ✅ | ❌ | ❌ FAIL |
| test_class_simple_v046.frm | ✅ | ✅ | ✅ PASS |
| test_class_v046.frm | ✅ | ✅ | ✅ PASS |
| test_collection_constructors.frm | ✅ | ✅ | ✅ PASS |
| test_collection_literals_v041.frm | ✅ | ✅ | ✅ PASS |
| test_collections.frm | ✅ | ✅ | ✅ PASS |
| test_collections_all.frm | ✅ | ✅ | ✅ PASS |
| test_compound.frm | ✅ | ✅ | ✅ PASS |
| test_comprehensive_scope_validation.frm | ✅ | ❌ | ❌ FAIL |
| test_comprehensive_v0_20_features.frm | ✅ | ❌ | ❌ FAIL |
| test_const.frm | ✅ | ✅ | ✅ PASS |
| test_constructors.frm | ✅ | ✅ | ✅ PASS |
| test_controlled_hsm_loop.frm | ✅ | ❌ | ❌ FAIL |
| test_controlled_hsm_loop_verbose.frm | ✅ | ❌ | ❌ FAIL |
| test_correct_transition.frm | ✅ | ✅ | ✅ PASS |
| test_current_limitations.frm | ✅ | ✅ | ✅ PASS |
| test_debug.frm | ✅ | ✅ | ✅ PASS |
| test_debug_nil.frm | ✅ | ✅ | ✅ PASS |
| test_debug_simple.frm | ✅ | ❌ | ❌ FAIL |
| test_debug_system.frm | ✅ | ❌ | ❌ FAIL |
| test_debug_systems.frm | ✅ | ✅ | ✅ PASS |
| test_del_statement.frm | ✅ | ✅ | ✅ PASS |
| test_dict.frm | ✅ | ✅ | ✅ PASS |
| test_dict_advanced_patterns.frm | ✅ | ✅ | ✅ PASS |
| test_dict_advanced_patterns_fixed.frm | ✅ | ✅ | ✅ PASS |
| test_dict_basic.frm | ✅ | ✅ | ✅ PASS |
| test_dict_comp_basic.frm | ✅ | ✅ | ✅ PASS |
| test_dict_comp_patterns.frm | ✅ | ✅ | ✅ PASS |
| test_dict_comp_simple.frm | ✅ | ✅ | ✅ PASS |
| test_dict_comprehensions.frm | ✅ | ✅ | ✅ PASS |
| test_dict_comprehensive.frm | ✅ | ✅ | ✅ PASS |
| test_dict_conditional.frm | ✅ | ✅ | ✅ PASS |
| test_dict_constructor_patterns.frm | ✅ | ✅ | ✅ PASS |
| test_dict_from_sequences.frm | ✅ | ❌ | ❌ FAIL |
| test_dict_fromkeys.frm | ✅ | ✅ | ✅ PASS |
| test_dict_in_system.frm | ✅ | ❌ | ❌ FAIL |
| test_dict_lambda.frm | ✅ | ✅ | ✅ PASS |
| test_dict_lambda2.frm | ✅ | ✅ | ✅ PASS |
| test_dict_lambda3.frm | ✅ | ✅ | ✅ PASS |
| test_dict_literal.frm | ✅ | ❌ | ❌ FAIL |
| test_dict_merge.frm | ✅ | ✅ | ✅ PASS |
| test_dict_merge_complete.frm | ✅ | ❌ | ❌ FAIL |
| test_dict_nested_indexing_workaround.frm | ✅ | ✅ | ✅ PASS |
| test_dict_setdefault.frm | ✅ | ✅ | ✅ PASS |
| test_dict_simple.frm | ✅ | ✅ | ✅ PASS |
| test_dict_simple2.frm | ✅ | ✅ | ✅ PASS |
| test_dict_support.frm | ✅ | ✅ | ✅ PASS |
| test_dict_switch.frm | ✅ | ✅ | ✅ PASS |
| test_dict_union.frm | ✅ | ✅ | ✅ PASS |
| test_dict_unpacking.frm | ✅ | ❌ | ❌ FAIL |
| test_dict_update.frm | ✅ | ❌ | ❌ FAIL |
| test_domain_assignment.frm | ✅ | ❌ | ❌ FAIL |
| test_domain_simple.frm | ✅ | ✅ | ✅ PASS |
| test_domain_type_debug.frm | ✅ | ❌ | ❌ FAIL |
| test_dynamic_dict_creation.frm | ✅ | ✅ | ✅ PASS |
| test_elif_with_return.frm | ✅ | ✅ | ✅ PASS |
| test_empty_module.frm | ✅ | ✅ | ✅ PASS |
| test_empty_params.frm | ✅ | ✅ | ✅ PASS |
| test_empty_set_literal.frm | ✅ | ✅ | ✅ PASS |
| test_enum_basic.frm | ✅ | ✅ | ✅ PASS |
| test_enum_compliance.frm | ✅ | ❌ | ❌ FAIL |
| test_enum_custom_values.frm | ✅ | ❌ | ❌ FAIL |
| test_enum_iteration.frm | ✅ | ❌ | ❌ FAIL |
| test_enum_module_scope.frm | ✅ | ❌ | ❌ FAIL |
| test_enum_string_values.frm | ✅ | ❌ | ❌ FAIL |
| test_enums.frm | ✅ | ❌ | ❌ FAIL |
| test_enums_doc_calendar.frm | ✅ | ✅ | ✅ PASS |
| test_enums_doc_fruitsystem.frm | ✅ | ✅ | ✅ PASS |
| test_enums_doc_function.frm | ✅ | ❌ | ❌ FAIL |
| test_enums_doc_grocery_demo.frm | ✅ | ❌ | ❌ FAIL |
| test_enums_doc_grocery_full.frm | ✅ | ❌ | ❌ FAIL |
| test_enums_doc_values.frm | ✅ | ✅ | ✅ PASS |
| test_enums_simple.frm | ✅ | ✅ | ✅ PASS |
| test_enums_terminator.frm | ✅ | ✅ | ✅ PASS |
| test_error_handling_v049.frm | ✅ | ✅ | ✅ PASS |
| test_event_handlers_poc.frm | ✅ | ❌ | ❌ FAIL |
| test_exceptions_basic.frm | ✅ | ✅ | ✅ PASS |
| test_explicit_self_syntax.frm | ✅ | ❌ | ❌ FAIL |
| test_explicit_self_system_comprehensive.frm | ✅ | ❌ | ❌ FAIL |
| test_exponent_operator.frm | ✅ | ✅ | ✅ PASS |
| test_exponent_operator_basic.frm | ✅ | ✅ | ✅ PASS |
| test_external_loading.frm | ✅ | ✅ | ✅ PASS |
| test_external_loading_fixed.frm | ✅ | ✅ | ✅ PASS |
| test_features.frm | ✅ | ✅ | ✅ PASS |
| test_first_plus_simple.frm | ✅ | ✅ | ✅ PASS |
| test_first_system_only.frm | ✅ | ✅ | ✅ PASS |
| test_force_syntactic.frm | ✅ | ✅ | ✅ PASS |
| test_forward_event.frm | ✅ | ❌ | ❌ FAIL |
| test_fsl_bool.frm | ✅ | ✅ | ✅ PASS |
| test_fsl_conversion_ops.frm | ✅ | ✅ | ✅ PASS |
| test_fsl_import_required.frm | ✅ | ✅ | ✅ PASS |
| test_fsl_list_operations.frm | ✅ | ❌ | ❌ FAIL |
| test_fsl_list_operations_extended.frm | ✅ | ❌ | ❌ FAIL |
| test_fsl_no_import_error.frm | ✅ | ✅ | ✅ PASS |
| test_fsl_simple.frm | ✅ | ✅ | ✅ PASS |
| test_fsl_string_operations.frm | ✅ | ❌ | ❌ FAIL |
| test_func_ref_simple.frm | ✅ | ✅ | ✅ PASS |
| test_func_ref_use.frm | ✅ | ✅ | ✅ PASS |
| test_function_call.frm | ✅ | ✅ | ✅ PASS |
| test_function_isolation.frm | ✅ | ✅ | ✅ PASS |
| test_function_refs_basic.frm | ✅ | ✅ | ✅ PASS |
| test_function_refs_complete.frm | ✅ | ❌ | ❌ FAIL |
| test_function_scope_isolation.frm | ✅ | ✅ | ✅ PASS |
| test_functions_basic.frm | ✅ | ❌ | ❌ FAIL |
| test_functions_event_handler.frm | ✅ | ❌ | ❌ FAIL |
| test_functions_simple.frm | ✅ | ✅ | ✅ PASS |
| test_functions_with_system.frm | ✅ | ❌ | ❌ FAIL |
| test_generators_basic.frm | ✅ | ❌ | ❌ FAIL |
| test_handlers_simple.frm | ✅ | ❌ | ❌ FAIL |
| test_hierarchy.frm | ✅ | ❌ | ❌ FAIL |
| test_history.frm | ✅ | ✅ | ✅ PASS |
| test_if_elif_returns.frm | ✅ | ❌ | ❌ FAIL |
| test_if_simple.frm | ✅ | ✅ | ✅ PASS |
| test_if_with_simple_stmt.frm | ✅ | ✅ | ✅ PASS |
| test_import_conflicts.frm | ✅ | ✅ | ✅ PASS |
| test_import_fsl_individual.frm | ✅ | ✅ | ✅ PASS |
| test_import_fsl_no_import.frm | ✅ | ✅ | ✅ PASS |
| test_import_fsl_simple.frm | ✅ | ✅ | ✅ PASS |
| test_import_fsl_user_conflict.frm | ✅ | ✅ | ✅ PASS |
| test_import_fsl_wildcard.frm | ✅ | ✅ | ✅ PASS |
| test_import_mixed.frm | ✅ | ✅ | ✅ PASS |
| test_import_python_comprehensive.frm | ✅ | ✅ | ✅ PASS |
| test_import_simple.frm | ✅ | ✅ | ✅ PASS |
| test_import_statements.frm | ✅ | ✅ | ✅ PASS |
| test_import_validation_summary.frm | ✅ | ✅ | ✅ PASS |
| test_in_operator.frm | ✅ | ✅ | ✅ PASS |
| test_instantiation_debug.frm | ✅ | ✅ | ✅ PASS |
| test_instantiation_fix.frm | ✅ | ✅ | ✅ PASS |
| test_interface_type_annotation.frm | ✅ | ❌ | ❌ FAIL |
| test_json_file.frm | ✅ | ✅ | ✅ PASS |
| test_json_file_fixed.frm | ✅ | ✅ | ✅ PASS |
| test_json_loading.frm | ✅ | ✅ | ✅ PASS |
| test_just_return_assign.frm | ✅ | ✅ | ✅ PASS |
| test_just_transition.frm | ✅ | ✅ | ✅ PASS |
| test_just_transition_v2.frm | ✅ | ✅ | ✅ PASS |
| test_keyword.frm | ✅ | ✅ | ✅ PASS |
| test_lambda.frm | ✅ | ✅ | ✅ PASS |
| test_lambda_complete.frm | ✅ | ✅ | ✅ PASS |
| test_lambda_complete_fixed.frm | ✅ | ❌ | ❌ FAIL |
| test_lambda_simple.frm | ✅ | ✅ | ✅ PASS |
| test_lambda_simple_collections.frm | ✅ | ❌ | ❌ FAIL |
| test_lambda_working.frm | ✅ | ❌ | ❌ FAIL |
| test_legb_basic.frm | ✅ | ✅ | ✅ PASS |
| test_legb_resolution.frm | ✅ | ✅ | ✅ PASS |
| test_legb_scope_resolution.frm | ✅ | ✅ | ✅ PASS |
| test_list_comprehensions.frm | ✅ | ✅ | ✅ PASS |
| test_list_comprehensions_simple.frm | ✅ | ✅ | ✅ PASS |
| test_list_features.frm | ✅ | ❌ | ❌ FAIL |
| test_list_native_methods.frm | ✅ | ❌ | ❌ FAIL |
| test_list_operations_comprehensive.frm | ✅ | ✅ | ✅ PASS |
| test_loop_else.frm | ✅ | ✅ | ✅ PASS |
| test_match_case.frm | ✅ | ✅ | ✅ PASS |
| test_match_patterns_advanced.frm | ✅ | ✅ | ✅ PASS |
| test_matmul_syntax_only.frm | ✅ | ✅ | ✅ PASS |
| test_matmul_transpile.frm | ✅ | ✅ | ✅ PASS |
| test_matmul_transpile_only.frm | ✅ | ✅ | ✅ PASS |
| test_matmul_with_numpy.frm | ✅ | ✅ | ✅ PASS |
| test_matrix_multiplication.frm | ✅ | ✅ | ✅ PASS |
| test_method_call_simple.frm | ✅ | ✅ | ✅ PASS |
| test_minimal_call.frm | ✅ | ✅ | ✅ PASS |
| test_minimal_scope.frm | ✅ | ✅ | ✅ PASS |
| test_minimal_syntax.frm | ✅ | ✅ | ✅ PASS |
| test_minimal_transition.frm | ✅ | ✅ | ✅ PASS |
| test_minimal_transition_single.frm | ✅ | ✅ | ✅ PASS |
| test_minimal_two_systems.frm | ✅ | ✅ | ✅ PASS |
| test_missing_features.frm | ✅ | ✅ | ✅ PASS |
| test_mixed_entities.frm | ✅ | ✅ | ✅ PASS |
| test_mixed_function_system.frm | ✅ | ✅ | ✅ PASS |
| test_mixed_returns.frm | ✅ | ❌ | ❌ FAIL |
| test_mixed_system_states.frm | ✅ | ✅ | ✅ PASS |
| test_module_access.frm | ✅ | ✅ | ✅ PASS |
| test_module_declaration.frm | ✅ | ✅ | ✅ PASS |
| test_module_function_calls.frm | ✅ | ✅ | ✅ PASS |
| test_module_imports_no_backticks.frm | ✅ | ✅ | ✅ PASS |
| test_module_qualified_simple.frm | ✅ | ✅ | ✅ PASS |
| test_module_scope_basic.frm | ✅ | ✅ | ✅ PASS |
| test_module_scope_comprehensive.frm | ✅ | ❌ | ❌ FAIL |
| test_module_scope_variables.frm | ✅ | ❌ | ❌ FAIL |
| test_module_syntax.frm | ✅ | ✅ | ✅ PASS |
| test_module_system.frm | ✅ | ✅ | ✅ PASS |
| test_module_var_access.frm | ✅ | ✅ | ✅ PASS |
| test_module_var_simple.frm | ✅ | ✅ | ✅ PASS |
| test_multi_entity_demo.frm | ✅ | ❌ | ❌ FAIL |
| test_multi_entity_scopes.frm | ✅ | ❌ | ❌ FAIL |
| test_multi_entity_simple.frm | ✅ | ✅ | ✅ PASS |
| test_multi_systems_with_interface.frm | ✅ | ✅ | ✅ PASS |
| test_multi_systems_with_main.frm | ✅ | ✅ | ✅ PASS |
| test_multi_systems_with_transitions.frm | ✅ | ✅ | ✅ PASS |
| test_multifile_utils.frm | ✅ | ✅ | ✅ PASS |
| test_multiple_assignment_v052.frm | ✅ | ✅ | ✅ PASS |
| test_multiple_systems_valid.frm | ✅ | ✅ | ✅ PASS |
| test_native_print.frm | ✅ | ✅ | ✅ PASS |
| test_negative_indexing.frm | ✅ | ❌ | ❌ FAIL |
| test_nested_dict_simple.frm | ✅ | ✅ | ✅ PASS |
| test_nested_index.frm | ✅ | ✅ | ✅ PASS |
| test_nested_modules.frm | ✅ | ✅ | ✅ PASS |
| test_none_keyword.frm | ✅ | ❌ | ❌ FAIL |
| test_not_in_operator.frm | ✅ | ✅ | ✅ PASS |
| test_operations.frm | ✅ | ✅ | ✅ PASS |
| test_operations_call_bug.frm | ✅ | ✅ | ✅ PASS |
| test_operations_simple.frm | ✅ | ✅ | ✅ PASS |
| test_operations_single_entity.frm | ✅ | ✅ | ✅ PASS |
| test_os_path.frm | ✅ | ✅ | ✅ PASS |
| test_parent_dispatch.frm | ✅ | ❌ | ❌ FAIL |
| test_parent_dispatch_complete.frm | ✅ | ❌ | ❌ FAIL |
| test_parent_transition_detection.frm | ✅ | ❌ | ❌ FAIL |
| test_perf_module1.frm | ✅ | ✅ | ✅ PASS |
| test_perf_module2.frm | ✅ | ✅ | ✅ PASS |
| test_perf_module3.frm | ✅ | ✅ | ✅ PASS |
| test_perf_module4.frm | ✅ | ✅ | ✅ PASS |
| test_perf_module5.frm | ✅ | ✅ | ✅ PASS |
| test_python_logical_keywords.frm | ✅ | ✅ | ✅ PASS |
| test_python_logical_operators.frm | ✅ | ✅ | ✅ PASS |
| test_python_logical_simple.frm | ✅ | ✅ | ✅ PASS |
| test_python_style.frm | ✅ | ✅ | ✅ PASS |
| test_qualified_names.frm | ✅ | ✅ | ✅ PASS |
| test_return_assign.frm | ✅ | ✅ | ✅ PASS |
| test_return_assign_actions.frm | ✅ | ❌ | ❌ FAIL |
| test_return_assign_event_handler.frm | ✅ | ❌ | ❌ FAIL |
| test_safe_dict_parsing.frm | ✅ | ✅ | ✅ PASS |
| test_scope_isolation.frm | ✅ | ✅ | ✅ PASS |
| test_scope_operations.frm | ✅ | ✅ | ✅ PASS |
| test_seat_booking_simple.frm | ✅ | ✅ | ✅ PASS |
| test_seat_booking_simple_working.frm | ✅ | ✅ | ✅ PASS |
| test_seat_booking_workflow.frm | ✅ | ✅ | ✅ PASS |
| test_self_call.frm | ✅ | ✅ | ✅ PASS |
| test_self_call_debug.frm | ✅ | ✅ | ✅ PASS |
| test_self_domain_vars.frm | ✅ | ❌ | ❌ FAIL |
| test_self_variable_exhaustive.frm | ✅ | ✅ | ✅ PASS |
| test_set_comprehensions.frm | ✅ | ✅ | ✅ PASS |
| test_set_literal.frm | ✅ | ✅ | ✅ PASS |
| test_simple_call.frm | ✅ | ✅ | ✅ PASS |
| test_simple_call_chain_debug.frm | ✅ | ✅ | ✅ PASS |
| test_simple_condition.frm | ✅ | ✅ | ✅ PASS |
| test_simple_elif.frm | ✅ | ✅ | ✅ PASS |
| test_simple_hierarchy.frm | ✅ | ❌ | ❌ FAIL |
| test_simple_hsm_loop.frm | ✅ | ❌ | ❌ FAIL |
| test_simple_module.frm | ✅ | ✅ | ✅ PASS |
| test_simple_multi.frm | ✅ | ✅ | ✅ PASS |
| test_simple_operation.frm | ✅ | ✅ | ✅ PASS |
| test_simple_print.frm | ✅ | ✅ | ✅ PASS |
| test_simple_scope.frm | ✅ | ✅ | ✅ PASS |
| test_simple_seat.frm | ✅ | ❌ | ❌ FAIL |
| test_simple_system.frm | ✅ | ✅ | ✅ PASS |
| test_simple_validation.frm | ✅ | ✅ | ✅ PASS |
| test_single_lifecycle.frm | ✅ | ✅ | ✅ PASS |
| test_single_system.frm | ✅ | ✅ | ✅ PASS |
| test_single_system_only.frm | ✅ | ✅ | ✅ PASS |
| test_single_system_transitions.frm | ✅ | ❌ | ❌ FAIL |
| test_single_transition.frm | ✅ | ✅ | ✅ PASS |
| test_slicing.frm | ✅ | ✅ | ✅ PASS |
| test_slicing_comprehensive.frm | ✅ | ✅ | ✅ PASS |
| test_slicing_simple.frm | ✅ | ✅ | ✅ PASS |
| test_slicing_simple2.frm | ✅ | ✅ | ✅ PASS |
| test_special_dicts.frm | ✅ | ✅ | ✅ PASS |
| test_special_dicts_fixed.frm | ✅ | ✅ | ✅ PASS |
| test_special_dicts_simple.frm | ✅ | ✅ | ✅ PASS |
| test_state_parameters.frm | ✅ | ❌ | ❌ FAIL |
| test_state_parameters_simple.frm | ✅ | ✅ | ✅ PASS |
| test_state_var.frm | ✅ | ✅ | ✅ PASS |
| test_state_vars_complex.frm | ✅ | ❌ | ❌ FAIL |
| test_state_vars_simple.frm | ✅ | ✅ | ✅ PASS |
| test_state_vars_transition.frm | ✅ | ❌ | ❌ FAIL |
| test_states_simple.frm | ✅ | ✅ | ✅ PASS |
| test_static_calls.frm | ✅ | ✅ | ✅ PASS |
| test_static_comprehensive_v062.frm | ✅ | ❌ | ❌ FAIL |
| test_static_operations.frm | ✅ | ✅ | ✅ PASS |
| test_string_operations_comprehensive.frm | ✅ | ✅ | ✅ PASS |
| test_string_slicing_phase1.frm | ✅ | ✅ | ✅ PASS |
| test_string_slicing_simple.frm | ✅ | ✅ | ✅ PASS |
| test_system_isolation.frm | ✅ | ❌ | ❌ FAIL |
| test_system_no_function.frm | ✅ | ✅ | ✅ PASS |
| test_system_only_operations.frm | ✅ | ✅ | ✅ PASS |
| test_system_operation_calls.frm | ✅ | ✅ | ✅ PASS |
| test_system_return.frm | ✅ | ✅ | ✅ PASS |
| test_system_return_comprehensive.frm | ✅ | ✅ | ✅ PASS |
| test_system_return_simple.frm | ✅ | ✅ | ✅ PASS |
| test_system_scope_isolation.frm | ✅ | ❌ | ❌ FAIL |
| test_traffic_light_persist.frm | ✅ | ✅ | ✅ PASS |
| test_traffic_light_simple.frm | ✅ | ✅ | ✅ PASS |
| test_transition_return.frm | ✅ | ✅ | ✅ PASS |
| test_transition_with_return.frm | ✅ | ✅ | ✅ PASS |
| test_try_except.frm | ✅ | ✅ | ✅ PASS |
| test_try_except_javascript.frm | ✅ | ✅ | ✅ PASS |
| test_try_except_simple.frm | ✅ | ✅ | ✅ PASS |
| test_tuple_literal.frm | ✅ | ✅ | ✅ PASS |
| test_two_systems_no_function.frm | ✅ | ✅ | ✅ PASS |
| test_two_systems_print.frm | ✅ | ✅ | ✅ PASS |
| test_type_annotation_fix.frm | ✅ | ❌ | ❌ FAIL |
| test_type_annotations.frm | ✅ | ✅ | ✅ PASS |
| test_type_fix.frm | ✅ | ✅ | ✅ PASS |
| test_underscore_actions.frm | ✅ | ✅ | ✅ PASS |
| test_unpacking_operator.frm | ✅ | ✅ | ✅ PASS |
| test_v030_edge_cases.frm | ✅ | ✅ | ✅ PASS |
| test_v030_functions_only.frm | ✅ | ✅ | ✅ PASS |
| test_v030_hierarchical_systems.frm | ✅ | ✅ | ✅ PASS |
| test_v030_lifecycle_demo.frm | ✅ | ✅ | ✅ PASS |
| test_v030_mixed_entities.frm | ✅ | ❌ | ❌ FAIL |
| test_v030_multi_system_basic.frm | ✅ | ✅ | ✅ PASS |
| test_v030_simple_lifecycle.frm | ✅ | ✅ | ✅ PASS |
| test_v030_system_lifecycle.frm | ✅ | ❌ | ❌ FAIL |
| test_v030_system_lifecycle_simple.frm | ✅ | ✅ | ✅ PASS |
| test_v030_system_with_functions.frm | ✅ | ✅ | ✅ PASS |
| test_v030_three_systems.frm | ✅ | ✅ | ✅ PASS |
| test_v031_comprehensive.frm | ✅ | ❌ | ❌ FAIL |
| test_v039_features.frm | ✅ | ❌ | ❌ FAIL |
| test_v040_comments_floor_div.frm | ✅ | ✅ | ✅ PASS |
| test_v040_string_features.frm | ✅ | ✅ | ✅ PASS |
| test_v040_strings_simple.frm | ✅ | ✅ | ✅ PASS |
| test_v053_collection_fixes.frm | ✅ | ❌ | ❌ FAIL |
| test_v053_list_fix.frm | ✅ | ✅ | ✅ PASS |
| test_v053_multi_var.frm | ✅ | ❌ | ❌ FAIL |
| test_v054_collection_constructors.frm | ✅ | ✅ | ✅ PASS |
| test_v054_star_expressions.frm | ✅ | ❌ | ❌ FAIL |
| test_v056_features.frm | ✅ | ✅ | ✅ PASS |
| test_v056_walrus_and_literals.frm | ✅ | ✅ | ✅ PASS |
| test_validation_comprehensive.frm | ✅ | ❌ | ❌ FAIL |
| test_validation_with_main.frm | ✅ | ❌ | ❌ FAIL |
| test_with_statement.frm | ✅ | ❌ | ❌ FAIL |
| test_with_statement_basic.frm | ✅ | ❌ | ❌ FAIL |
| test_working_features.frm | ✅ | ✅ | ✅ PASS |
| test_xor_operator.frm | ✅ | ✅ | ✅ PASS |
| test_xor_simple.frm | ✅ | ✅ | ✅ PASS |
| test_your_example.frm | ✅ | ✅ | ✅ PASS |
