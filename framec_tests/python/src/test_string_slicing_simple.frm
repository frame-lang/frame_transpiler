// Simple test of string slicing without negative indices

fn test_basic_slicing() {
    var text = "Hello, World!"
    
    // Basic slicing with start and end
    var hello = text[0:5]
    print("Slice [0:5]:", hello)  // "Hello"
    
    // Slice from start
    var world = text[7:13]
    print("Slice [7:13]:", world)  // "World!"
    
    // Slice to end (from index)
    var from_comma = text[5:]
    print("Slice [5:]:", from_comma)  // ", World!"
    
    // Slice from beginning (to index)
    var to_comma = text[:5]
    print("Slice [:5]:", to_comma)  // "Hello"
    
    // Full slice (copy)
    var copy = text[:]
    print("Slice [:]:", copy)  // "Hello, World!"
    
    return
}

fn test_step_slicing() {
    var text = "0123456789"
    
    // Every second character
    var evens = text[::2]
    print("Slice [::2]:", evens)  // "02468"
    
    // Every second from index 1
    var odds = text[1::2]
    print("Slice [1::2]:", odds)  // "13579"
    
    // Every third character
    var thirds = text[::3]
    print("Slice [::3]:", thirds)  // "0369"
    
    // Complex slice with all three
    var complex = text[2:8:2]
    print("Slice [2:8:2]:", complex)  // "246"
    
    return
}

fn main() {
    print("=== Basic Slicing ===")
    test_basic_slicing()
    
    print("\n=== Step Slicing ===")
    test_step_slicing()
    
    print("\n=== All slicing tests completed ===")
}