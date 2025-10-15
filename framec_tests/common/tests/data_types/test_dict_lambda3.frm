# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test the exact pattern from test_lambda.frm

fn test() {
    print("Before dict")
    
    var operations = {
        "add": lambda a, b: a + b,
        "subtract": lambda a, b: a - b,
        "multiply": lambda a, b: a * b,
        "divide": lambda a, b: a / b
    }
    
    print("After dict")
}

fn main() {
    test()
}