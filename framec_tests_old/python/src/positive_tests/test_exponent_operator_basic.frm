# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test exponent operator ** in Frame v0.38

fn testBasicExponent() {
    print("=== Basic Exponent Tests ===")
    
    var base = 2
    var exp = 3
    var result = base ** exp
    print("2 ** 3 = " + str(result))  # Should be 8
    
    var square = 5 ** 2
    print("5 ** 2 = " + str(square))  # Should be 25
    
    var cube = 3 ** 3
    print("3 ** 3 = " + str(cube))  # Should be 27
    
    var decimal_power = 2.5 ** 2
    print("2.5 ** 2 = " + str(decimal_power))  # Should be 6.25
    
    var zero_power = 10 ** 0
    print("10 ** 0 = " + str(zero_power))  # Should be 1
    
    var neg_base = (-2) ** 3
    print("(-2) ** 3 = " + str(neg_base))  # Should be -8
    
    return
}

fn testRightAssociativity() {
    print("\n=== Right Associativity Test ===")
    
    # Should be 2 ** (3 ** 2) = 2 ** 9 = 512
    var result = 2 ** 3 ** 2
    print("2 ** 3 ** 2 = " + str(result))  # Should be 512
    
    # Explicit parentheses for clarity
    var explicit = 2 ** (3 ** 2)
    print("2 ** (3 ** 2) = " + str(explicit))  # Should be 512
    
    # Different grouping
    var left_group = (2 ** 3) ** 2
    print("(2 ** 3) ** 2 = " + str(left_group))  # Should be 64
    
    return
}

fn testPrecedence() {
    print("\n=== Precedence Tests ===")
    
    # Power has higher precedence than multiplication
    var result1 = 2 * 3 ** 2
    print("2 * 3 ** 2 = " + str(result1))  # Should be 18 (2 * 9)
    
    var result2 = 3 ** 2 * 2
    print("3 ** 2 * 2 = " + str(result2))  # Should be 18 (9 * 2)
    
    # Power has higher precedence than unary minus
    var result3 = -2 ** 2
    print("-2 ** 2 = " + str(result3))  # Should be -4 (-(2 ** 2))
    
    # Addition and power
    var result4 = 1 + 2 ** 3
    print("1 + 2 ** 3 = " + str(result4))  # Should be 9 (1 + 8)
    
    # Division and power
    var result5 = 16 / 2 ** 2
    print("16 / 2 ** 2 = " + str(result5))  # Should be 4 (16 / 4)
    
    return
}

fn testInExpressions() {
    print("\n=== Complex Expression Tests ===")
    
    var x = 3
    var y = 2
    var z = 4
    
    var expr1 = x ** y + z
    print("3 ** 2 + 4 = " + str(expr1))  # Should be 13
    
    var expr2 = x + y ** z
    print("3 + 2 ** 4 = " + str(expr2))  # Should be 19
    
    var expr3 = (x + y) ** 2
    print("(3 + 2) ** 2 = " + str(expr3))  # Should be 25
    
    var expr4 = x ** y ** 2
    print("3 ** 2 ** 2 = " + str(expr4))  # Should be 81 (3 ** 4)
    
    return
}

fn main() {
    print("=== Frame v0.38 Exponent Operator Test ===\n")
    
    testBasicExponent()
    testRightAssociativity()
    testPrecedence()
    testInExpressions()
    
    print("\n=== All Tests Complete ===")
    return
}

# Run tests