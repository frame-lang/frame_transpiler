// Test case for operations call transpiler bug
// sys3.run() should generate sys3.run(), not sys3.self.run()

fn main() {
    var sys = TestSystem()
    sys.run()  // This should generate sys.run(), not sys.self.run()
}

system TestSystem {
    operations:
        run() {
            print("Operation called correctly")
        }
        
    machine:
        $Start {
        }
}