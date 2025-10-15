# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
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
                print(self.domain_var)
            }
        }
        
    domain:
        var domain_var:string = "test"
}