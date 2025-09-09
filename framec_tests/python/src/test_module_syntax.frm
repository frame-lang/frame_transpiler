# Test module syntax parsing (v0.34)
# This tests that the module keyword and syntax are recognized

# An empty module
module utils {
}

# A module with a variable (not yet supported in codegen)
module config {
    var setting = 42
}

fn main() {
    print("Module syntax test")
}