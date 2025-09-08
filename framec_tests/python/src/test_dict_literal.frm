// test_dict_literal.frm
// Test dictionary literal support in Frame v0.38


fn test_dict_literals() {
    // Empty dictionary
    var empty = {}
    print("Empty dict: " + str(empty))
    
    // Dictionary with string keys
    var person = {"name": "Alice", "age": "30", "city": "NYC"}
    print("Person dict: " + str(person))
    
    // Dictionary with number keys 
    var numbers = {1: "one", 2: "two", 3: "three"}
    print("Numbers dict: " + str(numbers))
    
    // Mixed key types
    var mixed = {"key1": 100, "key2": 200, 42: "answer"}
    print("Mixed dict: " + str(mixed))
    
    // Nested dictionaries
    var nested = {
        "user": {"id": 1, "name": "Bob"},
        "settings": {"theme": "dark", "lang": "en"}
    }
    print("Nested dict: " + str(nested))
    
    // Dictionary in variable assignment
    var config = {"debug": true, "port": 8080}
    print("Config: " + str(config))
    
    // Dictionary as function argument
    process_dict({"x": 10, "y": 20})
    
    // Dictionary with variable values
    var x = 100
    var y = 200
    var coords = {"x": x, "y": y}
    print("Coords: " + str(coords))
    
    return
}

fn process_dict(data) {
    print("Processing dict: " + str(data))
    return
}

// Test in system context
system DictSystem {
    interface:
        configure(config): string
        getSettings(): string
    
    machine:
        $Ready {
            configure(config) {
                // Update settings with config dict
                settings = config
                print("Settings updated: " + str(settings))
                system.return = "configured"
            }
            
            getSettings() {
                print("Current settings: " + str(settings))
                system.return = str(settings)
            }
        }
    
    domain:
        var settings = {"mode": "test", "count": 0}
}

fn main() {
    print("Testing dictionary literals...")
    test_dict_literals()
    
    print("\nTesting dictionary in system...")
    var sys = DictSystem()
    sys.configure({"mode": "production", "count": 10})
    var result = sys.getSettings()
    print("Retrieved settings: " + str(result))
    
    print("\nAll dictionary tests completed!")
    return
}