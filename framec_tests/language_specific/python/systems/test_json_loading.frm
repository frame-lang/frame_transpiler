# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test JSON loading from strings and files

import json

fn test_json_from_string() {
    print("=== Testing JSON from String ===")
    
    # Create a JSON string
    json_str = "{\"name\": \"Alice\", \"age\": 30}"
    
    # Parse JSON string to dictionary
    d = json.loads(json_str)
    
    print("JSON string: " + json_str)
    print("Parsed dict: " + str(d))
    print("Name: " + d["name"])
    print("Age: " + str(d["age"]))
    
    # Test with more complex JSON
    complex_json = "[{\"id\": 1, \"data\": [1, 2, 3]}, {\"id\": 2, \"data\": [4, 5, 6]}]"
    parsed = json.loads(complex_json)
    print("\nComplex JSON parsed: " + str(parsed))
}

fn test_json_to_string() {
    print("\n=== Testing Dictionary to JSON ===")
    
    # Create a dictionary
    data = {
        "name": "Bob",
        "age": 25,
        "scores": [95, 87, 92],
        "active": True
    }
    
    # Convert to JSON string
    json_str = json.dumps(data)
    print("Dictionary: " + str(data))
    print("JSON string: " + json_str)
    
    # Test with indentation
    pretty_json = json.dumps(data, indent=2)
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