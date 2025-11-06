# TypeScript logical operators — native bodies

fn test_and_operator() {
    var a = true
    var b = false
    var c = true
    if (a and c) {
        print("Both a and c are true")
    }
    if (a and b) {
        print("This should not print")
    } else {
        print("a and b is false")
    }
    var x = 5
    var y = 10
    if (x > 0 and y > 0 and x < y) {
        print("Complex and expression works")
    }
}

fn test_or_operator() {
    var a = true
    var b = false
    var c = false
    if (a or b) {
        print("At least one is true")
    }
    if (b or c) {
        print("This should not print")
    } else {
        print("Neither b nor c is true")
    }
    var x = -5
    var y = 10
    if (x < 0 or y < 0 or x == y) {
        print("Complex or expression works")
    }
}

fn test_not_operator() {
    var a = true
    var b = false
    if (not b) {
        print("b is not true")
    }
    if (not a) {
        print("This should not print")
    } else {
        print("a is true (not not a)")
    }
    var x = 5
    if (not (x > 10)) {
        print("x is not greater than 10")
    }
    if (not not a) {
        print("Double negation works")
    }
}

fn test_mixed_operators() {
    var a = true
    var b = false
    var c = true
    if ((a and c) or b) {
        print("(a and c) or b is true")
    }
    if (not (b or false)) {
        print("not (b or false) is true")
    }
    if (a and not b) {
        print("a and not b is true")
    }
    var x = 5
    var y = 10
    var z = 15
    if ((x < y and y < z) or not (x == 0)) {
        print("Complex mixed expression works")
    }
}

fn main() {
    print("=== Testing TypeScript Logical Operators ===")
    print("\n1. Testing '&&' operator:")
    test_and_operator()
    print("\n2. Testing '||' operator:")
    test_or_operator()
    print("\n3. Testing '!' operator:")
    test_not_operator()
    print("\n4. Testing mixed operators:")
    test_mixed_operators()
    print("\n=== All Logical Operator Tests Complete ===")
}
