# Frame V4 Complete Documentation

**Consolidated for Review**

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
@@persist            # Generate JSON serialization methods
@@system Name { }    # Declare a state machine system
```

## Runtime Statements

| Statement | Syntax | Purpose |
|-----------|--------|---------|
| Transition | `-> $State` | Change state with enter/exit lifecycle |
| Transition (pop) | `-> pop$` | Transition to popped state (with lifecycle) |
| Forward | `=> $^` | Delegate event to parent state |
| Stack Push | `push$` | Save current state (or explicit state) to stack |
| Stack Pop | `pop$` | Pop state from stack (value producer) |

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

    operations:
        utility(): str { }

    domain:
        x = 0
}
```

---

# Grammar

This section defines the complete grammar for Frame V4.

## Design Principles

**`@@` means "Frame is talking."** The `@@` token marks any Frame construct — directives, declarations, annotations. It appears at module level (`@@target`), before systems (`@@persist`), or anywhere Frame needs to communicate with the Framepiler. The only place `@@` is not needed is inside handler bodies (code blocks), where the four runtime statements are already syntactically distinct.

**Inside code blocks, Frame syntax is inherently distinct.** The `$` symbol, `->` and `=>` operators, and `push$`/`pop$` keywords cannot collide with any target language. No attention token is needed.

**Frame recognizes exactly 4 runtime statements.** Everything else within a handler body is native code in the target language and passes through unchanged.

---

## Module Level

### @@target Directive

Required at the start of every Frame module. Specifies the target language for code generation.

```
@@target <language>
```

**Supported languages:** `python_3`, `typescript`, `rust`

### @@system Declaration

Declares a state machine system.

```
@@system <Identifier> {
    ...
}
```

---

## System Annotations

Annotations modify how the Framepiler generates a system. They appear before `@@system` and follow the pattern `@@keyword` or `@@keyword(params)`. Multiple annotations can be stacked.

```frame
@@persist
@@system SessionManager {
    ...
}
```

With parameters:

```frame
@@persist(exclude=[temp_cache, debug_mode])
@@system SessionManager {
    ...
}
```

Multiple annotations:

```frame
@@persist
@@async
@@system SessionManager {
    ...
}
```

### @@persist

Generates JSON serialization and deserialization methods for the system. The Framepiler knows the complete inventory of states, domain variables, and stack structure, and generates idiomatic save/restore code for the target language.

**Syntax:**

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

**Snapshot format:**

All targets produce and consume the same language-neutral JSON schema:

```json
{
    "schemaVersion": 1,
    "systemName": "Counter",
    "state": "Active",
    "stateParams": {},
    "domain": {
        "count": 5,
        "label": "main"
    },
    "stack": []
}
```

| Field | Type | Description |
|-------|------|-------------|
| `schemaVersion` | number | Format version for migration support |
| `systemName` | string | Name of the Frame system |
| `state` | string | Current state name (without `$`) |
| `stateParams` | object | Parameters passed to current state |
| `domain` | object | Domain variable values |
| `stack` | array | Saved states from `push$` operations |

**Generated methods by target:**

Python:
```python
def _save(self) -> str        # Returns JSON string
@classmethod
def _restore(cls, data: str)  # Returns new instance from JSON
```

TypeScript:
```typescript
save(): string                        // Returns JSON string
static restore(data: string): System  // Returns new instance from JSON
```

Rust:
```rust
fn save(&self) -> String                          // Returns JSON string (serde_json)
fn restore(data: &str) -> Result<Self, Error>     // Returns new instance from JSON
```

**Example:**

```frame
@@target python_3

@@persist(exclude=[temp_cache])
@@system Counter {
    machine:
        $Active {
            increment() {
                self.count += 1
            }
        }

    domain:
        count: int = 0
        label: str = "default"
        temp_cache: list = []       // NOT persisted
}
```

The Framepiler generates `_save()` and `_restore()` methods that serialize `count` and `label` but omit `temp_cache`. What the developer does with the JSON string — store it in Redis, write it to a file, send it over a network — is their concern. Frame's responsibility ends at the serialization boundary.

**Cross-language restore:** Because the snapshot format is language-neutral JSON, a system saved by Python can be restored by TypeScript or Rust, provided the system definition matches.

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

### Hierarchical State (HSM)

A state can declare a parent using `=>`:

```
$<StateName> => $<ParentState> {
    <handlers>
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

### Exit Handler

Called when leaving a state.

```
$<() {
    // exit code
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

This combines the transition operator with `pop$` as a value producer. The full transition lifecycle (exit/enter handlers) is invoked.

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

Pushes a state onto the state stack.

**Push current state (default):**
```
push$
```

**Push an explicit state:**
```
push$ $<StateName>
```

**Examples:**
```frame
handleInterrupt() {
    push$              // Save current state
    -> $Interrupt      // Go handle interrupt
}

setupFallback() {
    push$ $SafeMode    // Push a specific state as fallback
}
```

### 4. Stack Pop

Pops the top state from the state stack and returns it as a value. `pop$` is a **value producer** — it is not fused to a transition and does not change the current state on its own.

The idiomatic usage is as a transition target:

```frame
resume() {
    -> pop$            // Transition to saved state (full lifecycle)
}
```

A bare `pop$` statement pops and discards the value. This is useful for cleaning up the stack without transitioning:

```frame
cleanup() {
    pop$               // Discard top of stack
    pop$               // Discard another
}
```

---

## Native Code

Everything that is not one of the 4 Frame statements is native code and passes through unchanged. Native code is written in the target language.

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
| Transition (pop) | `-> pop$` | Transition to popped state (with lifecycle) |
| Forward | `=> $^` | Delegate event to parent state |
| Stack Push | `push$` | Save current state (or explicit state) to stack |
| Stack Pop | `pop$` | Pop state from stack (value producer) |

### Token Conventions

| Token | Meaning |
|-------|---------|
| `@@` | Frame is talking — directives, declarations, annotations |
| `$` | State — state names, enter/exit, push/pop |
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

### Annotations

| Annotation | Level | Purpose |
|------------|-------|---------|
| `@@target` | Module | Specify target language |
| `@@system` | Module | Declare a state machine |
| `@@persist` | System | Generate JSON serialization/deserialization |

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

The parser identifies Frame constructs (systems, states, handlers), stores Frame statements (transitions, forwards, push/pop), and records spans for native regions without parsing native code.

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
    NativeText { span: RegionSpan },
    FrameSegment { span: RegionSpan, kind: FrameSegmentKind },
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

**Algorithm:** Scan handler body for Frame segments, generate expansion code for each, build output with native regions verbatim and Frame segments replaced.

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
| E111 | Duplicate system parameter |
| E113 | Section ordering violation |
| E114 | Duplicate section |
| E115 | Multiple `fn main` functions |

### Frame Statement Errors (E4xx)

| Code | Description |
|------|-------------|
| E400 | Terminal statement must be last in handler |
| E401 | Frame statement not allowed in actions/operations |
| E402 | Unknown state in transition |
| E403 | Invalid parent forwards |
| E404 | Handler body must be inside state |
| E405 | State parameter arity mismatch |
| E406 | Invalid interface method calls |
| E416 | Start params must match start state |
| E417 | Enter handler params mismatch |
| E418 | Domain param has no matching variable |

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
| `-> pop$` | `self._transition(self._state_stack.pop())` |
| `=> $^` | `self._forward_to_parent()` |
| `push$` | `self._state_stack.append(self._state)` |
| `push$ $Fallback` | `self._state_stack.append("Fallback")` |
| `pop$` | `self._state_stack.pop()` |

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

### Persistence Codegen

When `@@persist` is present, the Framepiler generates `_save()` and `_restore()` methods by enumerating the system's domain variables, state, and stack. See [@@persist](#persist) in the Grammar section for the full specification.

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

Frame tracks source spans throughout the pipeline, enabling source maps. Maps generated code positions to Frame source positions, enables debugging Frame source in IDE, and maps native compiler errors back to Frame. Source maps are span-based (bookkeeping), not AST-based.

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

Frame V4 testing uses a shared test environment with Docker-based test runners for each target language (for CI) and local test paths for development iteration. Tests are Frame source files that compile to native code, execute, and validate behavior.

---

## Test Environment

### Directory Structure

```
framepiler_test_env/
├── common/
│   └── test-frames/
│       └── v4/
│           ├── prt/           # PRT language tests
│           │   ├── 01_minimal.frm
│           │   ├── 02_interface.frm
│           │   └── ...
│           ├── validation/    # Compiler error tests
│           └── regression/    # Bug regression tests
├── framepiler/
│   └── docker/
│       └── target/release/
│           └── frame-docker-runner   # CI test runner
```

### Running Tests

**Local development (fast iteration):**
```bash
# Compile and run a single test directly
./target/release/framec test.frm -l python_3 -o /tmp/test.py
python3 /tmp/test.py

# Validation-only (no codegen)
./target/release/framec test.frm --validate-only
```

**Docker (CI / full suite):**
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
        pushAndGo()
        returnBack()
        getState(): str

    machine:
        $Main {
            pushAndGo() {
                push$
                -> $Temporary
            }
            getState(): str { return "Main" }
        }

        $Temporary {
            returnBack() {
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

### 3. Run Locally First

```bash
./target/release/framec test.frm -l python_3 -o /tmp/test.py
python3 /tmp/test.py
```

### 4. Run via Docker

```bash
frame-docker-runner python_3 10_new_feature --framec ./target/release/framec
```

### 5. Add to All Languages

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
| `$$[+]` | `push$` |
| `$$[-]` | `pop$` / `-> pop$` |
| `#SystemName ... ##` | `@@system SystemName { }` |
| `\|event\|` handler syntax | `event()` method syntax |
| `.frm` extension | `.frm` (or language-specific) |

**Breaking limitation:** V3 Frame imports (`import X from "./file.frm"`) have no V4 equivalent. V4 does not support multi-file Frame projects. If your V3 codebase uses inter-module Frame dependencies, you will need to restructure: either inline the shared functionality into each system, or extract it into native modules that are imported using the target language's import system. This is a significant architectural change for large V3 codebases.

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

module MySystem {
    ...
}
```

**V4:**
```
@@target python_3

def helper_function(x):
    return x * 2

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

Frame imports are not supported. Use native imports instead:

```
@@target python_3

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

### Stack Operations

**V3:**
```
$$[+]          // Push current state
$$[-]          // Pop and go to state
```

**V4:**
```
push$          // Push current state
-> pop$        // Pop and transition (with lifecycle)
pop$           // Pop and discard (no transition)
```

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
| Frame constants | Native constants |
| Frame enums | Native enums |
| Pipe event syntax | Method syntax |

### New in V4

| Feature | Description |
|---------|-------------|
| `@@persist` | JSON persistence code generation |
| `push$ $StateName` | Push explicit state to stack |
| `-> pop$` | Transition to popped state (with lifecycle) |
| Native type passthrough | Any native type syntax works |
| HSM `=> $^` | Forward to parent state |

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

Convert Frame module imports to native imports or inline the functionality. See "Breaking limitation" note above for large codebases.

### Step 5: Update Event Syntax

```diff
  $State {
-     |processData|[items]| {
+     processData(items: list) {
```

### Step 6: Update Stack Operations

```diff
- $$[+]
+ push$

- $$[-]
+ -> pop$
```

### Step 7: Test Compilation

```bash
./target/release/framec compile -l python_3 -o output/ your_file.frm
```

---

# Execution Plan

**Status:** Active Development
**Last Updated:** 2026-02-15
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
| Validation | 14 codes | Expansion planned |
| `@@persist` codegen | Design complete | Implementation needed |
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

## Phase 1: Persistence Codegen (Priority: HIGH)

**Goal:** Implement `@@persist` code generation for all PRT languages

**Timeline:** 2 weeks

### Week 1: Core Implementation

Tasks:
1. Parse `@@persist` annotation (with optional `domain=[]` / `exclude=[]` params)
2. Generate `_save()` method: enumerate domain vars, state, stack → JSON
3. Generate `_restore()` class method: JSON → new instance
4. Python backend implementation and tests

### Week 2: PRT Completion

Tasks:
1. TypeScript backend (`save()`/`static restore()` with `JSON.stringify`/`parse`)
2. Rust backend (`save()`/`restore()` with `serde_json`)
3. Selective persistence tests (domain/exclude filtering)
4. Cross-language snapshot compatibility tests

---

## Phase 2: Validation Expansion (Priority: HIGH)

**Goal:** Add high-value validation codes that catch real bugs

Validation codes are organized into two tiers based on their practical value.

### Tier 1: Bug Catchers (Priority: HIGH)

These catch actual bugs in state machine designs. Implement first.

| Code | Name | Description | Severity |
|------|------|-------------|----------|
| E413 | `missing-start-state` | No start state defined | Error |
| E414 | `empty-machine` | Machine has no states | Error |
| E440 | `hsm-cycle` | Parent chain has cycle | Error |
| E442 | `forward-no-parent` | `=> $^` without parent | Error |
| E405 | (enhance) | State parameter arity (already exists, improve messages) | Error |
| E420 | `unhandled-interface-event` | Interface method not handled by any state | Warning |
| E423 | `interface-event-mismatch` | Interface/handler signature mismatch | Error |
| E434 | `transition-target-params` | Missing required state params in transition | Error |

**Timeline:** 3-4 weeks

Tasks:
1. Create `validation/` module structure with `ValidationPass` trait
2. Implement `ValidationRunner` to orchestrate passes
3. Add CLI flags: `--warn-as-error`, `--suppress=E4XX`
4. Implement Tier 1 codes with tests (3+ test cases per code, positive and negative)

### Tier 2: Design Guidance (Priority: MEDIUM)

These enforce good practices but don't catch bugs. Some require sophisticated analysis and carry false positive risk. Implement incrementally after Tier 1.

| Code | Name | Description | Severity | Risk |
|------|------|-------------|----------|------|
| E410 | `unreachable-state` | No incoming transitions | Warning | **High FP risk:** states reachable via `-> pop$` look unreachable to static analysis |
| E411 | `dead-end-state` | No outgoing transitions | Info | **FP risk:** terminal states are intentional |
| E422 | `dead-handler` | Handler never called | Info | Low risk |
| E424 | `shadowed-handler` | Child shadows parent handler | Info | Low risk, may be intentional |
| E430 | `self-transition-no-effect` | Self-transition with no enter/exit handlers | Warning | Low risk |
| E433 | `transition-in-enter-handler` | Transition in enter handler | Warning | **FP risk:** guard states that redirect are a legitimate pattern |
| E450 | `pop-without-push` | Stack underflow potential | Warning | Requires interprocedural analysis |
| E451 | `push-without-pop` | Stack leak potential | Warning | Requires interprocedural analysis |
| E461 | `unused-domain-var` | Declared but never referenced | Info | Cannot know about native code usage |

**Timeline:** Best-effort, ongoing. Each code ships when its analysis is robust enough that false positives are rare.

**Key constraint for Tier 2:** Frame cannot see inside native code. A domain variable that appears unused may be referenced in a native block. A state that appears unreachable may be the target of `-> pop$`. Tier 2 codes must account for this fundamental limitation, and many should default to Info severity with clear documentation about when they may be wrong.

---

## Phase 3: Source Maps (Priority: MEDIUM, Independent Track)

**Goal:** Enable Frame source debugging in VS Code

**Timeline:** 2 weeks (can be done in parallel with any other phase)

This phase has no dependencies on Phases 1 or 2 and nothing depends on it. Schedule based on priority relative to other work.

### Week 1: Implementation

Tasks:
1. Create `codegen/source_map.rs` module
2. Track spans during splicer operation
3. Generate source map JSON (JavaScript source map v3 format)
4. Emit `.map` files alongside generated code

### Week 2: Integration

Tasks:
1. Wire to CLI with `--source-map` flag
2. Test with VS Code debugger
3. Document usage

---

## Phase 4: Testing & Verification (Priority: HIGH, Ongoing)

**Goal:** Comprehensive test coverage for V4

### Test Categories

1. **PRT Tests** (`test-frames/v4/prt/`) — Same Frame source verified across Python, Rust, TypeScript
2. **Validation Tests** (`test-frames/v4/validation/`) — Each error code gets 3+ test cases (positive and negative)
3. **Persistence Tests** (`test-frames/v4/persistence/`) — Save/restore cycle verification
4. **Regression Tests** (`test-frames/v4/regression/`) — Fixed bugs should never reoccur

### Test Infrastructure

```
v4/
├── prt/
│   ├── basic_transition.frm
│   ├── interface_methods.frm
│   ├── enter_exit.frm
│   ├── hsm_forward.frm
│   └── stack_ops.frm
├── validation/
│   ├── structural/
│   ├── events/
│   ├── transitions/
│   └── hsm/
├── persistence/
│   ├── basic_save_restore.frm
│   ├── selective_domain.frm
│   └── stack_persistence.frm
└── regression/
```

---

## Success Criteria

| Metric | Target |
|--------|--------|
| `@@persist` codegen | Working for all 3 PRT languages |
| Tier 1 error codes | All 8 implemented with tests |
| Tier 2 error codes | Best-effort, low false positive rate |
| False positive rate | <1% on existing passing tests |
| Performance | Validation adds <10% to transpile time |
| PRT tests | 100% pass rate |

---

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| False positives annoy users | Tier 2 codes default to Info; easy `--suppress` |
| Interprocedural analysis too complex | Defer E450/E451 until simpler codes are solid |
| `@@persist` type limitations | JSON only; document non-serializable type constraints |
| Tier 2 analysis can't see native code | Document limitations clearly; never Error severity for codes that can't account for native usage |
| Performance regression | Lazy evaluation; skip irrelevant passes |
| Breaking existing workflows | Warnings don't block; new errors are warnings first |

---

## Deferred (V5+)

The following are explicitly out of scope for V4:

1. **Multi-file Frame projects** — V4 does not support Frame imports. Use native imports.
2. **Native code analysis** — V5 may add optional cross-environment symbol resolution
3. **Type checking across boundaries** — V5 scope
4. **C#/Java/C/C++ backends** — Low priority, complete when needed
5. **Auto-persistence hooks** — `@@persist auto` (inject save into transitions) deferred

---

## Getting Started

```bash
# Build transpiler
cargo build --release -p framec

# Compile a Frame file
./target/release/framec test.frm -l python_3 -o /tmp/test.py

# Run validation only
./target/release/framec test.frm --validate-only

# Run test suite (Docker)
cd framepiler_test_env
./framepiler/docker/target/release/frame-docker-runner python_3 --all
```

---

*Document revised: 2026-02-15*