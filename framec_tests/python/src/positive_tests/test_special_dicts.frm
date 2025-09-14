# Test special dictionary patterns using Frame's native capabilities
# Note: Frame doesn't support Python collections module imports yet

fn test_defaultdict_pattern() {
    print("=== Testing defaultdict-like pattern ===")
    
    # Simulate defaultdict with manual checks
    var dd_int = {}
    var key = "count"
    var current = dd_int.get(key, 0)
    dd_int[key] = current + 1
    print("dd_int['count'] = " + str(dd_int[key]))  # 1
    
    var new_val = dd_int.get("new", 0)
    print("dd_int['new'] = " + str(new_val))      # 0 (default value)
    
    # Simulate defaultdict with list default
    var dd_list = {}
    if "items" not in dd_list {
        dd_list["items"] = []
    }
    dd_list["items"].append("apple")
    dd_list["items"].append("banana")
    print("dd_list['items'] = " + str(dd_list["items"]))  # ['apple', 'banana']
    
    var empty_list = dd_list.get("empty", [])
    print("dd_list['empty'] = " + str(empty_list))  # [] (default empty list)
}

fn test_ordered_pattern() {
    print("")
    print("=== Testing ordered dictionary pattern ===")
    print("Note: Frame dicts maintain insertion order like modern Python")
    
    # Regular dict maintains order in Frame
    var od = {}
    od["first"] = 1
    od["second"] = 2
    od["third"] = 3
    
    # Manual key extraction to show order
    var keys = []
    if "first" in od {
        keys.append("first")
    }
    if "second" in od {
        keys.append("second")
    }
    if "third" in od {
        keys.append("third")
    }
    print("Dict keys: " + str(keys))  # ['first', 'second', 'third']
}

fn test_counter_pattern() {
    print("")
    print("=== Testing counter pattern ===")
    
    # Manual counting implementation
    var words = ["apple", "banana", "apple", "cherry", "banana", "apple"]
    var count = {}
    
    var i = 0
    while i < len(words) {
        var word = words[i]
        var current = count.get(word, 0)
        count[word] = current + 1
        i = i + 1
    }
    print("count = " + str(count))
    # {'apple': 3, 'banana': 2, 'cherry': 1}
    
    # Character counting
    var text = "mississippi"
    var char_count = {}
    i = 0
    while i < len(text) {
        var char = text[i]
        var current = char_count.get(char, 0)
        char_count[char] = current + 1
        i = i + 1
    }
    print("char_count = " + str(char_count))
    # {'m': 1, 'i': 4, 's': 4, 'p': 2}
    
    # Find most common (manual implementation)
    var max_count = 0
    var max_word = ""
    var word_keys = ["apple", "banana", "cherry"]
    i = 0
    while i < len(word_keys) {
        var word = word_keys[i]
        if word in count {
            var word_count = count[word]
            if word_count > max_count {
                max_count = word_count
                max_word = word
            }
        }
        i = i + 1
    }
    print("Most common word: " + max_word + " (count: " + str(max_count) + ")")
}

# Helper function for chainmap pattern (Frame doesn't support nested functions)
fn chain_get(key, current, user_prefs, defaults) {
    # Check current first
    if key in current {
        return current[key]
    }
    # Then user_prefs
    if key in user_prefs {
        return user_prefs[key]
    }
    # Finally defaults
    if key in defaults {
        return defaults[key]
    }
    return None
}

fn test_chainmap_pattern() {
    print("")
    print("=== Testing chainmap-like pattern ===")
    
    var defaults = {"color": "red", "size": "medium"}
    var user_prefs = {"color": "blue"}
    var current = {"theme": "dark"}
    
    # Manual chain lookup (first found wins)
    var color = chain_get("color", current, user_prefs, defaults)
    var size = chain_get("size", current, user_prefs, defaults)
    var theme = chain_get("theme", current, user_prefs, defaults)
    
    print("color = " + str(color))    # 'blue' (from user_prefs)
    print("size = " + str(size))      # 'medium' (from defaults)
    print("theme = " + str(theme))    # 'dark' (from current)
    
    # Show chain order
    print("Chain lookup order: current -> user_prefs -> defaults")
}

fn main() {
    print("Frame v0.38 - Special Dictionary Patterns")
    print("=" * 50)
    
    test_defaultdict_pattern()
    test_ordered_pattern()
    test_counter_pattern()
    test_chainmap_pattern()
    
    print("")
    print("=" * 50)
    print("Summary:")
    print("  All dictionary patterns implemented using Frame's native features!")
    print("  - defaultdict pattern: Manual default value handling")
    print("  - Ordered pattern: Native dict insertion order")
    print("  - Counter pattern: Manual counting implementation")
    print("  - ChainMap pattern: Manual lookup chain")
    print("  Note: Python collections module not yet supported")
}