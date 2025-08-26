fn main() {
    var sys = TestSystem()
    sys.test("hello")
}

system TestSystem {
    interface:
        test(msg:string)
        
    machine:
        $Start {
            test(msg:string) {
                print(msg)
            }
        }
}