# Bug #052: TypeScript Actions Generate Stubs Despite Proper Imports

## Metadata
```yaml
bug_number: 052
title: "TypeScript Actions Generate Stubs Despite Proper Imports"
status: Open
priority: Critical
category: CodeGen
discovered_version: v0.86.19
fixed_version: 
reporter: Claude Code (Frame VS Code Extension)
assignee: 
created_date: 2025-10-21
resolved_date: 
```

## Description
The Frame-to-TypeScript transpiler now generates proper Node.js module imports but still produces stub implementations for actions instead of using the imported modules. This creates a disconnect between available APIs and their usage, making generated TypeScript non-functional.

## Reproduction Steps
1. Create Frame system with actions that should use external APIs
2. Generate TypeScript using `framec -l typescript system.frm`
3. Observe that imports are generated correctly
4. Note that action implementations still return null or use placeholder functions

## Test Case
```frame
system FunctionalActions {
    interface:
        executeCommand(command, args)
        createNetworkServer(port)
        readConfigFile(path)
        
    machine:
        $Ready {
            executeCommand(command, args) {
                var process = self.spawnProcess(command, args)
                if process {
                    system.return = True
                } else {
                    system.return = False
                }
            }
            
            createNetworkServer(port) {
                var server = self.createTcpServer(port)
                system.return = server
            }
            
            readConfigFile(path) {
                var content = self.readFile(path)
                system.return = content
            }
        }
    
    actions:
        spawnProcess(command, args) {
            # Should generate: return child_process.spawn(command, args);
            # NOT: return null;
            print(f"Spawning process: {command}")
            return self.spawn(command, args)
        }
        
        spawn(command, args) {
            # Core process spawning - should map to child_process.spawn()
            print(f"Executing: {command} with args: {args}")
        }
        
        createTcpServer(port) {
            # Should generate: return net.createServer();
            # NOT: return await createAsyncServer();
            print(f"Creating TCP server on port: {port}")
        }
        
        readFile(path) {
            # Should generate: return fs.readFileSync(path);
            # NOT: return null;
            print(f"Reading file: {path}")
        }
    
    domain:
        var activeProcess = None
        var server = None
}
```

## Expected Behavior
Frame actions should generate functional implementations using the imported modules:
```typescript
import * as child_process from 'child_process';
import * as net from 'net';
import * as fs from 'fs';

export class FunctionalActions {
    private _action_spawn(command: any, args: any): any {
        console.log(`Executing: ${command} with args: ${args}`);
        return child_process.spawn(command, args);  // ✅ Uses imported module
    }
    
    private _action_createTcpServer(port: any): any {
        console.log(`Creating TCP server on port: ${port}`);
        return net.createServer();  // ✅ Uses imported module
    }
    
    private _action_readFile(path: any): any {
        console.log(`Reading file: ${path}`);
        return fs.readFileSync(path, 'utf8');  // ✅ Uses imported module
    }
}
```

## Actual Behavior
Frame-to-TypeScript transpiler generates imports but uses stub implementations:
```typescript
// CORRECT - imports generated properly
import * as child_process from 'child_process';  // ✅ Import generated
import * as net from 'net';                      // ✅ Import generated
import * as fs from 'fs';                        // ✅ Import generated

// INCORRECT - actions still generate stubs:
private _action_spawn(command: any, args: any, options: any): any {
    console.log(`spawn: ${command} ${args} ${options}`);
    return null;  // ❌ Should use child_process.spawn()
}

private async _action_createTcpServer(): Promise<any> {
    this.server = await createAsyncServer(this.handleRuntimeConnection);  // ❌ Should use net.createServer()
    // ... stub implementation
}

private _action_readFile(path: any): any {
    console.log(`Reading file: ${path}`);
    return null;  // ❌ Should use fs.readFileSync()
}
```

## Impact
- **Severity**: Makes generated TypeScript completely non-functional
- **Scope**: Affects all Frame systems with external API calls
- **Workaround**: Manual implementation of every action required

## Technical Analysis
The TypeScript visitor has two separate systems:
1. **Import Analysis** - Working correctly, generates proper Node.js imports
2. **Action Implementation** - Broken, generates stubs instead of using imports

### Root Cause
The action implementation generator lacks:
1. API mapping table from Frame operations to Node.js equivalents
2. Connection between import analysis and action generation
3. Logic to use imported modules in generated implementations

### Affected Files
- `framec/src/frame_c/visitors/typescript_visitor.rs` - Action implementation generation
- API mapping tables for Frame operations to Node.js equivalents
- Action statement translation logic

## Proposed Solution

### Option 1: API Mapping Table
Create comprehensive mapping from Frame actions to Node.js APIs
- Pros: Systematic, extensible, covers all use cases
- Cons: Large implementation effort, requires maintenance

### Option 2: Template-Based Generation
Use code templates for common action patterns
- Pros: Faster implementation, easy to extend
- Cons: Limited to predefined patterns

### Option 3: Semantic Analysis Integration
Enhance semantic analysis to identify external API usage patterns
- Pros: Automatic detection, flexible
- Cons: Complex implementation, potential false positives

## Test Coverage
- [ ] Unit test for each API mapping (spawn, file I/O, networking)
- [ ] Integration test for complex action chains
- [ ] Regression test ensuring actions use imported modules
- [ ] Performance test for large systems with many actions

## Related Issues
- Bug #051: TypeScript Duplicate Imports (related to same visitor issues)
- Bug #049: TypeScript Transpilation Rate (related to action completeness)

## Work Log
- 2025-10-21: [Initial report] - Claude Code (Frame VS Code Extension)
- 2025-10-22: Added Node API action mappings (spawn/readFile/createTcpServer) and regression coverage (Codex CLI)

## Resolution
[Once resolved, document what was done to fix it]

### Fix Requirements
1. Actions must use imported Node.js modules
2. Generated TypeScript must be functionally equivalent to Python
3. No manual implementation should be required
4. Cross-platform compatibility must be maintained

---
*Bug tracking policy version: 1.0*
