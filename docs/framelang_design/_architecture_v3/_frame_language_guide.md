# Frame Language V3 Guide (Going Native)

**Status**: Draft V3 guide for the “going native” architecture.  
**Scope**: Only Frame constructs that remain in V3; all other syntax is owned by the target language.

This file is the human‑oriented companion to:

- `docs/framelang_design/architecture_v3/grammar.md` (formal V3 grammar)
- `docs/framelang_design/architecture_v3/codegen.md` (per‑target codegen/runtime)

It replaces the legacy v0.xx language overview at [frame‑lang.org] for the V3
pipeline while reusing the same conceptual structure: systems, states,
transitions, parameters, and mixed native + Frame bodies.

## 1. Modules, Targets, and Layout

Every V3 file begins with a **prolog** that selects the target language:

```frame
@target python_3
```

Supported targets are `python_3`, `typescript`, `rust`, `csharp`, `c`, `cpp`,
and `java`. The rest of the file is a sequence of:

- `system` declarations,
- top‑level `fn` functions (including an optional `fn main`), and
- arbitrary native items (classes, helper functions, imports, etc.).

In V3, **Frame is responsible only for the system skeleton and SOL‑anchored
Frame statements** inside otherwise‑native bodies. Expressions, types, classes,
and most control flow are native.

## 2. Systems and Blocks

A system describes one or more state machines plus associated helpers:

```frame
system TrafficLight {
    operations:
        logTransition(state) { print(state) }

    interface:
        tick()
        status(): str = "UNKNOWN"

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

    actions:
        record(state) {
            system.logTransition(state)
        }
}
```

V3 enforces a canonical block order when multiple blocks are present:

1. `operations:`
2. `interface:`
3. `machine:`
4. `actions:`
5. optional native `domain:` (treated as native, not Frame)

Each block is optional, but when present it must respect this order. The V3
validator uses the `ModuleAst` produced by `SystemParserV3` to report:

- **E113** — blocks out of order (expected operations → interface → machine → actions → domain).
- **E114** — duplicate `operations:`, `interface:`, `machine:`, `actions:`, or `domain:` blocks.

## 3. System Parameters (Start, Enter, Domain)

Systems may declare **system parameters** that describe how they are
constructed and started. These are fully part of the V3 grammar and are
validated semantically:

```frame
system TestSystem($(startState), $>(enterEvent), domain) {
    interface:
        run()

    machine:
        $Idle { run() { -> $Running() } }
        $Running { run() { -> $Idle() } }
}
```

Grammar (see `architecture_v3/grammar.md`):

- `system_decl ::= 'system' IDENT system_params? '{' system_item* '}'`
- `system_params ::= '(' system_param_list ')'`
- `system_param_list ::= system_param (',' system_param)*`
- `system_param ::= start_state_param | enter_event_param | domain_param`
- `start_state_param ::= '$(' param_list? ')'`
- `enter_event_param ::= '$>' '(' param_list? ')'`
- `domain_param ::= IDENT`
- Param grouping is name-based (no positional buckets): any declared identifier
  listed in a `$(...)` belongs to the start-state set; any listed in a
  `$>(...)` belongs to the enter-event set. Remaining declared identifiers are
  domain params. Multiple `$(...)`/`$>(...)` groups are allowed in one header.
  Declared names must be unique; unknown names in a group are errors. State
  headers never repeat `$(...)`; they declare state params directly, and
  mapping is by name.

V3 semantics (using `ModuleAst` + `Arcanum`):

- `$(...)` **start parameters**:
  - Names must match the **start state’s** parameter list (order and count).
  - Mismatches produce **E416**.
- `$>(...)` **enter parameters**:
  - Names must match the start state’s `$>()` handler parameter list.
  - If the system declares `$>(...)` but the start state has no `$>()` handler,
    or the parameter names disagree, **E417** is reported.
- `domain` identifiers:
  - Each domain parameter name must correspond to a variable declared in the
    system’s `domain:` block (enforced by `DomainBlockScannerV3`).
  - Missing variables produce **E418**.

Runtime:

- Python V3:
  - `compile_module` partitions constructor arguments into start, enter,
    and domain groups based on the parsed `system_params`.
  - Seeds the initial `FrameCompartment` using the first declared state (via
    `Arcanum`) and fires an initial `$enter` event.
  - This behavior is validated by `v3_systems_runtime/positive/traffic_light_system_exec.frm`.
- TypeScript V3:
  - The constructor partitions system params and seeds `_compartment`
    structurally; a full `_frame_router` is a roadmap item.

## 4. States, State Parameters, and Handlers

Inside `machine:`, V3 supports **hierarchical states**, **state parameters**,
and **entry/exit handlers**:

```frame
system RangeCounter {
    interface:
        configure(min, max)
        increment()
        value(): int = 0

    machine:
        $Idle {
            configure(min, max) {
                -> $Configured(min, max)
            }
        }

        $Configured(min, max) {
            $>() { current = min }

            increment() {
                current = current + 1
                if current > max { current = min }
            }
        }
}
```

Grammar:

- `states_block ::= 'machine:' state_decl*`
- `state_decl ::= '$' IDENT state_params? ( '=>' '$' IDENT )? '{' handler_decl* '}'`
- `state_params ::= '(' param_list? ')'`
- `handler_decl ::= IDENT '(' param_list? ')' native_body`
- `enter_handler ::= '$>' '(' param_list? ')' native_body`
- `exit_handler ::= '<$' '(' param_list? ')' native_body`

Semantics:

- **Hierarchy**: `$Child => $Parent { … }` inherits behavior from `$Parent`
  (tracked in `Arcanum` and used by the parent‑forward rule E403).
- **State parameters**: captured in the Arcanum symbol table and used to
  validate transition argument arity (E405).
- **Entry/Exit**: `$>()` fires on entry, `<$()` on exit. V3 tests cover entry
  handlers broadly; exit handler support is covered structurally and will gain
  more runtime tests over time.

## 5. Transitions, Parent Forward, and Stack

V3 retains the classic transition operators as SOL‑anchored Frame statements
inside native bodies:

```frame
    machine:
        $A {
            e() {
                -> $B()
            }
        }
        $B {
            e() {
                => $^
            }
        }
```

Grammar:

- `frame_stmt ::= sol ( transition | parent_forward | stack_op )`
- `transition ::= '->' WS* '$' IDENT args_opt`
- `parent_forward ::= '=>' WS* '$^'`
- `stack_op ::= 'push$' | 'pop$'`

Semantics:

- Transitions are **terminal** within a handler (E400): nothing but comments
  may follow after a transition in the same block.
- Parent forward (E403) requires a parent state.
- Stack operations manage a state stack (`push$` push, `pop$` pop). V3 tests
  cover both inline and block‑level usages.

## 6. Actions, Operations, and Attributes

Actions and operations are **native‑body helpers**:

```frame
system Logger {
    actions:
        log(message): Result = None {
            print(message)
        }

    operations:
        @native
        helper(x): Result = None {
            return x
        }
}
```

Grammar:

- `actions_block ::= 'actions:' action_decl*`
- `action_decl ::= IDENT '(' param_list? ')' ( type_and_default? ) native_body`
- `operations_block ::= 'operations:' operation_decl*`
- `operation_decl ::= (attribute* ) IDENT '(' param_list? ')' ( type_and_default? ) native_body`
- `attribute ::= '@' IDENT`
- `type_and_default ::= ':' IDENT ( '=' expr_stub )?`

Semantics:

- **No Frame in actions/operations**: E401 is reported if a Frame statement
  appears in an action/operation body.
- Attributes are target‑specific hints (e.g., `@native`); V3 treats them as
  opaque and forwards them to codegen/runtime.

## 7. Functions and `fn main`

V3 supports top‑level Frame functions whose bodies are native code (with
optional Frame statements):

```frame
fn helper(x) {
    print(x)
}

fn main() {
    helper(42)
}
```

Rules:

- Functions may appear anywhere after the `@target` prolog.
- At most **one** `fn main` per module; duplicates are reported as E115.
- The function body is native and is spliced/expanded through the same MIR
  pipeline as handler bodies.

## 8. System Return and System Calls

V3 keeps the **`system.return`** convention for setting interface return
values from handlers and actions:

```frame
system Calc {
    interface:
        compute(a, b): int = 0

    machine:
        $Ready {
            compute(a, b) {
                system.return = a + b
            }
        }
}
```

Semantics:

- `system.return` behaves like a special variable bound to the current
  interface call. When the interface method returns, its value is taken from
  `system.return` (if set).
- Only `system.return` is special; bare `system` or other `system.*` accesses
  are treated as native expressions.

V3 also allows calling interface methods and other helpers via `system` from
handlers, actions, and operations:

```frame
system Chained {
    interface:
        step()
        status(): str = "idle"

    machine:
        $Idle {
            step() {
                system.status()
            }

            status() {
                system.return = "idle"
            }
        }
}
```

These calls are parsed as native member calls; the runtime/codegen and
validator enforce the V3 policy that:

- `system.method()` used as a system‑level call must target an `interface:`
  method (E406 if it does not).
- `system.return` is a native pseudo‑variable that can be read or written from
  handlers, actions, and non‑static operations. Event handlers additionally
  get sugar: `return expr` is expanded to `system.return = expr; return`, while
  a bare `return` leaves the current `system.return` value unchanged.

Runtime behavior for `system.return` (mapping to the underlying per‑call
return stack and header defaults) is implemented in the V3 Python and
TypeScript generators, and covered by the `v3_capabilities/system_return`
fixtures (handlers, actions, and operations).

## 9. Capability Matrix and Tests

The V3 capability matrix at:

- `docs/framelang_design/capability_matrix.md`

serves as the authoritative checklist that:

- Lists each Frame‑level V3 feature (systems, parameters, transitions, stack,
  system.return, etc.).
- Links to the V3 fixtures that validate it (per PRT language).
- Separately tracks legacy‑only features that remain in the old grammar but
  are not yet part of the V3 runtime.

When adding or changing V3 features:

1. Update `architecture_v3/grammar.md` and, if needed, `codegen.md`.
2. Add or update fixtures under `framec_tests/language_specific/<lang>/v3_*`.
3. Update `capability_matrix.md` with the feature and fixture links.
4. Run `framec_tests/runner/frame_test_runner.py` for the relevant languages
   and categories to keep V3 coverage honest.
