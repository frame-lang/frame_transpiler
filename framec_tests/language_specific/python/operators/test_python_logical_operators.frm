@target python

# Python logical operators — native bodies

fn test_and_operator() {
    a = True
    b = False
    c = True
    if a and c:
        print("Both a and c are true")
    if a and b:
        print("This should not print")
    else:
        print("a and b is false")
    x = 5
    y = 10
    if x > 0 and y > 0 and x < y:
        print("Complex and expression works")
}

fn test_or_operator() {
    a = True
    b = False
    c = False
    if a or b:
        print("At least one is true")
    if b or c:
        print("This should not print")
    else:
        print("Neither b nor c is true")
    x = -5
    y = 10
    if x < 0 or y < 0 or x == y:
        print("Complex or expression works")
}

fn test_not_operator() {
    a = True
    b = False
    if not b:
        print("b is not true")
    if not a:
        print("This should not print")
    else:
        print("a is true (not not a)")
    x = 5
    if not (x > 10):
        print("x is not greater than 10")
    if not not a:
        print("Double negation works")
}

fn test_mixed_operators() {
    a = True
    b = False
    c = True
    if (a and c) or b:
        print("(a and c) or b is true")
    if not (b or False):
        print("not (b or false) is true")
    if a and not b:
        print("a and not b is true")
    x = 5
    y = 10
    z = 15
    if (x < y and y < z) or not (x == 0):
        print("Complex mixed expression works")
}

fn main() {
    print("=== Testing Python Logical Operators ===")
    print("\n1. Testing 'and' operator:")
    test_and_operator()
    print("\n2. Testing 'or' operator:")
    test_or_operator()
    print("\n3. Testing 'not' operator:")
    test_not_operator()
    print("\n4. Testing mixed operators:")
    test_mixed_operators()
    print("\n=== All Logical Operator Tests Complete ===")
}

