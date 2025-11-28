@target python_3

# Cross-language snapshot export (Python).
# This fixture runs a simple TrafficLight system to a known state and
# prints a single JSON snapshot to stdout for external tools.

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
        domain = None
}

fn main() {
    from frame_persistence_py import snapshot_system, snapshot_to_json

    # Start in Red and advance once so we snapshot in Green.
    tl = TrafficLight("red", "red", None)
    tl.tick()

    snap = snapshot_system(tl)
    print(snapshot_to_json(snap, indent=None))
}

