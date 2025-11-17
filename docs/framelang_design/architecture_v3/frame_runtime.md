# Frame Runtime V3 — Compartments, Events, and System Execution

**Status**: Design‑level description of the V3 runtime model.  
**Scope**: Explains the common runtime semantics in pseudocode, then shows how
they map onto each supported target. Where current implementations are still
partial, that status is called out explicitly.

This document is the runtime companion to:

- `architecture_v3/grammar.md` — Frame constructs that still exist in V3.
- `architecture_v3/frame_language_guide.md` — Human language guide.
- `architecture_v3/codegen.md` — Per‑target codegen structure.

The original v0.x runtime concepts (system start parameters, compartments,
state/transition parameters, system.return, etc.) remain the semantic source of
truth. V3 re‑uses those semantics but delivers them via class‑based runtimes and
native bodies.

---

## 1. Core Concepts

### 1.1 FrameEvent

Frame events are the payload passed into the runtime:

```pseudocode
struct FrameEvent:
    message: string          # event name, e.g. "tick" or "status"
    params: list<any>        # event arguments (positional)
```

- The exact representation is native (Python object, TS class, etc.), but every
  runtime exposes something equivalent to `FrameEvent(message, params)`.

### 1.2 FrameCompartment

Each active state lives inside a **compartment**, which holds:

```pseudocode
struct FrameCompartment:
    state: string            # state identifier, e.g. "__TrafficLight_state_Red"
    stateArgs: list<any>     # state parameters bound at entry
    stateVars: map<string, any>  # state variables for this state
    enterArgs: list<any>     # enter-event arguments for the current transition
    exitArgs: list<any>      # exit-event arguments for the current transition
    parentCompartment: FrameCompartment?  # for parent-forward
    forwardEvent: FrameEvent?             # for => $^ forwarding
```

Notes:

- In some current V3 implementations, only a subset of these fields are
  materialized (e.g., `FrameCompartment` is thinner and state variables are
  held in the system object). This document describes the **semantic shape**.

### 1.3 System Class Skeleton

Every V3 system is expressed as a native class with a small, fixed runtime API:

```pseudocode
class SystemName:
    _compartment: FrameCompartment
    _stack: list<FrameCompartment>   # for $$[+]/$$[-]

    def __init__(self, ...systemParams...):
        # create initial compartment, apply system parameters
        self._compartment = initial_compartment(...)
        self._stack = []

    def _frame_transition(self, next_compartment: FrameCompartment):
        # switch to new compartment and trigger enter handler
        self._compartment = next_compartment
        # fire synthetic "$enter" event if appropriate
        enter_event = FrameEvent("$enter", next_compartment.enterArgs)
        self._frame_router(enter_event, next_compartment)

    def _frame_router(self, event: FrameEvent, compartment: FrameCompartment = None):
        c = compartment or self._compartment
        msg = event.message
        state = c.state

        # dispatch to state-specific handler
        if state == "__SystemName_state_Red":
            if msg == "tick":       self._s_Red_tick(event, c)
            elif msg == "$enter":   self._s_Red_enter(event, c)
            # ...
        elif state == "__SystemName_state_Green":
            # ...
        # else: unknown event/state — ignore or log

    def _frame_stack_push(self):
        self._stack.push(self._compartment)

    def _frame_stack_pop(self):
        if self._stack:
            prev = self._stack.pop()
            self._frame_transition(prev)
```

Handlers/actions/operations are emitted as methods on `SystemName` that
ultimately use `FrameEvent`, `FrameCompartment`, `_frame_transition`, and the
stack helpers.

---

## 2. Runtime Semantics in Pseudocode

This section describes a target‑independent runtime in pseudocode. Each
language‑specific runtime (Python/TS/Rust/…) should behave as an instance of
this abstract machine.

### 2.1 System Construction and Start Parameters

System parameters (when present) follow this shape:

```frame
system MySystem($(startStateParams), $>(enterParams), domainObj) { ... }
```

Instantiation in user code:

```pseudocode
sys = MySystem(startArg1, startArg2, enterArg1, enterArg2, domainValue)
```

Abstract runtime behavior:

```pseudocode
def initial_compartment(sysParams) -> FrameCompartment:
    # 1. Determine start state: first state declared in machine:,
    #    or a specific start state if the language later introduces
    #    an explicit start-state annotation.
    start_state_name = first_machine_state_name()

    # 2. Partition system parameters into start, enter, domain.
    #    For v3, these are positional and flattened.
    (startArgs, enterArgs, domainArgs) = split_sys_params(sysParams)

    c = FrameCompartment()
    c.state = "__MySystem_state_" + start_state_name
    c.stateArgs = startArgs
    c.enterArgs = enterArgs
    c.exitArgs = []
    c.parentCompartment = None
    c.forwardEvent = None

    # 3. Apply domain override (semantically — in many implementations,
    #    this is realized as fields on SystemName rather than on the compartment).
    apply_domain_overrides(domainArgs)

    return c

class MySystem:
    def __init__(self, *sysParams):
        self._compartment = initial_compartment(sysParams)
        self._stack = []
        # Optionally, fire initial "$enter" for start state
        enter_event = FrameEvent("$enter", self._compartment.enterArgs)
        self._frame_router(enter_event, self._compartment)
```

**Note (current V3 status)**:

- The v2 runtime implemented this behavior concretely (via system initializer
  lists). V3 grammar and docs preserve the concept.
- **Python**: `compile_module_demo` now implements this wiring for V3 modules:
  it parses `system` parameter lists, partitions constructor arguments into
  start/enter/domain groups, seeds the initial `FrameCompartment` for the
  first declared state (via Arcanum), and fires an initial `$enter` event.
  This behavior is exercised by
  `framec_tests/language_specific/python/v3_systems_runtime/positive/traffic_light_system_exec.frm`.
- **TypeScript**: constructor/start wiring is implemented structurally (the
  class constructor partitions system params and seeds `_compartment`), but
  `_frame_router` is still a stub. V3 TS tests run in transpile/validate
  mode only; full runtime parity is a roadmap item.

### 2.2 Event Dispatch

Abstract event dispatch:

```pseudocode
def dispatch(self, msg: string, *args):
    event = FrameEvent(msg, list(args))
    self._frame_router(event, None)
```

The `_frame_router` is responsible for:

1. Choosing the effective compartment (explicit argument or `self._compartment`).
2. Switching on `(state, event.message)`.
3. Invoking the appropriate handler method.

Handlers can:

- Perform native work.
- Use Frame statements:
  - `-> $State(args...)` transitions.
  - `=> $^` parent forward.
  - `$$[+]` and `$$[-]` stack operations.
  - `system.return = value` to set interface return values.
- Call other system methods (subject to the V3 rule that `system.method()`
  used as a Frame‑aware system call must target an interface method; see
  E406).

### 2.3 Transition Semantics

Pseudocode translation for a Frame transition:

```frame
-> $TargetState(arg1, arg2)
```

Runtime expansion:

```pseudocode
def expand_transition(target_state: string, exitArgs, enterArgs, stateArgs):
    next_compartment = FrameCompartment()
    next_compartment.state = "__System_state_" + target_state
    next_compartment.exitArgs = exitArgs
    next_compartment.enterArgs = enterArgs
    next_compartment.stateArgs = stateArgs
    next_compartment.parentCompartment = self._compartment.parentCompartment

    self._frame_transition(next_compartment)
    return  # terminal in the handler
```

**Terminal rule**:

- V3 enforces that transitions, forwards, and stack operations are **terminal**
  within a handler (E400): no additional executable statements after the Frame
  statement in the same block, except comments/whitespace.

### 2.4 Parent Forward (`=> $^`)

Forward semantics:

```pseudocode
def expand_forward():
    # Non-terminal: call parent handler but continue after return.
    parent = self._compartment.parentCompartment
    if parent is not None:
        self._frame_router(current_event, parent)
    # handler may continue after this point
```

V3 includes validation:

- If there is no parent state in the system hierarchy, parent forward raises
  **E403: Cannot forward to parent: no parent available**.

### 2.5 Stack Operations (`$$[+]`, `$$[-]`)

Push:

```pseudocode
def expand_stack_push():
    self._stack.push(self._compartment)
```

Pop:

```pseudocode
def expand_stack_pop():
    if self._stack:
        prev = self._stack.pop()
        self._frame_transition(prev)
```

These semantics match the v2 notion of a state stack: transitions can save/restore
previous compartments, including state variables and arguments.

### 2.6 `system.return` Semantics

`system.return` is the mechanism for setting interface return values:

```pseudocode
def interface_status(self) -> string:
    # generated wrapper
    event = FrameEvent("status", [])
    self._frame_router(event)
    return self._system_return_default_or_current("status")
```

Handler:

```pseudocode
status() {
    system.return = "idle"
    # no explicit return from handler; wrapper reads system.return
}
```

Abstract rules:

- `system.return` is a special variable associated with the **current interface
  call**.
- When an interface wrapper returns, it reads `system.return` (if set) and
  returns that value; otherwise it uses the default return value from the
  interface header (`: type = default`).
- Actions may also assign `system.return` so they can participate in computing
  the interface return.
- Operations **cannot** assign `system.return` (V3 reports E407).

**Current V3 status**:

- Placement rules are enforced (allowed in handlers/actions; disallowed in
  operations) and covered by V3 fixtures.
- Full wiring of `system.return` into interface wrappers is partially
  implemented in Python and TS codegen and will be documented as it stabilizes.

### 2.7 `system.method()` Calls

Semantics:

- Calls of the form `system.methodName(...)` inside handlers, actions, and
  operations are used to invoke **interface methods** on the same system.
- V3 validator rule:
  - If `methodName` is not declared in the `interface:` block, the call is
    flagged as `E406: system.methodName call must target an interface method`.
- The actual call is native (`self.status()`, `this.status()`, etc.) and routed
  through the same runtime as external interface calls.

---

## 3. Per‑Language Runtimes

This section summarizes how the abstract runtime maps onto each target.

### 3.1 Python Runtime

**Runtime library**: `frame_runtime_py`

- Exposes:
  - `FrameEvent` (Python class or dataclass).
  - `FrameCompartment`.
  - Any additional helpers required by the generated code.

**Generated system structure** (design target):

```python
from frame_runtime_py import FrameEvent, FrameCompartment

class TrafficLight:
    def __init__(self, *sys_params):
        self._compartment = initial_compartment("TrafficLight", sys_params)
        self._stack = []
        enter_ev = FrameEvent("status", [])
        self._frame_router(enter_ev, self._compartment)

    def _frame_transition(self, next_compartment: FrameCompartment):
        self._compartment = next_compartment
        enter_event = FrameEvent("$enter", next_compartment.enterArgs or [])
        self._frame_router(enter_event, next_compartment)

    def _frame_router(self, __e: FrameEvent, c: FrameCompartment | None = None):
        compartment = c or self._compartment
        msg = getattr(__e, "_message", None)
        state = compartment.state
        if state == "__TrafficLight_state_Red":
            if msg == "tick":
                self._s_Red_tick(__e, compartment)
            elif msg == "$enter":
                self._s_Red_enter(__e, compartment)
        elif state == "__TrafficLight_state_Green":
            # ...
        # else: ignore or log

    def _frame_stack_push(self):
        self._stack.append(self._compartment)

    def _frame_stack_pop(self):
        if self._stack:
            prev = self._stack.pop()
            self._frame_transition(prev)
```

**Handler/actions/operations**:

- Handlers are emitted as methods with signatures derived from Frame headers.
- Actions/operations are emitted as internal `_action_*` / `_operation_*`
  methods plus public wrapper methods (FRM names) that call them.
- Frame statements inside those bodies are expanded via `PyExpanderV3` into
  `_frame_transition`, `_frame_router`, `_frame_stack_push/pop` calls.

**System/state params and system.return**:

- System/state parameter semantics and `system.return` behavior follow the
  abstract model above; current implementation is evolving toward this design
  and tested by the `v3_*` Python suites.

### 3.2 TypeScript Runtime

**Runtime library**: `frame_runtime_ts`

- Exposes:
  - `FrameEvent`.
  - `FrameCompartment`.

**Generated system structure** (design target):

```ts
import { FrameEvent, FrameCompartment } from "frame_runtime_ts";

class TrafficLight {
  private _compartment: FrameCompartment;
  private _stack: FrameCompartment[] = [];

  constructor(...sysParams: any[]) {
    this._compartment = initialCompartment("TrafficLight", sysParams);
    const enterEvent = new FrameEvent("$enter", this._compartment.enterArgs);
    this._frame_router(enterEvent, this._compartment);
  }

  private _frame_transition(next: FrameCompartment): void {
    this._compartment = next;
    const enterEvent = new FrameEvent("$enter", next.enterArgs);
    this._frame_router(enterEvent, next);
  }

  private _frame_router(event: FrameEvent, comp?: FrameCompartment): void {
    const c = comp ?? this._compartment;
    const msg = event.message;
    const state = c.state;
    if (state === "__TrafficLight_state_Red") {
      if (msg === "tick") this._s_Red_tick(event, c);
      else if (msg === "$enter") this._s_Red_enter(event, c);
    } else if (state === "__TrafficLight_state_Green") {
      // ...
    }
  }

  private _frame_stack_push(): void { this._stack.push(this._compartment); }
  private _frame_stack_pop(): void {
    const prev = this._stack.pop();
    if (prev) this._frame_transition(prev);
  }
}
```

Frame statements are expanded into TS via `TsExpanderV3`, mirroring the Python
semantics but using TS/JS idioms (blocks, `const` for temporary compartments,
etc.).

System/state params and `system.return` follow the same abstract semantics as in
Python; tests live under `language_specific/typescript/v3_*`.

### 3.3 Rust Runtime

Rust currently has:

- A lightweight `FrameCompartment` struct in generated code and in some
  runtime helpers.
- Expansions via `RustExpanderV3` that use:

```rust
fn __frame_transition(state: &str) { println!("TRANSITION:{}", state); }
fn __frame_forward() { println!("FORWARD:PARENT"); }
fn __frame_stack_push() { println!("STACK:PUSH"); }
fn __frame_stack_pop() { println!("STACK:POP"); }
```

Rust’s V3 runtime behaves like a façade: it proves that Frame statements map to
valid Rust and can execute smoke tests, but it does not yet provide a full
structural class‑based system runtime equivalent to Python/TS.

### 3.4 C / C++ / Java / C#

For C/C++/Java/C#, V3 currently focuses on:

- Generating valid native code for Frame statements via expander facades.
- Providing minimal `main()` wrappers and runtime helpers in the exec‑smoke
  tests to show that transitions, forward, and stack operations compile and run.

These targets do not yet have a fully featured class‑based runtime in the V3
style; they are closer to v2’s function‑level or procedural patterns.

---

## 4. Implementation Status Summary

- **Semantics**:
  - Transition, forward, and stack semantics are fully specified and enforced
    at the MIR/validator level and in the V3 expander implementations.
  - `system.method()` and `system.return` **placement rules** are enforced via
    E406/E407 and covered by V3 fixtures.
  - System/state parameter handling is fully specified at the grammar and
    symbol‑table level; runtime glue is partially implemented.

- **Runtimes**:
  - Python/TypeScript: primary implementation targets for the full V3 runtime.
  - Rust: façade runtime with correct Frame statement semantics; class‑based
    system runtime is evolving.
  - C/C++/Java/C#: façade‑style support via expander facades and exec‑smoke
    tests; full V3 runtime to be designed later.

As the Python/TS/Rust runtimes converge on this design, this document should be
kept in sync with both the code and the V3 tests. Any new runtime feature
(e.g., richer system parameter semantics, history semantics on the stack) must
first be added here, then implemented and tested. 
