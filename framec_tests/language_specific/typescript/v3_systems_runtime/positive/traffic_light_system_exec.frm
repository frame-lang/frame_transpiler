@target typescript

// V3 TrafficLight system runtime smoke (TypeScript).
// Uses system/state parameters and fn main().
// @run-expect: Red
// @run-expect: Green
// @run-expect: Yellow
// @run-expect: Red

system TrafficLight($(color), domain) {
    interface:
        tick()

    machine:
        $Red(color) {
            tick() {
                console.log("Red")
                -> $Green("green")
            }
        }

        $Green(color) {
            tick() {
                console.log("Green")
                -> $Yellow("yellow")
            }
        }

        $Yellow(color) {
            tick() {
                console.log("Yellow")
                -> $Red("red")
            }
        }

    domain:
        domain = null
}

fn main() {
    const tl = new TrafficLight("red", "red", null);
    tl.tick();
    tl.tick();
    tl.tick();
    tl.tick();
}
