@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    sys = TestSystem("a")
}

system TestSystem ($(A)) {
    machine:
        $Start(A) {
            $>() {
                print(A)
                return
            }
        }
}
