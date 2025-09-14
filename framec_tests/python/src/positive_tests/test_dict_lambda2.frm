# Test multiple lambdas in dictionary

fn test() {
    var ops = {
        "add": lambda a, b: a + b,
        "sub": lambda x, y: x - y
    }
    print("Test")
}

fn main() {
    test()
}