> **⚠️ DEPRECATED - DO NOT READ UNLESS INSTRUCTED ⚠️**
>
> This document is archived and may contain outdated or incorrect information about Frame syntax.
> For current Frame V4 syntax, see `frame_v4_lang_reference.md` in the parent directory.

---

# Frame v4 Design Questions and Decisions Needed

## Overview
This document captures outstanding questions about Frame v4 language design and implementation, identified by reviewing v3 features against current v4 documentation. Each section requires design decisions before v4 implementation can be finalized.

---

## 1. Project Architecture

### 1.1 Project Configuration System
**Context**: v3 uses `frame.toml` for project configuration with entry points, build options, and module search paths.

**Questions**:
- [N] Will v4 continue using `frame.toml` for project configuration?
- [-] If yes, what fields are supported?
- [Y] If no, how will multi-file projects be configured? **Use native project files**
- [Y] Will we support native project files (package.json, Cargo.toml, etc.) instead?

**Decision**: **Frame as preprocessor - use native project configuration exclusively**

### 1.2 Module System and Frame Imports
**Context**: v3 supports module search paths and importing Frame modules. v4 docs only show native imports.

**Questions**:
- [Y] Can Frame systems import other Frame systems?
- [Y] What's the syntax? `import OtherSystem from './other.frm'`? **Yes, .frm extension indicates Frame import**
- [Y] How are Frame modules resolved? **Relative paths, converted to native imports of generated files**
- [Y] Can Frame systems be published as packages? **Yes, as native packages after compilation**

**Decision**: **Frame imports use .frm extension, preprocessor converts to native imports**

### 1.3 Multiple Systems Per File
**Context**: v3 supports multi-entity modules (multiple systems in one .frm file).

**Questions**:
- [Y] Can a single .frm file contain multiple systems in v4? (NOTE: .frm is not standard now)
- [ ] If yes, how are they accessed/exported?
- [ ] Are there any restrictions on multi-system files?

**Decision needed**: _________________________

---

## 2. Language Features

### 2.1 Frame Control Flow Statements
**Context**: v3 has Frame-specific control flow that may or may not be in v4.

**Questions**:
- [ ] Is the `assert` statement supported in v4?
- [ ] Is the `loop` construct supported or only native loops?
- [ ] What about `continue` statement (for continuing after returns)?
- [ ] Are there any Frame-specific control flow statements?

**Decision needed**: _________________________

### 2.2 Change State Operator (`:>`)
**Context**: v3 has both `->` (transition) and `:>` (change state) with different semantics.

**Questions**:
- [ ] Is the `:>` change state operator supported in v4?
- [ ] What's the semantic difference from `->` transition?
- [ ] When should each be used?

**Decision needed**: _________________________

### 2.3 History States
**Context**: HSM pattern requires history states but they're not documented in v4.

**Questions**:
- [ ] Are history states (`$H`, `$H*`) supported in v4?
- [ ] What's the syntax for shallow vs deep history?
- [ ] How do history states interact with state parameters?
- [ ] How do they interact with state stack operations?

**Decision needed**: _________________________

### 2.4 Conditional Transitions (Guards)
**Context**: v3 supports guard conditions on transitions.

**Questions**:
- [ ] Are conditional transitions supported?
- [ ] What's the syntax? `?condition? -> $State`?
- [ ] Can guards access event parameters and domain variables?
- [ ] What happens if guard fails?

**Decision needed**: _________________________

### 2.5 Constants and Enums
**Context**: v4 shows native syntax in blocks but no Frame-level constructs.

**Questions**:
- [ ] Are Frame-level constants supported? `const MAX = 100`?
- [ ] Are Frame-level enums supported? `enum Status { ... }`?
- [ ] Or should everything use native syntax in domain blocks?

**Decision needed**: _________________________

### 2.6 Start State Determination
**Context**: v3 uses first state in machine block as initial state.

**Questions**:
- [ ] How is the initial state determined in v4?
- [ ] Can it be explicitly specified? `machine start=$Ready:`?
- [ ] What if first state has parameters?

**Decision needed**: _________________________

---

## 3. System Capabilities

### 3.1 System Inheritance/Composition
**Context**: States can inherit (`$Parent => $Child`) but system inheritance unclear.

**Questions**:
- [ ] Can systems extend other systems? `system Child extends Parent`?
- [ ] Can systems compose/include other systems?
- [ ] How would system inheritance interact with interface/machine blocks?

**Decision needed**: _________________________

### 3.2 Generic/Parameterized Systems
**Context**: Modern languages support generics/templates.

**Questions**:
- [ ] Can Frame systems be parameterized? `system Queue<T> { }`?
- [ ] If yes, how do type parameters work with native code blocks?
- [ ] How would this work across different target languages?

**Decision needed**: _________________________

### 3.3 Access Modifiers
**Context**: v4 has interface=public, actions=private. Need finer control?

**Questions**:
- [ ] Can individual methods be marked public/private/protected?
- [ ] Can domain variables have access modifiers?
- [ ] Is there a `protected` concept for inherited systems/states?

**Decision needed**: _________________________

### 3.4 Async Event Handlers
**Context**: v4 shows async in actions but not clear for event handlers.

**Questions**:
- [ ] Can event handlers be async? `async handleData() { }`?
- [ ] How do async handlers interact with transitions?
- [ ] What about async enter/exit handlers?
- [ ] How does this work with the event dispatch mechanism?

**Decision needed**: _________________________

---

## 4. Runtime and Code Generation

### 4.1 Event Router Generation
**Context**: v3 generates `_frame_router` method. v4 approach unclear.

**Questions**:
- [ ] How does v4 generate the event dispatch mechanism?
- [ ] Is there still a router method or different approach?
- [ ] How much is generated vs. developer-implemented?
- [ ] How does it handle event parameters?

**Decision needed**: _________________________

### 4.2 Runtime Library Contents
**Context**: v4 mentions frame_runtime_py/ts/rs but not their contents.

**Questions**:
- [ ] What exactly is provided in runtime libraries?
- [ ] FrameEvent class? FrameCompartment? State stack?
- [ ] Is the runtime minimal or comprehensive?
- [ ] Can developers override runtime behavior?

**Decision needed**: _________________________

### 4.3 Source Maps and Debugging
**Context**: v3 has detailed source mapping. v4 simplified but debugging needs remain.

**Questions**:
- [ ] How does v4 maintain source line mappings?
- [ ] Will source maps be generated?
- [ ] How will debugging tools map generated code to Frame source?

**Decision needed**: _________________________

### 4.4 Error Handling Semantics
**Context**: Error behavior during state operations needs definition.

**Questions**:
- [ ] Can event handlers throw/raise exceptions?
- [ ] What happens to transitions if handler errors?
- [ ] What if exit handler fails? Enter handler fails?
- [ ] Is there automatic rollback or error state?

**Decision needed**: _________________________

---

## 5. State Machine Semantics

### 5.1 Transition Atomicity
**Context**: Need to define transaction boundaries for state changes.

**Questions**:
- [ ] Are state transitions atomic?
- [ ] If exit handler fails, does transition abort?
- [ ] Can transitions be rolled back?
- [ ] What's the exact order of operations?

**Decision needed**: _________________________

### 5.2 Event Queue and Priority
**Context**: Concurrent event handling needs clear semantics.

**Questions**:
- [ ] Is there an event queue or immediate dispatch?
- [ ] Can events have priorities?
- [ ] What happens if event arrives during transition?
- [ ] Are events processed synchronously or can they be async?

**Decision needed**: _________________________

### 5.3 State Parameter Lifetime
**Context**: State parameters passed in transitions need lifetime rules.

**Questions**:
- [ ] How long do state parameters persist?
- [ ] Until next transition? Until exit? Forever?
- [ ] Are they part of compartment or separate?
- [ ] What happens on state stack pop?

**Decision needed**: _________________________

### 5.4 Compartment Semantics
**Context**: State-local variables stored in compartments.

**Questions**:
- [ ] When are compartments created/destroyed?
- [ ] Do they persist across transitions to same state?
- [ ] How do they interact with state parameters?
- [ ] What about with hierarchical states?

**Decision needed**: _________________________

---

## 6. Migration and Compatibility

### 6.1 Deprecated Features List
**Context**: Need clear list of what's removed from v3.

**Questions**:
- [ ] Which v3 features are explicitly deprecated?
- [ ] Which are removed vs. changed syntax?
- [ ] Is there a migration guide planned?

**Decision needed**: _________________________

### 6.2 Version Detection
**Context**: Compiler needs to detect v3 vs v4 syntax.

**Questions**:
- [ ] How does compiler detect version?
- [ ] Check for `@@persist`, `@@system`, native annotations?
- [ ] Explicit version pragma? `@@version 4`?
- [ ] Command-line flag?

**Decision needed**: _________________________

### 6.3 Backwards Compatibility
**Context**: Transition period needs compatibility strategy.

**Questions**:
- [ ] Will there be a v3 compatibility mode?
- [ ] How long will v3 be supported?
- [ ] Can v3 and v4 code interoperate?
- [ ] Deprecation timeline?

**Decision needed**: _________________________

---

## 7. Implementation Strategy

### 7.1 Language Priority
**Context**: v3 has 7 languages, v4 proposes PRT first.

**Questions**:
- [ ] Confirm PRT (Python, Rust, TypeScript) for phase 1?
- [ ] When will C#, C, C++, Java be added?
- [ ] Will all languages have feature parity?
- [ ] Different feature sets per language?

**Decision needed**: _________________________

### 7.2 Feature Rollout
**Context**: How v4 features become available.

**Questions**:
- [ ] Version flag (`--frame-version=v4`) or feature flags?
- [ ] Gradual feature enablement or all at once?
- [ ] Beta period with feedback?
- [ ] How to communicate changes?

**Decision needed**: _________________________

### 7.3 Testing Strategy
**Context**: v4 changes require comprehensive testing.

**Questions**:
- [ ] Keep v3 tests and adapt or new test suite?
- [ ] How to test v3->v4 migration?
- [ ] Performance benchmarks v3 vs v4?
- [ ] Compatibility test suite?

**Decision needed**: _________________________

---

## 8. Critical Design Decisions

### 8.1 Frame vs Native Balance
**Core question**: How much should Frame define vs. delegating to native languages?

**Options**:
1. Minimal Frame: Only state machine structure, everything else native
2. Balanced: Frame control flow and key constructs, native for implementation  
3. Comprehensive: Full Frame language with native blocks for implementation

**Decision needed**: _________________________

### 8.2 Module System Design
**Core question**: Frame-specific or native module systems?

**Options**:
1. Frame module system with own resolution and imports
2. Purely native imports and module systems
3. Hybrid with Frame systems exposed via native modules

**Decision needed**: _________________________

### 8.3 Runtime Generation Philosophy  
**Core question**: How much runtime should Frame generate?

**Options**:
1. Minimal: Generate structure, developer implements dispatch
2. Moderate: Generate router and basic runtime
3. Comprehensive: Full runtime with all Frame features

**Decision needed**: _________________________

### 8.4 Project Structure Philosophy
**Core question**: Frame-specific or native project structure?

**Options**:
1. Keep frame.toml and Frame project structure
2. Use native project files (package.json, Cargo.toml)
3. Support both with adapters

**Decision needed**: _________________________

---

## Priority Questions Requiring Immediate Answers

1. **Module system**: Can Frame systems import other Frame systems?
2. **History states**: Are they supported and what's the syntax?
3. **Project config**: frame.toml or native?
4. **Router generation**: How much does Frame generate?
5. **Version detection**: How to detect v3 vs v4?

---

## Notes Section
_Space for additional thoughts, concerns, or clarifications:_

________________________________________________
________________________________________________
________________________________________________
________________________________________________
________________________________________________
