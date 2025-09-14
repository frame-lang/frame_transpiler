# Test that FSL operations require explicit import in v0.34

# This should work - FSL is explicitly imported

fn test_with_import() {
    var x = 42
    var s = str(x)       # Should work
    var i = int("123")   # Should work
    var f = float("3.14") # Should work
    print(s)
    print(i)
    print(f)
}

fn main() {
    test_with_import()
}