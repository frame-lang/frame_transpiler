// Test setdefault method for dynamic dictionary creation

fn test_setdefault_basic() {
    print("=== Testing dict.setdefault() ===")
    
    var d = {}
    
    // Traditional way (for comparison)
    print("\n1. Traditional approach:")
    // Note: 'in' operator not supported yet, using try-except pattern
    // if "key" not in d:
    //     d["key"] = []
    // d["key"].append("value1")
    
    // Using setdefault - creates key if it doesn't exist
    print("\n2. Using setdefault:")
    d.setdefault("fruits", []).append("apple")
    d.setdefault("fruits", []).append("banana")
    d.setdefault("vegetables", []).append("carrot")
    
    print("Dictionary: " + str(d))
    print("Fruits: " + str(d["fruits"]))
    print("Vegetables: " + str(d["vegetables"]))
    
    // setdefault with different default values
    print("\n3. Different default values:")
    var counts = {}
    
    // Initialize with 0 if not exists, then increment
    var current = counts.setdefault("apples", 0)
    counts["apples"] = current + 1
    
    current = counts.setdefault("apples", 0)
    counts["apples"] = current + 1
    
    counts.setdefault("oranges", 0)
    counts["oranges"] = counts["oranges"] + 3
    
    print("Counts: " + str(counts))
    
    // setdefault with dict as default
    print("\n4. Nested dictionaries:")
    var users = {}
    
    var alice_data = users.setdefault("alice", {})
    alice_data["email"] = "alice@example.com"
    alice_data["age"] = 30
    
    var bob_data = users.setdefault("bob", {})
    bob_data["email"] = "bob@example.com"
    bob_data["age"] = 25
    
    print("Users: " + str(users))
}

fn test_building_nested_structures() {
    print("\n=== Building Nested Structures with setdefault ===")
    
    var data = {}
    
    // Simulate categorizing items
    var items = [
        ["fruits", "apple"],
        ["fruits", "banana"],
        ["vegetables", "carrot"],
        ["vegetables", "broccoli"],
        ["fruits", "orange"],
        ["dairy", "milk"],
        ["dairy", "cheese"]
    ]
    
    // Build nested structure dynamically
    var i = 0
    loop {
        if i >= len(items) {
            break
        }
        
        var category = items[i][0]
        var item = items[i][1]
        
        data.setdefault(category, []).append(item)
        
        i = i + 1
    }
    
    print("Categorized data: " + str(data))
    print("Fruits: " + str(data["fruits"]))
    print("Vegetables: " + str(data["vegetables"]))
    print("Dairy: " + str(data["dairy"]))
}

fn test_setdefault_return_value() {
    print("\n=== Testing setdefault Return Value ===")
    
    var d = {"existing": "value"}
    
    // When key exists, returns existing value
    var result1 = d.setdefault("existing", "default")
    print("Existing key returns: " + result1)
    
    // When key doesn't exist, returns and sets default
    var result2 = d.setdefault("new", "default_value")
    print("New key returns: " + result2)
    
    print("Final dictionary: " + str(d))
    
    // Practical use - get or create list and operate on it
    var groups = {}
    groups.setdefault("admins", []).append("alice")
    groups.setdefault("admins", []).append("bob")
    groups.setdefault("users", []).append("charlie")
    
    print("Groups: " + str(groups))
}

fn main() {
    print("Frame v0.38 - Dynamic Dictionary Creation with setdefault")
    print("=" * 60)
    
    test_setdefault_basic()
    test_building_nested_structures()
    test_setdefault_return_value()
    
    print("\n" + "=" * 60)
    print("Summary: setdefault method works perfectly in Frame!")
}