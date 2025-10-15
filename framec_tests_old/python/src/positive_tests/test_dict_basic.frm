# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Basic dictionary test

fn test() {
    var squares = {x: x * x for x in range(5)}
    print(squares)
    return
}

fn main() {
    test()
    return
}