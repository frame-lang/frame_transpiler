# Frame V4 Unified Summary

**Version:** 1.0
**Date:** February 2026
**Purpose:** Single-document summary of Frame V4 for review
**Source Documents:** frame_v4_lang_reference.md, frame_v4_architecture.md, frame_v4_codegen_spec.md, frame_v4_runtime.md, frame_v4_error_codes.md, frame_v4_plan.md

---

## Executive Summary

Frame V4 is a **pure preprocessor** for state machine specifications. Native code (imports, functions, test harnesses) passes through unchanged, while `@@system` blocks are expanded into target language classes. This "oceans model" keeps Frame minimally invasive while providing powerful state machine semantics.

**Target Languages:** Python 3, TypeScript, Rust, C (in development)

**Current Test Status:** Python 144/144 (100%), TypeScript 126/126 (100%), Rust 130/130 (100%), C 139/139 (100%) - 539/539 total (100%)

---

## Part 1: Language Syntax

### 1.1 File Structure

```frame
# Native preamble (imports, helpers) - passes through unchanged
import logging

@@target python_3                    # Required: target language
@@codegen { frame_event: on }        # Optional: code generation options
@@persist                            # Optional: enable serialization

@@system SystemName ($(state_params), $>(enter_params), domain_params) {
    interface:
        method(param: type): return_type = default_value

    machine:
        $StateName => $ParentState {
            $.stateVar: type = initializer

            $>(enter_param) { }      # Enter handler
            <$(exit_param) { }       # Exit handler
            eventName(params) { }    # Event handler
            => $^                    # Default forward to parent (optional)
        }

    actions:
        helperMethod() { }           # Private, can access state vars

    operations:
        publicMethod(): type { }     # Public, bypasses state machine

    domain:
        var instanceVar: type = value
}

# Native postamble (test harness) - passes through unchanged
if __name__ == '__main__':
    s = SystemName()
```

### 1.2 The 7 Frame Constructs

| Construct | Syntax | Purpose |
|-----------|--------|---------|
| **Transition** | `-> $State` | Change state with full lifecycle |
| **Forward** | `=> $^` | Delegate to parent state (HSM) |
| **Stack Push** | `push$` | Save current compartment to stack |
| **Stack Pop** | `pop$` / `-> pop$` | Pop from stack (discard or transition) |
| **State Variable** | `$.varName` | Per-state instance variable |
| **System Context** | `@@.param`, `@@:return` | Interface call context access |
| **System Return** | `system.return` | Interface return value (alias for `@@:return`) |

### 1.3 Transition Forms

| Form | Meaning |
|------|---------|
| `-> $State` | Simple transition |
| `-> $State(args)` | Transition with state parameters |
| `-> (args) $State` | Transition with enter arguments |
| `(args) -> $State` | Transition with exit arguments |
| `-> => $State` | Transition with event forwarding |
| `-> pop$` | Transition to popped state from stack |
| `(exit) -> => (enter) $State(state)` | Full form with all positions |

### 1.4 System Context (`@@`)

| Syntax | Meaning |
|--------|---------|
| `@@.param` | Interface parameter (shorthand) |
| `@@:params[param]` | Interface parameter (explicit) |
| `@@:return` | Get/set interface return value |
| `@@:event` | Interface method name being handled |
| `@@:data[key]` | Call-scoped data (survives transitions) |

### 1.5 HSM (Hierarchical State Machines)

**V4 uses explicit-only forwarding.** Unhandled events are ignored unless explicitly forwarded.

```frame
$Child => $Parent {
    specific_event() {
        // Handle in child
    }

    other_event() {
        // Partial handling, then forward
        log("preprocessing")
        => $^   // Explicit forward to parent
    }

    => $^       // Default: forward ALL other events to parent
}
```

---

## Part 2: Runtime Architecture

### 2.1 The Compartment (Central Data Structure)

The compartment is Frame's closure concept for states - it preserves state identity plus all scoped data:

| Field | Type | Purpose |
|-------|------|---------|
| `state` | string | Current state identifier |
| `state_args` | dict | Arguments passed via `$State(args)` |
| `state_vars` | dict | State variables (`$.varName`) |
| `enter_args` | dict | Arguments passed via `-> (args) $State` |
| `exit_args` | dict | Arguments passed via `(args) -> $State` |
| `forward_event` | Event? | Stashed event for `-> =>` forwarding |

### 2.2 Deferred Transition Model

Frame uses a **deferred transition** model where state changes are cached during event handling and processed by a central kernel after handler completion:

```
Interface method call
  -> Create FrameEvent + FrameContext
  -> Push context onto _context_stack
  -> Kernel processes event
     -> Router dispatches to state method
        -> State method routes to handler
           -> Handler may call __transition(compartment)
           -> Handler returns
        -> State method returns
     -> Kernel checks __next_compartment
     -> If pending: exit current, switch, enter new (loop)
  -> Pop context, return _return value
```

### 2.3 Generated System Structure

```
class System:
    # Inner classes
    FrameEvent           (if frame_event = on)
    FrameContext         (if frame_event = on)
    SystemCompartment    (always)

    # Fields
    __compartment        # Current state's compartment
    __next_compartment   # Pending transition target
    _state_stack         # Compartment stack (if push$/pop$ used)
    _context_stack       # Interface call context stack
    <domain vars>        # From domain: section

    # Runtime infrastructure
    __kernel()           # Event processing loop
    __router()           # Dispatch to state method
    __transition()       # Cache next compartment
    _state_X()           # Per-state dispatch method

    # User-defined
    interface_methods()  # Public API
    __actions()          # Private helpers
    operations()         # Public, bypass state machine
```

### 2.4 State Variable Behavior

| Event | Behavior |
|-------|----------|
| `-> $State` (normal transition) | State vars **reset** to initial values |
| `-> pop$` (history transition) | State vars **preserved** from saved compartment |
| Within state | State vars persist until state exits |

---

## Part 3: Transpiler Pipeline

### 3.1 Pipeline Stages

```
Source (.frm)
    |
    v
Frame Parser -> FrameAst
    |
    v
Arcanum -> Symbol table (states, events, variables)
    |
    v
Validator -> Errors/warnings or pass
    |
    v
System Codegen -> CodegenNode tree
    |
    v
Language Backend -> Target source code (.py, .ts, .rs, .c)
```

Each stage is a pure function. Errors halt the pipeline and report diagnostics.

### 3.2 Key Files

| File | Purpose |
|------|---------|
| `frame_parser.rs` | Parse `@@system` blocks into FrameAst |
| `arcanum.rs` | Build symbol table from AST |
| `frame_validator.rs` | Validate AST (E402, E403, E405) |
| `native_region_scanner.rs` | Find Frame statements in handler bodies |
| `system_codegen.rs` | Generate CodegenNode from AST |
| `backends/python.rs` | Python code emitter |
| `backends/typescript.rs` | TypeScript code emitter |
| `backends/rust_backend.rs` | Rust code emitter |
| `backends/c.rs` | C code emitter |

### 3.3 NativeRegionScanner

Scans handler bodies for Frame constructs within native code:

| Pattern | Region Type |
|---------|-------------|
| `->` `$<ident>` | Transition |
| `->` `=>` | Forwarding transition |
| `=>` `$^` | Forward to parent |
| `push$` | Stack push |
| `pop$` (not after `->`) | Stack pop (discard) |
| `$.` `<ident>` | State variable access |
| `@@.` `<ident>` | Context parameter |
| `@@:return` | Context return |
| `return` `<expr>` (in handler) | Return sugar |

**Critical:** Skip recognition inside string literals and comments.

---

## Part 4: Language Backend Patterns

### 4.1 Backend-Specific Dispatch

| Backend | Router | State Dispatch | Variable Access |
|---------|--------|----------------|-----------------|
| Python | `if/elif` | `if/elif` on event | `self.__compartment.state_vars["name"]` |
| TypeScript | `switch` | `switch` on event | `this.#compartment.stateVars["name"]` |
| Rust | `match` | `match` on event | `self.__compartment.state_vars.get("name")` |
| C | `if/else if` | `if/else if` on event | `FrameDict_get(compartment->state_vars, "name")` |

### 4.2 Backend-Specific Compartment

All backends use the canonical 6-field structure with language-appropriate idioms:

| Backend | State Vars Storage | State Stack |
|---------|-------------------|-------------|
| Python | `dict` | `list[Compartment]` |
| TypeScript | `Record<string, any>` | `Array<Compartment>` |
| Rust | `HashMap<String, serde_json::Value>` | `Vec<Compartment>` |
| C | `FrameDict*` | `FrameVec*` |

---

## Part 5: Persistence

### 5.1 `@@persist` Annotation

```frame
@@persist
@@system MySystem { ... }
```

Generates `save_state()` and `restore_state()` methods.

### 5.2 What Gets Persisted

- Current state (compartment state name)
- State stack (for push$/pop$ history)
- State arguments (`state_args`)
- State variables (`state_vars`)
- Enter/exit arguments
- Forward event reference
- Domain variables

### 5.3 What Gets Reinitialized on Restore

- `_context_stack` - initialized to empty
- `__next_compartment` - initialized to null/None

### 5.4 Generated Methods

| Language | Save | Restore |
|----------|------|---------|
| Python | `save_state()` -> `bytes` | `restore_state(data: bytes)` [static] |
| TypeScript | `saveState()` -> `any` | `restoreState(data: any)` [static] |
| Rust | `save_state(&mut self)` -> `String` | `restore_state(json: &str)` [static] |

---

## Part 6: Error Codes

### 6.1 Error Ranges

| Range | Category |
|-------|----------|
| E0xx | Parse errors |
| E1xx | Structural errors |
| E4xx | Semantic errors |
| W4xx | Warnings |

### 6.2 Key Error Codes

| Code | Name | Description |
|------|------|-------------|
| E001 | `parse-error` | Malformed Frame syntax |
| E105 | `missing-target` | `@@target` directive missing |
| E402 | `unknown-state` | Transition targets undefined state |
| E403 | `invalid-forward` | `=> $^` in state without parent |
| E404 | `duplicate-state` | State name declared more than once |
| E405 | `param-arity-mismatch` | Wrong number of parameters |
| E420 | `duplicate-state-var` | State variable declared twice |
| E430 | `hsm-cycle` | Circular parent chain |

---

## Part 7: Implementation Status

### 7.1 Test Status by Language

| Language | Status | Tests |
|----------|--------|-------|
| Python 3 | Complete | 144/144 (100%) |
| TypeScript | Complete | 126/126 (100%) |
| Rust | Complete | 130/130 (100%) |
| C | Complete | 139/139 (100%) |
| **Total** | **100%** | **539/539** |

### 7.2 Implemented Features

- Core transitions (`-> $State`)
- State variables (`$.varName`)
- Enter/exit handlers (`$>`, `<$`)
- Domain variables
- Interface methods with return values
- Actions and operations
- HSM with explicit forwarding (`=> $^`)
- State stack (`push$`, `pop$`, `-> pop$`)
- Event forwarding (`-> =>`)
- System context (`@@.param`, `@@:return`, `@@:event`, `@@:data`)
- Persistence (`@@persist`)
- State parameters (`-> $State(args)`)
- Enter/exit arguments

### 7.3 Future Roadmap

**Phase 12:** C Language Implementation (in progress)
- Full parity with Python/TypeScript/Rust
- FrameDict/FrameVec runtime library

**Phase 13:** System Usage Tagging (planned)
- `@@System()` syntax for tracking and validating system usage in native code
- Typo detection at transpile time
- Foundation for IDE tooling

**Phase 14:** Parent State Parameters (planned)
- `$Child => $Parent(args)` syntax for parameterized HSM inheritance
- Pass configuration/context to parent states
- Enable reusable, template-style parent state hierarchies

**Phase 15:** GraphViz/Diagram Generation (planned)
- Port V3 GraphViz DOT generation to V4 pipeline
- State machine visualization for documentation
- Foundation for VSCode extension live preview

---

## Quick Reference

### Minimal Example

```frame
@@target python_3

@@system Counter {
    interface:
        increment()
        getValue(): int

    machine:
        $Counting {
            $.count: int = 0

            increment() {
                $.count = $.count + 1
            }

            getValue(): int {
                @@:return = $.count
            }
        }

    domain:
        var name: str = "counter"
}

if __name__ == '__main__':
    c = Counter()
    c.increment()
    c.increment()
    print(c.getValue())  # Output: 2
```

### HSM Example

```frame
@@target python_3

@@system Lamp {
    interface:
        toggle()

    machine:
        $Off {
            toggle() {
                -> $On
            }
        }

        $On => $Off {
            toggle() {
                -> $Off
            }
        }

        $Dimmed => $On {
            toggle() {
                -> $Off
            }
        }
}
```

### History Stack Example

```frame
@@target python_3

@@system Dialog {
    interface:
        show(name: str)
        close()
        back()

    machine:
        $Main {
            show(name: str) {
                push$                    # Save current state
                -> (name) $Modal
            }
        }

        $Modal {
            $.dialogName: str = ""

            $>(name: str) {
                $.dialogName = name
            }

            show(name: str) {
                push$                    # Can stack modals
                -> (name) $Modal
            }

            close() {
                -> pop$                  # Return to previous state
            }

            back() {
                -> pop$                  # Same as close
            }
        }
}
```

---

## Document Index

| Document | Purpose |
|----------|---------|
| `frame_v4_lang_reference.md` | Complete language specification |
| `frame_v4_runtime.md` | Runtime architecture and semantics |
| `frame_v4_architecture.md` | Transpiler architecture and pipeline |
| `frame_v4_codegen_spec.md` | Generated code specification |
| `frame_v4_error_codes.md` | Error and warning code reference |
| `frame_v4_plan.md` | Implementation plan and status |
| `frame_v4_c_implementation_plan.md` | C language implementation details |

---

*This document consolidates the Frame V4 specification for review purposes. For authoritative details, consult the individual specification documents.*
