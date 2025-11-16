# Release Notes — v0.86.43 (2025-11-15)

Type: Bug-fix (TypeScript V3 runnable module: actions, domain, interface params)

## Highlights

- **Bug #074 — TypeScript generator omits actions/domain declarations and drops interface params**
  - V3 TypeScript runnable modules now:
    - Emit domain fields declared under `domain:` as class properties:
      - Supports both `var name: Type` and `name = expr` forms.
      - Generates `public name: Type = expr;` (or `public name: any = expr;` when no explicit type).
    - Emit actions and operations declared under `actions:`/`operations:` as class methods:
      - Sync: `public actionName(params) { … }`
      - Async: `public async actionName(params) { … }`
      - Bodies use the spliced Frame expansions and preserve TypeScript syntax (semicolons, braces).
    - Propagate handler/interface parameters into method signatures:
      - Interface handlers like `runtimeMessage(payload)` now become `public runtimeMessage(__e: FrameEvent, compartment: FrameCompartment, payload) { … }`.
  - These changes remove TS2339/TS2304 errors for missing actions, domain fields, and event parameters in generated TS.

## Testing

- Added V3 CLI regression fixtures for TypeScript:
  - `language_specific/typescript/v3_cli/positive/actions_and_domain_emit_issues.frm`
    - Uses `@tsc-compile` to run `tsc` over the CLI-generated module.
    - Asserts presence of domain properties, action methods, and the `runtimeMessage` payload parameter.
  - `language_specific/typescript/v3_cli/positive/multi_state_interface_router.frm`
    - Updated to assert the new `runtimeMessage(__e, compartment, payload)` signature while preserving the no-duplicate-methods guarantee from #073.
- All TypeScript V3 CLI tests pass, and the new fixtures compile cleanly under `tsc` in the test harness.

## Version

- Workspace version bumped to **0.86.43**; `framec --version` reports `0.86.43`.

