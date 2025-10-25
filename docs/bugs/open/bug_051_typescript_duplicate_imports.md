# Bug #051: TypeScript Generator Produces Duplicate Imports

## Metadata
```yaml
bug_number: 051
title: "TypeScript Generator Produces Duplicate Imports"
status: Open
priority: High
category: CodeGen
discovered_version: v0.86.19
fixed_version: 
reporter: Claude Code (Frame VS Code Extension)
assignee: 
created_date: 2025-10-21
resolved_date: 
```

## Description
The Frame-to-TypeScript transpiler generates duplicate import statements for Node.js modules, causing TypeScript compilation errors. The same modules are imported multiple times in the generated file.

## Reproduction Steps
1. Create Frame system that uses multiple external APIs (file operations, networking, process spawning)
2. Generate TypeScript using `framec -l typescript system.frm`
3. Attempt to compile generated TypeScript with `tsc`
4. Observe duplicate import compilation errors

## Test Case
```frame
system ImportTest {
    interface:
        processFile(filePath)
        startServer(port)
        
    machine:
        $Ready {
            processFile(filePath) {
                var content = self.readFile(filePath)
                system.return = content
            }
            
            startServer(port) {
                var server = self.createTcpServer(port)
                system.return = server
            }
        }
    
    actions:
        readFile(filePath) {
            # Should generate fs.readFileSync() - requiring fs import
            print(f"Reading file: {filePath}")
        }
        
        createTcpServer(port) {
            # Should generate net.createServer() - requiring net import
            print(f"Creating server on port: {port}")
        }
        
        spawn(command, args) {
            # Should generate child_process.spawn() - requiring child_process import
            print(f"Spawning: {command} with {args}")
        }
    
    domain:
        var lastResult = None
}
```

## Expected Behavior
Each Node.js module should be imported only once at the top of the generated TypeScript file:
```typescript
// CORRECT - single imports
import * as fs from 'fs';
import * as net from 'net';
import * as child_process from 'child_process';

export class ImportTest {
    // ... implementation using imported modules
}
```

## Actual Behavior
Frame-to-TypeScript transpiler generates duplicate imports:
```typescript
// INCORRECT - duplicate imports causing compilation errors
import * as net from 'net';
import * as child_process from 'child_process';
import * as fs from 'fs';
// ... other code ...
// Node.js module imports for API mapping
import * as child_process from 'child_process';  // ❌ DUPLICATE
import * as net from 'net';                      // ❌ DUPLICATE
import * as fs from 'fs'                         // ❌ DUPLICATE
```

TypeScript compilation errors:
```
src/debug/state_machines/FrameDebugAdapter.ts(22,13): error TS2300: Duplicate identifier 'net'.
src/debug/state_machines/FrameDebugAdapter.ts(23,13): error TS2300: Duplicate identifier 'child_process'.
src/debug/state_machines/FrameDebugAdapter.ts(25,13): error TS2300: Duplicate identifier 'fs'.
src/debug/state_machines/FrameDebugAdapter.ts(66,13): error TS2300: Duplicate identifier 'child_process'.
src/debug/state_machines/FrameDebugAdapter.ts(67,13): error TS2300: Duplicate identifier 'net'.
src/debug/state_machines/FrameDebugAdapter.ts(68,13): error TS2300: Duplicate identifier 'fs'.
```

## Impact
- **Severity**: Blocks TypeScript compilation completely
- **Scope**: Affects all Frame systems using external APIs
- **Workaround**: Manual removal of duplicate imports required

## Technical Analysis
The TypeScript visitor appears to have multiple import generation points without deduplication logic.

### Root Cause
Two separate code paths in the TypeScript visitor generate imports:
1. Initial import analysis generates imports at file header
2. API mapping system generates additional imports during action processing
3. No deduplication mechanism prevents duplicate imports

### Affected Files
- `framec/src/frame_c/visitors/typescript_visitor.rs` - Import generation logic
- Import statement deduplication mechanisms
- Module dependency tracking systems

## Proposed Solution

### Option 1: Import Deduplication Set
Track imported modules in a HashSet during visitor traversal
- Pros: Simple, effective, preserves existing logic
- Cons: Requires visitor state management

### Option 2: Post-Generation Import Cleanup
Process generated TypeScript to remove duplicate imports
- Pros: Non-invasive to existing visitor logic
- Cons: Less efficient, additional processing step

## Test Coverage
- [ ] Unit test for import deduplication added to TypeScript test suite
- [ ] Integration test with multiple API usage patterns
- [ ] Regression test to prevent future duplicate imports
- [ ] Manual testing with real Frame debugging systems

## Related Issues
- Related to Bug #049: TypeScript Transpilation Rate issues
- Connects to action implementation stub generation

## Work Log
- 2025-10-21: [Initial report] - Claude Code (Frame VS Code Extension)
- 2025-10-22: Implemented TypeScript visitor import deduplication and node module tracking (Codex CLI)

## Resolution
[Once resolved, document what was done to fix it]

---
*Bug tracking policy version: 1.0*
