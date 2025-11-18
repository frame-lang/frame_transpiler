@target python_3

# V3 TrafficLight system runtime smoke (Python).
# Uses system/state parameters and fn main().
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
    # initial: start-state params, enter_color: $> params, domain: domain object
    tl = TrafficLight("red", "red", None)
    tl.tick()
    tl.tick()
    tl.tick()
    tl.tick()
}
