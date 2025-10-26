# Negative test: nested function definitions are not supported in Frame

fn outer() {
    print("Outer start")
    
    fn inner() {
        print("Inner should not be allowed")
    }
    
    print("Outer end")
    return
}

fn main() {
    outer()
    return
}
