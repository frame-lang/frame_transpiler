# TypeScript logical operators — native bodies

fn test_and_operator() {
    var a = true
    var b = false
    var c = true
    if (a && c) {
        print("Both a and c are true")
    }
    if (a && b) {
        print("This should not print")
    } else {
        print("a and b is false")
    }
    var x = 5
    var y = 10
    if (x > 0 && y > 0 && x < y) {
        print("Complex and expression works")
    }
}

fn test_or_operator() {
    var a = true
    var b = false
    var c = false
    if (a || b) {
        print("At least one is true")
    }
    if (b || c) {
        print("This should not print")
    } else {
        print("Neither b nor c is true")
    }
    var x = -5
    var y = 10
    if (x < 0 || y < 0 || x === y) {
        print("Complex or expression works")
    }
}

fn test_not_operator() {
    var a = true
    var b = false
    if (!b) {
        print("b is not true")
    }
    if (!a) {
        print("This should not print")
    } else {
        print("a is true (not not a)")
    }
    var x = 5
    if (!(x > 10)) {
        print("x is not greater than 10")
    }
    if (!!a) {
        print("Double negation works")
    }
}

fn test_mixed_operators() {
    var a = true
    var b = false
    var c = true
    if ((a && c) || b) {
        print("(a and c) or b is true")
    }
    if (!(b || false)) {
        print("not (b or false) is true")
    }
    if (a && !b) {
        print("a and not b is true")
    }
    var x = 5
    var y = 10
    var z = 15
    if ((x < y && y < z) || !(x === 0)) {
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

