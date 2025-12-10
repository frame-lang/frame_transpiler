@target python_3

# @core
# @run-expect: Red
# @run-expect: Green
# @run-expect: Yellow
# @run-expect: Red

@persist system TrafficLight($(color), domain) {
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
    tl = TrafficLight("red", "red", None)
    tl.tick()

    json = TrafficLight.save_to_json(tl)
    tl2 = TrafficLight.restore_from_json(json)

    tl2.tick()
    tl2.tick()
    tl2.tick()
}
