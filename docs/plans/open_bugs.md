# Frame Transpiler Open Bugs

<!-- NEXT BUG NUMBER: #49 -->

**Last Updated:** 2025-10-13  
**Current Version:** v0.82.2  
**Test Status:** 🎉 **100% PASS RATE** (397/397 tests passing)  
**Active Bugs:** 2  
**Resolved Bugs:** 48 (See closed_bugs.md for full history)  

## Active Bugs

### Bug #39: Missing Frame Semantic Metadata for Debugger Integration

**Discovered**: 2025-10-12  
**Severity**: Medium  
**Component**: Debug Output Generator (framec v0.81.4)  
**Reporter**: VS Code Extension v0.11.9 Frame Debug Adapter Testing  

**Description**:
The Frame transpiler's `--debug-output` JSON format lacks semantic metadata about Frame language constructs, forcing debuggers to parse generated Python code to infer Frame structure. This creates fragile, implementation-dependent debugging that breaks when Python generation changes.

**Current Debug Output**:
```json
{
  "python": "# Generated Python code...",
  "sourceMap": { "mappings": [...] },
  "metadata": {
    "frameVersion": "0.81.4",
    "generatedAt": "2025-10-12T00:38:02.354033+00:00",
    "checksum": "sha256:..."
  }
}
```

**Needed Frame Semantic Metadata**:
```json
{
  "metadata": {
    "frameVersion": "0.81.4",
    "systems": [
      {
        "name": "HelloWorld",
        "states": [
          {"name": "$Start", "pythonHandler": "__helloworld_state_Start"},
          {"name": "$End", "pythonHandler": "__helloworld_state_End"}
        ],
        "interfaceMethods": [
          {
            "name": "print_it", 
            "pythonMethod": "print_it",
            "implementations": [
              {"state": "$Start", "pythonHandler": "__handle_start_print_it", "frameLine": 10}
            ]
          }
        ],
        "enterHandlers": [
          {"state": "$Start", "pythonHandler": "__handle_start_enter", "frameLine": 12}
        ],
        "stateTransitions": [
          {"from": "$Start", "to": "$End", "event": "print_it", "frameLine": 11}
        ]
      }
    ]
  }
}
```

**Impact**:
- **Fragile Debugging**: Python parser in VS Code extension breaks when generation changes
- **Missing Context**: Debugger can't show Frame state machine structure
- **Poor UX**: Users see Python internals instead of Frame semantics

**Proposed Solution**:
Add semantic metadata to `--debug-output` JSON that describes:
1. System structure (states, transitions, interface methods)
2. Python-to-Frame mappings for runtime debugging
3. State machine topology for visualization

---

### Bug #37: State Diagram Generation Missing Conditional Transitions

**Discovered**: 2025-10-11  
**Severity**: Low  
**Component**: State Diagram Generator (framec v0.81.4)  
**Reporter**: VS Code Extension v0.11.4 Frame Debug Adapter Testing  

**Description**:
The state diagram generation is missing conditional transition arrows from states with `if/else` branching logic. Specifically, in the Frame Debug Adapter state machine, the transition from `$Configuring` to `$WaitingForEntry` (when `stopOnEntry` is true) is not shown in the generated state diagram.

**Frame Code**:
```frame
$Configuring {
    onRuntimeReady() {
        if self.stopOnEntry {
            -> $WaitingForEntry    // This transition is missing from diagram
        } else {
            -> $Running
        }
    }
}
```

**Expected**: State diagram should show conditional transition arrow from `Configuring` to `WaitingForEntry`  
**Actual**: `WaitingForEntry` state appears unreachable in the diagram

**Impact**: 
- Makes state machines harder to understand and debug
- Valid state transitions appear missing
- Confuses developers about actual state machine behavior

**Possible Cause**:
The GraphViz visitor may not be traversing into if/else statement blocks to find transition statements.

**Files to Investigate**:
- `framec/src/frame_c/visitors/graphviz_visitor.rs`
- Look for `visit_if_stmt_node` and how it handles nested transitions

---

## Recently Resolved

See `closed_bugs.md` for complete history of resolved bugs including:
- Bug #48: TypeScript generation complex expression support (v0.82.2)
- Bug #46: Python import support (Won't Fix - feature already exists)
- Bug #40: Interface method source mapping
- Bug #38: String concatenation with escape sequences
- Bug #36: Interface method source mappings
- Bug #35: Source mapping classification
- And 42 more resolved issues...