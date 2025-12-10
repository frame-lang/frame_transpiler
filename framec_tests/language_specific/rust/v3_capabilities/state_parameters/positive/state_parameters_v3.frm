@target rust

system S($(color), domain) {
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
                -> $Red("red")
            }
        }

    domain:
        domain: &'static str = "red"
}
