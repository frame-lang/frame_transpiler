fn main() {
    var sys = TestSystem()
    sys.test()
}

system TestSystem {
    interface:
        test()
        
    machine:
        $Start {
            test() {
                print(domain_var)
            }
        }
        
    domain:
        var domain_var:string = "test"
}