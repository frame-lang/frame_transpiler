# Python-specific: domain native assignment
fn main() {
    var sys = TestSystem()
    sys.test()
}

system TestSystem {
    interface:
        test()
    machine:
        $Start { test() { print(self.domain_var) } }
    domain:
        domain_var:string = "test"
}

