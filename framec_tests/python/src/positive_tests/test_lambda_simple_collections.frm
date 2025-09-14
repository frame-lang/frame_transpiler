# Simple test for lambdas in collections

fn main() {
    print("=== Testing Lambdas in Collections ===")
    
    # Lambda in dictionary
    var ops = {"add": lambda a, b: a + b}
    print("Dict with lambda created")
    print("Add result: " + str(ops["add"](5, 3)))
    
    # Lambda in list
    var funcs = [lambda x: x * 2]
    print("List with lambda created")
    print("Double result: " + str(funcs[0](7)))
    
    print("=== All Tests Passed ===")
}