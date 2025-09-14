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
                print(domain_var)
            }
        }
        
    domain:
        var domain_var:string = "domain test"
}