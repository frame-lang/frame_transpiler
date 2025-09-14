fn main() {
    var test = TestSystem()
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