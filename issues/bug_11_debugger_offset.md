# Bug Report: VS Code Debugger Line Offset Issue

**Bug ID:** #11  
**Severity:** Medium  
**Status:** Active  
**Component:** VS Code Frame Extension (NOT transpiler)  
**Reporter:** VS Code Frame Extension Team  
**Date:** 2024-12-30  

## Summary
This is a VS Code extension architectural issue, NOT a transpiler bug. The debugger highlights the wrong Frame line because the extension injects ~700 lines of debug instrumentation that shifts all line numbers, but the source maps don't account for this offset.

## Problem Description
When stepping through Frame code in VS Code:
- Debugger stops at the correct location (e.g., line 9)
- But highlights the wrong line (e.g., line 6)
- Execution is correct, only the visual indicator is wrong

## Root Cause (CONFIRMED)

### The Architecture Problem
1. **Original Generated Python:** Frame line 6 → Python line 21
2. **After Debug Instrumentation:** Frame line 6 → Python line 728 (shifted by ~700 lines!)
3. **Source Map Issue:** Maps are for original code (line 21), but debugger reports instrumented line (728)

### What Happens
```
Frame Code → Transpiler → Python Code (with source map)
                              ↓
                   VS Code Extension adds debug runtime
                              ↓
                   Python Code + 700 lines of debug code
                              ↓
                   Debugger reports line 728, but source map expects line 21
```

## This is NOT a Transpiler Bug
The Frame transpiler correctly generates source maps for the Python code it produces. The issue is that the VS Code extension then modifies this code by injecting debug instrumentation, invalidating the source maps.

## Solution Required (in VS Code Extension)
The VS Code extension needs to either:

### Option 1: Track and Subtract Offset
```typescript
// In FrameRuntime.ts
private adjustPythonLine(reportedLine: number): number {
    const DEBUG_RUNTIME_LINES = this.getDebugRuntimeLineCount();
    return reportedLine - DEBUG_RUNTIME_LINES;
}
```

### Option 2: Generate Adjusted Source Maps
Create new source maps that account for the injected debug code:
```typescript
private adjustSourceMap(originalMap: SourceMap, offset: number): SourceMap {
    return {
        ...originalMap,
        mappings: originalMap.mappings.map(m => ({
            ...m,
            pythonLine: m.pythonLine + offset
        }))
    };
}
```

### Option 3: Use Different Injection Method
- Use Python's `-m` flag with a debug module
- Use environment variables to enable debugging
- Use a separate debug harness file

## Current Workaround in Extension
The extension currently uses stdin to pass code to Python, which helps but doesn't fully solve the offset issue because the debug runtime is still prepended to the user code.

## Test Case
```frame
fn main() {
    print("Line 6")    # Debugger stops here correctly
    var x = 42         # But highlights previous line
}
```

## Impact
- Confusing debugging experience
- Users can't visually see which line is about to execute
- Makes step-by-step debugging difficult to follow
- **This is purely a visual issue** - execution is correct

## Files Involved (VS Code Extension)
- `src/debug/FrameRuntime.ts` - Where debug instrumentation is injected
- `src/debug/FrameDebugAdapter.ts` - Debug adapter protocol implementation

## Priority
Medium - This is a quality-of-life issue that makes debugging less intuitive, but doesn't affect correctness. The fix needs to be in the VS Code extension, not the transpiler.

## Note for Transpiler Team
**No action required from transpiler team.** This issue is documented here for awareness, but the fix must be implemented in the VS Code Frame extension. The transpiler's source maps are correct.