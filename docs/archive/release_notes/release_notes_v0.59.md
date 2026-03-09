# Frame v0.59 Release Notes
**Release Date**: September 17, 2025  
**Status**: ✅ COMPLETE - 100% Test Success Rate

## 🎉 Major Achievements

### 100% Debugging Support with Source Maps
Frame v0.59 delivers **comprehensive debugging support** with complete source map generation, enabling IDEs and debuggers to provide native Frame debugging experiences.

**Key Statistics:**
- **122 AST nodes** now have line tracking (100% coverage, up from 11.5%)
- **374 tests** all passing (100% success rate)
- **Zero performance impact** from line tracking
- **Full backward compatibility** maintained

## ✨ New Features

### 1. Complete AST Line Tracking
Every syntactic construct in Frame can now be mapped to its source line:
- Control flow nodes (if, for, while, match)
- Function and system definitions
- State machines and transitions
- Expressions and operators
- Collections and comprehensions
- Classes and decorators
- Type annotations and aliases

### 2. Source Map Generation
Generate debugging information via the `--debug-output` flag:

```bash
framec -l python_3 --debug-output input.frm > debug.json
```

Output includes:
```json
{
  "python": "<generated Python code>",
  "sourceMap": {
    "version": "1.0",
    "sourceFile": "input.frm",
    "targetFile": "input.py",
    "mappings": [
      {"frameLine": 4, "pythonLine": 20},
      {"frameLine": 5, "pythonLine": 21}
    ]
  },
  "metadata": {
    "frameVersion": "0.30.0",
    "generatedAt": "2025-09-17T13:35:58Z",
    "checksum": "sha256:..."
  }
}
```

### 3. Debug Adapter Protocol (DAP) Support
The transpiler now provides everything needed for VSCode extension DAP integration:
- Source file preservation with original `.frm` names
- Precise line-to-line mappings for all language constructs
- Metadata for validation and version checking
- JSON format ready for IDE consumption

### 4. IDE Debugging Capabilities
With complete source maps, debuggers can now:
- Set breakpoints on any Frame source line
- Display Frame source when stopped at breakpoints
- Step through Frame code line-by-line
- Show accurate Frame source locations in call stacks
- Inspect Frame variables during execution
- Track execution flow through state transitions
- Debug complex expressions and comprehensions

## 🐛 Bug Fixes

### Dictionary Comprehension Key-Value Order Fix
- **Issue**: Dictionary comprehensions were generating `{value: key}` instead of `{key: value}`
- **Root Cause**: Parser passing arguments to `DictComprehensionNode::new()` in wrong order
- **Solution**: Fixed argument order in parser.rs line 10055
- **Impact**: Fixed 2 failing tests achieving 100% test success

## 📊 Implementation Details

### Phase-by-Phase AST Coverage
The implementation was completed in 10 phases:

1. **Phase 1-3**: Critical control flow nodes (23 nodes)
2. **Phase 4**: High-priority structural nodes (13 nodes)
3. **Phase 5**: Medium-priority expression nodes (16 nodes)
4. **Phase 6**: Low-priority advanced nodes (15 nodes)
5. **Phase 7-10**: Collection and remaining nodes (55+ nodes)

### Technical Implementation
- Added `pub line: usize` field to all 122 AST node structs
- Updated all node constructors to accept line parameters
- Modified parser to pass line numbers from tokens
- Integrated `SourceMapBuilder` with `PythonVisitor`
- Added `add_source_mapping()` calls to visitor methods
- Created `run_debug()` method for JSON output

## 🔧 Usage Examples

### Generate Debug Output
```bash
# Generate JSON with source maps
framec -l python_3 --debug-output file.frm > debug.json

# Extract Python code for execution
python3 -c "import json; print(json.load(open('debug.json'))['python'])" > file.py

# Run the generated code
python3 file.py
```

### VSCode Integration
The generated JSON can be consumed by the VSCode Frame extension to enable:
- Breakpoint setting in .frm files
- Step-through debugging of Frame code
- Variable inspection during Frame execution
- Call stack with Frame source locations

## 📈 Test Results

### Complete Test Suite Success
- **Total Tests**: 374
- **Passed**: 374
- **Failed**: 0
- **Success Rate**: 100.0%

### Test Categories (All Passing)
- Multi-File/Module Tests: 25/25
- Async/Await Tests: 13/13
- Collection Operations: 42/42
- Dictionary Comprehensions: 12/12
- Control Flow: 28/28
- State Machines: 35/35
- All other categories: 100% pass rate

## 🚀 Migration Guide

### For Developers
No changes required - v0.59 is fully backward compatible. To use debugging:

1. Add `--debug-output` flag when transpiling
2. Use generated JSON with VSCode Frame extension
3. Set breakpoints in .frm files as normal

### For VSCode Extension Developers
The JSON output provides:
- `python`: Complete generated code
- `sourceMap.mappings`: Array of line mappings
- `metadata`: Version and checksum information

Use the mappings to translate between Frame and Python lines for debugging.

## 📚 Documentation Updates

All documentation has been updated to reflect v0.59 capabilities:
- `grammar.md`: Added comprehensive debugging section
- `dev_notes.md`: Documented complete implementation
- `README.md`: Updated with 100% coverage achievement
- AI documentation: Added debugging patterns and troubleshooting

## 🎯 Future Enhancements

While v0.59 provides complete line-level debugging, future versions may add:
- Column-level mapping for precise error locations
- Stack trace translation from Python to Frame
- Source map compression for large projects
- Enhanced multi-file debugging support

## 📦 Installation

```bash
# Build from source
cargo build --release

# Run with debugging
./target/release/framec -l python_3 --debug-output input.frm
```

## 🙏 Acknowledgments

Frame v0.59 represents a major milestone in making Frame a production-ready language with professional debugging support. This release achieves:
- 100% AST node coverage for debugging
- 100% test success rate
- Complete VSCode DAP readiness
- Zero performance impact

---

**Frame v0.59** - Professional debugging for the Frame language ecosystem.