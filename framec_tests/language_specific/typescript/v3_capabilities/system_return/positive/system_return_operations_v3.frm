@target typescript

// V3 capability: system.return from operations (TypeScript).
// Operations can read/write the per-call system.return slot just like
// handlers and actions. This fixture ensures that the compiler accepts
// the pattern and wires `system.return` through the TypeScript runtime
// return stack.

system SystemReturnOpDemoTs {
  operations:
    compute_default() {
      system.return = "op-default"
    }

  interface:
    status(): string = "idle"
    getDefault(): string = "op-default"

  machine:
    $Idle {
      status() {
        // Handler assigns system.return explicitly.
        system.return = "idle"
      }

      getDefault() {
        // Delegate to an operation that assigns system.return.
        this.compute_default()
      }
    }
}
