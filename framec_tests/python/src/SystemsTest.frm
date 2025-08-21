fn main() {
    var sys = NoParameters()
}

system NoParameters {
    machine:
        $Start {
            $>() {
                print("NoParameters started")
                return
            }
        }
}