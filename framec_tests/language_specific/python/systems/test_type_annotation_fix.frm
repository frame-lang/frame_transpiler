# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    sys = TestSystem()
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
        domain_var:string = "domain test"
}