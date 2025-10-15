# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test safe alternatives to eval for dictionary parsing

import json
import ast

fn test_json_parsing() {
    print("=== Safe JSON Parsing (Recommended) ===")
    
    # JSON is the safest way to parse string representations
    var json_str = "{\"name\": \"Alice\", \"age\": 30, \"active\": true}"
    
    var d = json.loads(json_str)
    print("Parsed from JSON: " + str(d))
    print("Name: " + d["name"])
    print("Age: " + str(d["age"]))
    
    # JSON with arrays
    var complex_json = "{\"users\": [{\"id\": 1, \"name\": \"Bob\"}, {\"id\": 2, \"name\": \"Carol\"}]}"
    var data = json.loads(complex_json)
    print("\nComplex JSON parsed: " + str(data))
    var users = data["users"]
    print("First user: " + str(users[0]))
}

fn test_ast_literal_eval() {
    print("\n=== Using ast.literal_eval (Safe for Python literals) ===")
    
    # ast.literal_eval safely evaluates Python literal structures
    # It only evaluates literals: strings, numbers, tuples, lists, dicts, booleans, None
    
    var dict_str = "{'name': 'David', 'scores': [85, 90, 95], 'active': True}"
    var d = ast.literal_eval(dict_str)
    
    print("Parsed with ast.literal_eval: " + str(d))
    print("Name: " + d["name"])
    print("Scores: " + str(d["scores"]))
    
    # Works with nested structures
    var nested_str = "{'users': {'alice': {'age': 30}, 'bob': {'age': 25}}}"
    var nested = ast.literal_eval(nested_str)
    print("\nNested structure: " + str(nested))
    
    # Works with mixed types
    var mixed_str = "{'int': 42, 'float': 3.14, 'list': [1, 2, 3], 'tuple': (4, 5, 6), 'bool': False}"
    var mixed = ast.literal_eval(mixed_str)
    print("Mixed types: " + str(mixed))
}

fn test_manual_parsing() {
    print("\n=== Manual String Processing (Most Control) ===")
    
    # For simple key-value pairs, manual parsing gives full control
    var input_str = "key1=value1,key2=value2,key3=value3"
    
    var d = {}
    var pairs = input_str.split(",")
    
    var i = 0
    while i < len(pairs) {
        var pair = pairs[i]
        var parts = pair.split("=")
        if len(parts) == 2 {
            var key = parts[0].strip()
            var value = parts[1].strip()
            d[key] = value
        }
        
        i = i + 1
    }
    
    print("Manually parsed: " + str(d))
    
    # Parse query string style
    var query = "name=Eve&age=28&city=Boston"
    var params = {}
    var items = query.split("&")
    
    i = 0
    while i < len(items) {
        var item = items[i]
        var kv = item.split("=")
        if len(kv) == 2 {
            params[kv[0]] = kv[1]
        }
        
        i = i + 1
    }
    
    print("Query params: " + str(params))
}

fn test_dangerous_eval() {
    print("\n=== Why eval() is Dangerous ===")
    
    # DO NOT USE eval() in production!
    # It can execute arbitrary code
    
    print("Example of what NOT to do:")
    print("  eval() can execute any Python code")
    print("  eval('__import__(\"os\").system(\"ls\")')")
    print("  This would execute system commands!")
    
    print("\nAlways use:")
    print("  1. json.loads() for JSON data")
    print("  2. ast.literal_eval() for Python literals")
    print("  3. Manual parsing for custom formats")
}

fn main() {
    print("Frame v0.38 - Safe Dictionary Parsing")
    print("=" * 50)
    
    test_json_parsing()
    test_ast_literal_eval()
    test_manual_parsing()
    test_dangerous_eval()
    
    print("\n" + "=" * 50)
    print("Summary:")
    print("  [OK] JSON parsing with json.loads()")
    print("  [OK] Safe literal evaluation with ast.literal_eval()")
    print("  [OK] Manual parsing for custom formats")
    print("  [WARNING] Never use eval() with untrusted input!")
}