# Frame Transpiler Test Status

## Last Run: 2025-12-18

**Total Tests**: 379  
**Passed**: 379  
**Failed**: 0  
**Success Rate**: 100%

## Summary

Frame v0.66 achieves **100% test success** by establishing explicit `self.` prefix as a requirement for all internal method calls within systems. This major release also makes semantic call resolution an integral part of the parser, removing the previous feature flag.

## Recent Changes

### v0.66 Explicit Self/System Call Syntax (2025-12-18)
- **Breaking Change**: All internal method calls now require explicit `self.` prefix
- Added `ResolvedCallType::SystemInterface` for interface methods called within systems
- Removed `FRAME_SEMANTIC_RESOLUTION` feature flag - semantic resolution always enabled
- Updated all 379 tests to use explicit self syntax
- Enhanced `SemanticCallAnalyzer` to properly resolve interface methods
- Fixed action/operation resolution with underscore prefix handling
- Achieved 100% test success rate (379/379 passing)

### v0.65 Complete Code Simplification (2025-09-21)
- Removed ALL backward compatibility code
- Deleted ~500 lines of complex helper methods and tracking code
- Removed use_semantic_resolution flag completely
- Made semantic resolution the only code path
- Simplified visit_call_expression_node from ~200 to ~50 lines
- Achieved 60% complexity reduction in visitor methods
- No environment variables needed (FRAME_SEMANTIC_RESOLUTION removed)

### v0.64 Visitor Simplification (2025-09-21)
- Implemented simplified call generation using resolved types
- Added use_semantic_resolution flag to PythonConfig
- Created handle_call_with_resolved_type() methods
- Reduced complex call chain analysis logic
- Feature flag integration via FRAME_SEMANTIC_RESOLUTION=1
- No test regressions - all 379 tests continued to pass

### v0.63 Accurate Semantic Resolution (2025-09-21)
- Implemented accurate semantic call resolution
- Actions correctly identified via symbol table lookups  
- Operations correctly identified via symbol table lookups
- External calls properly distinguished from internal calls
- Parser maintains system/class/function context throughout parsing
- No test regressions - all 379 tests continued to pass

## Passing Test Categories

- ✅ **Core Language Features**: All basic Frame syntax working
- ✅ **Multi-Entity Support**: Multiple systems and functions per file  
- ✅ **Module System**: Module declarations, qualified names, nested modules
- ✅ **Async/Await**: Async functions, interface methods, await expressions
- ✅ **Python Operators**: Logical, bitwise, identity, membership operators
- ✅ **Collections**: Lists, dictionaries, sets with comprehensions
- ✅ **Lambda Expressions**: Lambda functions and first-class functions
- ✅ **Pattern Matching**: Match-case statements with various patterns
- ✅ **Class Support**: Basic OOP with classes and methods
- ✅ **Advanced Features**: Walrus operator, type aliases, f-strings
- ✅ **Control Flow**: Loop else clauses, del statement
- ✅ **Import System**: Python imports and Frame file imports
- ✅ **Semantic Resolution**: Actions, Operations, SystemInterface calls correctly identified
- ✅ **Explicit Self Syntax**: All internal calls use proper `self.` prefix

## Migration Guide for v0.66

### Required Syntax Changes

**Before v0.66 (implicit calls):**
```frame
system Example {
    machine:
        $Start {
            process() {
                _doAction()      // Implicit action call
                calculate()      // Implicit operation call
                next()          // Implicit interface call
            }
        }
}
```

**After v0.66 (explicit calls):**
```frame
system Example {
    machine:
        $Start {
            process() {
                self.doAction()      // Explicit action call
                self.calculate()     // Explicit operation call
                self.next()         // Explicit interface call
            }
        }
}
```

## Test Infrastructure
- Test runner: `framec_tests/runner/frame_test_runner.py`
- Test location: `framec_tests/python/src/`
- Generated files: Same directory as source
- Matrix report: `framec_tests/reports/test_matrix_v0.31.md`
- JSON results: `framec_tests/reports/test_results_v0.31.json`

## Notes

- v0.66 establishes explicit syntax as core language requirement
- Semantic resolution is now always enabled (no feature flag)
- All 379 tests have been updated to use explicit self syntax
- Breaking change requires updating existing Frame source code
- Improved code clarity and Python alignment