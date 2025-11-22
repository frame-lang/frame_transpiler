@target typescript

// V3 capability: system.return header defaults and handler returns (TypeScript).
//
// This fixture exercises:
// - Interface header defaults: a1(): int = 10, a2(a: number): int = a,
//   a3(a: number): int = this.x + a
// - Handler bodies that use:
//   - bare `return` (leave system.return at the header default)
//   - `return expr` sugar (override system.return)

system SystemReturnHeaderDefaultsTs {

  interface:
    a1(): number = 10
    a2(a: number): number = a
    a3(a: number): number = this.x + a

  machine:
    $Idle {
      a1() {
        if (this.x < 5) {
          return;
        } else {
          return 0;
        }
      }

      a2(a: number) {
        return;
      }

      a3(a: number) {
        return a;
      }
    }

  actions:
    bumpX(delta: number) {
      this.x = this.x + delta;
    }

  domain:
    x: number = 3
}

