# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test module declarations (v0.34)
# This tests that module declarations parse correctly
# but doesn't use qualified names yet


# A simple module that contains functions
module utils {
    fn helper() {
        return 42
    }
}

# Main function at the module level
fn main() {
    # For now, we can't call utils.helper() as qualified names aren't implemented
    print("Module declaration parsed successfully")
}