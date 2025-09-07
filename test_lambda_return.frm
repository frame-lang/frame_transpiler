// Test returning lambdas from functions

fn make_adder(n) {
    var adder = lambda x: x + n
    return adder
}

fn main() {
    var add5 = make_adder(5)
    print("5 + 3 = " + str(add5(3)))
    
    var add10 = make_adder(10)
    print("10 + 3 = " + str(add10(3)))
}