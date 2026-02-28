# Frame V4 Specification

**Status:** Normative — All four target languages implemented
**Version:** 1.4
**Date:** 2026-02-27
**Test Status:** 553/554 passing (99.8%) — Python 152/152, TypeScript 133/133, Rust 133/133, C 135/136

---

## Documents

| Document | Purpose |
|----------|---------|
| [frame_v4_lang_reference.md](frame_v4_lang_reference.md) | Complete language specification |
| [frame_v4_runtime.md](frame_v4_runtime.md) | Runtime architecture and semantics |
| [frame_v4_architecture.md](frame_v4_architecture.md) | Transpiler architecture and pipeline |
| [frame_v4_codegen_spec.md](frame_v4_codegen_spec.md) | Generated code specification for all targets |
| [frame_v4_error_codes.md](frame_v4_error_codes.md) | Error and warning code reference |
| [frame_v4_plan.md](frame_v4_plan.md) | Implementation plan and status |
| [frame_v4_c_implementation_plan.md](frame_v4_c_implementation_plan.md) | C language implementation plan |

---

## Target Languages

| Language | Status | File Extension |
|----------|--------|----------------|
| Python 3 | ✅ Complete | `.fpy` |
| TypeScript | ✅ Complete | `.fts` |
| Rust | ✅ Complete | `.frs` |
| C | 🚧 In Development | `.fc` |

---

## Quick Reference

### 7 Frame Constructs

| Construct | Syntax | Purpose |
|-----------|--------|---------|
| Transition | `-> $State` | Change state with lifecycle |
| Forward | `=> $^` | Delegate to parent state |
| Stack Push | `push$` | Save state to stack |
| Stack Pop | `pop$` | Pop from stack |
| State Variable | `$.varName` | Per-state instance variable |
| System Context | `@@.param`, `@@:return` | Interface call context access |
| System Return | `system.return` | Interface method return value (alias for `@@:return`) |

### File Structure

```frame
@@target python_3
@@codegen { frame_event: on, runtime: kernel }

@@persist
@@system Name ($(state_params), $>(enter_params), domain_params) {
    interface:
        method(): type = default

    machine:
        $State {
            $.var: type = init

            $>() { }
            $<() { }
            event() { }
        }

    actions:
        helper() { }

    operations:
        utility(): type { }

    domain:
        var x: type = value
}
```

---

## Archive

Superseded documentation is preserved in `archive/`.
