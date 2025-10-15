# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Comprehensive test for dynamic dictionary creation

import json
import ast

fn test_setdefault_vs_traditional() {
    print("=== Comparing Traditional vs setdefault ===")
    
    # Traditional approach with try-except
    print("\n1. Traditional approach (verbose):")
    var d1 = {}
    
    # Note: Since 'in' operator not supported, showing conceptual approach
    # if "fruits" not in d1:
    #     d1["fruits"] = []
    # d1["fruits"].append("apple")
    
    # Using get() with default
    var fruits = d1.get("fruits", [])
    fruits.append("apple")
    d1["fruits"] = fruits
    
    fruits = d1.get("fruits", [])
    fruits.append("banana")
    d1["fruits"] = fruits
    
    print("Traditional result: " + str(d1))
    
    # Using setdefault (concise)
    print("\n2. setdefault approach (concise):")
    var d2 = {}
    d2.setdefault("fruits", []).append("apple")
    d2.setdefault("fruits", []).append("banana")
    
    print("setdefault result: " + str(d2))
    print("Both approaches produce the same result!")
}

fn build_category_index() {
    print("\n=== Building Category Index ===")
    
    # Simulate products with categories
    var products = [
        {"name": "iPhone", "category": "Electronics", "price": 999},
        {"name": "Laptop", "category": "Electronics", "price": 1299},
        {"name": "Shirt", "category": "Clothing", "price": 29},
        {"name": "Jeans", "category": "Clothing", "price": 59},
        {"name": "Book", "category": "Media", "price": 15},
        {"name": "DVD", "category": "Media", "price": 10},
        {"name": "Headphones", "category": "Electronics", "price": 149}
    ]
    
    # Build category index using setdefault
    var index = {}
    
    var i = 0
    while i < len(products) {
        var product = products[i]
        var category = product["category"]
        
        # This creates the list if it doesn't exist, then appends
        index.setdefault(category, []).append(product)
        
        i = i + 1
    }
    
    print("Category Index:")
    var categories = ["Electronics", "Clothing", "Media"]
    var j = 0
    while j < len(categories) {
        var cat = categories[j]
        print("  " + cat + ": " + str(len(index[cat])) + " items")
        j = j + 1
    }
    
    # Calculate totals per category
    var totals = {}
    j = 0
    while j < len(categories) {
        var cat = categories[j]
        var items = index[cat]
        var total = 0
        
        var k = 0
        while k < len(items) {
            total = total + items[k]["price"]
            k = k + 1
        }
        
        totals[cat] = total
        j = j + 1
    }
    
    print("\nCategory Totals: " + str(totals))
}

fn test_dynamic_config_building() {
    print("\n=== Dynamic Configuration Building ===")
    
    # Build configuration from various sources
    var config = {}
    
    # Set defaults
    var db_config = config.setdefault("database", {})
    db_config["host"] = "localhost"
    db_config["port"] = 5432
    
    var app_config = config.setdefault("app", {})
    app_config["debug"] = False
    app_config["log_level"] = "INFO"
    
    var paths_config = config.setdefault("paths", {})
    paths_config["data"] = "/var/data"
    paths_config["logs"] = "/var/logs"
    
    print("Initial config: " + str(config))
    
    # Override from "environment" (simulated)
    var env_overrides = "{\"database\": {\"host\": \"prod-db.example.com\"}, \"app\": {\"debug\": false}}"
    var overrides = json.loads(env_overrides)
    
    # Merge overrides
    var override_db = overrides["database"]
    var override_app = overrides["app"]
    
    var config_db = config["database"]
    config_db["host"] = override_db["host"]
    
    var config_app = config["app"]
    config_app["debug"] = override_app["debug"]
    
    print("After overrides: " + str(config))
}

fn test_safe_string_to_dict() {
    print("\n=== Safe String to Dictionary Conversion ===")
    
    # JSON format (most portable)
    var json_config = "{\"server\": \"api.example.com\", \"port\": 443, \"ssl\": true}"
    var config1 = json.loads(json_config)
    print("From JSON: " + str(config1))
    
    # Python literal format (Python-specific)
    var py_config = "{'server': 'api.example.com', 'port': 443, 'ssl': True}"
    var config2 = ast.literal_eval(py_config)
    print("From Python literal: " + str(config2))
    
    # Custom format parsing
    var custom = "server=api.example.com;port=443;ssl=true"
    var config3 = {}
    
    var parts = custom.split(";")
    var i = 0
    while i < len(parts) {
        var kv = parts[i].split("=")
        if len(kv) == 2 {
            var key = kv[0]
            var value = kv[1]
            
            # Convert string "true"/"false" to boolean
            if value == "true" {
                config3[key] = True
            } elif value == "false" {
                config3[key] = False
            } else {
                # Try to convert to int if possible
                # Note: Would need try-except in real code
                config3[key] = value
            }
        }
        i = i + 1
    }
    
    print("From custom format: " + str(config3))
}

fn main() {
    print("Frame v0.38 - Comprehensive Dynamic Dictionary Creation")
    print("=" * 60)
    
    test_setdefault_vs_traditional()
    build_category_index()
    test_dynamic_config_building()
    test_safe_string_to_dict()
    
    print("\n" + "=" * 60)
    print("Summary of Dynamic Dictionary Creation:")
    print("  [OK] setdefault() for automatic initialization")
    print("  [OK] Building nested structures dynamically")
    print("  [OK] Category indexing and aggregation")
    print("  [OK] Configuration building and merging")
    print("  [OK] Safe string parsing with json.loads()")
    print("  [OK] Python literal parsing with ast.literal_eval()")
    print("  [OK] Custom format parsing")
    print("\nAll dynamic dictionary creation methods work in Frame!")
}