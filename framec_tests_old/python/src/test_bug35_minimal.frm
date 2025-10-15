system SimpleTest {
    interface:
        test()
        
    machine:
        $Start {
            test() {
                print("Hello")
            }
        }
}