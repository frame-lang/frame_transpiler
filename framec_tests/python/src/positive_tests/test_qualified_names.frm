# Test qualified name access (v0.34)
# This tests module.function() syntax (currently not working)


module utils {
    fn helper() {
        return 42
    }
    
    module math {
        fn add(a, b) {
            return a + b
        }
    }
}

fn main() {
    # These qualified names should work but don't yet:
    # var result = utils.helper()
    # var sum = utils.math.add(3, 4)
    
    # For now, just test that the module parses
    print("Qualified names test - parsing only")
}