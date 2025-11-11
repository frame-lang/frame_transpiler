@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Minimal test to debug call chain parsing

fn main() {
    obj = TestSystem()
    obj.run()  # This should build a call chain: [obj] -> run()
}

system TestSystem {
    operations:
        run() {
            print("running")
            run_internal()  # Standalone operation call
        }
        
        run_internal() {
            print("internal")
        }
    
    machine:
        $Start {
        }
}
