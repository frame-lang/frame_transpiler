# Frame LLVM Runtime (Phase 2 – Week 8)

This crate hosts the experimental runtime support for the LLVM backend. The
initial scope is intentionally small – it mirrors the data structures used by
the Python and TypeScript runtimes so the visitor can begin wiring a common API:

* `FrameEvent` – lightweight event wrapper.
* `FrameCompartment` – carries state name plus enter/exit args.
* `FrameKernel` – placeholder kernel loop; queue scaffolding exists for future
  forwarded-event work but currently short-circuits once handlers finish. Parent
  dispatch (`=> $^`) pushes the current compartment into the parent stack via
  `frame_runtime_kernel_push_compartment`, and compartments now expose
  `frame_runtime_compartment_set_forward_event` for upcoming queue plumbing 
  while enter/exit forwarding remains a TODO.
* `FrameBox` / `FrameRc` – helpers for future memory-management work.

As we extend the backend the runtime will absorb more responsibilities
(transitions, enter/exit handlers, host integrations). For now it exists to give
the visitor a real crate to link against and to provide space for incremental
Week 8 deliverables. Queue plumbing stubs arrived in v0.86.22 but now short-
circuit until forwarded events are emitted from the visitor; the upcoming work
delivers those semantics alongside macOS-friendly packaging so `clang` links
cleanly on both Intel and Apple Silicon machines.

### Building on macOS

The unified test runner automatically rebuilds `libframe_runtime_llvm.dylib`.
If you are compiling manually on macOS:

```bash
cargo build -p frame_runtime_llvm
install_name_tool -id "@rpath/libframe_runtime_llvm.dylib" \
  target/debug/libframe_runtime_llvm.dylib
```

That keeps the install name compatible with the smoke tests and with the future
multiplatform packaging flow outlined in the LLVM plan. Parent-dispatch smoke
tests now print both child and parent handlers; enter/exit forwarding will land
once runtime forwarded events are fully wired.
