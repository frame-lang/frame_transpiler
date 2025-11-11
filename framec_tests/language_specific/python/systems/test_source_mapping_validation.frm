@target python

fn main() {
    # Test basic constructs to validate source mapping coverage
    
    # Variable assignments
    x = 42
    name = "test"
    
    # Collections
    list_val = [1, 2, 3]
    dict_val = {"key": "value"}
    set_val = {1, 2, 3}
    tuple_val = (1, 2)
    
    # Expressions
    result = x + 10
    negated = -x
    comparison = x > 10
    
    # Function calls
    print("Hello World")
    print(list_val)
    print(dict_val)
    print(set_val)
    print(tuple_val)
    
    # Conditional
    if x > 0:
        print("Positive")
}
