// Test nested module declarations (v0.34)

from fsl import str

// Define a nested module for utilities
module utils {
    fn format_number(n) {
        return str(n)
    }
    
    // Nested module within utils
    module math {
        fn add(a, b) {
            return a + b
        }
        
        fn multiply(a, b) {
            return a * b
        }
    }
}

fn main() {
    // Test that we can use module functions
    // Note: qualified access not yet implemented
    print("Module test complete")
}