# Frame V3 AI Guide — Generating Correct Frame Programs

**Audience:** LLMs / AI assistants writing Frame V3 source.  
**Goal:** Make it hard to generate syntactically invalid Frame, and easy to stay aligned with the V3 architecture.

Authoritative references you must treat as ground truth:

- Grammar: `docs/framelang_design/architecture_v3/grammar.md`
- Human guide: `docs/framelang_design/architecture_v3/frame_language_guide.md`
- Codegen/runtime: `docs/framelang_design/architecture_v3/codegen.md`

When in doubt, open those files and follow their rules, not your own intuition.

---

## 1. Global Rules For All Targets

1. **Always emit a prolog first**
   - Syntax:
     ```frame
     @target python_3
     ```
   - Valid targets (V3): `python_3`, `typescript`, `rust`, `csharp`, `c`, `cpp`, `java`.
   - The prolog is **authoritative**: it determines how native bodies are interpreted.

2. **Extensions are advisory only**
   - Recommended extensions (per `architecture_v3_overview`):
     - Generic: `.frm`
     - Python: `.fpy`
     - TypeScript: `.frts`
     - Rust: `.frs`
     - C: `.fc`
     - C++: `.fcpp`
     - C#: `.frcs`
     - Java: `.fjava`
   - The parser only trusts `@target`, **not** the extension, to decide the language.

3. **Know what is Frame vs native**
   - Frame (V3) is *only*:
     - `@target` prolog.
     - `system` declarations, system parameters, and block headers.
     - State declarations and handler headers (`$State`, `e()`, `$>()`, `<$()`).
     - `actions:` and `operations:` headers (bodies are native).
     - Top‑level `fn` headers (bodies are native).
     - SOL‑anchored Frame statements inside native bodies:
       - `-> $State(args?)`
       - `=> $^`
       - `push$` / `pop$`
   - **Everything else** in bodies is native syntax for the target language.

4. **Block order matters**
   - Canonical per‑system block order (V3 validator, E113):
     1. `operations:`
     2. `interface:`
     3. `machine:`
     4. `actions:`
     5. `domain:` (when present)
   - Do not reorder or repeat blocks unless you intentionally want E113/E114.

---

## 2. Systems and Blocks — Canonical Shape

Minimal example (Python target, but the Frame shape is the same for all targets):

```frame
@target python_3

system TrafficLight($(startColor), $>(enterColor), domain) {
    operations:
        logTransition(state) {
            print(state)
        }

    interface:
        tick()
        status(): str = "UNKNOWN"

    machine:
        $Red {
            $>() { print("Red") }
            tick() { -> $Green("green") }
        }

        $Green(color) => $Red {
            $>() { print("Green") }
            tick() { -> $Yellow("yellow") }
        }

        $Yellow(color) {
            $>() { print("Yellow") }
            tick() { -> $Red("red") }
        }

    actions:
        record(state) {
            system.logTransition(state)
        }

    domain:
        log = None
}
```

Key points for generation:

- `system Name(system_params?) { … }`
  - `system_params` may include:
    - `$(...)` start parameters.
    - `$>(...)` enter parameters.
    - Plain identifiers for domain parameters.
- Blocks when present must follow the order above.
- `domain:` is native code for the target language:
  - Its body must be valid Python/TypeScript/Rust/etc., and is run in the system’s initialization context.
  - There is no separate declaration DSL; you write native field declarations/initialization exactly as you would in a normal host module.

---

## 3. States and Handlers

Inside `machine:`, you describe states and their handlers:

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

            value() {
                system.return = current
            }
        }
}
```

Generation rules:

- State header:
  ```frame
  $StateName(params?) (=> $ParentState)? { ... }
  ```
- Handler headers (inside a state):
  - Entry: `$>() { … }`
  - Exit: `<$() { … }`
  - Event handler: `eventName(params?) { … }`
- Handler bodies are **native code** for the target language, but you may embed SOL‑anchored Frame statements (see §5).

---

## 4. Actions and Operations

Actions and operations are helpers with native bodies:

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

Rules:

- `actions:`:
  ```frame
  name(params?) (type_and_default?) { native_body }
  ```
- `operations:`:
  ```frame
  @attribute*
  name(params?) (type_and_default?) { native_body }
  ```
- `type_and_default` is Frame‑level only in headers:
  ```frame
  name(params): Type = DefaultExpr
  ```
- **Do not** embed Frame statements (`->`, `=>`, `$$`) in action/operation bodies in new code unless you know the validator allows it for that path.

---

## 5. Frame Statements Inside Native Bodies

Frame statements are **SOL‑anchored** inside native text. Examples:

```frame
machine:
    $A {
        e() {
            // Native control flow
            if shouldTransition:
                -> $B(1, 2)
        }
    }

    $B {
        e() {
            // Forward to parent state
            => $^
        }
    }
```

Patterns to use:

- Transition:
  ```frame
  -> $StateName()            # no args
  -> $StateName(arg1, arg2)  # with args
  ```
- Parent forward:
  ```frame
  => $^
  ```
- Stack operations:
  ```frame
  push$
  pop$
  ```

Constraints:

- These must start at SOL (subject to whatever indentation the native language allows).
- In V3 semantics, transitions are **terminal** in a handler: nothing but comments should follow them in the same block.

---

## 6. Top‑Level Functions (`fn`) — Important `:` vs `->` Rule

Top‑level Frame functions have Frame headers and native bodies:

```frame
fn helper(msg) {
    print(msg)
}

fn helper_with_type(msg): str {
    return msg
}

fn main() {
    helper("hello")
}
```

Generation rules:

- You must use **Frame’s return type syntax**, **not** the target language’s.
  - Correct:
    ```frame
    fn compute(lines): usize {
        // Rust body with -> usize in generated code,
        // but the Frame header uses ':'.
    }
    ```
  - Incorrect in Frame source (do not generate):
    ```frame
    fn compute(lines) -> usize { ... }   # WRONG in .frm/.frs
    ```
  - The compiler will translate `: usize` in the Frame header into `-> usize` in the generated Rust (or equivalent for other languages).
- At most one `fn main` per module; duplicates produce an error (E115).
- Function bodies are native and share the same MIR/expander pipeline as handlers:
  - You may embed `-> $State`, `=> $^`, `push$`, `pop$` inside Fn bodies.

When generating Frame helpers that eventually become Rust functions (e.g., in `.frs`):

- Use Rust types in the **type position**, but still Frame syntax:
  ```frame
  @target rust

  fn compute_base_indent(lines: &Vec<&str>) : usize {
      // Rust body here
  }
  ```

---

## 7. `system.return` and Header Defaults

V3 uses `system.return` as a per‑call return slot, with header defaults:

```frame
system Calc {
    interface:
        a1(): int = 10
        a2(a): int = a
        a3(a): int = self.x + a

    machine:
        $Ready {
            a1() {
                if x < 5:
                    return      # keeps system.return = 10
                else:
                    return 0    # sets system.return = 0
            }

            a2(a) {
                return          # keeps default value a
            }

            a3(a) {
                return a        # overrides default
            }
        }

    domain:
        x = 0
}
```

Guidelines:

- Interface headers may specify a default:
  ```frame
  name(params): Type = Expr
  ```
- In handlers:
  - `system.return = expr` explicitly sets the return value.
  - `return expr` sugar is allowed:
    - Expanded by codegen to `system.return = expr; return`.
  - Bare `return` leaves `system.return` unchanged (keep default or previous value).

Do not invent new magic `system.*` fields; only `system.return` is special.

---

## 8. Per‑Target Notes

### Python (`@target python_3`)

- Bodies are standard Python 3:
  - Use indentation for blocks.
  - End comment‑only blocks with `pass` where needed (tests already cover this).
  - Stage 7 may run `python -m py_compile` on generated modules.
- Frame statements use Python’s indentation rules but remain SOL‑anchored:
  ```frame
  tick() {
      print("Red")
      -> $Green()
  }
  ```

### TypeScript (`@target typescript`)

- Bodies are standard TS:
  - Use `;` where appropriate.
  - Use `//` and `/* */` comments.
- Transitions lower to calls into `FrameCompartment` and `_frame_transition`, as described in `codegen.md`.
- Use normal TS types in header type positions:
  ```frame
  interface:
      status(): string = "idle"
  ```

### Rust (`@target rust`)

- Bodies are standard Rust:
  - Use `let`, `match`, `if`, etc.
  - You may use `->` in **native** function signatures inside bodies, but not in Frame headers.
- For Frame functions:
  ```frame
  fn normalize_lines(lines: &Vec<&str>, base_indent: usize, pad: &str) : Vec<String> {
      // Rust body here
  }
  ```
  - The `: Vec<String>` is Frame syntax; the generated Rust will use `-> Vec<String>`.

---

## 9. Common Pitfalls for AIs (Avoid These)

1. **Using `->` in Frame headers**
   - Do **not** generate `fn name(...) -> Type { ... }` in `.frm`/`.frs`.
   - Always use `fn name(...): Type { ... }` and let codegen map it.

2. **Forgetting `@target`**
   - The first non‑blank token must be `@target ...`.
   - Do not emit native imports or `system` before the prolog.

3. **Wrong block order**
   - Do not place `domain:` before `machine:` or `actions:` before `machine:`.
   - Use the canonical order: operations → interface → machine → actions → domain.

4. **Frame statements in the wrong places**
   - Avoid `->` / `=>` / `$$` in `actions:` / `operations:` unless you know the validator/test suite allows it.
   - Never put Frame operators in the `domain:` block.

5. **Treating native constructs as Frame**
   - Do not attempt to “Frame‑parse” Python/TS/Rust syntax inside bodies.
   - If a construct is not listed in `grammar.md` as Frame, assume it is native and let the target language rules apply.

6. **Inventing new syntax**
   - Do not introduce new keywords, operators, or header shapes.
   - If you need something that is not expressible with the current grammar, stop and update the architecture docs + grammar first.

---

## 10. Checklist Before Emitting Frame Code

For each file you generate:

1. **Pick a target and add `@target`**.
2. **Follow the block order** for each system.
3. **Use Frame headers correctly**:
   - `system`, `states`, `handlers`, `actions`, `operations`, `fn`, `fn main`.
   - `: Type` in headers, not `->`.
4. **Keep Frame vs native boundaries clear**:
   - Only the constructs listed in `grammar.md` are Frame.
5. **Use existing examples as templates**:
   - Look at fixtures under `framec_tests/language_specific/<lang>/v3_*`.
   - Copy patterns from `frame_language_guide.md` and `codegen.md`.

If you are unsure whether a construct is valid Frame:

- Check `docs/framelang_design/architecture_v3/grammar.md`.
- If it isn’t there, **do not generate it** without updating the docs and tests first.
