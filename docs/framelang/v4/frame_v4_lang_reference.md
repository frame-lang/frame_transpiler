# Frame V4 Language Reference

**Version:** 1.2
**Date:** February 2026  
**Audience:** Implementation team  
**Status:** Normative — this document defines the language. Ambiguities are bugs in this spec.

---

## 1. Notation

- `<angle_brackets>` — placeholder (must be substituted)
- `( )?` — optional group
- `|` — alternatives
- `//` — comment (not part of syntax)
- All whitespace in grammar productions is flexible unless noted otherwise
- Literal tokens are shown in `monospace`

---

## 2. Source File Structure

A Frame source file (`.frm`) has this structure:

```
<preamble>          // native code (optional)
@@target <lang>     // required, exactly once
@@codegen { ... }   // optional, at most once
<annotations>*      // zero or more @@persist, @@async, etc.
@@system <Name> (<params>)? {
    <sections>
}
<postamble>         // native code (optional)
```

Everything outside `@@target`, `@@codegen`, annotations, and `@@system` is native code and passes through unchanged.

### 2.1 `@@target`

```
@@target <language_id>
```

Required. Must appear before `@@system`. Specifies the target language for code generation.

**Valid `language_id` values:**

| ID | Language |
|----|----------|
| `python_3` | Python 3 |
| `typescript` | TypeScript |
| `rust` | Rust |
| `c` | C (C11) |

No other values are valid. Unknown values are parse errors (E001).

### 2.2 `@@codegen`

```
@@codegen {
    <key> : <value> ,
    ...
}
```

Optional. Must appear after `@@target` and before `@@system`. Configures code generation.

**Keys and values:**

| Key | Values | Default | Meaning |
|-----|--------|---------|---------|
| `frame_event` | `on` \| `off` | `off` | Generate FrameEvent class for event metadata |

Trailing comma after last entry is permitted. Unknown keys are warnings.

**Auto-enable:** The compiler auto-enables `frame_event` (with W401 warning if explicit `off`) when the spec requires it:

| Feature in spec | Forces `frame_event = on` |
|----------------|---------------------------|
| Enter/exit parameters on any transition | Yes |
| Event forwarding (`-> =>`) | Yes |
| `system.return` usage | Yes |
| Interface methods with return values | Yes |

**Note:** State stack generation is managed internally by the compiler and is not user-configurable.

### 2.3 Annotations

```
@@<annotation_name> (<params>)?
```

Zero or more. Appear after `@@codegen` (if present) and before `@@system`. Stack freely.

**Defined annotations:**

| Annotation | Parameters | Effect |
|------------|-----------|--------|
| `@@persist` | none \| `(domain=[<fields>])` \| `(exclude=[<fields>])` | Generate JSON serialize/deserialize methods |

Annotations apply to the immediately following `@@system`.

### 2.4 `@@system`

```
@@system <Identifier> ( <system_params> )? {
    ( operations: <operations_block> )?
    ( interface: <interface_block> )?
    ( machine: <machine_block> )?
    ( actions: <actions_block> )?
    ( domain: <domain_block> )?
}
```

Declares a state machine system. Sections are optional but **must appear in the order shown**: operations → interface → machine → actions → domain.

**System parameters** (three groups, all optional, in fixed positional order):

```
@@system Name ( $(<state_params>) , $>(<enter_params>) , <domain_params> )
```

| Group | Syntax | Target |
|-------|--------|--------|
| State params | `$(<param_list>)` | Start state's `compartment.state_args` |
| Enter params | `$>(<param_list>)` | Start state's `compartment.enter_args` |
| Domain params | bare `<param_list>` | Domain variable overrides |

Groups are positional. Omitting a group shifts later groups left. If only domain params exist, no `$()` or `$>()` prefixes are needed.

---

## 3. `interface:` Section

Declares public methods exposed to callers. These are the system's API.

```
<method_name> ( <params>? ) (: <return_type> (= <default_value>)? )?
```

**Examples:**

```frame
interface:
    start()
    stop()
    process(data, priority)
    getStatus(): str
    getDecision(): str = "yes"
```

**Rules:**
- Method names must be unique within the interface
- Parameters use Frame notation: `name: type` (e.g., `process(data: str, priority: int)`)
- Parameters can also be untyped if types are handled in native code: `process(data, priority)`
- Return type annotation is Frame syntax (used for codegen)
- Default return value is a native expression, used when no handler sets `system.return`
- A return type with no default implies `None`/`null`/`None` as default

---

## 4. `machine:` Section

Contains state definitions.

### 4.1 State Declaration

```
$<StateName> ( => $<ParentState> )? {
    <state_var_declarations>*
    <handlers>*
}
```

**Rules:**
- State names must be valid identifiers and unique within the system
- The first state listed is the start state
- `=> $ParentState` declares HSM parent (parent must exist, no cycles)

### 4.2 State Variable Declarations

Must appear at the top of the state block, before any handlers.

```
$.<varName> (: <type>)? = <initializer_expr>
```

**Components:**

| Part | Required | Description |
|------|----------|-------------|
| `$.` | Yes | State variable prefix — always means "this state's variable" |
| `<varName>` | Yes | Identifier |
| `: <type>` | No | Frame type annotation for typed backends |
| `= <initializer_expr>` | Yes | Native code expression; evaluated on every state entry |

**Examples:**

```frame
$Processing {
    $.counter: int = 0
    $.label: str = "default"
    $.cache = {}

    $>() { ... }
    process() { ... }
}
```

**Scope rules:**
- `$.x` always refers to the enclosing state's variable `x`
- No syntax exists to access another state's variables
- No duplicate variable names within a state
- State variable names may shadow domain variable names (no ambiguity due to `$.` prefix)

### 4.3 Event Handlers

```
<event_name> ( <params>? ) (: <return_type> (= <default_value>)? )? {
    <body>
}
```

**Return value initialization:** When a handler declares a return type with a default value (`= <expr>`), that expression is evaluated and stored in `@@:return` before the handler body executes. If the handler completes without setting `@@:return`, the default value is used.

```frame
compute(a, b): int = foo(a + b) {
    // @@:return already initialized to foo(a + b)
    if special_case:
        @@:return = 999    // Override the default
    // Otherwise the default value is returned
}
```

The body is a mix of native code and Frame statements. Native code passes through unchanged. Frame statements are expanded by the splicer.

### 4.4 Enter Handler

```
$> ( <params>? ) {
    <body>
}
```

Called when the state is entered via a transition. Parameters come from the transition's enter args.

### 4.5 Exit Handler

```
<$ ( <params>? ) {
    <body>
}
```

Called when the state is exited via a transition. Parameters come from the transition's exit args.

### 4.5.1 Enter/Exit Parameter Mapping

Enter and exit args are passed **positionally** to their respective handlers. The first transition arg maps to the first handler parameter, second to second, and so on.

```frame
$Idle {
    start() {
        -> ("from_idle", 42) $Active    # Position 0: "from_idle", Position 1: 42
    }
}

$Active {
    $>(source: str, value: int) {       # source ← position 0, value ← position 1
        print(f"Entered from {source} with {value}")
    }
}
```

The same applies to exit args:

```frame
$Active {
    stop() {
        ("goodbye", 99) -> $Idle        # Position 0: "goodbye", Position 1: 99
    }
}

$Active {
    <$(msg: str, code: int) {           # msg ← position 0, code ← position 1
        print(f"Exiting: {msg} ({code})")
    }
}
```

### 4.6 HSM Parent Forward (`=> $^`)

**V4 uses explicit-only forwarding.** Unhandled events are NOT automatically forwarded to parent states.

**In-handler forward:** Forwards the current event to parent at that point in the handler:

```
$Child => $Parent {
    event_a() {
        log("Child processing")
        => $^   // Forward to Parent's event_a handler
    }
}
```

**State-level default forward:** A bare `=> $^` as the last entry forwards ALL unhandled events:

```
$Child => $Parent {
    specific_event() { ... }
    => $^                       // All other events forward to parent
}
```

**Key V4 semantics:**
- `=> $^` is the **only** way to forward to parent
- `=> $^` can appear **anywhere** in a handler, not just at the end
- Without `=> $^`, unhandled events are **ignored**, not forwarded

---

## 5. Frame Statements

Frame V4 recognizes exactly **6 Frame constructs** within handler bodies. Everything else is native code.

### 5.1 Transition — `-> $State`

Transitions to a target state. Invokes exit handler on current state, then enter handler on target state.

**Grammar:**

```
( <exit_params> )? -> ( => )? ( <enter_params> )? $<TargetState> ( <state_params> )?
```

**Forms:**

| Form | Meaning |
|------|---------|
| `-> $State` | Simple transition |
| `-> $State(args)` | Transition with state args |
| `-> (args) $State` | Transition with enter args |
| `(args) -> $State` | Transition with exit args |
| `(exit) -> (enter) $State(state)` | Full form, all positions |
| `-> => $State` | Transition with event forwarding |
| `(exit) -> => (enter) $State(state)` | Forwarding with all positions |
| `-> pop$` | Transition to popped state from stack |

**Event forwarding** (`-> =>`): The current event is stashed on the target state's compartment. After the enter handler fires, the forwarded event is dispatched to the target state. This is a transition variant, not a separate statement.

**Transition to popped state** (`-> pop$`): Pops a compartment from the state stack and transitions to it. The full transition lifecycle (exit/enter) fires. No state variable reinitialization — the popped compartment retains its preserved state.

**Termination:** Transitions may optionally end with `;`. Every transition is implicitly followed by a `return` in generated code — code after a transition in the same handler is unreachable.

### 5.2 Forward to Parent — `=> $^`

Forwards the current event to the parent state's dispatch function.

```
=> $^
```

**Prerequisites:** The enclosing state must have a parent declared with `=> $ParentState`.

**Behavior:** Calls the parent state's dispatch function with the current event. This is an explicit forward — the developer chooses to delegate. Contrast with implicit forwarding in the state dispatch function for completely unhandled events.

### 5.3 Stack Push — `push$`

```
push$                   // push current state's compartment
```

Pushes the current state's compartment onto the state stack. The entire compartment (including state variables) is preserved and can be restored with `pop$` or `-> pop$`.

### 5.4 Stack Pop — `pop$`

```
pop$                    // pop and discard
```

Pops the top compartment from the state stack. As a standalone statement, the popped value is discarded. To transition to the popped state, use `-> pop$` (see 5.1).

### 5.5 State Variable Access — `$.varName`

```
$.counter               // read
$.counter = <expr>      // write
```

Reads or writes the current state's variable. Only valid within handlers of the state that declares the variable.

### 5.6 System Context Access — `@@`

Access the current interface call context. See Section 9 for full semantics.

```
@@.param            // interface parameter (shorthand)
@@:return = <expr>  // set the interface return value
@@:return           // read the interface return value
@@:event            // interface method name
@@:data[key]        // call-scoped data
@@:params[x]        // interface parameter (explicit)
```

**Return sugar in event handlers only:**

```
return <expr>           // in event handler: sugar for @@:return = <expr>; return
return <expr>           // in action: native function return (NOT sugar)
return                  // everywhere: native return, pass through unchanged
```

The splicer distinguishes handler context from action context.

**Legacy:** `system.return` is an alias for `@@:return` for backward compatibility.

---

## 5.7 Compartment: The Runtime Model

The **compartment** is Frame's central runtime data structure. Per the official Frame documentation, it is "a closure concept for states that preserve the state itself, the data from the various scopes as well as runtime data needed for Frame machine semantics."

### Compartment Structure

Every compartment has 6 fields:

| Field | Purpose |
|-------|---------|
| `state` | Current state identifier |
| `state_args` | Arguments passed to the state via `$State(args)` |
| `state_vars` | State variables declared with `$.varName` |
| `enter_args` | Arguments passed via `-> (args) $State` |
| `exit_args` | Arguments passed via `(args) -> $State` |
| `forward_event` | Stashed event for `-> =>` forwarding |

### State Variable Storage

State variables are always stored in `compartment.state_vars`:

```
$.counter = 5    →   self._compartment.state_vars["counter"] = 5
```

### State Stack = Compartment Stack

The state stack stores **entire compartments**, not just state names. This is what enables state variable preservation:

```frame
$StateA {
    $.counter: int = 0

    save_and_go() {
        $.counter = 10
        push$               // saves entire compartment (with counter=10)
        -> $StateB
    }
}

$StateB {
    restore() {
        -> pop$             // restores compartment (counter is still 10)
    }
}
```

### Reentry vs History

| Transition Type | State Var Behavior |
|----------------|-------------------|
| `-> $State` (normal) | State vars **reset** to initial values |
| `-> pop$` (history) | State vars **preserved** from saved compartment |

This distinction is fundamental to Frame's expressive power.

---

## 6. `actions:` Section

Private helper methods on the system class.

```
actions:
    <action_name> ( <params>? ) (: <return_type> (= <default_value>)? )? {
        <body>
    }
```

**Return value initialization:** When an action declares a return type with a default value (`= <expr>`), that expression is evaluated and stored in `@@:return` before the action body executes.

```frame
actions:
    validate(x): bool = check_basic(x) {
        // @@:return already initialized to check_basic(x)
        if extra_validation_needed:
            @@:return = deep_check(x)
    }
```

**Rules:**
- Actions are declared at the **system level**, not inside states
- Actions can access **domain variables** and `@@:return`
- Actions **cannot** access state variables (`$.`) — they have no state context
- Actions can return values to their callers (via `@@:return`)
- Actions **cannot** trigger transitions — transitions are only valid in event handlers
- Parameters use Frame notation: `action_name(param: type)`

---

## 7. `operations:` Section

Public methods that bypass the state machine entirely.

```
operations:
    <op_name> ( <params>? ) (: <return_type> (= <default_value>)? )? {
        <body>
    }
```

**Return value initialization (non-static only):** Non-static operations can declare a return type with a default value (`= <expr>`), which initializes `@@:return` before the operation body executes.

```frame
operations:
    calculate(x): int = baseline(x) {
        // @@:return already initialized to baseline(x)
        if adjustment_needed:
            @@:return = adjust(@@:return)
    }
```

**Rules:**
- Operations do not create events or go through the kernel
- Direct access to domain variables
- Non-static operations can use `@@:return` with initialization syntax
- No access to state variables (`$.`)
- Body is entirely native code (no Frame statements)

**Static operations** (no `self`/`this`):

```
operations:
    static add(a, b): int {
        return a + b
    }
```

Static operations use native `return` statements and cannot use `@@:return` or return value initialization.

---

## 8. `domain:` Section

Instance variables for the system.

```
domain:
    var <name> (: <type>)? = <initializer>
```

**Examples:**

```frame
domain:
    var timer = Timer()
    var count: int = 0
    var label: str = "default"
```

Domain variables are instance fields on the generated class. They persist across state transitions.

---

## 9. System Context (`@@`)

The **system context** provides access to the current interface call's parameters, return value, and call-scoped data. It is accessed via the `@@` prefix.

### 9.1 Architecture

Every interface call creates two objects:

**FrameEvent** — routing mechanism:
```
FrameEvent {
    _message: string      // interface method name
    _parameters: dict     // interface parameters
}
```

**FrameContext** — call context:
```
FrameContext {
    event: FrameEvent     // reference to interface event
    _return: any          // return value slot
    _data: dict           // call-scoped data
}
```

The context is pushed onto the `_context_stack` when an interface method is called and popped when it returns. Lifecycle events (`$>`, `<$`) use the existing context without pushing.

### 9.2 Syntax

| Syntax | Meaning | Description |
|--------|---------|-------------|
| `@@.x` | Parameter shorthand | Interface parameter `x` |
| `@@:params[x]` | Parameter explicit | Interface parameter `x` (explicit form) |
| `@@:event` | Event name | Interface method name being handled |
| `@@:return` | Return value | Get/set interface return value |
| `@@:data[key]` | Call data | Get/set call-scoped data |

**Key rule:** `@@` ALWAYS refers to the interface call context, even inside lifecycle handlers.

### 9.3 Examples

**Basic parameter and return access:**

```frame
@@system Calculator {
    interface:
        compute(a: int, b: int): int

    machine:
        $Ready {
            compute(a: int, b: int): int {
                @@:return = @@.a + @@.b
            }
        }
}
```

**Accessing interface context from lifecycle handlers:**

```frame
$Processing {
    compute(a: int, b: int): int {
        @@:data["started"] = time.now()
        -> $Done
    }

    <$(reason: str) {
        // 'reason' is from handler signature (exit_args)
        // @@.a refers to interface param, not exit param
        print(f"Exiting: {reason}, original a={@@.a}")
    }
}

$Done {
    $>() {
        elapsed = time.now() - @@:data["started"]
        print(f"Completed {@@:event} in {elapsed}ms")
        @@:return = @@.a + @@.b
    }
}
```

**Accessing context from actions:**

```frame
actions:
    validate(): bool {
        // Actions can access interface context
        if @@.a < 0 or @@.b < 0:
            @@:return = -1
            return False
        return True
    }
```

### 9.4 Context Lifecycle

```
calc.compute(1, 2) called
│
├─► FrameEvent("compute", {a: 1, b: 2}) created
├─► FrameContext(event, _return=None, _data={}) created
├─► Context PUSHED to _context_stack
│
├─► Kernel routes event to handler
│   ├─► Handler runs, may set @@:return, @@:data
│   ├─► Handler triggers -> $Next
│   │
│   ├─► FrameEvent("<$", exit_args) created (NOT pushed)
│   ├─► <$ handler runs, can access @@.a, @@:return
│   │
│   ├─► Compartment switch
│   │
│   ├─► FrameEvent("$>", enter_args) created (NOT pushed)
│   └─► $> handler runs, can access @@.a, @@:return
│
├─► Context POPPED from _context_stack
└─► Return context._return to caller
```

### 9.5 Reentrancy

Each interface call pushes its own context. Nested calls maintain separate contexts:

```
outer() called
  → context_stack: [{event: "outer", _return: None}]

  handler sets @@:return = "outer_value"
  → context_stack: [{event: "outer", _return: "outer_value"}]

  handler calls self.inner()
    → context_stack: [{...outer...}, {event: "inner", _return: None}]

    inner handler sets @@:return = "inner_value"
    → context_stack: [{...outer...}, {event: "inner", _return: "inner_value"}]

    inner() returns "inner_value"
    → context_stack: [{event: "outer", _return: "outer_value"}]

  outer handler continues, @@:return still "outer_value"

outer() returns "outer_value"
  → context_stack: []
```

### 9.6 Return Sugar

In event handlers (not actions/operations), `return <expr>` is sugar for `@@:return = <expr>; return`:

| Context | `return <expr>` meaning |
|---------|------------------------|
| Event handler | `@@:return = expr; return` (sugar) |
| Enter handler | `@@:return = expr; return` (sugar) |
| Exit handler | `@@:return = expr; return` (sugar) |
| Action | Native function return (no sugar) |
| Operation | Native function return (no sugar) |

`return` (bare, no expression) is always native in all contexts.

### 9.7 Context Not Available

`@@` is not available in:
- Static operations (no interface call active)
- Initial `$>` during construction (no interface call yet)

Accessing `@@` in these contexts is an error.

---

## 10. Persistence (`@@persist`)

The `@@persist` annotation generates serialization/deserialization methods for saving and restoring system state.

### 10.1 Usage

```frame
@@persist
@@system MySystem {
    // ...
}
```

### 10.2 Generated Methods by Language

| Language | Save Method | Returns | Restore Method | Takes |
|----------|-------------|---------|----------------|-------|
| Python | `save_state()` | `bytes` | `restore_state(data)` [static] | `bytes` |
| TypeScript | `saveState()` | `any` (object) | `restoreState(data)` [static] | `any` |
| Rust | `save_state(&mut self)` | `String` (JSON) | `restore_state(json)` [static] | `&str` |
| C | `save_state(self)` | `char*` (JSON) | `restore_state(json)` [static] | `const char*` |

### 10.3 What Gets Persisted

- **Current state** (compartment state name)
- **State stack** (for push$/pop$ history)
- **State arguments** (`state_args`)
- **State variables** (`state_vars`)
- **Enter/exit arguments** (`enter_args`, `exit_args`)
- **Forward event reference** (`forward_event`)
- **Domain variables** (all fields in `domain:` section)

### 10.4 What Gets Reinitialized on Restore

Runtime fields are reinitialized, not persisted:

- `_context_stack` — initialized to empty
- `__next_compartment` — initialized to null/None

### 10.5 Dependencies

| Language | Required Dependencies |
|----------|----------------------|
| Python | `pickle` (stdlib, included) |
| TypeScript | None (uses native objects) |
| Rust | `serde`, `serde_json` in Cargo.toml |
| C | `cjson` library (libcjson) |

### 10.6 Example Usage

**Python:**
```python
# Save
data = system.save_state()  # Returns bytes

# Restore
system2 = MySystem.restore_state(data)
```

**TypeScript:**
```typescript
// Save
const data = system.saveState();  // Returns object

// Restore
const system2 = MySystem.restoreState(data);
```

**Rust:**
```rust
// Save
let json = system.save_state();  // Returns String

// Restore
let system2 = MySystem::restore_state(&json);
```

**C:**
```c
#include <cjson/cJSON.h>

// Save
char* json = MySystem_save_state(system);  // Returns malloc'd string

// Restore
MySystem* system2 = MySystem_restore_state(json);

// Remember to free the JSON string when done
free(json);
```

**Note for C:** The C implementation uses the cJSON library for JSON serialization. The `save_state()` function returns a dynamically allocated string that the caller must free. String fields are copied using `strdup()` during restore to ensure memory safety after the JSON is freed.

---

## 11. Token Summary

### 10.1 Module-Level Tokens

| Token | Meaning |
|-------|---------|
| `@@target` | Declare target language |
| `@@codegen` | Configure code generation |
| `@@persist` | Enable serialization |
| `@@system` | Declare state machine |

### 10.2 State Machine Tokens

| Token | Meaning |
|-------|---------|
| `$<Name>` | State reference |
| `$>` | Enter handler |
| `<$` | Exit handler |
| `$^` | Parent state reference (HSM) |
| `$.` | State variable prefix |

### 10.3 Statement Tokens

| Token | Meaning |
|-------|---------|
| `->` | Transition operator |
| `=>` | Forward operator |
| `-> =>` | Transition with event forwarding |
| `-> pop$` | Transition to popped state |
| `push$` | Push to state stack |
| `pop$` | Pop from state stack |

### 10.4 Context Tokens

| Token | Meaning |
|-------|---------|
| `@@.x` | Interface parameter `x` (shorthand) |
| `@@:return` | Interface return value |
| `@@:event` | Interface event name |
| `@@:params[x]` | Interface parameter `x` (explicit) |
| `@@:data[key]` | Call-scoped data |

---

## 12. Native Code

Everything not recognized as a Frame construct is native code in the target language. Native code passes through the transpiler unchanged, with only indentation adjusted to match the generated code's structure.

The Framepiler does **not** parse, validate, or transform native code. The target language's compiler/interpreter is responsible for validating native code correctness.

**String and comment awareness:** The NativeRegionScanner must not recognize Frame constructs inside string literals or comments in the target language. Each target language has its own string/comment syntax that the scanner must respect.

---

## 13. Scanning Architecture

Frame V4 uses a layered scanning architecture with specialized state-machine scanners. Each scanner operates on raw bytes and respects language-specific string/comment syntax via the `SyntaxSkipper` trait.

### 12.1 Scanner Hierarchy

| Scanner | Purpose | Input | Output |
|---------|---------|-------|--------|
| `PrologScanner` | Find `@@target` pragma | Source bytes | Span of `@@target` line |
| `PragmaScanner` | Identify all `@@` pragmas vs native code | Source bytes + SyntaxSkipper | `Vec<PragmaRegion>` |
| `OutlineScanner` | Find system structure (states, handlers) | Source bytes + language | `Vec<OutlineItem>` |
| `ImportScanner` | Identify import statements | Source bytes | Import spans |
| `NativeRegionScanner` | Find Frame statements in handler bodies | Body bytes + SyntaxSkipper | `Vec<Region>` |
| `BodyCloser` | Find matching closing brace | Source bytes from `{` | Position of `}` |

### 12.2 SyntaxSkipper Trait

Language-specific string/comment handling is encapsulated in the `SyntaxSkipper` trait:

```rust
pub trait SyntaxSkipper {
    /// Get the body closer for this language
    fn body_closer(&self) -> Box<dyn BodyCloser>;

    /// Skip a comment starting at position i. Returns Some(new_pos) if skipped.
    fn skip_comment(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize>;

    /// Skip a string literal starting at position i. Returns Some(new_pos) if skipped.
    fn skip_string(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize>;

    /// Find end of Frame statement line, respecting strings/comments.
    fn find_line_end(&self, bytes: &[u8], start: usize, end: usize) -> usize;

    /// Find matching close paren, respecting strings.
    fn balanced_paren_end(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize>;
}
```

Each target language implements `SyntaxSkipper`:

| Language | Comments | Strings |
|----------|----------|---------|
| Python | `#`, `"""`, `'''` | `"..."`, `'...'`, `"""..."""`, `'''...'''` |
| TypeScript | `//`, `/* */` | `"..."`, `'...'`, `` `...` `` |
| Rust | `//`, `/* */` | `"..."`, `r#"..."#` |
| C/C++ | `//`, `/* */` | `"..."`, `'...'` |
| Java | `//`, `/* */` | `"..."` |
| C# | `//`, `/* */` | `"..."`, `@"..."`, `$"..."` |

### 12.3 PragmaScanner

The `PragmaScanner` identifies Frame pragma lines (`@@target`, `@@system`, `@@codegen`, etc.) while respecting language-specific strings and comments. This is essential for the "oceans model" where native code passes through unchanged.

**Purpose:** Separate Frame pragmas from native code regions.

**Algorithm:**
1. Scan byte-by-byte from start of file
2. At start-of-line (SOL), check for `@@` token
3. Use `SyntaxSkipper` to skip strings and comments
4. `@@` inside strings/comments is NOT a pragma
5. Return spans of pragma lines vs native text regions

**Output:**

```rust
pub enum PragmaRegion {
    /// Native code - passes through verbatim
    NativeText { span: RegionSpan },
    /// Frame pragma line (@@target, @@system, etc.)
    PragmaLine { span: RegionSpan, kind: PragmaKind },
}

pub enum PragmaKind {
    Target,      // @@target <lang>
    Codegen,     // @@codegen { ... }
    System,      // @@system Name { ... }
    Persist,     // @@persist
    Async,       // @@async
    RunExpect,   // @@run-expect <pattern>
    Other,       // Future pragmas
}
```

**Correctness guarantee:** `@@` appearing inside a string literal or comment in the native code will NOT be recognized as a pragma:

```python
# This is safe - @@target inside string is not a pragma
print("@@target python_3")

# This is safe - @@system inside comment is not a pragma
# @@system FakeSystem { }
```

### 12.4 Scanning Pipeline

The compilation pipeline uses scanners in sequence:

```
1. PrologScanner     → Find @@target, determine language
2. PragmaScanner     → Separate pragmas from native code
3. OutlineScanner    → Parse @@system structure
4. NativeRegionScanner → Find Frame statements in handler bodies
```

**Key invariant:** Native code regions identified by `PragmaScanner` pass through verbatim. The transpiler never parses or transforms content inside native regions.

---

## 14. Validation Errors

| Code | Condition |
|------|-----------|
| E001 | Parse error (malformed Frame syntax) |
| E402 | Unknown state reference (`-> $NonExistent`) |
| E403 | Duplicate state definition |
| E405 | Parameter mismatch (interface/handler arity) |
| E4xx | Cross-state variable access (TBD) |
| E4xx | `=> $^` in state without parent (TBD) |
| E4xx | `push$`/`pop$` with `state_stack = off` and no auto-enable (TBD) |
| E4xx | Transition in action (TBD) |
| E4xx | Duplicate state variable name (TBD) |
| E4xx | HSM cycle detected (TBD) |

---

## 15. Complete Example

```frame
# Native Python preamble
import logging

@@target python_3
@@codegen {
    frame_event: on,
    runtime: kernel
}

@@persist
@@system OrderProcessor ($(order_type), $>(initial_data), max_retries) {

    operations:
        static version(): str {
            return "1.0.0"
        }

    interface:
        submit(order)
        cancel(reason)
        getStatus(): str = "unknown"

    machine:
        $Idle {
            submit(order) {
                logging.info("Received order")
                $.order_data = order
                -> $Validating
            }
        }

        $Validating {
            $.order_data = None
            $.attempts: int = 0

            $>() {
                $.attempts = $.attempts + 1
                if validate($.order_data):
                    -> $Processing
                else:
                    if $.attempts >= self.max_retries:
                        -> $Failed
            }

            getStatus(): str {
                return "validating"
            }
        }

        $Processing {
            $>() {
                logging.info("Processing order")
            }

            cancel(reason) {
                (reason) -> $Cancelled
            }

            getStatus(): str {
                return "processing"
            }
        }

        $Cancelled {
            $>(reason) {
                logging.info(f"Cancelled: {reason}")
            }
        }

        $Failed {
            $>() {
                logging.error("Order failed")
            }
        }

    actions:
        validate(data) {
            return data is not None
        }

    domain:
        var max_retries: int = 3
}

# Native Python postamble
if __name__ == '__main__':
    proc = OrderProcessor("standard", {"source": "web"}, 5)
    proc.submit({"item": "widget", "qty": 3})
```