# Frame Capability Module: Collections Support (Simplified)
# Provides universal collection operations across all target languages

module Collections {
    # Create collections
    fn createList() {
        return []
    }
    
    fn createMap() {
        return {}
    }
    
    fn createSet() {
        return set()
    }
    
    # List operations
    fn append(lst, item) {
        lst.append(item)
        return lst
    }
    
    fn prepend(lst, item) {
        lst.insert(0, item)
        return lst
    }
    
    fn length(collection) {
        return len(collection)
    }
    
    fn isEmpty(collection) {
        return len(collection) == 0
    }
    
    # Map operations
    fn put(map, key, value) {
        map[key] = value
        return map
    }
    
    fn get(map, key) {
        return map[key]
    }
    
    fn removeKey(map, key) {
        var value = map[key]
        del map[key]
        return value
    }
    
    fn keys(map) {
        return list(map.keys())
    }
    
    fn values(map) {
        return list(map.values())
    }
    
    fn items(map) {
        return list(map.items())
    }
    
    # Set operations
    fn add(set_collection, item) {
        set_collection.add(item)
        return set_collection
    }
    
    fn remove(set_collection, item) {
        set_collection.remove(item)
        return set_collection
    }
    
    # Utility operations
    fn reverse(lst) {
        lst.reverse()
        return lst
    }
    
    fn sort(lst) {
        lst.sort()
        return lst
    }
    
    fn slice(lst, start, end) {
        return lst[start:end]
    }
    
    fn concat(lst1, lst2) {
        return lst1 + lst2
    }
    
    fn copy(collection) {
        return collection
    }
}