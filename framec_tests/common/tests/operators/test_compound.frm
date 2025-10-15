# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn test() {
    var x = 5
    x = x + 3  # Compound assignment += not supported yet
    print("x = " + str(x))
    return
}

fn main() {
    test()
    return
}