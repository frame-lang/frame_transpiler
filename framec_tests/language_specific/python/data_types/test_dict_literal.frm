@target python

# Python-specific: domain native assignment

fn test_dict_literals() {
    empty = {}
    print("Empty dict: " + str(empty))
    person = {"name": "Alice", "age": "30", "city": "NYC"}
    print("Person dict: " + str(person))
    numbers = {1: "one", 2: "two", 3: "three"}
    print("Numbers dict: " + str(numbers))
    mixed = {"key1": 100, "key2": 200, 42: "answer"}
    print("Mixed dict: " + str(mixed))
    nested = {
        "user": {"id": 1, "name": "Bob"},
        "settings": {"theme": "dark", "lang": "en"}
    }
    print("Nested dict: " + str(nested))
    config = {"debug": true, "port": 8080}
    print("Config: " + str(config))
    process_dict({"x": 10, "y": 20})
    x = 100
    y = 200
    coords = {"x": x, "y": y}
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
        settings = {"mode": "test", "count": 0}
}

fn main() {
    print("Testing dictionary literals...")
    test_dict_literals()
    print("\nTesting dictionary in system...")
    sys = DictSystem()
    sys.configure({"mode": "production", "count": 10})
    result = sys.getSettings()
    print("Retrieved settings: " + str(result))
    print("\nAll dictionary tests completed!")
    return
}

