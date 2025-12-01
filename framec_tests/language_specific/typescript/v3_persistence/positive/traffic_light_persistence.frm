@target typescript
// @run-expect: Red
// @run-expect: Green
// @run-expect: Yellow
// @run-expect: Red

@persist system TrafficLight($(color), domain) {
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

    const json = TrafficLight.saveToJson(tl);
    const tl2 = TrafficLight.restoreFromJson(json);

    tl2.tick();
    tl2.tick();
    tl2.tick();
}
