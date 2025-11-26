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

Validation (PRT / Arcanum-backed):

- For the PRT languages (Python/TypeScript/Rust), V3 uses the ModuleAst +
  Arcanum symbol table to validate transition targets:
  - If a handler transitions to `$TargetState` that is not declared in the
    enclosing system’s `machine:` block, validation reports  
    **E402: unknown state 'TargetState'**.
  - This check runs in both the V3 module path and the V3 CLI pipelines for
    Py/TS/Rust and is driven by the same Arcanum symbols used for codegen.
  - Non‑PRT languages continue to use a coarse structural known‑state set for
    E402 until their V3 paths are upgraded.

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

For Py/TS/Rust, the parent‑availability check is Arcanum‑backed:

- The Arcanum records parent relationships between states (e.g.,
  `$B => $A { … }`), and the validator only allows `=> $^` when the enclosing
  state has a parent in that symbol table.
- If a handler contains a `Forward` MIR item but the Arcanum indicates no
  parent for the enclosing state, E403 is emitted in the V3 module/CLI paths.

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
    # generated interface method
    event = FrameEvent("status", [])
    self._frame_router(event)
    return self._system_return_default_or_current("status")
```

Handler:

```pseudocode
status() {
    system.return = "idle"
    # no explicit return from handler; generated interface method reads system.return
}
```

Abstract rules:

- `system.return` is a special variable associated with the **current call to
  an interface method**. Each interface invocation allocates a per‑call slot on
  a return stack (`_system_return_stack`).
- When a generated interface method returns, it reads the current `system.return`
  value from the top of that stack and returns it; if no handler or helper has
  written to it, the value comes from the interface header default
  (`name(params): Type = Expr`).
- Handlers, actions, and non‑static operations may all assign to
  `system.return` so they can participate in computing the interface return.
- In event handlers only, a `return expr` statement is treated as sugar for
  `system.return = expr; return` in the underlying target code. A bare
  `return` does not modify `system.return`.

Example with header defaults and mixed returns:

```frame
system Example {
    interface:
        a1(): int = 10
        a2(a): int = a
        a3(a): int = self.x + a

    machine:
        $Idle {
            a1() {
                if self.x < 5:
                    return        # leaves system.return = 10
                else:
                    return 0      # sugar for system.return = 0; return
            }

            a2(a) {
                return            # leaves system.return = a
            }

            a3(a) {
                return a          # overrides header default
            }
        }

    domain:
        x = 3
}
```

For `a1()`:
- When `self.x < 5`, the bare `return` leaves `system.return` at the header
  default (`10`), so the generated interface method returns `10`.
- Otherwise, `return 0` updates `system.return` to `0` and the generated interface method returns
  `0`.

For `a2(a)`, the header default (`a`) is preserved by the bare `return`, so
the final value is the argument. For `a3(a)`, the handler body overrides the
header default by assigning a new value via `return a`.

**Current V3 status**:

- Per‑call `system.return` stacks and header defaults are implemented in the
  V3 Python and TypeScript generators (module path), including handler sugar
  and support for actions/operations participating in the value.
- Placement rules are enforced structurally in the validator (handlers,
  actions, and operations share the same semantics) and exercised via the
  V3 `system_return_*` capability fixtures.

### 2.7 Handler Placement (`machine:` vs `interface:`)

V3 enforces that:

- Handlers inside `machine:` must be nested within a state block (`$State { … }`);
- Interface handlers live under `interface:` and are *not* required to be
  inside a state block.

Implementation details:

- The validator uses the ModuleAst + Arcanum to collect all state spans in
  each system’s `machine:` section.
- For handlers whose headers lie inside `machine:`, the validator checks that
  their header offsets fall within some `$State` span; otherwise it emits  
  **E404: handler body must be inside a state block**.
- Handlers whose headers lie outside any `machine:` section (e.g., interface
  handlers) are excluded from this check. This Arcanum‑backed behavior is
  implemented for the PRT languages (Py/TS/Rust) in the V3 module and CLI
  paths; non‑PRT languages continue to rely on the existing structural checks.

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
  methods plus generated interface methods (FRM names) that call them when
  exposed on the public surface.
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

Rust currently has two layers:

1. **Exec‑smoke façade** (unchanged):

   - For exec‑smoke fixtures, the legacy façade still emits free functions:

     ```rust
     fn __frame_transition(state: &str) { println!("TRANSITION:{}", state); }
     fn __frame_forward() { println!("FORWARD:PARENT"); }
     fn __frame_stack_push() { println!("STACK:PUSH"); }
     fn __frame_stack_pop() { println!("STACK:POP"); }
     ```

   - These are used only by `v3_exec_smoke` and related marker tests to
     validate that Frame statements expand into valid Rust statements and
     produce the expected markers on stdout.

2. **V3 module‑path runtime scaffold** (new, PRT work in progress):

   - For V3 module‑path compiles (`framec compile -l rust` under the V3 path),
     the generator now emits a minimal struct‑based runtime similar in shape to
     Python/TypeScript:

     ```rust
     #[allow(dead_code)]
     #[derive(Debug, Clone, Copy, PartialEq, Eq)]
     enum StateId { /* per-system states */ }

     impl Default for StateId { fn default() -> Self { StateId::StartState } }

     #[derive(Debug, Clone)]
     struct FrameEvent { message: String }

     #[derive(Debug, Clone, Default)]
     struct FrameCompartment {
         state: StateId,
         forward_event: Option<FrameEvent>,
         exit_args: Option<()>,
         enter_args: Option<()>,
         parent_compartment: Option<*const FrameCompartment>,
         state_args: Option<()>,
     }

     struct SystemName {
         compartment: FrameCompartment,
         _stack: Vec<FrameCompartment>,
     }

     impl SystemName {
         fn new() -> Self {
             Self {
                 compartment: FrameCompartment {
                     state: StateId::StartState,
                     ..Default::default()
                 },
                 _stack: Vec::new(),
             }
         }

         fn _frame_transition(&mut self, next: &FrameCompartment) {
             // Basic transition: update the active state id; other fields remain unchanged for now.
             self.compartment.state = next.state;
         }

         fn _frame_stack_push(&mut self) {
             self._stack.push(self.compartment.clone());
         }

         fn _frame_stack_pop(&mut self) {
             if let Some(prev) = self._stack.pop() {
                 self._frame_transition(&prev);
             }
         }

         /// Minimal router: dispatch based on event message name by calling
         /// the corresponding handler method. Each handler method performs
         /// its own per-state dispatch on `self.compartment.state`.
         fn _frame_router(&mut self, e: Option<FrameEvent>) {
             if let Some(ev) = e {
                 match ev.message.as_str() {
                     // one arm per handler name (e.g., "e", "tick", "start", …)
                     // "e" => self.e(),
                     _ => { }
                 }
             }
         }
     }
     ```

   - For each handler name (e.g. `e`, `tick`), the generator emits a single
     method on the system:

     ```rust
     impl SystemName {
         fn e(&mut self) {
             match self.compartment.state {
                 StateId::A => {
                     // body for $A.e()
                 }
                 StateId::B => {
                     // body for $B.e()
                 }
                 _ => { }
             }
         }
     }
     ```

   - `_frame_router` and these handler methods are used in the V3 module‑path
     codegen; exec‑smoke continues to use the façade functions.

   - `system.return` semantics and per‑call return stacks are **not yet**
     implemented for Rust; they are tracked as part of the PRT Stage 7–13
     parity work. For now, Rust’s struct‑based runtime supports state/stack and
     basic message routing but does not enforce header defaults or return‑slot
     sugar.

### 3.4 C / C++ / Java / C#

For C/C++/Java/C#, V3 currently focuses on:

- Generating valid native code for Frame statements via expander facades.
- Providing minimal `main()` entrypoints and runtime helpers in the exec‑smoke
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
