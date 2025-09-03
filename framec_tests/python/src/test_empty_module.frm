// Test empty module declarations (v0.34)
// This tests that module syntax is recognized

from fsl import str

// An empty module is allowed
module utils {
}

// Functions at module level still work
fn main() {
    x = 42
    s = str(x)
    print(s)
}