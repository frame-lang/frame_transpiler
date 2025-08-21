fn main() {
    var sys2 = StartStateParameters("hello")
}

system StartStateParameters($(p1)) {
    machine:
        $S1(p1) { 
            $>() {
                print(p1)
                return
            }
        }
}