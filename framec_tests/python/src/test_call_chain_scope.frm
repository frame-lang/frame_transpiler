// Test specifically call chain scope issue

fn main() {
    var simple_var = "test"
    print(simple_var)  // Should work correctly
    
    var obj_var = TestSystem()  
    obj_var.some_operation()  // This should generate: obj_var.some_operation(), NOT: self.some_operation()
}

system TestSystem {
    operations:
        some_operation() {
            print("Operation called")
        }
    
    machine:
        $Start {
        }
}