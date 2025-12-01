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
    # Start in Red and advance once.
    tl = TrafficLight("red", "red", None)
    tl.tick()

    # Take a snapshot, round-trip through JSON, and restore into a fresh instance
    # using the generated class helpers.
    data = TrafficLight.save_to_json(tl)
    tl2 = TrafficLight.restore_from_json(data)

    # Continue execution from the restored state.
    tl2.tick()
    tl2.tick()
    tl2.tick()
}
