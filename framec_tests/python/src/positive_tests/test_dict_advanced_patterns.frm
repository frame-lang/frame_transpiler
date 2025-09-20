# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test advanced dictionary patterns
# Note: Frame doesn't support imports yet, using native dict patterns

fn test_dict_based_dispatch() {
    print("=== Dictionary-Based Dispatch Pattern ===")
    
    # Use dictionary to map values to results
    var responses = {
        "yes": "Processing your request...",
        "no": "Request cancelled.",
        "maybe": "Please provide more information.",
        "help": "Available options: yes, no, maybe"
    }
    
    var user_inputs = ["yes", "no", "maybe", "help", "unknown"]
    
    print("\nSimple value dispatch:")
    var i = 0
    while i < len(user_inputs) {
        var input = user_inputs[i]
        var response = responses.get(input, "Unknown option. Type 'help' for options.")
        print("Input: '" + input + "' -> " + response)
        i = i + 1
    }
}

fn test_state_machine_with_dict() {
    print("\n=== State Machine with Dictionary ===")
    
    # Simple state machine using dictionaries
    var state_transitions = {
        "idle": {"start": "running", "reset": "idle"},
        "running": {"pause": "paused", "stop": "stopped", "error": "error"},
        "paused": {"resume": "running", "stop": "stopped"},
        "stopped": {"start": "running", "reset": "idle"},
        "error": {"reset": "idle"}
    }
    
    var current_state = "idle"
    var commands = ["start", "pause", "resume", "stop", "reset"]
    
    print("Initial state: " + current_state)
    
    var i = 0
    while i < len(commands) {
        var command = commands[i]
        var state_dict = state_transitions.get(current_state, {})
        var next_state = state_dict.get(command, current_state)
        
        if next_state != current_state {
            print("Command: " + command + " -> State changed from " + current_state + " to " + next_state)
            current_state = next_state
        } else {
            print("Command: " + command + " -> Invalid in state " + current_state)
        }
        
        i = i + 1
    }
}

fn test_recursive_dict_pattern() {
    print("\n=== Recursive Dict Pattern ===")
    
    # Note: Frame doesn't support lambda or nested function definitions,
    # so we demonstrate nested dict usage with regular dictionaries
    
    print("\nBuilding nested structure with regular dicts:")
    
    # Create a structure for user preferences using regular dicts
    var user_prefs = {}
    
    # Add nested preferences
    user_prefs["alice"] = {
        "theme": "dark",
        "language": "en",
        "notifications": true
    }
    
    user_prefs["bob"] = {
        "theme": "light",
        "language": "es",
        "notifications": false
    }
    
    print("User preferences: " + str(user_prefs))
    
    # Multi-level nesting with manual approach
    var tree = {}
    
    # Build a tree structure manually
    tree["users"] = {}
    tree["users"]["alice"] = {}
    tree["users"]["alice"]["settings"] = {}
    tree["users"]["alice"]["settings"]["theme"] = "dark"
    tree["users"]["alice"]["settings"]["privacy"] = {}
    tree["users"]["alice"]["settings"]["privacy"]["profile"] = "public"
    tree["users"]["alice"]["settings"]["privacy"]["email"] = "hidden"
    
    tree["users"]["bob"] = {}
    tree["users"]["bob"]["settings"] = {}
    tree["users"]["bob"]["settings"]["theme"] = "light"
    
    print("\nManually built tree structure:")
    print(str(tree))
    
    # Access nested values safely
    var alice_theme = tree.get("users", {}).get("alice", {}).get("settings", {}).get("theme", "default")
    print("Alice's theme: " + alice_theme)
}

fn test_config_with_defaults() {
    print("\n=== Configuration with Nested Defaults ===")
    
    # Build configuration with multiple levels of defaults
    var default_config = {
        "server": {
            "host": "localhost",
            "port": 8080,
            "ssl": False
        },
        "database": {
            "host": "localhost",
            "port": 5432,
            "name": "myapp"
        },
        "logging": {
            "level": "INFO",
            "file": "/var/log/app.log"
        }
    }
    
    # User overrides
    var user_config = {
        "server": {
            "host": "api.example.com",
            "ssl": True
        },
        "logging": {
            "level": "DEBUG"
        }
    }
    
    # Merge configs (simple approach)
    var final_config = {}
    
    # Copy defaults
    var sections = ["server", "database", "logging"]
    var i = 0
    while i < len(sections) {
        var section = sections[i]
        final_config[section] = {}
        
        # Copy default values
        var default_section = default_config.get(section, {})
        var keys = ["host", "port", "ssl", "name", "level", "file"]
        var j = 0
        while j < len(keys) {
            var key = keys[j]
            if key in ["host", "port", "ssl"] and section == "server" {
                var user_section = user_config.get("server", {})
                final_config[section][key] = user_section.get(key, default_section.get(key))
            } elif key in ["host", "port", "name"] and section == "database" {
                final_config[section][key] = default_section.get(key)
            } elif key in ["level", "file"] and section == "logging" {
                var user_section = user_config.get("logging", {})
                final_config[section][key] = user_section.get(key, default_section.get(key))
            }
            j = j + 1
        }
        
        i = i + 1
    }
    
    print("Final configuration after merge:")
    print(str(final_config))
}

fn test_counting_pattern() {
    print("\n=== Counting Pattern with Regular Dicts ===")
    
    var words = ["apple", "banana", "apple", "cherry", "banana", "apple", "date"]
    var word_count = {}
    
    var i = 0
    while i < len(words) {
        var word = words[i]
        var current = word_count.get(word, 0)
        word_count[word] = current + 1
        i = i + 1
    }
    
    print("Word counts: " + str(word_count))
    
    # Grouping pattern
    var items = [
        {"name": "apple", "type": "fruit"},
        {"name": "carrot", "type": "vegetable"},
        {"name": "banana", "type": "fruit"},
        {"name": "broccoli", "type": "vegetable"},
        {"name": "orange", "type": "fruit"}
    ]
    
    var grouped = {}
    
    i = 0
    while i < len(items) {
        var item = items[i]
        var item_type = item["type"]
        if not (item_type in grouped) {
            grouped[item_type] = []
        }
        grouped[item_type].append(item["name"])
        i = i + 1
    }
    
    print("Grouped items: " + str(grouped))
}

fn main() {
    print("Frame v0.38 - Advanced Dictionary Patterns")
    print("=" * 60)
    
    test_dict_based_dispatch()
    test_state_machine_with_dict()
    test_recursive_dict_pattern()
    test_config_with_defaults()
    test_counting_pattern()
    
    print("\n" + "=" * 60)
    print("Summary of Advanced Patterns:")
    print("  [OK] Dictionary-based dispatch")
    print("  [OK] State machine using dictionaries")
    print("  [OK] Nested dict structures")
    print("  [OK] Configuration with defaults and merging")
    print("  [OK] Counting and grouping patterns")
    print("\nNote: Lambda syntax not supported - using alternative approaches")
}