@target rust

// V3 capability: system.return header defaults and handler returns (Rust).
//
// Mirrors the Python/TypeScript fixtures:
// - Interface header defaults: a1(): i64 = 10, a2(a: i64): i64 = a,
//   a3(a: i64): i64 = self.x + a
// - Handler bodies that use:
//   - bare `return;` (leave system.return at the header default)
//   - `return expr;` sugar (override system.return)

system SystemReturnHeaderDefaultsRs {

    interface:
        a1(): i64 = 10
        a2(a: i64): i64 = a
        a3(a: i64): i64 = self.x + a

    machine:
        $Idle {
            a1() {
                if self.x < 5 {
                    return;
                } else {
                    return 0;
                }
            }

            a2(a: i64) {
                return;
            }

            a3(a: i64) {
                return a;
            }
        }

    actions:
        bump_x(delta: i64) {
            self.x = self.x + delta;
        }

    domain:
        x: i64 = 3
}

