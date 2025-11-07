# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    test = TestSystem()
    test.do()
    return
}

system TestSystem {
    interface:
        do()
    
    machine:
        $Start {
            do() {
                print("TestSystem.do() called")
                return
            }
        }
}