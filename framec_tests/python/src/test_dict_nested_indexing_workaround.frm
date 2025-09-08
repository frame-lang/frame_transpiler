// Test workaround for nested dict indexing
// Since parser doesn't support dict["key1"]["key2"] directly,
// we use intermediate variables as a workaround

fn test_nested_dict_access() {
    print("=== Testing Nested Dict Access (Workaround) ===")
    
    // Create a nested dictionary structure
    var data = {
        "users": {
            "alice": {"age": 30, "city": "Paris"},
            "bob": {"age": 25, "city": "London"}
        },
        "settings": {
            "theme": "dark",
            "language": "en"
        }
    }
    
    // Workaround: Use intermediate variables for nested access
    var users = data["users"]
    var alice = users["alice"]
    var alice_age = alice["age"]
    print("Alice's age: " + str(alice_age))
    
    var bob = users["bob"]
    var bob_city = bob["city"]
    print("Bob's city: " + bob_city)
    
    var settings = data["settings"]
    var theme = settings["theme"]
    print("Theme: " + theme)
    
    // Using .get() with chaining works
    var alice_city = data.get("users", {}).get("alice", {}).get("city", "Unknown")
    print("Alice's city (via get): " + alice_city)
}

fn test_nested_assignment() {
    print("\n=== Testing Nested Dict Assignment (Workaround) ===")
    
    // Build nested structure step by step
    var tree = {}
    tree["level1"] = {}
    
    var level1 = tree["level1"]
    level1["level2"] = {}
    
    var level2 = level1["level2"]
    level2["value"] = "deep value"
    
    // Verify it worked
    var check_level1 = tree["level1"]
    var check_level2 = check_level1["level2"]
    var check_value = check_level2["value"]
    print("Deep value: " + check_value)
    
    // Also works with get()
    var deep = tree.get("level1", {}).get("level2", {}).get("value", "not found")
    print("Deep value (via get): " + deep)
}

fn main() {
    print("Frame v0.38 - Nested Dict Indexing Workaround")
    print("=" * 50)
    
    test_nested_dict_access()
    test_nested_assignment()
    
    print("\n" + "=" * 50)
    print("Workaround successful!")
    print("Note: Use intermediate variables for nested dict access")
    print("      until parser supports dict['key1']['key2'] syntax")
}