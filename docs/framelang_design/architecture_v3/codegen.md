# Frame Language V3 — Code Generation & Runtime Shape (Going Native)

Purpose
- Describe how V3 `system` and `fn` constructs map onto target‑language code.
- Align the new, system‑only grammar with the existing runtime model (compartments, router, transitions) from the legacy Frame docs, while keeping classes/structs native.

Scope
- Python, TypeScript, Rust as primary “PRT” targets.
- C/C++/Java/C# stay facade/wrapper‑only for now (compile‑time and debugger artifacts).

High‑Level Model
- `system` → a target‑language class/struct that owns:
  - A runtime compartment object (current state, args, vars, enter/exit args, forward event).
  - A router method that dispatches events by state.
  - One or more handler methods per event, grouped by state.
- `fn` (including `fn main`) → top‑level/native functions in the target module that:
  - Use the generated system class/struct and runtime kernel.
  - Have bodies treated as native code with embedded Frame statements lowered via the same MIR/expander pipeline used for handlers.

## Python Target

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

- **State IDs and compartment**
  ```python
  class TrafficLight:
      def __init__(self):
          # initial state is the first declared state
          self._compartment = FrameCompartment("__TrafficLight_state_Red")
  ```

- **Router (event dispatch)**
  ```python
      def _frame_router(self, __e: FrameEvent, compartment: FrameCompartment = None):
          c = compartment or self._compartment
          if c.state == "__TrafficLight_state_Red":
              self._s_Red(__e, c)
          elif c.state == "__TrafficLight_state_Green":
              self._s_Green(__e, c)
          elif c.state == "__TrafficLight_state_Yellow":
              self._s_Yellow(__e, c)
  ```

- **Transition primitive**
  ```python
      def _frame_transition(self, next_compartment: FrameCompartment):
          self._compartment = next_compartment
          # call entry handler for the new state
          self._frame_router(FrameEvent("$enter", None), self._compartment)
  ```

- **State handlers**
  - Each state gets a handler method that switches on event name, e.g.:
  ```python
      def _s_Red(self, __e: FrameEvent, compartment: FrameCompartment):
          if __e.name == "$enter":
              print("Red")
          elif __e.name == "tick":
              # expansion of `-> $Green()`
              next_compartment = FrameCompartment("__TrafficLight_state_Green")
              self._frame_transition(next_compartment)
              return
  ```

- **Interface methods**
  - `interface: tick()` becomes a thin wrapper that builds a `FrameEvent` and calls `_frame_router`:
  ```python
      def tick(self):
          __e = FrameEvent("tick", None)
          self._frame_router(__e, self._compartment)
  ```

- **Domain and actions/operations**
  - `domain:` variables become instance attributes initialized in `__init__`.
  - `actions:` and `operations:` become helper methods on the class (e.g., `_action_handle(...)` and `handle(...)` wrappers), with bodies treated as native Python + embedded Frame statements lowered via MIR.

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

### Systems → TS classes

- For a system `system Name { … }`, codegen emits:
  ```ts
  export class Name {
    public _compartment: FrameCompartment = new FrameCompartment("__Name_state_A");
    _frame_transition(n: FrameCompartment) { this._compartment = n; }
    _frame_router(__e: FrameEvent, c?: FrameCompartment) { /* real router */ }
    _frame_stack_push() { /* stack impl */ }
    _frame_stack_pop() { /* stack impl */ }
  }
  ```

- State routing uses a `switch` on `c.state` inside `_frame_router`, with one case per compiled state ID.

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

- Interface methods become public methods that construct a `FrameEvent` and call `_frame_router`.

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

## Rust Target (outline)

- Systems map to:
  - A `struct` for the system, containing:
    - A compartment struct (`FrameCompartment` analog) holding `StateId`, args, vars, etc.
  - An `enum StateId { A, B, … }` (now the default).
  - Methods:
    - `fn new() -> Self` initializing the compartment to the start state.
    - `fn frame_router(&mut self, e: FrameEvent)` switching on `self.compartment.state`.
    - Per‑state handler methods (`fn s_A_tick(&mut self, e: FrameEvent)`, etc.).

- Transitions lower to:
  ```rust
  let mut next_compartment = FrameCompartment::new(StateId::B);
  // exit/enter/state args wiring
  self.frame_transition(next_compartment);
  return;
  ```

- `fn main` in Frame maps to a native Rust `fn main()` in the generated module/binary crate when that mode is enabled; otherwise, it is a regular helper function.

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

