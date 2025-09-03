// Test that FSL operations fail without import in v0.34

// No FSL import - operations should NOT be recognized as FSL

fn test_without_import() {
    var x = 42
    // Without import, str() should not be recognized as FSL
    // The parser should treat it as an undefined function
    var s = str(x)  // Should fail or be undefined
    print(s)
}

fn main() {
    test_without_import()
}