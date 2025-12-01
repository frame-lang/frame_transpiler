@target rust

// V3 TrafficLight system persistence smoke (Rust).
// Mirrors the Python/TypeScript persistence fixtures and exercises the
// generated `save_to_json` / `restore_from_json` helpers for `@persist`
// systems.
// @run-expect: Red
// @run-expect: Green
// @run-expect: Yellow
// @run-expect: Red

@persist system TrafficLight($(color), domain) {
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
    // Start in Red and advance once.
    let mut tl = TrafficLight("red", "red", None);
    tl.tick();

    // Take a snapshot, round-trip through JSON, and restore into a fresh instance
    // using the generated helpers on the Rust system struct.
    let json = tl.save_to_json();
    let mut tl2 = TrafficLight::restore_from_json(&json);

    // Continue execution from the restored state.
    tl2.tick(); // Green
    tl2.tick(); // Yellow
    tl2.tick(); // Red
}
