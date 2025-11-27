@target python_3

# V3 TrafficLight system persistence smoke (Python).
# Uses frame_persistence_py to snapshot and restore a running system.
# @run-expect: Red
# @run-expect: Green
# @run-expect: Yellow
# @run-expect: Red

system TrafficLight($(color), domain) {
    interface:
        tick()

    machine:
        $Red(color) {
            tick() {
                print("Red")
                -> $Green("green")
            }
        }

        $Green(color) {
            tick() {
                print("Green")
                -> $Yellow("yellow")
            }
        }

        $Yellow(color) {
            tick() {
                print("Yellow")
                -> $Red("red")
            }
        }

    domain:
        domain = None
}

fn main() {
    from frame_persistence_py import (
        snapshot_system,
        restore_system,
        snapshot_to_json,
        snapshot_from_json,
    )

    # Start in Red and advance once.
    tl = TrafficLight("red", "red", None)
    tl.tick()

    # Take a snapshot, round-trip through JSON, and restore into a fresh instance.
    snap = snapshot_system(tl)
    data = snapshot_to_json(snap)
    snap2 = snapshot_from_json(data)
    tl2 = restore_system(snap2, lambda: TrafficLight("red", "red", None))

    # Continue execution from the restored state.
    tl2.tick()
    tl2.tick()
    tl2.tick()
}

