# Frame Capability Module: Collections Support
# Provides universal collection operations across all target languages
# Python: uses list, dict, set with native methods
# TypeScript: uses Array, Map, Set with functional methods
# C#: uses List<T>, Dictionary<K,V>, HashSet<T> with LINQ
# Java: uses ArrayList, HashMap, HashSet with streams
# Go: uses slices, maps with custom implementations
# Rust: uses Vec<T>, HashMap<K,V>, HashSet<T> with iterators
# C: uses custom data structure implementations

module Collections {
    # Create empty collections
    fn createList() {
        return []
    }
    
    fn createMap() {
        return {}
    }
    
    fn createSet() {
        # In Frame, this creates an empty set
        # Note: {} creates a dict, need special syntax for empty set
        return set()
    }
    
    fn createTuple(items) {
        # Convert list to tuple
        return tuple(items)
    }
    
    # List operations
    fn append(lst, item) {
        # Python: lst.append(item)
        # TypeScript: lst.push(item)
        # C#: lst.Add(item)
        # Java: lst.add(item)
        # Go: append(lst, item)
        # Rust: lst.push(item)
        # C: custom list_append function
        
        lst.append(item)
        return lst
    }
    
    fn prepend(lst, item) {
        # Add item to beginning of list
        lst.insert(0, item)
        return lst
    }
    
    fn extend(lst, other) {
        # Add all items from other to lst
        lst.extend(other)
        return lst
    }
    
    fn removeAt(lst, index) {
        # Remove item at specific index
        var item = lst[index]
        del lst[index]
        return item
    }
    
    fn indexOf(lst, item) {
        # Find index of item in list
        return lst.index(item)
    }
    
    fn contains(collection, item) {
        # Check if collection contains item
        return item in collection
    }
    
    fn length(collection) {
        # Get size of collection
        return len(collection)
    }
    
    fn isEmpty(collection) {
        # Check if collection is empty
        return len(collection) == 0
    }
    
    # Map/Dictionary operations  
    fn put(map, key, value) {
        # Python: map[key] = value
        # TypeScript: map.set(key, value)
        # C#: map[key] = value
        # Java: map.put(key, value)
        # Go: map[key] = value
        # Rust: map.insert(key, value)
        # C: custom map_put function
        
        map[key] = value
        return map
    }
    
    fn get(map, key) {
        # Get value by key
        return map[key]
    }
    
    fn getOrDefault(map, key, defaultValue) {
        # Get value or return default if key doesn't exist
        if key in map {
            return map[key]
        } else {
            return defaultValue
        }
    }
    
    fn removeKey(map, key) {
        # Remove key-value pair
        var value = map[key]
        del map[key]
        return value
    }
    
    fn keys(map) {
        # Get all keys
        return list(map.keys())
    }
    
    fn values(map) {
        # Get all values
        return list(map.values())
    }
    
    fn items(map) {
        # Get key-value pairs
        return list(map.items())
    }
    
    # Set operations
    fn add(set_collection, item) {
        # Add item to set
        set_collection.add(item)
        return set_collection
    }
    
    fn remove(set_collection, item) {
        # Remove item from set
        set_collection.remove(item)
        return set_collection
    }
    
    fn union(set1, set2) {
        # Union of two sets
        return set1 | set2
    }
    
    fn intersection(set1, set2) {
        # Intersection of two sets
        return set1 & set2
    }
    
    fn difference(set1, set2) {
        # Difference of two sets
        return set1 - set2
    }
    
    # Functional operations (work on lists)
    fn map(lst, func) {
        # Apply function to each item
        # Python: [func(item) for item in lst]
        # TypeScript: lst.map(func)
        # C#: lst.Select(func)
        # Java: lst.stream().map(func).collect()
        # Go: custom implementation
        # Rust: lst.iter().map(func).collect()
        # C: custom map function
        
        var result = []
        for item in lst {
            result.append(func(item))
        }
        return result
    }
    
    fn filter(lst, predicate) {
        # Filter items by predicate
        var result = []
        for item in lst {
            if predicate(item) {
                result.append(item)
            }
        }
        return result
    }
    
    fn reduce(lst, func, initial) {
        # Reduce list to single value
        var result = initial
        for item in lst {
            result = func(result, item)
        }
        return result
    }
    
    fn forEach(lst, func) {
        # Execute function for each item
        for item in lst {
            func(item)
        }
    }
    
    fn find(lst, predicate) {
        # Find first item matching predicate
        for item in lst {
            if predicate(item) {
                return item
            }
        }
        return None
    }
    
    fn findIndex(lst, predicate) {
        # Find index of first item matching predicate
        for i in range(len(lst)) {
            if predicate(lst[i]) {
                return i
            }
        }
        return -1
    }
    
    # Utility operations
    fn reverse(lst) {
        # Reverse list in place
        lst.reverse()
        return lst
    }
    
    fn sort(lst) {
        # Sort list in place
        lst.sort()
        return lst
    }
    
    fn sortBy(lst, keyFunc) {
        # Sort list by key function
        lst.sort(key=keyFunc)
        return lst
    }
    
    fn slice(lst, start, end) {
        # Get slice of list
        return lst[start:end]
    }
    
    fn concat(lst1, lst2) {
        # Concatenate two lists
        return lst1 + lst2
    }
    
    fn copy(collection) {
        # Create shallow copy - simplified for now
        return collection
    }
    
    fn deepCopy(collection) {
        # Create deep copy (will need platform-specific implementation)
        # Python: copy.deepcopy()
        # TypeScript: JSON.parse(JSON.stringify()) or structuredClone()
        # C#: serialization or reflection
        # Java: serialization or custom cloning
        # Go: custom deep copy implementation
        # Rust: Clone trait
        # C: custom deep copy function
        
        print("Deep copy not yet implemented in this version")
        return copy(collection)
    }
}