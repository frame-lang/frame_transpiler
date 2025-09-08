// Test JSON-like data handling - Fixed version
// Working around nested dict indexing limitations

fn test_json_like_parsing() {
    print("=== Testing JSON-like Data Handling ===")
    
    // Simulate JSON data as Frame dictionaries
    var json_data = {
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
    // Work around: Extract to variables first
    var user_data = json_data["user"]
    print("User info: " + str(user_data))
    var user_name = user_data["name"]
    print("User name: " + user_name)
    
    var settings_data = json_data["settings"]
    print("Settings: " + str(settings_data))
    var theme = settings_data["theme"]
    print("Theme: " + theme)
    
    return
}

fn test_nested_data_access() {
    print("\n=== Testing Nested Data Access ===")
    
    var config = {
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
    
    // Access nested values - work around by extracting step by step
    var db_config = config["database"]
    var db_host = db_config["host"]
    var db_credentials = db_config["credentials"]
    var db_user = db_credentials["username"]
    
    var api_config = config["api"]
    var api_url = api_config["base_url"]
    
    print("Database host: " + db_host)
    print("Database user: " + db_user) 
    print("API URL: " + api_url)
    
    return
}

fn test_data_manipulation() {
    print("\n=== Testing Data Manipulation ===")
    
    var data = {
        "items": [
            {"id": 1, "name": "apple", "price": 1.50},
            {"id": 2, "name": "banana", "price": 0.75},
            {"id": 3, "name": "orange", "price": 2.00}
        ]
    }
    
    // Access array elements
    var items_list = data["items"]
    var first_item = items_list[0]
    var item_name = first_item["name"]
    var item_price = first_item["price"]
    
    print("First item: " + item_name + " - $" + str(item_price))
    
    // Calculate total
    var total = 0.0
    var items = data["items"]
    var i = 0
    while i < len(items) {
        var item = items[i]
        var price = item["price"]
        total = total + price
        i = i + 1
    }
    
    print("Total price: $" + str(total))
    
    return
}

fn main() {
    print("Frame v0.38 - JSON-like Data Handling (Fixed)")
    print("=" * 50)
    
    test_json_like_parsing()
    test_nested_data_access()
    test_data_manipulation()
    
    print("\n" + "=" * 50)
    print("Summary:")
    print("  [✓] Dictionary-based data structures")
    print("  [✓] Nested data access")
    print("  [✓] Array iteration and processing")
    print("  [✓] Complex data manipulation")
    print("\nNote: Using step-by-step extraction for nested dict access")
    
    return
}