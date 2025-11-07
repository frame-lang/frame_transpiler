# TypeScript-specific copy

fn test_dict_literals() {
    var empty = {}
    print("Empty dict: " + str(empty))
    var person = {"name": "Alice", "age": "30", "city": "NYC"}
    print("Person dict: " + str(person))
    var numbers = {1: "one", 2: "two", 3: "three"}
    print("Numbers dict: " + str(numbers))
    var mixed = {"key1": 100, "key2": 200, 42: "answer"}
    print("Mixed dict: " + str(mixed))
    var nested = {
        "user": {"id": 1, "name": "Bob"},
        "settings": {"theme": "dark", "lang": "en"}
    }
    print("Nested dict: " + str(nested))
    var config = {"debug": true, "port": 8080}
    print("Config: " + str(config))
    process_dict({"x": 10, "y": 20})
    var x = 100
    var y = 200
    var coords = {"x": x, "y": y}
    print("Coords: " + str(coords))
    return
}

fn process_dict(data) { print("Processing dict: " + str(data)); return }

system DictSystem {
    interface:
        configure(config): string
        getSettings(): string
    machine:
        $Ready {
            configure(config) {
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

