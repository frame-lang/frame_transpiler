@target rust

// @core
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
    let mut tl = TrafficLight::new(
        serde_json::json!("red"),
        serde_json::json!("red"),
        "red",
    );
    tl.tick();

    let json = tl.save_to_json();
    let mut tl2 = TrafficLight::restore_from_json(&json);

    tl2.tick();
    tl2.tick();
    tl2.tick();
}
