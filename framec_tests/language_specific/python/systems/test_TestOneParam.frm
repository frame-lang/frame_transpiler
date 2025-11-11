@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    sys2 = StartStateParameters("hello")
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
