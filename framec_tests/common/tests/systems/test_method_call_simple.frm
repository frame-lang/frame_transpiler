# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Very simple method call test

fn main() {
    var sys = TestSystem()
    # This should generate: sys.run(), NOT: sys.self.run()  
    sys.run()
}

system TestSystem {
    operations:
        run() {
            print("running")
        }
    
    machine:
        $Start {
        }
}