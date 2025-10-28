# Bug #054: TypeScript Transpiler Generates JavaScript Boolean Literals in Python String Literals

**Date**: 2025-10-28  
**Reporter**: Claude  
**Status**: Fixed  
**Priority**: Medium  
**Affects**: TypeScript transpiler  
**Version**: v0.86.23  
**Fixed in**: v0.86.24  

## Summary

The TypeScript transpiler generates JavaScript boolean literals (`true`/`false`) inside Python string literals, while the Python transpiler correctly generates Python boolean literals (`True`/`False`). This causes Python syntax errors when the generated strings are executed as Python code.

## Example

**Frame specification**:
```frame
createMinimalPythonCode() {
    var code = "import socket\n"
    code = code + "        return True\n"
    code = code + "        return False\n"
    return code
}
```

**Python transpiler output** (✅ Correct):
```python
def _action_createMinimalPythonCode(self):
    code = "import socket\n"
    code = code + "        return True\n"     # ✅ Python boolean
    code = code + "        return False\n"    # ✅ Python boolean
    return code
```

**TypeScript transpiler output** (❌ Incorrect):
```typescript
private _action_createMinimalPythonCode(): any {
    var code = "import socket\n";
    code = FrameRuntime.add(code, "        return true\n");   // ❌ JavaScript boolean
    code = FrameRuntime.add(code, "        return false\n");  // ❌ JavaScript boolean
    return code;
}
```

## Error Caused

When the TypeScript-generated string is executed as Python code:
```
NameError: name 'true' is not defined. Did you mean: 'True'?
NameError: name 'false' is not defined. Did you mean: 'False'?
```

## Expected Behavior

Both transpilers should handle boolean literals in string contexts consistently:
- When generating code for the target language, boolean literals should be preserved as-is
- When generating string literals containing code for other languages, the boolean literals should match the target language's syntax

## Context

This bug affects the Frame VS Code debugger which uses Frame state machines to generate Python runtime code. The TypeScript transpiler generates JavaScript boolean literals inside Python code strings, causing Python syntax errors.

## Workaround

None available. The Frame specification boolean literals need to be processed correctly by the TypeScript transpiler.

## Minimal Reproduction

1. Create Frame specification with boolean literals in string concatenation
2. Transpile to TypeScript 
3. Observe JavaScript boolean literals instead of Python boolean literals in strings
4. Execute generated string as Python code → syntax error

## Related Issues

- Bug #053: TypeScript missing property declarations (fixed in v0.86.23)
- Frame VS Code debugger minimal TCP connection test failing due to Python boolean syntax errors

## Fix Applied

**Root Cause**: Post-processing string replacements in the TypeScript visitor were blindly replacing `True`/`False` throughout the entire generated code, including inside string literals.

**Solution**: 
1. Removed all post-processing boolean replacement hacks (lines 1372-1377 in typescript_visitor.rs)
2. Boolean conversion now happens only during proper AST traversal in `ExprType::LiteralExprT` handler
3. String literals preserve their content as-is, boolean literals convert to TypeScript booleans

**Additional Cleanup**: Removed all other post-processing string manipulation hacks to prevent similar issues and ensure the visitor generates correct code directly.

**Result**: String literals correctly preserve `"True"`/`"False"` while boolean variables correctly convert to `true`/`false`.