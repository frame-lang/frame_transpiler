@target rust

// V3 TrafficLight system runtime smoke (Rust).
// Uses system/state parameters and fn main().
// @run-expect: Red
// @run-expect: Green
// @run-expect: Yellow
// @run-expect: Red

system TrafficLight($(color), domain) {
    interface:
        tick()

    machine:
        $Red($(color)) {
            tick() {
                println!("Red");
                -> $Green("green")
            }
        }

        $Green($(color)) {
            tick() {
                println!("Green");
                -> $Yellow("yellow")
            }
        }

        $Yellow($(color)) {
            tick() {
                println!("Yellow");
                -> $Red("red")
            }
        }

    domain:
        domain: &'static str = "red"
}

fn main() {
    let mut tl = TrafficLight("red", "red", None);
    tl.tick();
    tl.tick();
    tl.tick();
    tl.tick();
}
