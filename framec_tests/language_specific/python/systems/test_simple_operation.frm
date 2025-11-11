@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    sys = TestSystem()
    return
}

system TestSystem {
    operations:
        run() {
            print("running")
            self.run_internal()
        }
        
        run_internal() {
            print("internal")
        }
    
    machine:
        $Start {
        }
}
