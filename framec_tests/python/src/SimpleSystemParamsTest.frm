# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    var sys = TestSystem("a")
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