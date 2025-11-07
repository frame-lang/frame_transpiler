# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test all working lambda features in Frame v0.38

fn main() {
    print("=== Lambda Features Test ===\n")
    
    # 1. Basic lambda assignment
    square = lambda x: x * x
    print("1. Basic lambda: square(5) = " + str(square(5)))
    
    # 2. Multi-parameter lambda
    add = lambda a, b: a + b
    print("2. Multi-param: add(3, 4) = " + str(add(3, 4)))
    
    # 3. Lambda in dictionary literal - WORKS!
    ops = {
        "multiply": lambda x, y: x * y,
        "divide": lambda x, y: x / y
    }
    print("3. Dict lambda: multiply(6, 7) = " + str(ops["multiply"](6, 7)))
    
    # 4. Lambda in list literal - WORKS!
    transforms = [
        lambda n: n + 10,
        lambda n: n * 10,
        lambda n: n - 10
    ]
    print("4. List lambda: transform[0](5) = " + str(transforms[0](5)))
    
    # 5. Lambda with closure
    factor = 100
    scale = lambda x: x * factor
    print("5. Closure: scale(3) with factor=100 = " + str(scale(3)))
    
    # 6. Lambda as function return
    make_adder = lambda n: lambda x: x + n
    add5 = make_adder(5)
    print("6. Returned lambda: add5(7) = " + str(add5(7)))
    
    # 7. Mixed collection with lambdas
    mixed = {
        "increment": lambda x: x + 1,
        "operations": [lambda x: x * 2, lambda x: x / 2]
    }
    print("7. Mixed collection: increment(9) = " + str(mixed["increment"](9)))
    
    print("\n=== All Lambda Features Working! ===")
}