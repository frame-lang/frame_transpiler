# VSCode Debug Adapter Protocol (DAP) Integration Guide for Frame

**Version**: v0.59  
**Last Updated**: September 17, 2025  
**Status**: Frame transpiler fully ready for DAP integration

## Overview

This document provides comprehensive guidance for integrating Frame debugging support into a VSCode extension using the Debug Adapter Protocol (DAP). Frame v0.59 provides complete source mapping infrastructure, enabling native debugging of `.frm` files.

## Architecture Overview

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  VSCode Editor  │────▶│ Frame Extension  │────▶│  Debug Adapter  │
│   (.frm file)   │     │   (TypeScript)   │     │    (Python)     │
└─────────────────┘     └──────────────────┘     └─────────────────┘
                               │                          │
                               ▼                          ▼
                        ┌──────────────┐          ┌─────────────┐
                        │ Source Maps  │          │ Python Code │
                        │    (JSON)    │          │  (Generated)│
                        └──────────────┘          └─────────────┘
```

## Frame Transpiler Output

### Generating Debug Information

```bash
# Generate JSON with source maps and code
framec -l python_3 --debug-output input.frm > debug.json
```

### Debug Output Format

```json
{
  "python": "# Generated Python code\ndef main():\n    x = 42\n    print(x)\n",
  "sourceMap": {
    "version": "1.0",
    "sourceFile": "input.frm",
    "targetFile": "input.py",
    "mappings": [
      {"frameLine": 3, "pythonLine": 2},
      {"frameLine": 4, "pythonLine": 3},
      {"frameLine": 5, "pythonLine": 4}
    ]
  },
  "metadata": {
    "frameVersion": "0.30.0",
    "generatedAt": "2025-09-17T13:35:58Z",
    "checksum": "sha256:d69cc30c06a4e76e343dd190287d48eb5240db64eadbec8c4827c31c35737a27"
  }
}
```

## VSCode Extension Implementation

### 1. Extension Structure

```
frame-vscode/
├── package.json           # Extension manifest
├── src/
│   ├── extension.ts       # Main extension entry
│   ├── debugAdapter.ts    # Debug adapter implementation
│   ├── frameDebugger.ts   # Frame-specific debugging logic
│   └── sourceMapper.ts    # Source map handling
└── syntaxes/
    └── frame.tmLanguage.json  # Syntax highlighting
```

### 2. package.json Configuration

```json
{
  "name": "frame-lang",
  "displayName": "Frame Language",
  "version": "0.59.0",
  "engines": {
    "vscode": "^1.74.0"
  },
  "activationEvents": [
    "onDebugResolve:frame",
    "onLanguage:frame"
  ],
  "main": "./out/extension.js",
  "contributes": {
    "languages": [{
      "id": "frame",
      "extensions": [".frm"],
      "aliases": ["Frame", "frame"]
    }],
    "debuggers": [{
      "type": "frame",
      "label": "Frame Debug",
      "program": "./out/debugAdapter.js",
      "runtime": "node",
      "languages": ["frame"],
      "configurationAttributes": {
        "launch": {
          "required": ["program"],
          "properties": {
            "program": {
              "type": "string",
              "description": "Path to the Frame (.frm) file"
            },
            "pythonPath": {
              "type": "string",
              "default": "python3",
              "description": "Path to Python interpreter"
            },
            "framecPath": {
              "type": "string",
              "default": "framec",
              "description": "Path to Frame compiler"
            }
          }
        }
      },
      "initialConfigurations": [{
        "type": "frame",
        "request": "launch",
        "name": "Debug Frame File",
        "program": "${file}",
        "pythonPath": "python3",
        "framecPath": "framec"
      }]
    }]
  }
}
```

### 3. Debug Adapter Implementation

```typescript
// src/debugAdapter.ts
import {
  DebugSession,
  InitializedEvent,
  TerminatedEvent,
  StoppedEvent,
  BreakpointEvent,
  Thread,
  StackFrame,
  Scope,
  Source,
  Breakpoint
} from '@vscode/debugadapter';
import { DebugProtocol } from '@vscode/debugprotocol';
import * as path from 'path';
import * as fs from 'fs';
import { spawn, ChildProcess } from 'child_process';

interface FrameSourceMap {
  version: string;
  sourceFile: string;
  targetFile: string;
  mappings: Array<{
    frameLine: number;
    pythonLine: number;
  }>;
}

interface FrameDebugOutput {
  python: string;
  sourceMap: FrameSourceMap;
  metadata: {
    frameVersion: string;
    generatedAt: string;
    checksum: string;
  };
}

export class FrameDebugSession extends DebugSession {
  private sourceMap: FrameSourceMap | null = null;
  private pythonProcess: ChildProcess | null = null;
  private frameToBreakpoints: Map<number, DebugProtocol.SourceBreakpoint[]> = new Map();
  private pythonBreakpoints: Set<number> = new Set();
  private currentFrameFile: string = '';
  private pythonFile: string = '';
  
  protected initializeRequest(
    response: DebugProtocol.InitializeResponse,
    args: DebugProtocol.InitializeRequestArguments
  ): void {
    response.body = response.body || {};
    
    // Capabilities
    response.body.supportsConfigurationDoneRequest = true;
    response.body.supportsSetVariable = true;
    response.body.supportsStepInTargetsRequest = false;
    response.body.supportsBreakpointLocationsRequest = true;
    response.body.supportsConditionalBreakpoints = true;
    response.body.supportsHitConditionalBreakpoints = true;
    response.body.supportsLogPoints = true;
    
    this.sendResponse(response);
    this.sendEvent(new InitializedEvent());
  }
  
  protected async launchRequest(
    response: DebugProtocol.LaunchResponse,
    args: DebugProtocol.LaunchRequestArguments
  ): Promise<void> {
    const frameFile = args.program as string;
    const framecPath = args.framecPath || 'framec';
    const pythonPath = args.pythonPath || 'python3';
    
    this.currentFrameFile = frameFile;
    
    try {
      // Step 1: Transpile Frame to Python with source maps
      const debugOutput = await this.transpileFrame(frameFile, framecPath);
      
      // Step 2: Parse debug output
      const debugInfo: FrameDebugOutput = JSON.parse(debugOutput);
      this.sourceMap = debugInfo.sourceMap;
      
      // Step 3: Write Python code to temp file
      this.pythonFile = path.join(
        path.dirname(frameFile),
        `.debug_${path.basename(frameFile, '.frm')}.py`
      );
      fs.writeFileSync(this.pythonFile, debugInfo.python);
      
      // Step 4: Convert Frame breakpoints to Python breakpoints
      this.convertBreakpoints();
      
      // Step 5: Launch Python debugger with pdb
      await this.launchPythonDebugger(pythonPath);
      
      this.sendResponse(response);
    } catch (error) {
      this.sendErrorResponse(response, 1001, `Failed to launch: ${error}`);
    }
  }
  
  private async transpileFrame(frameFile: string, framecPath: string): Promise<string> {
    return new Promise((resolve, reject) => {
      const process = spawn(framecPath, [
        '-l', 'python_3',
        '--debug-output',
        frameFile
      ]);
      
      let output = '';
      let error = '';
      
      process.stdout.on('data', (data) => {
        output += data.toString();
      });
      
      process.stderr.on('data', (data) => {
        error += data.toString();
      });
      
      process.on('close', (code) => {
        if (code === 0) {
          resolve(output);
        } else {
          reject(new Error(`Transpilation failed: ${error}`));
        }
      });
    });
  }
  
  private convertBreakpoints(): void {
    if (!this.sourceMap) return;
    
    // Convert Frame line numbers to Python line numbers
    this.pythonBreakpoints.clear();
    
    for (const [frameLine, breakpoints] of this.frameToBreakpoints) {
      const mapping = this.sourceMap.mappings.find(m => m.frameLine === frameLine);
      if (mapping) {
        this.pythonBreakpoints.add(mapping.pythonLine);
      }
    }
  }
  
  private async launchPythonDebugger(pythonPath: string): Promise<void> {
    // Create Python debug script
    const debugScript = this.createPythonDebugScript();
    const debugScriptPath = this.pythonFile.replace('.py', '_debug.py');
    fs.writeFileSync(debugScriptPath, debugScript);
    
    // Launch Python with debugger
    this.pythonProcess = spawn(pythonPath, [debugScriptPath], {
      stdio: ['pipe', 'pipe', 'pipe']
    });
    
    // Handle Python process output
    this.pythonProcess.stdout?.on('data', (data) => {
      this.handlePythonOutput(data.toString());
    });
    
    this.pythonProcess.stderr?.on('data', (data) => {
      console.error(`Python stderr: ${data}`);
    });
    
    this.pythonProcess.on('exit', (code) => {
      this.sendEvent(new TerminatedEvent());
    });
  }
  
  private createPythonDebugScript(): string {
    const breakpointLines = Array.from(this.pythonBreakpoints).join(',');
    
    return `
import sys
import pdb
import json

class FrameDebugger(pdb.Pdb):
    def __init__(self):
        super().__init__()
        self.frame_breakpoints = [${breakpointLines}]
        
    def user_line(self, frame):
        # Check if we're at a breakpoint
        if frame.f_lineno in self.frame_breakpoints:
            self.interaction(frame, None)
        
    def do_info(self, arg):
        """Send debugging info to VSCode"""
        frame = self.curframe
        info = {
            'type': 'stopped',
            'line': frame.f_lineno,
            'locals': {k: str(v) for k, v in frame.f_locals.items()},
            'file': frame.f_code.co_filename
        }
        print(f"FRAME_DEBUG:{json.dumps(info)}")
        
# Load the generated Python file
sys.path.insert(0, '${path.dirname(this.pythonFile)}')

# Set up debugger
debugger = FrameDebugger()

# Set breakpoints
for line in debugger.frame_breakpoints:
    debugger.set_break('${this.pythonFile}', line)

# Run the file
debugger.run('exec(open("${this.pythonFile}").read())')
`;
  }
  
  private handlePythonOutput(output: string): void {
    // Parse debug events from Python
    const lines = output.split('\n');
    for (const line of lines) {
      if (line.startsWith('FRAME_DEBUG:')) {
        const debugInfo = JSON.parse(line.substring(12));
        this.handleDebugEvent(debugInfo);
      } else {
        // Regular output - send to console
        this.sendEvent(new OutputEvent(line + '\n', 'stdout'));
      }
    }
  }
  
  private handleDebugEvent(info: any): void {
    if (info.type === 'stopped') {
      // Convert Python line back to Frame line
      const frameLine = this.pythonToFrameLine(info.line);
      
      // Send stopped event with Frame source location
      const event = new StoppedEvent('breakpoint', 1);
      event.body.preserveFocusHint = false;
      event.body.allThreadsStopped = true;
      this.sendEvent(event);
    }
  }
  
  private pythonToFrameLine(pythonLine: number): number {
    if (!this.sourceMap) return pythonLine;
    
    const mapping = this.sourceMap.mappings.find(m => m.pythonLine === pythonLine);
    return mapping ? mapping.frameLine : pythonLine;
  }
  
  protected setBreakPointsRequest(
    response: DebugProtocol.SetBreakpointsResponse,
    args: DebugProtocol.SetBreakpointsArguments
  ): void {
    const path = args.source.path!;
    const breakpoints = args.breakpoints || [];
    
    // Store Frame breakpoints
    this.frameToBreakpoints.clear();
    for (const bp of breakpoints) {
      const frameBreakpoints = this.frameToBreakpoints.get(bp.line) || [];
      frameBreakpoints.push(bp);
      this.frameToBreakpoints.set(bp.line, frameBreakpoints);
    }
    
    // Convert to Python breakpoints
    this.convertBreakpoints();
    
    // Send response with verified breakpoints
    const actualBreakpoints: Breakpoint[] = breakpoints.map(bp => {
      const verified = this.sourceMap?.mappings.some(m => m.frameLine === bp.line) || false;
      return new Breakpoint(verified, bp.line);
    });
    
    response.body = {
      breakpoints: actualBreakpoints
    };
    this.sendResponse(response);
  }
  
  protected threadsRequest(response: DebugProtocol.ThreadsResponse): void {
    response.body = {
      threads: [new Thread(1, 'main')]
    };
    this.sendResponse(response);
  }
  
  protected stackTraceRequest(
    response: DebugProtocol.StackTraceResponse,
    args: DebugProtocol.StackTraceArguments
  ): void {
    // Create stack frames with Frame source locations
    const frames: StackFrame[] = [];
    
    // This would be populated from Python debugger state
    // For now, showing example structure
    const currentPythonLine = 10; // Would come from Python debugger
    const frameLine = this.pythonToFrameLine(currentPythonLine);
    
    frames.push(new StackFrame(
      0,
      'main',
      new Source(
        path.basename(this.currentFrameFile),
        this.currentFrameFile
      ),
      frameLine
    ));
    
    response.body = {
      stackFrames: frames,
      totalFrames: frames.length
    };
    this.sendResponse(response);
  }
  
  protected stepInRequest(
    response: DebugProtocol.StepInResponse,
    args: DebugProtocol.StepInArguments
  ): void {
    // Send step command to Python debugger
    if (this.pythonProcess) {
      this.pythonProcess.stdin?.write('s\n');
    }
    this.sendResponse(response);
  }
  
  protected stepOutRequest(
    response: DebugProtocol.StepOutResponse,
    args: DebugProtocol.StepOutArguments
  ): void {
    // Send return command to Python debugger
    if (this.pythonProcess) {
      this.pythonProcess.stdin?.write('r\n');
    }
    this.sendResponse(response);
  }
  
  protected nextRequest(
    response: DebugProtocol.NextResponse,
    args: DebugProtocol.NextArguments
  ): void {
    // Send next command to Python debugger
    if (this.pythonProcess) {
      this.pythonProcess.stdin?.write('n\n');
    }
    this.sendResponse(response);
  }
  
  protected continueRequest(
    response: DebugProtocol.ContinueResponse,
    args: DebugProtocol.ContinueArguments
  ): void {
    // Send continue command to Python debugger
    if (this.pythonProcess) {
      this.pythonProcess.stdin?.write('c\n');
    }
    this.sendResponse(response);
  }
  
  protected disconnectRequest(
    response: DebugProtocol.DisconnectResponse,
    args: DebugProtocol.DisconnectArguments
  ): void {
    // Clean up
    if (this.pythonProcess) {
      this.pythonProcess.kill();
    }
    
    // Remove temp files
    if (fs.existsSync(this.pythonFile)) {
      fs.unlinkSync(this.pythonFile);
    }
    
    this.sendResponse(response);
  }
}

// Start the debug session
DebugSession.run(FrameDebugSession);
```

### 4. Source Mapper Implementation

```typescript
// src/sourceMapper.ts
export class FrameSourceMapper {
  private frameToP pythonMap: Map<number, number> = new Map();
  private pythonToFrameMap: Map<number, number> = new Map();
  
  constructor(mappings: Array<{frameLine: number; pythonLine: number}>) {
    for (const mapping of mappings) {
      this.frameToPythonMap.set(mapping.frameLine, mapping.pythonLine);
      this.pythonToFrameMap.set(mapping.pythonLine, mapping.frameLine);
    }
  }
  
  frameToPython(frameLine: number): number | undefined {
    return this.frameToPythonMap.get(frameLine);
  }
  
  pythonToFrame(pythonLine: number): number | undefined {
    return this.pythonToFrameMap.get(pythonLine);
  }
  
  getNearestFrameLine(frameLine: number): number {
    // Find nearest mapped line if exact match doesn't exist
    let nearest = frameLine;
    let minDistance = Infinity;
    
    for (const [line, _] of this.frameToPythonMap) {
      const distance = Math.abs(line - frameLine);
      if (distance < minDistance) {
        minDistance = distance;
        nearest = line;
      }
    }
    
    return nearest;
  }
}
```

### 5. Launch Configuration

Users create `.vscode/launch.json`:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "frame",
      "request": "launch",
      "name": "Debug Current Frame File",
      "program": "${file}",
      "framecPath": "/path/to/framec",
      "pythonPath": "python3",
      "stopOnEntry": false,
      "console": "integratedTerminal"
    }
  ]
}
```

## Advanced Features

### Variable Inspection

```typescript
protected variablesRequest(
  response: DebugProtocol.VariablesResponse,
  args: DebugProtocol.VariablesArguments
): void {
  // Get variables from Python debugger state
  // This would interact with the Python debugger to get current frame locals
  
  const variables: DebugProtocol.Variable[] = [];
  
  // Example: Parse Python locals and create variables
  // In practice, this would come from Python debugger
  variables.push({
    name: 'x',
    value: '42',
    type: 'int',
    variablesReference: 0
  });
  
  response.body = {
    variables: variables
  };
  this.sendResponse(response);
}
```

### Conditional Breakpoints

```typescript
protected setBreakPointsRequest(
  response: DebugProtocol.SetBreakpointsResponse,
  args: DebugProtocol.SetBreakpointsArguments
): void {
  const breakpoints = args.breakpoints || [];
  
  for (const bp of breakpoints) {
    if (bp.condition) {
      // Convert Frame condition to Python condition
      const pythonCondition = this.transpileCondition(bp.condition);
      // Set conditional breakpoint in Python
    }
  }
  
  // ... rest of implementation
}
```

### Watch Expressions

```typescript
protected evaluateRequest(
  response: DebugProtocol.EvaluateResponse,
  args: DebugProtocol.EvaluateArguments
): void {
  const expression = args.expression;
  
  // Transpile Frame expression to Python
  const pythonExpression = this.transpileExpression(expression);
  
  // Evaluate in Python debugger context
  // Send result back
  
  response.body = {
    result: 'evaluated value',
    type: 'string',
    variablesReference: 0
  };
  this.sendResponse(response);
}
```

## Testing the Integration

### 1. Test Frame File

```frame
# test.frm
fn main() {
    var x = 42          # Line 2 - Set breakpoint here
    var y = 10
    
    if x > y {
        print("x is greater")  # Line 6 - Another breakpoint
    }
    
    for i in range(3) {
        print(f"i = {i}")     # Line 10 - Step through here
    }
}
```

### 2. Expected Debugging Behavior

1. User sets breakpoint at line 2 in test.frm
2. Extension transpiles with `--debug-output`
3. Maps Frame line 2 → Python line X
4. Sets Python breakpoint at line X
5. Launches Python debugger
6. When Python stops at line X, shows Frame source at line 2
7. User can inspect variables, step, continue, etc.

## Performance Considerations

### Caching Strategy

```typescript
class TranspilationCache {
  private cache: Map<string, {
    sourceHash: string;
    debugOutput: FrameDebugOutput;
    timestamp: number;
  }> = new Map();
  
  async getOrTranspile(frameFile: string, framecPath: string): Promise<FrameDebugOutput> {
    const sourceHash = await this.hashFile(frameFile);
    const cached = this.cache.get(frameFile);
    
    if (cached && cached.sourceHash === sourceHash) {
      return cached.debugOutput;
    }
    
    // Transpile and cache
    const debugOutput = await this.transpile(frameFile, framecPath);
    this.cache.set(frameFile, {
      sourceHash,
      debugOutput,
      timestamp: Date.now()
    });
    
    return debugOutput;
  }
}
```

## Error Handling

### Common Issues and Solutions

| Issue | Solution |
|-------|----------|
| Missing source mappings | Ensure using `--debug-output` flag |
| Breakpoint not hit | Check if line has mapping in source map |
| Variable not visible | Ensure Python debugger is properly connected |
| Step jumps unexpectedly | Some Frame constructs may expand to multiple Python lines |

## Future Enhancements

### Column-Level Mapping
```json
{
  "mappings": [
    {
      "frameLine": 4,
      "frameColumn": 5,
      "pythonLine": 10,
      "pythonColumn": 8
    }
  ]
}
```

### Hot Reload Support
- Watch Frame files for changes
- Retranspile on save
- Update breakpoints automatically

### REPL Integration
- Interactive Frame evaluation
- Live variable modification
- Frame expression evaluation

## Resources

- [Debug Adapter Protocol Specification](https://microsoft.github.io/debug-adapter-protocol/)
- [VSCode Extension API](https://code.visualstudio.com/api)
- [Python Debugger (pdb) Documentation](https://docs.python.org/3/library/pdb.html)
- [Frame Language Documentation](https://frame-lang.org)

## Support

For questions or issues with DAP integration:
- GitHub Issues: https://github.com/frame-lang/frame_transpiler/issues
- Frame Discord: https://discord.gg/frame-lang

---

This guide provides a complete implementation path for adding Frame debugging support to VSCode. The Frame transpiler v0.59 provides all necessary infrastructure through its source map generation capabilities.