# Basic test without power operator

fn test() {
    var squares = {x: x * x for x in range(5)}
    print(squares)
}

fn main() {
    test()
}