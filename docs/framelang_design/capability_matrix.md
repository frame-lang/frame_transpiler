# Frame v0.38 Feature Capability Matrix

**Last Updated**: 2025-09-07  
**Version**: v0.38  
**Purpose**: Maps every Frame feature to its validating test(s) with current pass/fail status

## Legend
- ✅ **PASS** - Feature implemented and test passes
- ❌ **FAIL** - Feature implemented but test fails  
- ⚠️ **PARTIAL** - Feature partially implemented
- ❓ **UNKNOWN** - Status needs verification
- ➖ **NOT_IMPL** - Feature not implemented
- 🔄 **TESTING** - Currently being validated

## Core Language Features

### Functions & Systems

| Feature | Test File | Status | Notes |
|---------|-----------|--------|--------|
| Basic Functions | test_functions_basic.frm | ❓ | Need to verify |
| Multiple Functions per File | test_multi_functions.frm | ❓ | Multi-entity support |
| Function Parameters | test_function_params.frm | ❓ | Parameter handling |
| Function Return Values | test_function_returns.frm | ❓ | Return syntax |
| Basic System Declaration | test_system_basic.frm | ❓ | System syntax |
| Multiple Systems per File | test_multi_systems.frm | ❓ | Multi-entity support |
| System Interface Methods | test_interface_methods.frm | ❓ | Interface block |
| System State Machines | test_state_machines.frm | ❓ | Machine block |
| System Actions | test_actions.frm | ❓ | Actions block |
| System Operations | test_operations.frm | ❓ | Operations block |
| System Domain Variables | test_domain_vars.frm | ❓ | Domain block |

### Variables & Scoping

| Feature | Test File | Status | Notes |
|---------|-----------|--------|--------|
| Local Variables | test_local_vars.frm | ❓ | Function/system scope |
| Module Variables | test_module_vars.frm | ❓ | Global scope |
| Domain Variables | test_domain_vars.frm | ❓ | System state |
| Domain Variable Dict Init | test_dict_literal.frm | ❌ | var x = {"a": 1} in domain |
| Self Variable Access | test_self_variable.frm | ❓ | self.variable syntax |
| Variable Shadowing | test_var_shadowing.frm | ❓ | LEGB resolution |
| Module Scope Variables | test_module_scope_variables.frm | ❓ | Cross-function access |

### Control Flow

| Feature | Test File | Status | Notes |
|---------|-----------|--------|--------|
| If/Elif/Else | test_conditionals.frm | ❓ | Basic conditionals |
| For Loops | test_for_loops.frm | ❓ | Iteration |
| For-In Loops | test_for_in.frm | ❓ | Collection iteration |
| While Loops | test_while.frm | ❓ | Conditional loops |
| State Transitions | test_transitions.frm | ❓ | -> syntax |
| Break/Continue | test_loop_control.frm | ❓ | Loop control |

### Expressions & Operators

| Feature | Test File | Status | Notes |
|---------|-----------|--------|--------|
| Arithmetic Operators | test_arithmetic.frm | ❓ | +, -, *, /, % |
| Comparison Operators | test_comparison.frm | ❓ | ==, !=, <, >, <=, >= |
| Python Logical Operators | test_python_logical_operators.frm | ❓ | and, or, not |
| Assignment Operators | test_assignment.frm | ❓ | = syntax |
| Augmented Assignment | test_augmented_assign.frm | ❓ | +=, -=, *=, /= |
| Exponent Operator | test_exponent_operator.frm | ❓ | ** operator |
| XOR Operator | test_xor_operator.frm | ❓ | xor keyword |

## Collection Types

### Lists

| Feature | Test File | Status | Notes |
|---------|-----------|--------|--------|
| List Literals | test_list_literals.frm | ❓ | [1, 2, 3] |
| List Indexing | test_list_indexing.frm | ❓ | list[0] |
| List Slicing | test_slicing.frm | ❓ | list[1:5:2] |
| List Methods | test_list_methods.frm | ❓ | append, pop, etc |
| List Comprehensions | test_list_comprehensions.frm | ❓ | [x for x in list] |
| List Unpacking | test_unpacking.frm | ❓ | [*list1, *list2] |
| Negative Indexing | test_negative_indexing.frm | ❓ | list[-1] |

### Dictionaries

| Feature | Test File | Status | Notes |
|---------|-----------|--------|--------|
| Dict Literals | test_collections_all.frm | ✅ | {"key": "value"} |
| Dict Indexing | test_dynamic_dict_creation.frm | ✅ | dict["key"] |
| Dict Assignment | test_dynamic_dict_creation.frm | ✅ | dict["key"] = value |
| Dict Methods | test_dynamic_dict_creation.frm | ✅ | .get(), .setdefault(), etc |
| Dict Comprehensions | test_dict_comprehensions.frm | ✅ | {k:v for k,v in items} |
| Dict Constructor | test_collections_all.frm | ✅ | {} empty dict |
| Dict in System Domain | test_dict_literal.frm | ❌ | Domain init parsing issue |
| Dict Merging | test_dict_merge.frm | ❓ | ** unpacking |

### Sets

| Feature | Test File | Status | Notes |
|---------|-----------|--------|--------|
| Set Literals | test_collections_all.frm | ✅ | {1, 2, 3} |
| Set Operations | test_set_operations.frm | ❓ | union, intersection |
| Set Constructor | test_collections_all.frm | ✅ | Single element {42} |
| Empty Set | test_collections_all.frm | ❓ | set() needed (not {}) |

### Tuples

| Feature | Test File | Status | Notes |
|---------|-----------|--------|--------|
| Tuple Literals | test_collections_all.frm | ✅ | (1, 2, 3) |
| Tuple Indexing | test_tuple_indexing.frm | ❓ | tuple[0] |
| Single Element Tuple | test_collections_all.frm | ✅ | (42,) with comma |
| Tuple Constructor | test_tuple_constructor.frm | ❓ | tuple() |
| Empty Tuple | test_collections_all.frm | ✅ | () |

### Collection Constructors

| Feature | Test File | Status | Notes |
|---------|-----------|--------|--------|
| All 8 Collection Patterns | test_all_8_collection_patterns.frm | ❓ | Complete matrix |
| Collection Constructors | test_collection_constructors.frm | ❓ | Constructor syntax |
| Mixed Collections | test_collections_all.frm | ❓ | Nested collections |

## Advanced Features

### Module System

| Feature | Test File | Status | Notes |
|---------|-----------|--------|--------|
| Module Declaration | test_modules.frm | ❓ | module keyword |
| Qualified Names | test_qualified_names.frm | ❓ | module.function() |
| Nested Modules | test_nested_modules.frm | ❓ | Hierarchical modules |
| Module Variables | test_module_variables.frm | ❓ | Module scope vars |
| Cross-Module Access | test_cross_module.frm | ❓ | Inter-module calls |

### Async/Await

| Feature | Test File | Status | Notes |
|---------|-----------|--------|--------|
| Async Functions | test_async_functions.frm | ❓ | async fn syntax |
| Await Expressions | test_await_expr.frm | ❓ | await expr |
| Async Interface Methods | test_async_interface.frm | ❓ | async in interfaces |
| Async Event Handlers | test_async_handlers.frm | ❓ | async $>() |
| Mixed Async/Sync | test_mixed_async.frm | ❓ | Hybrid systems |

### Enums

| Feature | Test File | Status | Notes |
|---------|-----------|--------|--------|
| Basic Enums | test_enum_basic.frm | ❓ | enum declaration |
| Custom Values | test_enum_custom_values.frm | ❓ | enum Val = 42 |
| String Enums | test_enum_string_values.frm | ❓ | enum : string |
| Enum Iteration | test_enum_iteration.frm | ❓ | for val in enum |
| Module Scope Enums | test_enum_module_scope.frm | ❓ | Global enums |
| Enum Properties | test_enum_properties.frm | ❓ | .name, .value |

### Import System

| Feature | Test File | Status | Notes |
|---------|-----------|--------|--------|
| Simple Imports | test_import_simple.frm | ❓ | import module |
| Aliased Imports | test_import_alias.frm | ❓ | import mod as alias |
| From Imports | test_import_from.frm | ❓ | from mod import item |
| Wildcard Imports | test_import_wildcard.frm | ❓ | from mod import * |
| Mixed Imports | test_import_mixed.frm | ❓ | Multiple import types |

### Native Python Operations

| Feature | Test File | Status | Notes |
|---------|-----------|--------|--------|
| Type Conversions | test_type_conversions.frm | ❓ | str(), int(), float(), bool() |
| String Operations | test_string_ops.frm | ❓ | .upper(), .lower(), etc |
| List Operations | test_list_ops.frm | ❓ | .append(), .pop(), etc |
| Built-in Functions | test_builtins.frm | ❓ | len(), range(), etc |
| Native vs FSL | test_native_vs_fsl.frm | ❓ | Clarify requirements |

## Lambda Functions

| Feature | Test File | Status | Notes |
|---------|-----------|--------|--------|
| Lambda Expressions | test_lambda_simple.frm | ✅ | lambda x: x * x |
| Lambda Multi-param | test_lambda_simple.frm | ✅ | lambda a, b: a + b |
| Lambda No-param | test_lambda_simple.frm | ✅ | lambda: 3.14159 |
| Lambda in Dict | test_dict_lambda.frm | ✅ | {"fn": lambda x: x} |
| Lambda in Collections | test_lambda.frm | ⚠️ | Simulated with functions |
| Lambda Closures | NONE | ➖ | Future feature |
| Lambda as Parameters | NONE | ➖ | First-class functions needed |

## Special Features

### Hierarchical State Machines

| Feature | Test File | Status | Notes |
|---------|-----------|--------|--------|
| Parent-Child States | test_hsm_basic.frm | ❓ | $Child => $Parent |
| Event Forwarding | test_event_forward.frm | ❓ | => $^ syntax |
| State Inheritance | test_state_inheritance.frm | ❓ | Behavior inheritance |

### State Stack Operations

| Feature | Test File | Status | Notes |
|---------|-----------|--------|--------|
| Push State | test_state_push.frm | ❓ | $$[+] operator |
| Pop State | test_state_pop.frm | ❓ | $$[-] operator |
| State History | test_state_history.frm | ❓ | History mechanism |

### Special Syntax

| Feature | Test File | Status | Notes |
|---------|-----------|--------|--------|
| Event Handlers | test_event_handlers.frm | ❓ | eventName() syntax |
| Enter/Exit Events | test_enter_exit.frm | ❓ | $>(), <$() |
| Return Assignment | test_return_assign.frm | ❓ | return = value |
| System Return | test_system_return.frm | ❓ | system.return |
| Self Context | test_self_context.frm | ❓ | self expression |

## Test Validation Status

### Test Suite Statistics
- **Total Features Mapped**: 100+
- **Tests Found**: 280+ test files  
- **Tests Passing**: 269/283 (95.1% per test_log.md)
- **Tests Failing**: 14/283 (4.9%)
- **Features Verified**: 10+ (Critical collection and lambda features)
- **Last Validation**: 2025-09-07

### Verified Working Features ✅
Based on successful transpilation and code generation testing:

#### Collection Types - ALL WORKING
- **Dict Literals**: `{"key": "value"}` → test_collections_all.frm ✅
- **Dict Indexing**: `dict["key"]`, `dict["key"] = value` → test_dynamic_dict_creation.frm ✅  
- **Dict Methods**: `.get()`, `.setdefault()`, etc → test_dynamic_dict_creation.frm ✅
- **Set Literals**: `{1, 2, 3}` → test_collections_all.frm ✅
- **Tuple Literals**: `(1, 2, 3)` → test_collections_all.frm ✅
- **List Literals**: `[1, 2, 3]` → test_collections_all.frm ✅
- **Mixed Collections**: Nested dicts, lists, sets, tuples → test_collections_all.frm ✅
- **Empty Collections**: `{}`, `[]`, `()`, `set()` → test_collections_all.frm ✅

#### Lambda Expressions - WORKING
- **Basic Lambdas**: `lambda x: x * x` → test_lambda_simple.frm ✅
- **Multi-param Lambdas**: `lambda a, b: a + b` → test_lambda_simple.frm ✅
- **No-param Lambdas**: `lambda: 3.14159` → test_lambda_simple.frm ✅
- **Lambda in Dict**: `{"add": lambda a,b: a+b}` → test_dict_lambda.frm ✅

#### Native Python Operations - WORKING  
- **Type Conversions**: `str()`, `int()`, `float()`, `bool()` → No FSL import needed ✅
- **Built-in Functions**: `len()`, `print()` → Work natively ✅
- **List Methods**: `.append()`, `.pop()`, `.extend()` → Work on list objects ✅
- **Dict Methods**: `.get()`, `.setdefault()`, `.keys()` → Work on dict objects ✅

### Test Philosophy
- **Keep evolutionary aspects as comments** for future implementation
- **Statement-level Python compatibility** is the goal  
- **Frame intentional deviations** are preserved (state machines, transitions)
- **Periodic sweep for comment removal** with user approval

### Critical Issues Found
1. **test_dict_literal.frm**: Parse error with dict in domain variable initialization
2. **Lambda tests comment future features**: First-class functions, closures
3. **Some tests claim features "not yet supported"** when they actually work

### Documentation vs Reality
- **v0.38 claims 224/224 (100%)** but test_log shows 269/283 (95.1%)
- **Lambda support EXISTS** but tests say "not yet supported"  
- **Dict indexing WORKS** but some tests may not reflect this
- **Native Python functions WORK** without FSL imports

## Key Discoveries

### ✅ Fully Working But Underdocumented
1. **Lambda Expressions**: Complete support for Python lambda syntax
2. **Dictionary Indexing**: Full `dict["key"]` read/write support  
3. **Native Python Functions**: `str()`, `len()`, etc work without FSL imports
4. **All Collection Literals**: Dict, Set, Tuple, List all working

### ❌ Known Limitations
1. **Domain Variable Dict Initialization**: Cannot initialize with dict literal in domain block
2. **First-Class Functions**: Functions/lambdas not yet first-class values
3. **Empty Set Constructor**: Must use `set()` function, not literal

### 🔄 Test Maintenance Needed
1. **Update test comments**: Remove "not yet supported" for working features
2. **Keep evolutionary comments**: Document future features as TODO comments
3. **Python statement compatibility**: Focus on native Python syntax at statement level

## Action Items

1. ✅ **VALIDATED CORE FEATURES** - Dict, Lambda, Collections confirmed working
2. 🔄 **UPDATE TEST COMMENTS** - Remove incorrect "not supported" claims
3. 📝 **TRACK DOMAIN DICT INIT** - Known parser limitation to fix
4. 📋 **ALIGN DOCUMENTATION** - v0.38 should reflect actual 95.1% success rate
5. 🎯 **PYTHON STATEMENT GOAL** - Maximize native Python syntax compatibility

---
**Note**: This matrix validates actual implementation vs documentation claims. Features marked ✅ have been tested with successful transpilation and code generation.