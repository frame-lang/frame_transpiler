// Minimal test to debug call chain parsing

fn main() {
    var obj = TestSystem()
    obj.run()  // This should build a call chain: [obj] -> run()
}

system TestSystem {
    operations:
        run() {
            print("running")
            run_internal()  // Standalone operation call
        }
        
        run_internal() {
            print("internal")
        }
    
    machine:
        $Start {
        }
}