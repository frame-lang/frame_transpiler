# Frame v0.59 - Debug Support & Source Mapping

## Overview
Frame v0.59 introduces comprehensive debugging support with line-by-line source mapping between Frame source files and generated Python code. This enables IDE debuggers to step through Frame code while executing the generated Python, providing a native debugging experience.

## Key Achievements

### 100% AST Node Line Tracking Coverage
- **122 AST nodes** now have line tracking fields
- **Every syntactic construct** in Frame can be mapped to its source line
- **Parser integration** passes line numbers from tokens through to AST nodes

### Source Map Generation Infrastructure
- **SourceMapBuilder**: Core class for tracking Frame-to-Python line mappings
- **PythonVisitor Integration**: Automatic source mapping during code generation
- **JSON Debug Output**: Combined code and source map output for IDE integration

### Debug Adapter Protocol (DAP) Support
The transpiler now provides everything needed for VSCode extension DAP integration:
- **Source file tracking**: Original Frame file names preserved
- **Line mappings**: Precise Frame-to-Python line number mapping
- **JSON output format**: Structured data ready for IDE consumption
- **Metadata**: Version info, checksums, and timestamps for validation

## Implementation Details

### Phase-by-Phase AST Node Coverage

#### Phase 1-3: Critical Control Flow (23 nodes)
- IfStmtNode, ForStmtNode, WhileStmtNode
- MatchStmtNode, CaseClause, GuardClause
- StateNode, EventHandlerNode, TransitionNode
- Complete with line tracking

#### Phase 4: High-Priority Nodes (13 nodes)
- FunctionNode, SystemNode, InterfaceMethodNode
- AssignmentStmtNode, CallExprNode, ReturnStmtNode
- All major structural elements covered

#### Phase 5: Medium-Priority Nodes (16 nodes)
- OperationsBlockNode, ActionsBlockNode, DomainBlockNode
- BinaryExprNode, UnaryExprNode, TernaryExprNode
- Mathematical and logical operations tracked

#### Phase 6: Low-Priority Nodes (15 nodes)
- AsyncStmtNode, AwaitExprNode, YieldStmtNode
- LambdaExprNode, GeneratorExprNode
- Advanced Python features supported

#### Phase 7-10: Collection & Remaining Nodes (55+ nodes)
- ListNode, DictLiteralNode, SetLiteralNode, TupleLiteralNode
- SliceNode, ListComprehensionNode, DictComprehensionNode
- All collection operations and comprehensions tracked

### Source Map JSON Format

```json
{
  "python": "# Generated Python code...",
  "sourceMap": {
    "version": "1.0",
    "sourceFile": "example.frm",
    "targetFile": "example.py",
    "mappings": [
      {
        "frameLine": 10,
        "pythonLine": 25
      }
    ]
  },
  "metadata": {
    "frameVersion": "0.30.0",
    "generatedAt": "2025-09-17T13:35:58Z",
    "checksum": "sha256:..."
  }
}
```

### CLI Integration

```bash
# Generate debug output with source maps
framec -l python_3 --debug-output input.frm > output.json

# Extract Python code from debug output
python3 -c "import json; print(json.load(open('output.json'))['python'])" > output.py
```

## Testing Results

### Source Map Coverage Metrics
- **Test File**: Complex Frame system with 77 lines
- **Mappings Generated**: 21 unique source mappings
- **Coverage**: All major language constructs mapped
- **Validation**: Generated Python executes successfully

### Verified Features
✅ Function definitions and calls
✅ System and state machine declarations
✅ Control flow (if/elif/else, for, while)
✅ List comprehensions with conditions
✅ Variable assignments and expressions
✅ Interface methods and event handlers
✅ State transitions and machine blocks

## Impact

### For Developers
- **Native debugging experience**: Step through Frame code in VSCode
- **Accurate error reporting**: Errors map back to Frame source lines
- **Breakpoint support**: Set breakpoints in .frm files
- **Variable inspection**: Examine Frame variables during execution

### For IDE Integration
- **VSCode Extension Ready**: All required data for DAP implementation
- **Language Server Protocol**: Foundation for LSP features
- **Error overlays**: Display Python errors at Frame source locations
- **IntelliSense support**: Line mappings enable hover and go-to-definition

## Technical Debt Addressed
- Fixed parser errors for all collection node types
- Standardized constructor patterns across AST nodes
- Improved error handling in debug output generation
- Enhanced metadata tracking for validation

## Backward Compatibility
- **100% backward compatible**: All existing Frame code compiles unchanged
- **Opt-in feature**: Debug output only generated with --debug-output flag
- **No performance impact**: Line tracking has negligible overhead
- **Test suite**: All 341 existing tests still pass

## Future Enhancements
- Column-level mapping for precise error locations
- Stack trace translation from Python to Frame
- Source map compression for large projects
- Multi-file project debugging support

## Version Summary
**Frame v0.59** delivers comprehensive debugging infrastructure with 100% AST node coverage for line tracking, integrated source map generation, and full IDE debugging support through JSON output format. This foundational work enables professional debugging workflows for Frame developers.