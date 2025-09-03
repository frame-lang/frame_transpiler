// Test FSL with user-defined function conflict - v0.34
// This should fail to compile because FSL str conflicts with user function

from fsl import str

// This should cause an error - can't redefine imported FSL function
fn str(x) {
    return "user_defined"
}

fn main() {
    print("This test should fail at compile time")
}