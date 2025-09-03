// Test simple module qualified names (v0.34)
// This is a simplified test to check basic module.function() syntax

from fsl import str

// Simple module with a function
module utils {
    fn helper() {
        print("Helper called from utils module")
        return 42
    }
}

fn main() {
    // Direct call (works now)
    print("Testing module qualified names")
    
    // Qualified call (to be implemented)
    // var result = utils.helper()
    // print(str(result))
}