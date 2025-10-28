# Frame Transpiler Bug Index

## Statistics
- **Total Bugs**: 53
- **Open**: 6
- **Resolved**: 46
- **Won't Fix**: 1
- **Next Bug Number**: 054

## Quick Links
- [Bug Tracking Policy](BUG_TRACKING_POLICY.md)
- [Bug Template](TEMPLATE.md)
- [Open Bugs](open/)
- [Closed Bugs](closed/)

## Active Bugs

| Bug # | Title | Priority | Category | Status | Assignee |
|-------|-------|----------|----------|--------|----------|
| [#037](open/bug_037_state_diagram_missing_conditional_transitions.md) | State Diagram Missing Conditional Transitions | Low | CodeGen | Open | - |
| [#039](open/bug_039_missing_frame_semantic_metadata.md) | Missing Frame Semantic Metadata for Debugger | Medium | Tooling | Open | - |
| [#049](open/bug_049_typescript_transpilation_rate_low.md) | TypeScript Transpilation Rate Lower Than Python | Medium | CodeGen | Open | Claude |
| [#050](open/bug_050_test_runner_language_filtering.md) | Language-Specific Tests Running for All Languages | Low | Tooling | Open | - |
| [#051](open/bug_051_typescript_duplicate_imports.md) | TypeScript Generator Produces Duplicate Imports | High | CodeGen | Open | - |
| [#052](open/bug_052_typescript_actions_generate_stubs.md) | TypeScript Actions Generate Stubs Despite Proper Imports | Critical | CodeGen | Open | - |

## Recently Resolved

| Bug # | Title | Priority | Category | Status | Fixed Version |
|-------|-------|----------|----------|--------|---------------|
| [#053](closed/bug_053_typescript_missing_property_declarations.md) | TypeScript Missing Property Declarations and Runtime Imports | High | CodeGen | Resolved | v0.86.23 |
| [#048](closed/bug_048_unreachable_return_after_transition.md) | Unreachable Return After Transition Statements | High | Semantic | Won't Fix | N/A |
| [#047](closed/bug_047_typescript_complex_expression_support.md) | TypeScript Complex Expression Support | High | CodeGen | Resolved | v0.82.2 |
| [#046](closed/bug_046_python_import_support.md) | Python Import Support | Low | Documentation | Won't Fix | - |
| [#040](closed/bug_040_interface_method_source_mapping.md) | Interface Method Source Mapping | Medium | CodeGen | Resolved | v0.81.5 |
| [#038](closed/bug_038_string_concat_escape_sequences.md) | String Concatenation with Escape Sequences | High | CodeGen | Resolved | v0.81.4 |

## Bug Categories

### Parser (2)
- Bug #019: Parser Error with Functions After Systems
- Bug #020: Parser Error with State Parameters

### Semantic Analysis (3)
- Bug #048: Unreachable Return After Transition
- Bug #016: Circular Import Detection
- Bug #030: Spurious Unreachable Return Statements

### Code Generation (25)
- Bug #047: TypeScript Complex Expression Support
- Bug #049: TypeScript Transpilation Rate
- Bug #037: State Diagram Conditional Transitions
- Bug #040: Interface Method Source Mapping
- Bug #038: String Concatenation

### Runtime (5)
- Bug #029: Interface Method Routing
- Bug #031: Spurious Method Calls
- Bug #033: State Context Management

### Tooling (10)
- Bug #050: Test Runner Language Filtering
- Bug #039: Debugger Metadata
- Bug #018: Duplicate Source Mappings
- Bug #011: VS Code Debugger Offset

### Documentation (5)
- Bug #046: Python Import Support Docs
- Bug #044: Migration Guide
- Bug #042: API Documentation

## Search Index
<!-- For quick grep searching -->
<!-- Keywords: transition, return, unreachable, typescript, python, source map, debugger, test, import, state, diagram, interface, method -->

## Historical Notes
- Migrated from multiple tracking systems on 2025-10-13
- Bug #001-#036: Migrated from closed_bugs.md
- Bug #037-#047: Migrated from open_bugs.md and issues/
- Bug #048-#050: Active bugs as of migration
- Note: Bug #047 was numbered #48 in one tracking system, renumbered for consistency

## Maintenance Log
- 2025-10-13: Initial bug tracking system created, all bugs migrated

---
*Index Last Updated: 2025-10-13*  
*Policy Version: 1.0*