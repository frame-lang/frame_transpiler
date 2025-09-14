# Simple test for nested dict indexing issue

fn main() {
    var data = {}
    data["level1"] = {}
    
    # This should work but currently fails
    data["level1"]["level2"] = "value"
    
    # Accessing should also work
    var val = data["level1"]["level2"]
    print(val)
}