@target typescript

// Cross-language snapshot export (TypeScript).
// This fixture mirrors the Python snapshot_dump behavior: it runs a
// TrafficLight system to a known state and prints a single JSON snapshot.

system TrafficLight($(color), domain) {
    interface:
        tick()

    machine:
        $Red(color) {
            tick() {
                -> $Green("green")
            }
        }

        $Green(color) {
            tick() {
                -> $Yellow("yellow")
            }
        }

        $Yellow(color) {
            tick() {
                -> $Red("red")
            }
        }

    domain:
        domain = null
}
