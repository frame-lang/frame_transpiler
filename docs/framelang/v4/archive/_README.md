> **⚠️ DEPRECATED - DO NOT READ UNLESS INSTRUCTED ⚠️**
>
> This document is archived and may contain outdated or incorrect information about Frame syntax.
> For current Frame V4 syntax, see `frame_v4_lang_reference.md` in the parent directory.

---

# Frame V4 Documentation

This directory contains the complete documentation for Frame V4.

---

## Core Documents

| Document | Purpose |
|----------|---------|
| [grammar.md](grammar.md) | Complete Frame V4 grammar with syntax and examples |
| [architecture.md](architecture.md) | Compiler architecture, pipeline, and components |
| [testing.md](testing.md) | Testing infrastructure, pragmas, and test patterns |
| [migration.md](migration.md) | Migration guide from Frame V3 to V4 |
| [plan.md](plan.md) | V4 implementation execution plan |

---

## Quick Reference

### V4 Directives

```frame
@@target python_3    # Required at start of every file
@@persist            # Generate serialization methods
@@system Name { }    # Declare a state machine system
```

### Runtime Statements

| Statement | Syntax | Purpose |
|-----------|--------|---------|
| Transition | `-> $State` | Change state with enter/exit lifecycle |
| Transition (pop) | `-> pop$` | Transition to state from stack |
| Forward | `=> $^` | Delegate event to parent state |
| Stack Push | `push$` | Save current state to stack |
| Stack Pop | `pop$` | Change to state from stack (no lifecycle) |

### System Structure

```frame
@@target python_3

@@system Example {
    interface:
        method()

    machine:
        $State {
            method() { }
        }

    actions:
        helper() { }

    domain:
        x = 0
}
```

---

## Archive

Superseded documents are preserved in the `archive/` subdirectory for reference.
