> **⚠️ DEPRECATED - DO NOT READ UNLESS INSTRUCTED ⚠️**
>
> This document is archived and may contain outdated or incorrect information about Frame syntax.
> For current Frame V4 syntax, see `frame_v4_lang_reference.md` in the parent directory.

---

# Frame V4 Complete Documentation

**Consolidated for Review**

This document combines all Frame V4 documentation into a single file for review.

---

# Table of Contents

1. [Quick Reference](#quick-reference)
2. [Grammar](#grammar)
3. [Compiler Architecture](#compiler-architecture)
4. [Testing Infrastructure](#testing-infrastructure)
5. [Migration Guide (V3 to V4)](#migration-guide-v3-to-v4)
6. [Execution Plan](#execution-plan)

---

# Quick Reference

## V4 Directives

```frame
@@target python_3    # Required at start of every file
@@persist            # Generate serialization methods
@@system Name { }    # Declare a state machine system
```

## Runtime Statements

| Statement | Syntax | Purpose |
|-----------|--------|---------|
| Transition | `-> $State` | Change state with enter/exit lifecycle |
| Transition (pop) | `-> pop$` | Transition to state from stack |
| Forward | `=> $^` | Delegate event to parent state |
| Stack Push | `push$` | Save current state to stack |
| Stack Pop | `pop$` | Change to state from stack (no lifecycle) |

## System Structure

```frame
@@target python_3

@@system Example {
    interface:
        method()

    machine:
        $State {
            method() { }
        }

    actions:
        helper() { }

    domain:
        x = 0
}
```

---

# Grammar

This section defines the complete grammar for Frame V4.

---

## Module Level

### @@target Directive

Required at the start of every Frame module. Specifies the target language for code generation.

```
@@target <language>
```

**Supported languages:** `python_3`, `typescript`, `rust`

**Example:**
```frame
@@target python_3
```

### @@system Declaration

Declares a state machine system.

```
@@system <Identifier> {
    ...
}
```

**Example:**
```frame
@@system TrafficLight {
    interface:
        next()

    machine:
        $Red {
            next() {
                -> $Green
            }
        }
}
```

---

## System Annotations

Annotations modify how the Framepiler generates a system. They appear before `@@system`.

### @@persist

Generates JSON serialization and deserialization methods.

```
@@persist
@@persist(domain=[<field_list>])
@@persist(exclude=[<field_list>])
```

| Form | Behavior |
|------|----------|
| `@@persist` | Serialize all domain variables |
| `@@persist(domain=[f1, f2])` | Serialize only listed fields |
| `@@persist(exclude=[f1, f2])` | Serialize all except listed fields |

**Example:**
```frame
@@persist
@@system SessionManager {
    domain:
        user_id = ""
        session_token = ""
}
```

---

## System Structure

A system contains up to five sections in this order:

```
@@system <SystemName> {
    interface:
        <method_declarations>

    machine:
        <state_definitions>

    actions:
        <action_definitions>

    operations:
        <operation_definitions>

    domain:
        <variable_declarations>
}
```

### interface:

Declares public methods that dispatch events to the state machine.

```frame
interface:
    start()
    stop()
    getValue(): int
    process(data: str): bool
```

### machine:

Defines states and their event handlers.

```frame
machine:
    $Idle {
        start() {
            -> $Running
        }
    }

    $Running {
        stop() {
            -> $Idle
        }
    }
```

### actions:

Private methods callable from handlers. May not contain Frame statements.

```frame
actions:
    logEvent(msg: str) {
        print(f"Event: {msg}")
    }
```

### operations:

Utility methods not routed through the state machine. May not contain Frame statements.

```frame
operations:
    calculateHash(data: str): str {
        import hashlib
        return hashlib.md5(data.encode()).hexdigest()
    }
```

### domain:

Instance variables with optional initialization.

```frame
domain:
    count = 0
    name = "default"
    items = []
```

---

## State Declaration

### Basic State

```
$<StateName> {
    <handlers>
}
```

**Example:**
```frame
$Idle {
    start() {
        -> $Running
    }
}
```

### Hierarchical State (HSM)

A state can declare a parent using `=>`:

```
$<StateName> => $<ParentState> {
    <handlers>
}
```

**Example:**
```frame
$Active => $Base {
    // Inherits unhandled events from $Base
    specificEvent() {
        -> $Done
    }
}
```

### Default Forward (HSM)

Forward all unhandled events to parent:

```frame
$Child => $Parent {
    handled_event() {
        // explicitly handled
    }

    => $^   // forward everything else to parent
}
```

---

## Handlers

### Event Handler

```
<event_name>(<params>) {
    <body>
}
```

With return type:

```
<event_name>(<params>): <return_type> {
    <body>
}
```

**Example:**
```frame
process(data: str): bool {
    result = validate(data)
    if result:
        -> $Valid
    return result
}
```

### Enter Handler

Called when entering a state. Receives parameters from transitions.

```
$>() {
    // enter code
}

$>(<params>) {
    // enter code with parameters
}
```

**Example:**
```frame
$Processing {
    $>(task_id: str) {
        self.current_task = task_id
        print(f"Starting task: {task_id}")
    }
}
```

### Exit Handler

Called when leaving a state.

```
$<() {
    // exit code
}
```

**Example:**
```frame
$Connected {
    $<() {
        self.cleanup_connection()
    }
}
```

---

## Frame Statements

Frame V4 recognizes exactly 4 runtime statement types within handler bodies.

### 1. Transition

Transitions to a target state, invoking exit handler on current state and enter handler on target state.

**Basic:**
```
-> $<TargetState>
```

**With state parameters:**
```
-> $<TargetState>(<state_params>)
```

**With exit params** (passed to current state's exit handler):
```
(<exit_params>) -> $<TargetState>
```

**With enter params** (passed to target state's enter handler):
```
-> (<enter_params>) $<TargetState>
```

**Full form:**
```
(<exit_params>) -> (<enter_params>) $<TargetState>(<state_params>)
```

**Transition to popped state:**
```
-> pop$
```

**Examples:**
```frame
-> $Idle                      // Simple transition
-> $Active(user_id)           // With state parameter
(cleanup) -> $Shutdown        // With exit params
-> (init_val) $Running        // With enter params
-> pop$                       // Transition to saved state
```

### 2. Forward to Parent

Forwards the current event to the parent state in a hierarchical state machine.

```
=> $^
```

**Prerequisites:** The current state must have a parent declared with `$Child => $Parent`.

**Example:**
```frame
$Child => $Parent {
    knownEvent() {
        // handle locally
    }

    unknownEvent() {
        => $^   // let parent handle it
    }

    => $^       // forward all other events to parent
}
```

### 3. Stack Push

Pushes the current state onto the state stack.

```
push$
```

**Example:**
```frame
handleInterrupt() {
    push$              // Save current state
    -> $Interrupt      // Go handle interrupt
}
```

### 4. Stack Pop

Pops state from the stack. Two forms:

**As statement** (changes state without enter/exit lifecycle):
```
pop$
```

**As transition target** (full lifecycle with exit/enter):
```
-> pop$
```

**Examples:**
```frame
// Return to saved state without lifecycle
pop$

// Return to saved state with full lifecycle
-> pop$
```

---

## Native Code

Everything that is not one of the 4 Frame statements is native code and passes through unchanged.

**Example (Python target):**
```frame
process() {
    # This is native Python code
    result = compute_value()
    if result > threshold:
        -> $HighState
    else:
        log("Staying in current state")
}
```

The Frame statement (`-> $HighState`) is expanded. The Python code passes through unchanged.

---

## Statement Termination

Frame statements may optionally end with a semicolon:

```frame
-> $Next;      // Optional semicolon
-> $Next       // Also valid
push$;         // Optional semicolon
push$          // Also valid
```

---

## Complete Example

```frame
@@target python_3

from datetime import datetime

@@persist
@@system TaskProcessor {
    interface:
        submit(task: str)
        cancel()
        getStatus(): str

    machine:
        $Idle {
            $>() {
                self.started_at = None
            }

            submit(task: str) {
                self.current_task = task
                -> $Processing
            }

            getStatus(): str {
                return "idle"
            }
        }

        $Processing {
            $>() {
                self.started_at = datetime.now()
                print(f"Processing: {self.current_task}")
            }

            $<() {
                print("Stopping processor")
            }

            cancel() {
                -> $Idle
            }

            getStatus(): str {
                return f"processing: {self.current_task}"
            }
        }

    actions:
        logState(msg: str) {
            print(f"[{datetime.now()}] {msg}")
        }

    domain:
        current_task = ""
        started_at = None
}

if __name__ == '__main__':
    p = TaskProcessor()
    print(p.getStatus())
    p.submit("task-001")
    print(p.getStatus())
    p.cancel()
```

---

## Summary Tables

### Runtime Statements

| Statement | Syntax | Purpose |
|-----------|--------|---------|
| Transition | `-> $State` | Change state with enter/exit lifecycle |
| Transition (pop) | `-> pop$` | Transition to state from stack |
| Forward | `=> $^` | Delegate event to parent state |
| Stack Push | `push$` | Save current state to stack |
| Stack Pop | `pop$` | Change to state from stack (no lifecycle) |

### Token Conventions

| Token | Meaning |
|-------|---------|
| `@@` | Frame directive or annotation |
| `$` | State reference |
| `->` | Transition operator |
| `=>` | Forward operator / HSM parent declaration |

### System Sections

| Section | Purpose | Frame Statements Allowed |
|---------|---------|-------------------------|
| `interface:` | Public API | No (declarations only) |
| `machine:` | State handlers | Yes |
| `actions:` | Private methods | No |
| `operations:` | Utility methods | No |
| `domain:` | Instance variables | No (declarations only) |

---

# Compiler Architecture

This section describes the Frame V4 compiler implementation.

---

## Overview

Frame V4 is a **preprocessor** for state machine code. It:

1. **Parses** Frame syntax (`@@system`, states, transitions)
2. **Validates** Frame semantics (state exists, parameters match)
3. **Generates** target language code (Python, Rust, TypeScript)
4. **Preserves** native code exactly as written

Frame does NOT parse or validate native code. That's the target compiler's job.

---

## The Oceans Model

Frame uses the "oceans model" for mixed Frame/native code:

- **Native code is the ocean** — Preserved exactly as written
- **Frame constructs are islands** — Identified, validated, and expanded

```
Handler Body:
┌─────────────────────────────────────────────┐
│ x = compute_value()        ← Ocean (native) │
│ if x > threshold:          ← Ocean (native) │
│     -> $Exceeded           ← Island (Frame) │
│ else:                      ← Ocean (native) │
│     -> $Normal             ← Island (Frame) │
└─────────────────────────────────────────────┘
```

The Native Region Scanner finds islands. The Splicer replaces them with generated code. Everything else passes through unchanged.

---

## Two-Pass Validation Model

Frame V4 uses a two-pass validation architecture:

| Pass | When | What | Who |
|------|------|------|-----|
| **Pass 1** | Transpile-time | Frame semantics | Frame compiler |
| **Pass 2** | Compile/Run-time | Native semantics | Target compiler |

**Frame validates what only Frame knows:**
- State exists (`-> $Unknown` → E402)
- Parent exists for forward (`=> $^` without parent → E403)
- Parameter arity matches (`-> $State(a,b)` when State takes 1 param → E405)
- Terminal statements are last (code after `->` → E400)
- Sections are ordered correctly (E113)

**Native compiler validates the rest:**
- Variables exist
- Types are compatible
- Imports resolve
- Syntax is correct

---

## Compilation Pipeline

```
Source (.frm)
     │
     ▼
┌─────────────┐
│ Frame Parser │ ──→ Frame AST (systems, states, handlers)
└─────────────┘      Native code stored as spans, not parsed
     │
     ▼
┌─────────────┐
│   Arcanum   │ ──→ Symbol table (states, events, domain vars)
└─────────────┘
     │
     ▼
┌─────────────┐
│  Validator  │ ──→ Frame semantic errors (E4xx)
└─────────────┘
     │
     ▼
┌─────────────┐
│   Codegen   │ ──→ CodegenNode (language-agnostic IR)
└─────────────┘
     │
     ▼
┌─────────────┐
│   Backend   │ ──→ Target code (Python/Rust/TypeScript)
└─────────────┘
     │
     ▼
Target (.py/.rs/.ts)
```

---

## Core Components

### 1. Frame Parser (`frame_parser.rs`)

Parses Frame constructs into AST. Does NOT parse native code.

```rust
pub struct FrameParser {
    source: Vec<u8>,
    cursor: usize,
    target: TargetLanguage,
}
```

The parser:
- Identifies Frame constructs (systems, states, handlers)
- Stores Frame statements (transitions, forwards, etc.)
- Records spans for native regions (doesn't parse native code)

### 2. Arcanum — Symbol Table (`arcanum.rs`)

Tracks Frame-declared symbols for validation.

```rust
pub struct Arcanum {
    pub systems: HashMap<String, SystemEntry>,
}

pub struct SystemEntry {
    pub interface_methods: HashSet<String>,
    pub actions: HashSet<String>,
    pub operations: HashSet<String>,
    pub domain_vars: HashMap<String, FrameSymbol>,
    pub machines: HashMap<String, MachineEntry>,
}

pub struct StateEntry {
    pub name: String,
    pub params: Vec<FrameSymbol>,
    pub parent: Option<String>,
    pub handlers: HashMap<String, HandlerEntry>,
}
```

**Scope Hierarchy:**
```
System Scope
├── domain variables
├── interface methods
├── actions / operations
└── State Scope (per state)
    ├── state parameters
    └── Handler Scope (per handler)
        └── handler parameters
```

### 3. Frame Validator (`frame_validator.rs`)

Validates Frame semantics using Arcanum.

```rust
pub struct FrameValidator;

impl FrameValidator {
    pub fn validate(&self, ast: &FrameAst, arcanum: &Arcanum)
        -> Result<(), Vec<ValidationError>>;
}
```

### 4. Native Region Scanner (`native_region_scanner.rs`)

Identifies Frame "islands" within native code "oceans".

```rust
pub enum Region {
    NativeText { span: RegionSpan },     // Preserve
    FrameSegment { span: RegionSpan, kind: FrameSegmentKind },  // Expand
}

pub enum FrameSegmentKind {
    Transition,   // -> $State
    Forward,      // => $^
    StackPush,    // push$
    StackPop,     // pop$ or -> pop$
}
```

### 5. Splicer (`splice.rs`)

Combines native code with generated Frame expansions.

```rust
impl Splicer {
    pub fn splice(
        source: &[u8],
        regions: &[Region],
        expansions: &[String],
    ) -> SplicedBody;
}
```

**Algorithm:**
1. Scan handler body for Frame segments
2. Generate expansion code for each Frame segment
3. Build output: native regions verbatim, Frame segments replaced

### 6. Language Backends (`codegen/backends/`)

Emit target language code from CodegenNode.

```rust
pub trait LanguageBackend {
    fn emit(&self, node: &CodegenNode, ctx: &mut EmitContext) -> String;
    fn runtime_imports(&self) -> Vec<String>;
}
```

---

## Validation Error Codes

### Structure Errors (E1xx)

| Code | Description |
|------|-------------|
| E113 | Section ordering violation |
| E114 | Duplicate section |

### Frame Statement Errors (E4xx)

| Code | Description |
|------|-------------|
| E400 | Terminal statement must be last in handler |
| E401 | Frame statement not allowed in actions/operations |
| E402 | Unknown state in transition |
| E403 | Forward requires parent state |
| E404 | Duplicate state definition |
| E405 | Parameter arity mismatch |
| E406 | Invalid interface method call |

### Error Message Format

```
[ERROR_CODE] Message
  --> file.frm:line:column
   |
line | source code
   | ^^^ specific location
   |
   = help: Suggestion for fixing
```

---

## Code Generation

### Frame Statement Expansion

| Frame Statement | Python Expansion |
|-----------------|------------------|
| `-> $Green` | `self._transition("Green")` |
| `=> $^` | `self._forward_to_parent()` |
| `push$` | `self._state_stack.append(self._state)` |
| `pop$` | `self._state = self._state_stack.pop()` |
| `-> pop$` | `self._transition(self._state_stack.pop())` |

### Generated System Structure

**Python:**
```python
class SystemName:
    def __init__(self):
        self._state = "InitialState"
        self._state_stack = []
        # domain variables

    def _transition(self, target_state, ...):
        self._exit()
        self._state = target_state
        self._enter()

    # Interface methods dispatch to handlers
    # Handler methods contain spliced code
```

**TypeScript:**
```typescript
class SystemName {
    private _state: string;
    private _state_stack: string[];

    constructor() {
        this._state = "InitialState";
        this._state_stack = [];
    }

    private _transition(targetState: string, ...): void {
        this._exit();
        this._state = targetState;
        this._enter();
    }
}
```

---

## Native Build Integration

Frame integrates into native build toolchains as a preprocessor:

### Python
```toml
# pyproject.toml
[tool.frame]
source = "src/**/*.frm"
output = "src/generated"
```

### TypeScript
```json
{
  "scripts": {
    "prebuild": "framec compile src/**/*.frm --out src/generated",
    "build": "tsc"
  }
}
```

### Rust
```rust
// build.rs
use frame_compiler::compile_frame_files;

fn main() {
    compile_frame_files("src/**/*.frm", "src/generated");
}
```

---

## Source Maps

Frame tracks source spans throughout the pipeline, enabling source maps:

- Map generated code positions → Frame source positions
- Enable debugging Frame source in IDE
- Map native compiler errors back to Frame

Source maps are span-based (bookkeeping), not AST-based.

---

## Target Languages

### Priority (PRT)

| Language | Status | Backend |
|----------|--------|---------|
| Python 3 | Active | `backends/python.rs` |
| TypeScript | Active | `backends/typescript.rs` |
| Rust | Active | `backends/rust.rs` |

### Other Languages

| Language | Status |
|----------|--------|
| C# | Partial |
| Java | Partial |
| C | Experimental |
| C++ | Experimental |

---

## Key Design Principles

1. **Frame validates Frame.** Native compilers validate native code.
2. **Preserve native code exactly.** No reformatting, no reordering.
3. **Single pipeline.** No fallbacks, no multiple approaches.
4. **PRT first.** Python, Rust, TypeScript are priority languages.
5. **Source maps via spans.** Track positions, don't parse native.

---

# Testing Infrastructure

This section describes the testing infrastructure and test pragmas for Frame V4.

---

## Overview

Frame V4 testing uses a shared test environment with Docker-based test runners for each target language. Tests are Frame source files that compile to native code, execute, and validate behavior.

---

## Test Environment

### Directory Structure

```
framepiler_test_env/
├── common/
│   └── test-frames/
│       └── v4/
│           └── prt/           # PRT language tests
│               ├── 01_minimal.frm
│               ├── 02_interface.frm
│               └── ...
├── framepiler/
│   └── docker/
│       └── target/release/
│           └── frame-docker-runner   # Test runner binary
```

### Running Tests

```bash
# Set environment
export FRAMEPILER_TEST_ENV=$(pwd)/framepiler_test_env

# Run a specific test
frame-docker-runner python_3 01_minimal --framec ./target/release/framec

# Run all tests for a language
frame-docker-runner python_3 --all --framec ./target/release/framec

# Run tests for all PRT languages
for lang in python_3 typescript rust; do
    frame-docker-runner $lang --all --framec ./target/release/framec
done
```

---

## Test File Structure

A test file is a standard Frame source file with a test harness in native code:

```frame
@@target python_3

@@system TestSystem {
    interface:
        doSomething(): str

    machine:
        $Start {
            doSomething(): str {
                return "success"
            }
        }
}

# Test harness (native Python)
if __name__ == '__main__':
    s = TestSystem()
    result = s.doSomething()
    if result == "success":
        print("PASS: doSomething returned expected value")
    else:
        print(f"FAIL: expected 'success', got '{result}'")
        raise AssertionError("Test failed")
```

### Test Conventions

1. **Print PASS/FAIL messages** — Clear output for test runner
2. **Raise exception on failure** — Non-zero exit code signals failure
3. **Self-contained** — Each test file is independent
4. **Minimal** — Test one feature per file

---

## Test Pragmas

Test pragmas are Frame annotations that control test behavior.

### @@expect (Planned)

Declares expected compiler behavior for negative tests.

```frame
@@expect(error: E402)
@@target python_3

@@system BadSystem {
    machine:
        $Start {
            go() {
                -> $NonExistent   // Should produce E402
            }
        }
}
```

| Parameter | Description |
|-----------|-------------|
| `error: E###` | Expect specific error code |
| `warning: W###` | Expect specific warning |
| `success` | Expect successful compilation |

### @@skip (Planned)

Skip test for specific languages or conditions.

```frame
@@skip(languages: [rust])
@@target python_3

@@system PythonOnly {
    // Test uses Python-specific features
}
```

---

## Test Categories

### 1. Structural Tests

Validate Frame parsing and system structure.

```frame
@@target python_3

@@system StructureTest {
    interface:
        method1()

    machine:
        $State1 {
            method1() { }
        }

    actions:
        helper() { }

    domain:
        x = 0
}
```

### 2. Transition Tests

Validate state transitions and lifecycle.

```frame
@@target python_3

@@system TransitionTest {
    interface:
        go()
        getState(): str

    machine:
        $A {
            $>() { self.log.append("enter A") }
            $<() { self.log.append("exit A") }
            go() { -> $B }
            getState(): str { return "A" }
        }

        $B {
            $>() { self.log.append("enter B") }
            getState(): str { return "B" }
        }

    domain:
        log = []
}

if __name__ == '__main__':
    t = TransitionTest()
    assert t.getState() == "A"
    t.go()
    assert t.getState() == "B"
    assert t.log == ["enter A", "exit A", "enter B"]
    print("PASS")
```

### 3. HSM Tests

Validate hierarchical state machine features.

```frame
@@target python_3

@@system HSMTest {
    interface:
        event1()
        event2()

    machine:
        $Parent {
            event1() {
                self.handled_by = "parent"
            }
        }

        $Child => $Parent {
            event2() {
                self.handled_by = "child"
            }

            => $^   // Forward unhandled to parent
        }

    domain:
        handled_by = ""
}
```

### 4. Stack Tests

Validate push/pop operations.

```frame
@@target python_3

@@system StackTest {
    interface:
        push_and_go()
        return_back()
        getState(): str

    machine:
        $Main {
            push_and_go() {
                push$
                -> $Temporary
            }
            getState(): str { return "Main" }
        }

        $Temporary {
            return_back() {
                -> pop$
            }
            getState(): str { return "Temporary" }
        }
}
```

### 5. Native Code Tests

Validate native code preservation.

```frame
@@target python_3

import json

@@system NativeTest {
    interface:
        process(data: str): dict

    machine:
        $Ready {
            process(data: str): dict {
                # All of this is native Python
                parsed = json.loads(data)
                result = {
                    "count": len(parsed),
                    "keys": list(parsed.keys())
                }
                return result
            }
        }
}
```

### 6. Persistence Tests

Validate `@@persist` code generation.

```frame
@@target python_3

@@persist
@@system PersistTest {
    interface:
        setValue(v: int)
        getValue(): int

    machine:
        $Active {
            setValue(v: int) {
                self.value = v
            }
            getValue(): int {
                return self.value
            }
        }

    domain:
        value = 0
}

if __name__ == '__main__':
    p1 = PersistTest()
    p1.setValue(42)

    # Save state
    snapshot = p1._save()

    # Restore to new instance
    p2 = PersistTest._restore(snapshot)
    assert p2.getValue() == 42
    print("PASS")
```

---

## Validation Tests

Tests that verify compiler error detection.

### Expected Error Tests

```frame
@@expect(error: E402)
@@target python_3

@@system UnknownStateTest {
    machine:
        $Start {
            go() {
                -> $DoesNotExist
            }
        }
}
```

### Expected Success Tests

```frame
@@expect(success)
@@target python_3

@@system ValidSystem {
    machine:
        $Start {
            go() {
                -> $End
            }
        }
        $End { }
}
```

---

## Test Runner Output

```
Running tests for python_3...
  01_minimal ................ PASS
  02_interface .............. PASS
  03_transition ............. PASS
  04_native_code ............ PASS
  05_enter_exit ............. PASS
  06_domain_vars ............ PASS
  07_params ................. PASS
  08_hsm .................... PASS
  09_stack .................. PASS

Total: 9 passed, 0 failed
```

---

## Writing New Tests

### 1. Create Test File

```bash
touch framepiler_test_env/common/test-frames/v4/prt/10_new_feature.frm
```

### 2. Write Frame Code

```frame
@@target python_3

@@system NewFeatureTest {
    // Test the new feature
}

if __name__ == '__main__':
    # Test harness
    t = NewFeatureTest()
    # assertions
    print("PASS")
```

### 3. Run Test

```bash
frame-docker-runner python_3 10_new_feature --framec ./target/release/framec
```

### 4. Add to All Languages

Copy and adapt for TypeScript and Rust targets.

---

## Debugging Failed Tests

### 1. View Generated Code

```bash
./target/release/framec test.frm -l python_3 -o /tmp/test.py
cat /tmp/test.py
```

### 2. Run Manually

```bash
python3 /tmp/test.py
```

### 3. Check Compiler Output

```bash
./target/release/framec test.frm -l python_3 --verbose
```

---

## CI Integration

Tests run automatically on PR/push:

```yaml
# .github/workflows/test.yml
- name: Run PRT Tests
  run: |
    for lang in python_3 typescript rust; do
      frame-docker-runner $lang --all --framec ./target/release/framec
    done
```

---

# Migration Guide (V3 to V4)

This section covers migrating Frame code from V3 syntax to V4 syntax.

## Quick Reference

| V3 Syntax | V4 Syntax |
|-----------|-----------|
| `module Name { }` | `@@system Name { }` |
| `fn name() { }` (standalone) | Native function (outside @@system) |
| `import X from "./file.frm"` | Native import |
| `:> $State` | Not supported - use `->` |
| `#SystemName ... ##` | `@@system SystemName { }` |
| `.frm` extension | `.frm` (or language-specific) |

## Syntax Changes

### System Declaration

**V3:**
```
module Calculator {
    ...
}
```

**V4:**
```
@@system Calculator {
    ...
}
```

### Target Declaration

**V3:**
```
@@target python
```

**V4:**
```
@@target python_3
```

Note: V4 uses `python_3` not `python`.

### Standalone Functions

**V3:**
```
fn helperFunction(x) {
    return x * 2
}

async fn asyncHelper() {
    await doSomething()
}

module MySystem {
    ...
}
```

**V4:**
```
@@target python_3

def helper_function(x):
    return x * 2

async def async_helper():
    await do_something()

@@system MySystem {
    ...
}
```

Standalone `fn` functions become native functions in the preamble.

### Frame Imports

**V3:**
```
import Collections from "./collections.frm"
import Errors from "./errors.frm"

module MySystem {
    machine:
        $Start {
            process() {
                var list = Collections.createList()
            }
        }
}
```

**V4:**

Frame imports are not supported. Options:

1. **Inline the code** - Copy required functionality into your system
2. **Use native imports** - Convert Frame modules to native modules

```
@@target python_3

# Use native Python modules instead
from collections import deque
from dataclasses import dataclass

@@system MySystem {
    machine:
        $Start {
            process() {
                items = deque()
            }
        }
}
```

### Change State Operator

**V3:**
```
:> $NewState
```

**V4:**

The `:>` operator is not supported. Use `->` (transition).

```
-> $NewState     # Transition with exit/enter handlers
```

Note: V4 does not have a "change state without lifecycle" operator. All transitions invoke exit/enter handlers.

### Event Handler Syntax

**V3:**
```
$State {
    |event| {
        // handler
    }
}
```

**V4:**
```
$State {
    event() {
        // handler
    }
}
```

The pipe syntax `|event|` is not supported. Use method-style `event()`.

### Parameters

**V3:**
```
|event|[x, y]| {
    // use x, y
}
```

**V4:**
```
event(x: type, y: type) {
    // use x, y
}
```

## Feature Differences

### Not Supported in V4

| V3 Feature | V4 Alternative |
|------------|----------------|
| Frame modules (`module`) | Use `@@system` |
| Frame imports | Use native imports |
| Standalone `fn` | Native functions |
| `:>` change state | Use `->` (V4 has no lifecycle-free change) |
| Frame constants | Native constants |
| Frame enums | Native enums |
| Pipe event syntax | Method syntax |

### New in V4

| Feature | Description |
|---------|-------------|
| `@@persist` | Persistence code generation |
| Native type passthrough | Any native type syntax works |
| HSM `=> $^` | Forward to parent state |
| State stack `push$` / `pop$` | Push/pop state |

## Migration Process

### Step 1: Update Target

```diff
- @@target python
+ @@target python_3
```

### Step 2: Convert Module to System

```diff
- module Calculator {
+ @@system Calculator {
```

### Step 3: Move Standalone Functions to Preamble

Move `fn` declarations outside `@@system` and convert to native syntax.

### Step 4: Replace Frame Imports

Convert Frame module imports to native imports or inline the functionality.

### Step 5: Update Event Syntax

```diff
  $State {
-     |processData|[items]| {
+     processData(items: list) {
```

### Step 6: Replace :> Operator

```diff
- :> $NewState
+ -> $NewState
```

### Step 7: Test Compilation

```bash
./target/release/framec compile -l python_3 -o output/ your_file.frm
```

## Example Migration

### V3 Code

```
@@target python

import Errors from "./errors.frm"

fn validate(data) {
    if not data:
        return Errors.createError("Empty data")
    return Errors.createOk(data)
}

module Processor {
    domain:
        var result = None

    interface:
        process(data)
        getResult()

    machine:
        $Idle {
            |process|[data]| {
                var validation = validate(data)
                if Errors.isOk(validation) {
                    :> $Processing
                } else {
                    print("Validation failed")
                }
            }

            |getResult|| {
                return self.result
            }
        }

        $Processing {
            |process|[data]| {
                self.result = data.upper()
                -> $Done
            }
        }

        $Done {
            |getResult|| {
                return self.result
            }
        }
}
```

### V4 Code

```
@@target python_3

def validate(data):
    if not data:
        return {"ok": False, "error": "Empty data"}
    return {"ok": True, "value": data}

@@system Processor {
    domain:
        var result = None

    interface:
        process(data)
        getResult()

    machine:
        $Idle {
            process(data) {
                validation = validate(data)
                if validation["ok"]:
                    -> $Processing
                else:
                    print("Validation failed")
            }

            getResult() {
                return self.result
            }
        }

        $Processing {
            process(data) {
                self.result = data.upper()
                -> $Done
            }
        }

        $Done {
            getResult() {
                return self.result
            }
        }
}
```

## Test Migration

V3 tests can be migrated by:

1. Updating syntax as described above
2. Converting Frame module tests to native tests
3. Running through V4 compiler

Tests using only `@@system` syntax with native code in handlers should work with minimal changes.

---

# Execution Plan

**Status:** Active Development
**Last Updated:** 2026-02-13
**Prerequisites:** V4 is already default with working PRT backends

---

## Current State Assessment

### What's Complete

| Component | Status | Notes |
|-----------|--------|-------|
| V4 as default | DONE | No `FRAME_USE_V4` needed |
| Frame Parser | DONE | 55KB, handles all syntax |
| Arcanum Symbol Table | DONE | 28KB, scope resolution working |
| Python Backend | DONE | Full featured, tested |
| Rust Backend | DONE | Full featured, tested |
| TypeScript Backend | DONE | Full featured, tested |
| Native Code Preservation | DONE | Oceans model working |
| Splicer | DONE | Combines native + generated |

### What's Partially Complete

| Component | Status | Gap |
|-----------|--------|-----|
| Validation | 14 codes | 25+ more planned |
| Source Maps | Design only | Not implemented |
| C#/Java/C/C++ Backends | Stubs | Low priority |

### Current Error Codes (14 total)

**Structural (E1xx):**
- E111: Duplicate system parameter
- E113: System blocks out of order
- E114: Duplicate section blocks
- E115: Multiple `fn main` functions

**Semantic (E4xx):**
- E400: Transition must be last statement
- E401: Frame statements not allowed in actions/operations
- E402: Unknown state in transition
- E403: Invalid parent forwards
- E404: Handler body must be inside state
- E405: State parameter arity mismatch
- E406: Invalid interface method calls
- E416: Start params must match start state
- E417: Enter handler params mismatch
- E418: Domain param has no matching variable

---

## Phase 1: Validation Expansion (Priority: HIGH)

**Goal:** Implement 25+ additional error codes

**Timeline:** 6 weeks

### Week 1: Foundation & Infrastructure

Tasks:
1. Create `validation/` module structure with `ValidationPass` trait
2. Implement `ValidationRunner` to orchestrate passes
3. Implement `ValidationReport` with output formats (human, JSON, IDE)
4. Migrate existing 14 codes to new system
5. Add CLI flags: `--warn-as-error`, `--suppress=E4XX`

Files to modify:
- `v4/frame_validator.rs` → refactor into modular passes
- `v4/pipeline/config.rs` → add validation config options
- `framec/src/frame_c/cli.rs` → add CLI flags

### Week 2: Structural Checks (E41x)

New error codes:
- E410: `unreachable-state` - No incoming transitions
- E411: `dead-end-state` - No outgoing transitions
- E412: `orphan-state` - Isolated state
- E413: `missing-start-state` - No start state
- E414: `empty-machine` - Machine has no states

Tasks:
1. Implement `TransitionGraph` builder from Arcanum
2. Implement `StructuralPass` with reachability analysis
3. Add tests in `test-frames/v4/validation/structural/`

### Week 3: Event Handling Checks (E42x)

New error codes:
- E420: `unhandled-interface-event` - Interface method not handled
- E421: `unhandled-event-in-state` - No handler, no parent
- E422: `dead-handler` - Handler never called
- E423: `interface-event-mismatch` - Signature mismatch
- E424: `shadowed-handler` - Child shadows parent (info)

Tasks:
1. Implement `EventPass` with interface/handler matching
2. Wire to Arcanum for cross-referencing
3. Add tests

### Week 4: Transition Checks (E43x)

New error codes:
- E430: `self-transition-no-effect` - No enter/exit handlers
- E431: (exists) Transition after transition
- E432: `conditional-transition-incomplete` - Some branches missing
- E433: `transition-in-enter-handler` - Potential infinite loop
- E434: `transition-target-params` - Missing required params

Tasks:
1. Implement `TransitionPass` with control flow analysis
2. Add basic branch analysis for conditionals
3. Add tests

### Week 5: HSM Checks (E44x)

New error codes:
- E440: `hsm-cycle` - Parent chain has cycle
- E441: `hsm-depth-exceeded` - Too deep (>10 levels)
- E442: `forward-no-parent` - `=> $^` without parent
- E443: `forward-unhandled` - Parent doesn't handle
- E444: `orphan-parent` - Parent doesn't exist

Tasks:
1. Implement `HsmPass` with parent chain resolution
2. Add cycle detection
3. Add tests

### Week 6: Stack & Domain Checks (E45x, E46x)

New error codes:
- E450: `pop-without-push` - Stack underflow
- E451: `push-without-pop` - Stack leak
- E452: `stack-in-enter-exit` - Dangerous pattern
- E460: `undefined-domain-var` - Reference to unknown var
- E461: `unused-domain-var` - Declared but unused

Tasks:
1. Implement `StackPass` with interprocedural analysis
2. Implement `DomainPass` for variable tracking
3. Add tests

---

## Phase 2: Source Maps (Priority: MEDIUM)

**Goal:** Enable Frame source debugging in VS Code

**Timeline:** 2 weeks

### Design Decisions

1. **Format:** Use JavaScript source map v3 format (industry standard)
2. **Granularity:** Map at statement level (not expression)
3. **Coverage:** Frame constructs only (native code maps to itself)

### Week 1: Implementation

Tasks:
1. Create `codegen/source_map.rs` module
2. Track spans during splicer operation
3. Generate source map JSON
4. Emit `.map` files alongside generated code

Data structure:
```rust
pub struct SourceMapEntry {
    pub generated_line: u32,
    pub generated_col: u32,
    pub source_line: u32,
    pub source_col: u32,
    pub name: Option<String>,
}

pub struct SourceMap {
    pub file: String,
    pub source_root: String,
    pub sources: Vec<String>,
    pub mappings: Vec<SourceMapEntry>,
}
```

### Week 2: Integration

Tasks:
1. Wire to CLI with `--source-map` flag
2. Test with VS Code debugger
3. Document usage

---

## Phase 3: Testing & Verification (Priority: HIGH)

**Goal:** Comprehensive test coverage for V4

**Timeline:** Ongoing

### Test Categories

1. **Validation Tests** (`test-frames/v4/validation/`)
   - Each error code gets 3+ test cases
   - Positive (error should fire) and negative (should pass)

2. **Backend Tests** (`test-frames/v4/prt/`)
   - Parallel tests for Python, Rust, TypeScript
   - Same Frame source → verify all backends produce runnable code

3. **Regression Tests** (`test-frames/v4/regression/`)
   - Fixed bugs should never reoccur

### Test Infrastructure

Location: `framepiler_test_env/common/test-frames/v4/`

Structure:
```
v4/
├── validation/
│   ├── structural/
│   ├── events/
│   ├── transitions/
│   ├── hsm/
│   └── stack_domain/
├── prt/
│   ├── basic_transition.frm
│   ├── interface_methods.frm
│   ├── enter_exit.frm
│   ├── hsm_forward.frm
│   └── stack_ops.frm
└── regression/
```

---

## Phase 4: Documentation (Priority: MEDIUM)

**Goal:** User-facing documentation for V4

### Documents to Create/Update

1. **Migration Guide** - V3 to V4 differences
2. **Error Reference** - All error codes with examples
3. **Language Reference** - Complete Frame syntax
4. **IDE Integration** - VS Code setup with source maps

---

## Success Criteria

| Metric | Target |
|--------|--------|
| Error codes | 40+ (from 14) |
| Test coverage | Each error code has 3+ tests |
| False positive rate | <1% on existing passing tests |
| Performance | Validation adds <10% to transpile time |
| PRT tests | 100% pass rate |
| Source maps | Work in VS Code debugger |

---

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| False positives annoy users | Conservative severity; easy `--suppress` |
| Control flow analysis complex | Start simple; iterate |
| Performance regression | Lazy evaluation; skip irrelevant passes |
| Breaking existing workflows | Warnings don't block; new errors are warnings first |

---

## Deferred (V5+)

The following are explicitly out of scope for V4:

1. **Native code analysis** - V5 will add optional cross-environment symbol resolution
2. **Type checking across boundaries** - V5 scope
3. **Multi-file projects** - V5 scope
4. **C#/Java/C/C++ backends** - Low priority, complete when needed

Future work will be documented in V5 planning documents.

---

## Getting Started

To begin work:

```bash
# Build transpiler
cargo build --release -p framec

# Run existing tests
cd framepiler_test_env
./framepiler/docker/target/release/frame-docker-runner python_3 v4_*

# Run validation-only
./target/release/framec test.frm --validate-only
```

---

*Document created: 2026-02-13*
