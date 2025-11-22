@target python_3

# V3 capability: system.return header defaults and handler returns (Python).
#
# This fixture exercises:
# - Interface header defaults: a1(): int = 10, a2(a): int = a,
#   a3(a): int = self.x + a
# - Handler bodies that use:
#   - bare `return` (leave system.return at the header default)
#   - `return expr` sugar (override system.return)

system SystemReturnHeaderDefaultsPy {

    interface:
        a1(): int = 10
        a2(a): int = a
        a3(a): int = self.x + a

    machine:
        $Idle {
            a1() {
                if self.x < 5:
                    return
                else:
                    return 0
            }

            a2(a) {
                return
            }

            a3(a) {
                return a
            }
        }

    actions:
        bump_x(delta) {
            self.x = self.x + delta
        }

    domain:
        x = 3
}

