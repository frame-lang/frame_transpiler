# Frame LLVM Runtime (Phase 2 – Week 8)

This crate hosts the experimental runtime support for the LLVM backend. The
initial scope is intentionally small – it mirrors the data structures used by
the Python and TypeScript runtimes so the visitor can begin wiring a common API:

* `FrameEvent` – lightweight event wrapper.
* `FrameCompartment` – carries state name plus enter/exit args.
* `FrameKernel` – placeholder kernel loop that will grow alongside the backend.
* `FrameBox` / `FrameRc` – helpers for future memory-management work.

As we extend the backend the runtime will absorb more responsibilities
(transitions, enter/exit handlers, host integrations). For now it exists to give
the visitor a real crate to link against and to provide space for incremental
Week 8 deliverables.
