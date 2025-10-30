# TypeScript-Specific Statement Guidance

## Purpose
This document lists constructs the TypeScript backend emits that rely on TypeScript/Node.js features beyond the common Frame contract. Use these patterns to substitute Python-centric statements with idiomatic TypeScript code while preserving semantics.

## Runtime & Language Integration Points

| Area | TypeScript Behaviour | Notes |
| --- | --- | --- |
| Event Type | `FrameEvent` class/interface | Implemented in the TypeScript runtime library |
| Kernel Loop | `__kernel(event: FrameEvent): void` with `while (this.__next_compartment !== undefined)` | Mirrors Python transition loop |
| Dispatcher | `private __router(event: FrameEvent): void` using `switch`/lookup tables | Strong typing enforces legal states/events |
| Async I/O | Promises with `async/await` (`net.createConnection`, `socket.on`) | Replaces Python `asyncio` operations |
| Domain Storage | Class fields (`this.count`) | Must align with generated type declarations |
| Exception Model | `try { ... } catch (e) { ... }` with `unknown` cast | Keep error variables local (no `this.e`) |
| Logging/Print | `console.log()` | Replace Python `print()` calls |
| Data Structures | Native TypeScript (`Record<string, unknown>`, arrays) | Use interfaces and generics to type payloads |
| Task Scheduling | `Promise.resolve().then(...)`, `setImmediate(...)` | Stand-in for `asyncio.create_task` |

## Statements Replacing Python-Specific Logic

1. **Async Socket Operations**
   ```typescript
   private createConnection(host: string, port: number): Promise<net.Socket> {
       return new Promise((resolve, reject) => {
           const socket = net.createConnection(port, host);
           socket.once('connect', () => resolve(socket));
           socket.once('error', reject);
       });
   }
   ```
   *Use with `await this.createConnection(...)` instead of `asyncio.open_connection`.*

2. **Event Dispatch**
   ```typescript
   const event = new FrameEvent("onConnected", null);
   this.__kernel(event);
   ```
   *Keep event construction typed; leverage interfaces for payload shape.*

3. **Forwarded Events**
   ```typescript
   if (nextCompartment.forwardEvent === undefined) {
       this.__router(new FrameEvent("$>", this.__compartment.enterArgs));
   }
   ```
   *Ensure optional checks mirror the kernel contract.*

4. **State Stack Handling**
   ```typescript
   this.__stateStack.push(snapshot);
   const previous = this.__stateStack.pop();
   ```
   *Use typed arrays (`Array<StateSnapshot>`) to mirror Python list behaviour.*

5. **Domain Mutations**
   ```typescript
   this.count = this.count + 1;
   ```
   *Keep numeric types explicit to avoid unintended coercions.*

6. **Background Tasks**
   ```typescript
   private scheduleProcessing(): void {
       void Promise.resolve().then(() => this.__processMessages());
   }
   ```
   *Replaces `asyncio.create_task` without blocking the kernel loop.*

7. **Error Handling**
   ```typescript
   try {
       await this.doWork();
   } catch (error) {
       const message = error instanceof Error ? error.message : String(error);
       this.__kernel(new FrameEvent("onError", { error: message }));
   }
   ```
   *Convert errors to string payloads to match Frame expectations.*

## Notes for Target-Specific Pragmas
- Define TypeScript-only logic inside Frame pragmas only after satisfying the core contract.
- Prefer runtime helper modules (`frame_runtime_ts`) for complex behaviour so other targets can mirror it.
- Update this document whenever introducing new TypeScript-specific statements or Node.js APIs.

