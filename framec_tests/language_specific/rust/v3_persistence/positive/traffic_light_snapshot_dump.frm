@target rust

// Cross-language snapshot export (Rust).
// This system mirrors the TrafficLight machines used in the Python and
// TypeScript persistence fixtures. A separate Rust harness will drive
// it to Green and construct a SystemSnapshot via frame_persistence_rs.

system TrafficLight($(color), domain) {
    interface:
        tick()

    machine:
        $Red($(color)) {
            tick() {
                -> $Green("green")
            }
        }

        $Green($(color)) {
            tick() {
                -> $Yellow("yellow")
            }
        }

        $Yellow($(color)) {
            tick() {
                -> $Red("red")
            }
        }

    domain:
        domain: &'static str = "red"
}
