# Python-Specific Statement Guidance

## Purpose
This guide captures constructs emitted by the Python backend that rely on Python runtime features beyond the core Frame contract. Use it to understand how Frame semantics are expressed idiomatically in Python.

## Runtime & Language Integration Points

| Area | Python Behaviour | Notes |
| --- | --- | --- |
| Event Type | `FrameEvent` class | Implemented in Python runtime module |
| Kernel Loop | `__kernel` method with `while self.__next_compartment is not None` | Manages enter/exit sequencing |
| Dispatcher | `__router` method dispatches based on `__compartment.state` | Uses Python dictionaries for jump tables |
| Async I/O | `asyncio` APIs (`open_connection`, `create_task`) | Integrates with Python's asynchronous networking |
| Domain Storage | Instance attributes (`self.field`) | Direct attribute access; no mutability barriers |
| Exception Model | `try/except` with `Exception as e` | Local variable capture, rethrow to halt kernel |
| Logging/Print | `print()` statements | STDOUT semantics |
| Data Structures | Native Python types (`dict`, `list`, `str`, `int`, `bool`) | Must map to target equivalents |
| Timeouts/Tasks | `asyncio.create_task`, `asyncio.sleep` | Uses asyncio scheduling primitives |

## Core Python Runtime Patterns

1. **Async Socket Operations**
   ```python
   reader, writer = await asyncio.open_connection(host, port)
   writer.write(data)
   await writer.drain()
   ```
   *Native Python async networking; other targets provide comparable primitives.*

2. **Event Dispatch Calls**
   ```python
   event = FrameEvent("onConnected", None)
   self.__kernel(event)
   ```
   *Targets must preserve event construction but may use native class/struct semantics.*

3. **State Stack Operations**
   ```python
   self.__state_stack.append(snapshot)
   snapshot = self.__state_stack.pop()
   ```
   *Other languages may need manual stack structures or runtime helpers.*

4. **Forwarded Event Handling**
   ```python
   if next_compartment.forward_event is None:
       self.__router(FrameEvent("$>", self.__compartment.enter_args))
   ```
   *Targets should invoke kernel APIs, not inline logic.*

5. **Domain Mutations**
   ```python
   self.count = self.count + 1
   ```
   *Ensure target respects same mutability and numeric semantics.*

6. **Exception Capture**
   ```python
   try:
       ...
   except Exception as e:
       self.__kernel(FrameEvent("error", {"error": str(e)}))
   ```
   *Translate to equivalent `try/catch` with local error variables.*

7. **Task Scheduling**
   ```python
   asyncio.create_task(self.__process_messages())
   ```
   *Targets must schedule background work using their async runtime.*

## When Adding Python-Specific Pragmas
- Keep state definitions in Frame; only handler bodies invoke native features.
- Use runtime helper modules for heavy lifting (e.g., `frame_runtime_py`), so other targets can mirror behaviour.
- Document any new Python-only statements here to keep the spec in sync.
