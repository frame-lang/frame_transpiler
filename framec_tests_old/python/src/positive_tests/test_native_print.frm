# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test using native Python print without Frame built-in
# No import needed since print is a Python built-in

fn main() {
    print("Testing native Python print")
    print("No Frame built-in needed!")
    
    # Test with variables
    var message = "Hello from Frame"
    print(message)
    
    # Test with expressions
    var x = 5
    var y = 10
    print("x + y = " + str(x + y))
}

system TestSystem {
    interface:
        test()
    
    machine:
        $Start {
            test() {
                print("Print works in systems too!")
                return
            }
        }
}