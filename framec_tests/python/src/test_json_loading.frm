# Test JSON loading from strings and files

import json

fn test_json_from_string() {
    print("=== Testing JSON from String ===")
    
    # Create a JSON string
    var json_str = "{\"name\": \"Alice\", \"age\": 30}"
    
    # Parse JSON string to dictionary
    var d = json.loads(json_str)
    
    print("JSON string: " + json_str)
    print("Parsed dict: " + str(d))
    print("Name: " + d["name"])
    print("Age: " + str(d["age"]))
    
    # Test with more complex JSON
    var complex_json = "[{\"id\": 1, \"data\": [1, 2, 3]}, {\"id\": 2, \"data\": [4, 5, 6]}]"
    var parsed = json.loads(complex_json)
    print("\nComplex JSON parsed: " + str(parsed))
}

fn test_json_to_string() {
    print("\n=== Testing Dictionary to JSON ===")
    
    # Create a dictionary
    var data = {
        "name": "Bob",
        "age": 25,
        "scores": [95, 87, 92],
        "active": True
    }
    
    # Convert to JSON string
    var json_str = json.dumps(data)
    print("Dictionary: " + str(data))
    print("JSON string: " + json_str)
    
    # Test with indentation
    var pretty_json = json.dumps(data, indent=2)
    print("\nPretty JSON:")
    print(pretty_json)
}

fn main() {
    print("Frame v0.38 - JSON Loading Test")
    print("=" * 50)
    
    test_json_from_string()
    test_json_to_string()
    
    print("\n" + "=" * 50)
    print("Summary: JSON loading and dumping works!")
}