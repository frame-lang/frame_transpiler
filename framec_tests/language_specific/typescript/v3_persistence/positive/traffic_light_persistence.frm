@target typescript
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
    import {
        snapshotSystem,
        restoreSystem,
        snapshotToJson,
        snapshotFromJson,
    } from "../../../frame_persistence_ts";

    const tl = new TrafficLight("red", "red", null);
    tl.tick();

    const snap = snapshotSystem(tl);
    const json = snapshotToJson(snap);
    const snap2 = snapshotFromJson(json);
    const tl2 = restoreSystem(snap2, () => new TrafficLight("red", "red", null));

    tl2.tick();
    tl2.tick();
    tl2.tick();
}

