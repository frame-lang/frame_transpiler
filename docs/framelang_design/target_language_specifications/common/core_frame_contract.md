# Core Frame Contract Across Target Languages

## Purpose

This document captures the semantic surface every Frame target must implement before any target-specific substitutions occur. It serves as the common contract that Python, TypeScript, LLVM, and future backends share. Target specifications should treat this content as **do-not-change** behaviour and focus on how their platform realises it.

## Guiding Principles

1. **State Machines First** – Frame files describe deterministic state machines. Targets MAY choose idiomatic syntax, but MUST preserve the state topologies, transitions, and lifecycle hooks defined in Frame.
2. **Event-Driven Execution** – All behaviour is triggered through events routed by the runtime kernel. Targets MUST keep the split between event creation (interface/actions) and event handling (state dispatch).
3. **Runtime Kernel Responsibilities** – Regardless of language, the kernel owns the transition loop, enter/exit sequencing, forwarded events, and queue processing. Visitors emit code that *delegates* to the kernel, not inline replacements.
4. **Deterministic Semantics** – The same Frame program must produce equivalent observable behaviour on every backend (state progression, domain mutations, emitted outputs) even if implementation details differ.

## Structural Elements

| Frame Construct | Description | Required Runtime Support |
| --- | --- | --- |
| **System** | Top-level state machine definition | Allocation, initial compartment, lifecycle init/deinit |
| **Compartment** | Runtime snapshot of current state + payload | Storage for state name, domain view, enter/exit args, forwarded events |
| **State** | Named location with event handlers | Dispatch tables mapping events → handler bodies |
| **Interface** | Public event producers | Functions that package an event and hand it to the kernel |
| **Actions / Operations** | Reusable logic invoked by handlers | Method calls with access to domain/local scope |
| **Domain Variables** | Persistent state across events | Struct fields or equivalent storage updated via visitors |
| **Event Handlers** | Code triggered by events | Must run inside the kernel loop with access to enter/exit context |
| **Transitions** | Moves between states | Kernel-managed loop to execute exit → state change → enter |
| **Forwarded Events** | Parent/queue continuation messages | Kernel API to set/get queued events |
| **State Stack** | Push/pop semantics for hierarchical flows | Kernel operations to snapshot/restore compartments |

## Behavioural Requirements

- **Enter/Exit Ordering** – `$>()` runs after the kernel commits a new compartment; `$<()` runs before leaving the current compartment. Targets must not reorder these hooks.
- **Queue Semantics** – Events enqueued during handler execution run only after the current handler finishes and the kernel loop iterates.
- **Domain Visibility** – Domain fields are shared across handlers within the system. Targets must respect mutability semantics (assignments are visible to subsequent events).
- **Local Variables** – Handler locals are lexically scoped per invocation and must not leak into shared state unless explicitly assigned to domain/action parameters.
- **Error Propagation** – Exceptions/failures inside handlers propagate through the same kernel pathway; targets may map to their native exception model, but must stop the transition loop when the Frame semantics would halt.
- **Typed Payloads** – Enter args, state args, and forwarded event parameters retain their value semantics across targets (numeric vs boolean vs string) even if the runtime representation differs.

## Target Extension Boundaries

Targets may introduce native constructs (e.g., TypeScript modules, LLVM intrinsics) **only after** mapping the above contract. When adding target-specific pragmas or features, ensure that:

1. State definitions, transition graphs, and domain declarations remain in Frame.
2. Native code augments handler bodies or interfaces without bypassing the kernel APIs.
3. Instrumentation (logging, metrics) does not alter ordering guarantees.
4. Testing includes cross-target assertions to confirm contract fidelity.

## Reference Implementation Touchpoints

- **Parser / AST** – Emits a uniform representation (`FrameModule`, `SystemNode`, `EventHandlerNode`, etc.). All targets consume this AST before introducing native syntax.
- **Runtime APIs** – All targets rely on a small surface area (`FrameEvent`, compartment setters/getters, kernel loop). Any change here must be mirrored across runtimes.
- **Test Fixtures** – Smoke suites (`framec_tests`) validate the shared contract. When expanding tests, add equivalents for every supported target.

## Next Steps

- Mirror this contract inside each target specification, referencing concrete runtime APIs used to satisfy the requirements.
- Update TypeScript, Python, and LLVM docs to highlight where they differ *after* fulfilling the contract.
- Use this document as the checklist when evaluating new targets or pragma-based extensions.
