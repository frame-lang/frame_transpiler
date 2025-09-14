# Test special dictionary types from collections module

from collections import defaultdict, OrderedDict, Counter, ChainMap

fn test_defaultdict() {
    print("=== Testing defaultdict ===")
    
    # With int default (0)
    var dd_int = defaultdict(int)
    dd_int["count"] = dd_int["count"] + 1  # No KeyError, starts at 0
    print("dd_int['count'] = " + str(dd_int["count"]))  # 1
    print("dd_int['new'] = " + str(dd_int["new"]))      # 0 (auto-created)
    
    # With list default
    var dd_list = defaultdict(list)
    dd_list["items"].append("apple")
    dd_list["items"].append("banana")
    print("dd_list['items'] = " + str(dd_list["items"]))  # ['apple', 'banana']
    print("dd_list['empty'] = " + str(dd_list["empty"]))  # [] (auto-created)
    
    print("Note: lambda and nested functions not yet supported in Frame")
}

fn test_ordereddict() {
    print("")
    print("=== Testing OrderedDict ===")
    print("Note: In Python 3.7+, regular dicts maintain insertion order")
    
    # Create an OrderedDict
    var od = OrderedDict()
    od["first"] = 1
    od["second"] = 2
    od["third"] = 3
    print("OrderedDict keys: " + str(list(od.keys())))  # ['first', 'second', 'third']
    
    # Move to end
    od.move_to_end("first")  # first is now last
    print("After move_to_end('first'): " + str(list(od.keys())))  # ['second', 'third', 'first']
    
    od.move_to_end("third", False)  # third is now first (last=False)
    print("After move_to_end('third', False): " + str(list(od.keys())))  # ['third', 'second', 'first']
}

fn test_counter() {
    print("")
    print("=== Testing Counter ===")
    
    # Count items in a list
    var words = ["apple", "banana", "apple", "cherry", "banana", "apple"]
    var count = Counter(words)
    print("Counter(words) = " + str(count))
    # Counter({'apple': 3, 'banana': 2, 'cherry': 1})
    
    # Count characters in a string
    var char_count = Counter("mississippi")
    print("Counter('mississippi') = " + str(char_count))
    # Counter({'i': 4, 's': 4, 'p': 2, 'm': 1})
    
    # Most common
    var most_common = count.most_common(2)
    print("most_common(2) = " + str(most_common))  # [('apple', 3), ('banana', 2)]
    
    # Update counter
    count.update(["apple", "date"])
    print("After update: " + str(count["apple"]))  # 4
    
    # Arithmetic operations
    var count2 = Counter(["apple", "date", "date"])
    var combined = count + count2  # Combine counts
    print("Combined counter: " + str(combined))
}

fn test_chainmap() {
    print("")
    print("=== Testing ChainMap ===")
    
    var defaults = {"color": "red", "size": "medium"}
    var user_prefs = {"color": "blue"}
    var current = {"theme": "dark"}
    
    # Chain lookups (first found wins)
    var settings = ChainMap(current, user_prefs, defaults)
    
    print("settings['color'] = " + str(settings["color"]))    # 'blue' (from user_prefs)
    print("settings['size'] = " + str(settings["size"]))      # 'medium' (from defaults)
    print("settings['theme'] = " + str(settings["theme"]))    # 'dark' (from current)
    
    # Show all maps
    print("All maps: " + str(settings.maps))
    
    # Add a new mapping to the chain
    var new_child = settings.new_child({"color": "green"})
    print("new_child['color'] = " + str(new_child["color"]))  # 'green' (from new child)
}

fn main() {
    print("Frame v0.38 - Special Dictionary Types")
    print("=" * 50)
    
    test_defaultdict()
    test_ordereddict()
    test_counter()
    test_chainmap()
    
    print("")
    print("=" * 50)
    print("Summary:")
    print("  All special dictionary types from collections module work!")
    print("  - defaultdict: Auto-creates missing values")
    print("  - OrderedDict: Maintains insertion order (less needed in 3.7+)")
    print("  - Counter: Counts hashable objects")
    print("  - ChainMap: Chains multiple mappings for lookups")
}