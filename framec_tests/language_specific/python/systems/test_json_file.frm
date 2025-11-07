# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test JSON-like data handling (import and file I/O not yet supported)

fn test_json_like_parsing() {
    print("=== Testing JSON-like Data Handling ===")
    
    # Simulate JSON data as Frame dictionaries
    json_data = {
        "user": {
            "id": 123,
            "name": "Alice",
            "email": "alice@example.com"
        },
        "settings": {
            "theme": "dark",
            "language": "en",
            "notifications": True
        },
        "metadata": {
            "version": "1.0",
            "last_updated": "2025-01-22"
        }
    }
    
    print("Simulated JSON data:")
    print("User info: " + str(json_data["user"]))
    print("User name: " + json_data["user"]["name"])
    print("Settings: " + str(json_data["settings"]))
    print("Theme: " + json_data["settings"]["theme"])
    
    return
}

fn test_nested_data_access() {
    print("\n=== Testing Nested Data Access ===")
    
    config = {
        "database": {
            "host": "localhost",
            "port": 5432,
            "credentials": {
                "username": "admin",
                "password": "secret"
            }
        },
        "api": {
            "base_url": "https://api.example.com",
            "timeout": 30,
            "retries": 3
        }
    }
    
    # Access nested values
    db_host = config["database"]["host"]
    db_user = config["database"]["credentials"]["username"]
    api_url = config["api"]["base_url"]
    
    print("Database host: " + db_host)
    print("Database user: " + db_user) 
    print("API URL: " + api_url)
    
    return
}

fn test_data_manipulation() {
    print("\n=== Testing Data Manipulation ===")
    
    data = {
        "items": [
            {"id": 1, "name": "apple", "price": 1.50},
            {"id": 2, "name": "banana", "price": 0.75},
            {"id": 3, "name": "orange", "price": 2.00}
        ]
    }
    
    # Access array elements
    first_item = data["items"][0]
    item_name = first_item["name"]
    item_price = first_item["price"]
    
    print("First item: " + item_name + " - $" + str(item_price))
    
    # Calculate total
    total = 0.0
    items = data["items"]
    i = 0
    while i < len(items) {
        item = items[i]
        total = total + item["price"]
        i = i + 1
    }
    
    print("Total price: $" + str(total))
    
    return
}

fn main() {
    print("Frame v0.38 - JSON-like Data Handling")
    print("=" * 45)
    
    test_json_like_parsing()
    test_nested_data_access()
    test_data_manipulation()
    
    print("\n" + "=" * 45)
    print("Summary:")
    print("  [✓] Dictionary-based data structures")
    print("  [✓] Nested data access")
    print("  [✓] Array iteration and processing")
    print("  [✓] Complex data manipulation")
    print("\nFuture features:")
    print("  [○] JSON import/export")
    print("  [○] File I/O operations")  
    print("  [○] with statement support")
    
    return
}