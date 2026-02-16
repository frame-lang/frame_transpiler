# Frame V4 Implementation - Claude Context

## Architecture Summary

**Frame V4 is a preprocessor.** It:
- Parses Frame syntax (@@system, states, transitions)
- Validates Frame semantics (Arcanum symbol table)
- Generates target language code
- Preserves native code exactly as written

**Frame does NOT parse native code.** That's the target compiler's job.

## Key Documents

Read these for complete understanding:

1. **[docs/framelang_design/architecture_v4/README.md](docs/framelang_design/architecture_v4/README.md)** - V4 architecture overview
2. **[docs/framelang_design/architecture_v4/TWO_PASS_ARCHITECTURE.md](docs/framelang_design/architecture_v4/TWO_PASS_ARCHITECTURE.md)** - Two-pass validation model
3. **[docs/plans/VALIDATION_EXPANSION_PLAN.md](docs/plans/VALIDATION_EXPANSION_PLAN.md)** - Planned validation improvements
4. **[docs/architecture_v5/PLAN.md](docs/framepiler_design/architecture_v5/PLAN.md)** - Future native compiler integration (V5)

## Two-Pass Validation Model

| Pass | When | What | Who |
|------|------|------|-----|
| **Pass 1** | Transpile-time | Frame semantics | Frame compiler |
| **Pass 2** | Compile/Run-time | Native semantics | Target compiler (pyc/tsc/rustc) |

**Frame validates:**
- State existence (`-> $Unknown` → E402)
- Parent existence for forward
- Parameter arity
- Terminal statement position
- Section ordering

**Native compiler validates:**
- Variable existence
- Type compatibility
- Import resolution
- Syntax correctness

## The Oceans Model

Native code is the "ocean". Frame constructs are "islands".

```
Handler Body:
┌─────────────────────────────────────────────┐
│ x = compute_value()        ← Ocean (native) │
│ if x > threshold:          ← Ocean (native) │
│     -> $Exceeded           ← Island (Frame) │
└─────────────────────────────────────────────┘
```

- **NativeRegionScanner** finds Frame islands
- **Splicer** replaces islands with generated code
- Native code passes through unchanged

## Pipeline

```
Source (.frm)
     │
     ├──→ Frame Parser ──→ Frame AST
     │                         │
     │                         ▼
     │                     Arcanum (symbol table)
     │                         │
     │                         ▼
     │                     Validator (E4xx errors)
     │                         │
     │                         ▼
     │                     Codegen (CodegenNode IR)
     │                         │
     │                         ▼
     │                     Backend (Python/Rust/TS)
     │                         │
     ▼                         ▼
Native code ─────────────→ Target code
(preserved)                (generated + native)
```

## V4 Syntax

```frame
@@target python_3

# Native imports (preserved)
import math
from typing import List

@@system Calculator {
    interface:
        add(a: int, b: int): int

    machine:
        $Ready {
            add(a: int, b: int) {
                # Native code (preserved)
                result = a + b
                print(f"Result: {result}")

                # Frame statement (expanded)
                -> $Done
            }
        }

        $Done { }

    domain:
        var history: List = []
}

# Native code (preserved)
if __name__ == '__main__':
    calc = Calculator()
    calc.add(1, 2)
```

## Implementation Rules

### DO:
- Parse Frame constructs fully (AST)
- Store native code as byte spans
- Validate Frame semantics via Arcanum
- Use NativeRegionScanner to find Frame islands
- Use Splicer to combine native + generated code
- Preserve native code formatting exactly

### DON'T:
- Parse native code syntax
- Validate native code semantics
- Build native symbol tables
- Do cross-language type checking
- Reformat or modify native code

## Target Languages

**PRT (Priority):**
- Python 3 - Active
- Rust - Active
- TypeScript - Active

**Deferred:**
- C#, Java, C, C++ - Not actively maintained

## What's in V5 (Future)

V5 adds **optional** native code analysis for enhanced IDE support:
- Extract symbols from native code (using language parsers)
- Cross-reference Frame and native symbols
- "Did you mean?" suggestions for typos

This is opt-in and non-blocking. See `docs/architecture_v5/PLAN.md`.

## Common Mistakes to Avoid

1. **Parsing native code** - Don't. It's opaque bytes.
2. **Native validation** - Leave it to the target compiler.
3. **Cross-language types** - V5 scope, not V4.

## Quick Reference

### Key Files

| File | Purpose |
|------|---------|
| `v4/frame_parser.rs` | Parse Frame syntax |
| `v4/frame_ast.rs` | Frame AST types |
| `v4/arcanum.rs` | Symbol table |
| `v4/frame_validator.rs` | Frame validation |
| `v4/native_region_scanner.rs` | Find Frame islands |
| `v4/codegen/system_codegen.rs` | AST → CodegenNode |
| `v4/codegen/backends/*.rs` | CodegenNode → target code |
| `v4/pipeline/compiler.rs` | Main pipeline |

### Error Codes

| Code | Description |
|------|-------------|
| E001 | Parse error |
| E402 | Unknown state reference |
| E403 | Duplicate state definition |
| E405 | Parameter mismatch |
| E4xx | (More planned - see VALIDATION_EXPANSION_PLAN.md) |

---

**Remember: Frame V4 is a preprocessor. Parse Frame, preserve native, validate Frame only.**
