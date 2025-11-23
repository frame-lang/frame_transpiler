# Frame Language V3 — Code Generation & Runtime Shape (Going Native)

Purpose
- Describe how V3 `system` and `fn` constructs map onto target‑language code.
- Align the new, system‑only grammar with the existing runtime model (compartments, router, transitions) from the legacy Frame docs, while keeping classes/structs native.

Scope
- Python, TypeScript, Rust as primary “PRT” targets.
- C/C++/Java/C# stay facade‑only for now (compile‑time and debugger artifacts).

High‑Level Model (Backed by ModuleAst + Arcanum)
- `system` → a target‑language class/struct that owns:
  - A runtime compartment object (current state, args, vars, enter/exit args, forward event).
  - A router method that dispatches events by state.
  - One or more handler methods per event, grouped by state.
- `fn` (including `fn main`) → top‑level/native functions in the target module that:
  - Use the generated system class/struct and runtime kernel.
  - Have bodies treated as native code with embedded Frame statements lowered via the same MIR/expander pipeline used for handlers.

These mappings are derived from the outer AST (`ModuleAst`) and symbol table
(`Arcanum`):
- `SystemAst.sections` and `section_order` drive block ordering and uniqueness.
- `Arcanum` provides state names, parent relationships, parameters, and spans.
- `SystemParamsAst` plus per‑system state information determine system/start/
  enter/domain parameter wiring.

For the PRT languages (Python/TypeScript/Rust), these same AST/symbols also
drive key validation rules in the V3 module and CLI paths:

- **E402 (unknown state)**: transitions whose `$TargetState` does not appear in
  the Arcanum for the enclosing system’s `machine:` block are rejected.
- **E403 (no parent for `=> $^`)**: parent forwards are allowed only when the
  Arcanum records a parent for the enclosing state.
- **E404 (handler outside state)**: handlers inside `machine:` must reside
  within some `$State { … }` span; interface handlers under `interface:` are
  exempt.

Non‑PRT languages continue to use the existing structural known‑state and
section checks for these diagnostics until their V3 paths are upgraded.

## Python Target (V3)

### Imports and runtime

- At the top of each generated Python module:
  ```python
  from frame_runtime_py import FrameEvent, FrameCompartment
  ```
- `FrameCompartment` matches the conceptual compartment in the runtime docs:
  - `state` (string identifier like `"__System_state_Red"`)
  - `state_args`, `state_vars`, `enter_args`, `exit_args`
  - optional `forward_event`

### Systems → Python classes

Given:
```frame
@target python

system TrafficLight {
  interface:
    tick()

  machine:
    $Red {
      $>() { print("Red") }
      tick() { -> $Green() }
    }
    $Green {
      $>() { print("Green") }
      tick() { -> $Yellow() }
    }
    $Yellow {
      $>() { print("Yellow") }
      tick() { -> $Red() }
    }
}
```

V3 codegen emits a Python class with:

- **System parameters and initial compartment**
  ```python
  class TrafficLight:
      def __init__(self, *sys_params):
          # System params: ($(color), $>(enter_color), domain)
          # Parsed via ModuleAst/SystemParamsAst; counts and names come from the
          # system header and the Arcanum start-state declaration.
          start_count = 1
          enter_count = 1
          start_args = list(sys_params[0:start_count])
          enter_args = list(sys_params[start_count:start_count+enter_count])
          domain_args = list(sys_params[start_count+enter_count:])

          state_args = {}
          if len(start_args) > 0:
              state_args["color"] = start_args[0]
          if len(domain_args) > 0:
              self.domain = domain_args[0]

          # Start state is the first declared state (via Arcanum).
          self._compartment = FrameCompartment("__TrafficLight_state_Red", enter_args=enter_args, state_args=state_args)
          self._stack = []

          # Fire initial $enter event for the start state.
          enter_event = FrameEvent("$enter", enter_args)
          self._frame_router(enter_event, self._compartment)
  ```

- **Transition primitive**
  ```python
      def _frame_transition(self, next_compartment: FrameCompartment):
          self._compartment = next_compartment
          enter_event = FrameEvent("$enter", getattr(next_compartment, "enter_args", None))
          self._frame_router(enter_event, next_compartment)
  ```

-- **Router (event dispatch)**
  ```python
      def _frame_router(self, __e: FrameEvent, c: FrameCompartment = None):
          compartment = c or self._compartment
          msg = getattr(__e, "_message", None)
          if msg is None:
              return
          # Dispatch to event-specific internal handler based on message name.
          handler = getattr(self, f"_event_{msg}", None)
          if handler is None:
              return
          return handler(__e, compartment)
  ```

-- **Event handlers (per interface method)**
  - For each interface method (e.g., `tick()`), V3 emits an internal handler
    that switches on `c.state` and inlines the spliced handler bodies, e.g.:
  ```python
      def _event_tick(self, __e: FrameEvent, compartment: FrameCompartment):
          c = compartment or self._compartment
          if c.state == "__TrafficLight_state_Red":
              print("Red")
              next_compartment = FrameCompartment("__TrafficLight_state_Green")
              next_compartment.state_args = ["green"]
              self._frame_transition(next_compartment)
              return
          elif c.state == "__TrafficLight_state_Green":
              print("Green")
              next_compartment = FrameCompartment("__TrafficLight_state_Yellow")
              next_compartment.state_args = ["yellow"]
              self._frame_transition(next_compartment)
              return
          elif c.state == "__TrafficLight_state_Yellow":
              print("Yellow")
              next_compartment = FrameCompartment("__TrafficLight_state_Red")
              next_compartment.state_args = ["red"]
              self._frame_transition(next_compartment)
              return
  ```

- **Domain and actions/operations**
  - `domain:` variables become instance attributes initialized in `__init__`
    using domain parameters when present.
  - `actions:` and `operations:` become helper methods on the class (internal
    `_action_*` implementations plus public entrypoints), with bodies treated as
    native Python + embedded Frame statements lowered via MIR.

### Functions and `fn main`

- A top‑level function:
  ```frame
  fn helper(msg) {
      print(msg)
  }
  ```
  becomes:
  ```python
  def helper(msg):
      print(msg)
  ```

- A single `fn main` per module:
  ```frame
  fn main() {
      var tl = TrafficLight()
      tl.tick()
      tl.tick()
  }
  ```
  becomes:
  ```python
  def main():
      tl = TrafficLight()
      tl.tick()
      tl.tick()
  ```

- Function bodies are scanned/spliced exactly like handlers:
  - SOL‑anchored `-> $State`, `=> $^`, `$$+`, `$$-` inside `fn` bodies lower to the same `_frame_transition`, `_frame_router`, and stack operations.

## TypeScript Target

### Imports and runtime

- Each TS module begins with:
  ```ts
  import { FrameEvent, FrameCompartment } from 'frame_runtime_ts';
  ```
  (path is configurable via `FRAME_TS_EXEC_IMPORT` in exec/debug modes).

### Systems → TS classes (V3)

- For a system `system Name($(...), $>(...), domain) { … }`, codegen emits:
  ```ts
  export class Name {
    public _compartment: FrameCompartment = new FrameCompartment("__Name_state_A");
    private _stack: FrameCompartment[] = [];

    constructor(...sysParams: any[]) {
      const startCount = /* number of $(...) params */;
      const enterCount = /* number of $>(...) params */;
      const startArgs = sysParams.slice(0, startCount);
      const enterArgs = sysParams.slice(startCount, startCount + enterCount);
      const domainArgs = sysParams.slice(startCount + enterCount);

      const stateArgs: any = {};
      // populate stateArgs["paramName"] from startArgs
      // populate this.domainField from domainArgs when fields exist

      this._compartment = new FrameCompartment("__Name_state_A", enterArgs, undefined, stateArgs);
      const enterEvent = new FrameEvent("$enter", enterArgs);
      this._frame_router(enterEvent, this._compartment);
    }

    _frame_transition(n: FrameCompartment) {
      this._compartment = n;
      const enterEvent = new FrameEvent("$enter", n.enterArgs);
      this._frame_router(enterEvent, n);
    }

    _frame_router(__e: FrameEvent, c?: FrameCompartment) {
      // Current V3 TS impl largely mirrors Python structurally:
      // event-specific handlers are emitted as methods and invoked by
      // generated interface methods. Full router parity is a roadmap item.
      const _c = c || this._compartment;
      const _m = __e.message;
      void _c;
      void _m;
    }

    _frame_stack_push() { this._stack.push(this._compartment); }
    _frame_stack_pop() {
      const prev = this._stack.pop();
      if (prev) this._frame_transition(prev);
    }
  }
  ```

- Embedded Frame statements are lowered as:
  - Transition: `-> $State(args?)` →
    ```ts
    {
      const __frameNextCompartment_State = new FrameCompartment("__System_state_State");
      // exitArgs / enterArgs / stateArgs assignment
      this._frame_transition(__frameNextCompartment_State);
      return;
    }
    ```
  - Parent forward: `=> $^` →
    ```ts
    this._frame_router(__e, compartment.parentCompartment);
    ```
  - Stack ops: `$$+` / `$$-` →
    ```ts
    this._frame_stack_push(); // or _frame_stack_pop();
    ```

- Interface methods become public methods that construct a `FrameEvent` and call `_frame_router` in the long‑term design. In the current V3 TypeScript implementation, they are emitted as public methods with native bodies; full router parity is tracked in the V3 roadmap and tested structurally in `v3_systems` and `v3_capabilities`.

### Functions and `fn main`

- Top‑level functions become module‑level `function` declarations:
  ```frame
  fn main() {
      const tl = new TrafficLight();
      tl.tick();
  }
  ```
  →
  ```ts
  export function main(): void {
      const tl = new TrafficLight();
      tl.tick();
  }
  ```

- As with Python, function bodies are scanned/spliced so embedded Frame statements use the same kernel helpers.

## Rust Target (PRT parity plan)

- Goal: bring the Rust V3 target to parity with Python/TypeScript:
  - Real `FrameCompartment` struct and `FrameEvent` type for the runtime.
  - `_frame_transition`/`_frame_router` methods on the generated system struct.
  - `system.return` per‑call stack and generated interface methods mirroring the Python/TS design.
  - Curated exec suites (`v3_core`, `v3_control_flow`, `v3_scoping`, `v3_systems`) running against the real Rust runtime.

- Target structure (design):
  - A `struct` for the system, containing:
    - A compartment struct (`FrameCompartment` analog) holding `StateId`, args, vars, etc.
    - A stack of compartments for `$$[+]`/`$$[-]`.
    - A per‑call `system_return_stack: Vec<Value>` analogous to the Python/TS `_systemReturnStack`.
  - An `enum StateId { A, B, … }` (default‑on).
  - Methods:
    - `fn new(...) -> Self` initializing the compartment to the start state (first state in `machine:` per Arcanum) and firing `$enter`.
    - `fn frame_transition(&mut self, next: FrameCompartment)` switching the active compartment and firing `$enter`.
    - `fn frame_router(&mut self, e: FrameEvent)` switching on `(self.compartment.state, e.message)` and invoking state‑specific handlers.
    - Per‑state handler methods (`fn s_A_tick(&mut self, e: FrameEvent)`, etc.) and generated interface methods that manage `system.return`.

- Transitions will lower to:
  ```rust
  let mut next_compartment = FrameCompartment::new(StateId::B);
  // exit/enter/state args wiring
  self.frame_transition(next_compartment);
  return;
  ```

- `fn main` in Frame will map to a native Rust `fn main()` in the generated module/binary crate when that mode is enabled; otherwise, it is a regular helper function. Parity with Python/TS for `fn main` wiring is part of this plan.

## Non‑PRT Targets (C/C++/Java/C#)

- For now V3 keeps these as:
  - Comment‑only/facade expansions for Frame statements (used by exec‑smoke and debugger probes).
  - System skeletons that can be filled in by native code.
- Full runtime parity (compartment + router) is deferred; codegen for these targets is documented under the per‑language specs in `target_language_specifications/`.

## Invariants and Guarantees

- Exactly one `fn main` per module (when present); validated at the outline/AST level.
- Embedded Frame statements inside handlers, actions/ops, and functions:
  - Are SOL‑anchored and lowered via the same MIR + expander pipeline per target.
  - Obey the terminal‑last rule (validators enforce that transitions/forwards/stacks are last in their containing block, modulo comments/whitespace).
- Systems do not rely on Frame‑level classes:
  - All “class” behavior is provided by native classes/structs on the target, with Frame semantics encoded in the generated methods and runtime calls.
