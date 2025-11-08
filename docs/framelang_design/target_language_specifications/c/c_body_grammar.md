# Frame Target Body Grammar — C

Version: 0.2 (Draft)
Date: 2025-11-08

This document specifies how C statements may be embedded inside Frame action and handler bodies, and how they interleave with Frame statements while preserving the core runtime semantics. The common Frame grammar remains authoritative for structure, state machines, transitions, and kernel‑visible behavior.

References:
- Common grammar: `../common/frame_common_grammar.md`
- Core contract: `../common/core_frame_contract.md`

## Prolog
- File‑level target declaration: `@target c` (or `#[target: c]` in header).
- The Frame parser owns module structure, system/state/handler headers, transitions, forwards, and domain declarations.
- C statements appear inside handler/action/operation bodies and are captured as target‑specific regions.

## Mixed Body Model (Interleaving)

Within a handler/action body, the source is segmented into a sequence of Frame statements and native C statements. Ordering is preserved exactly as written. The kernel semantics are driven only by Frame statements.

MixedBody := { FrameStmt | CStmt }*

Allowed interleavable Frame statements (within mixed bodies):
- Interface dispatch: calling interface methods to enqueue events (arguments evaluated first).
- Transitions: `-> $State` and `-> (args) $State` (enter/exit ordering preserved by the kernel).
- Parent/forwarding: `=> $^` and queued forwards.
- Prints: `print(...)` lowered to runtime print helpers.
- Action calls and Operation calls (operations may return a value used in expressions).
- Domain declarations: `var name [: type] = expr` (target visitor maps to emitted storage as appropriate).
- Domain assignments: `self.field = expr`.
- Operation returns: `return expr` (in operation contexts only).
- State stack ops: push/pop via standard intrinsics emitted by the visitor.

Allowed Frame expressions (usable in the above):
- Literals: int, float, bool, string.
- Identifiers and domain fields: `x`, `self.field`.
- Arithmetic and boolean logic: `+ - * / %`, comparisons, `and/or/not` mapping to C’s operators where applicable.
- Calls: action/operation calls; interface argument expressions; native module calls that return scalars or opaque handles.
- String concatenation/formatting as defined by the common grammar and mapped by the visitor.

Disallowed as “native replacements” inside mixed bodies:
- Reimplementing transitions or queue semantics in C; use the Frame forms so the kernel maintains ordering and enter/exit behavior.
- Structural declarations (systems, states, interface/action/operation headers) within C regions.

## Lexical Rules (C Regions)

In `TargetSpecific(C)` regions the scanner recognizes:
- Comments: `//` line comments and `/* … */` block comments.
- Strings: `'…'` and `"…"` with standard C escapes.
- Braces: `{` and `}` update a local brace depth counter used for region boundary detection.

Note:
- Backticks and template strings are not recognized (TypeScript‑specific).
- Python comment forms (`#`) have no special meaning in C regions.

## Region Boundary Detection

Native C regions end when either of these is detected at the start of a new logical line while the brace depth is zero:
- A closing `}` that terminates the surrounding Frame body, or
- An upcoming Frame keyword (e.g., `system`, `machine`, `actions:`, `operations:`, `domain:`) or a recognized block header.

This ensures nested braces and strings inside the C code do not prematurely end the region.

## Statements and Operators (C Subset)

As a baseline, statements follow C99:
- Control flow: `if/else`, `for`, `while`, `do { } while`, `switch`, `break`, `continue`, `return`.
- Declarations: block‑scoped declarations are permitted inside native regions; the visitor/runtime decide whether such locals are visible across boundaries (they are not visible to Frame expressions).
- Operators: arithmetic, bitwise, logical (`&&`, `||`, `!`), comparisons, assignment.

Portability guidance:
- Prefer portable C99 features. Avoid compiler extensions and VLAs unless your target toolchain standardizes them.
- Preprocessor directives inside bodies are discouraged; include files and macros should be handled by the generated module header/footer managed by the visitor.

## Data and Type Bridging

Values crossing between Frame and native C should be either:
- Scalars: `int`, `double`, `bool`, `char*` (mapped from Frame’s int/float/bool/string), or
- Opaque handles: e.g., `void*`/`i8*` representing runtime or capability objects declared via `native module` interfaces.

Recommended patterns:
- Use Frame domain fields (`self.field`) to exchange data between C code and Frame statements within a mixed body.
- Use Frame interface/transition/action/operation calls to invoke kernel‑visible effects; pass only supported scalar/opaque values as arguments.

## Semantics and Ordering

- Interface calls enqueue events; they do not synchronously dispatch handlers.
- Transitions commit a new compartment and then honor exit/enter ordering before subsequent statements continue.
- Forwarded events and state‑stack operations interact solely through the runtime APIs the visitor emits; do not hand‑roll these in C.

## Diagnostics and Source Mapping

- Native C blocks are captured as passthrough ASTs; the compiler does not parse C syntax. Syntax errors surface at C compiler time (when compiling generated code).
- The scanner records line offsets so diagnostics can display both Frame source line and target line where possible.

## Domain Declarations (C Target)

Domain variables are declared in the Frame `domain:` block:

```
domain:
    var count: int = 0
    var flag: bool = false
```

The C visitor maps these to struct members with appropriate initializers and alignment.

## Example: Mixed Body

```
@target c

system Demo {
    interface:
        trigger(msg: string, n: int)

    machine:
        $Start {
            trigger(msg: string, n: int) {
                // C native statements
                int x = n + 1;
                printf("got: %s\n", msg);

                // Frame transition with evaluated args
                -> (msg, x) $Next

                // More C
                puts("transition queued");
            }
        }

        $Next {
            $>(m: string, k: int) {
                print("entered Next")
                print(m)
                print(k)
            }
        }
}
```

The C code executes exactly where written relative to the Frame statements; the transition updates state via the kernel and invokes `$>` accordingly.

## Initially Unsupported / Policy Notes

- No C preprocessor management inside bodies (the visitor controls headers/defines at file scope).
- Collections/structs crossing boundaries should use opaque handles provided by declared native modules.
- No attempt is made to reinterpret native control flow into Frame semantics; use Frame constructs for state changes and eventing.
