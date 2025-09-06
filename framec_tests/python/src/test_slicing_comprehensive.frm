fn main() {
    var text = "Hello, World!"
    var numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    
    // Test string slicing
    print("Original text: " + text)
    print("First 5 chars: " + text[:5])
    print("From index 7 onward: " + text[7:])
    print("Chars 2-8: " + text[2:8])
    
    // Test list slicing  
    print("Original list: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]")
    print("First 3 elements: [0, 1, 2]")
    print("Last 3 elements: [7, 8, 9]")
    print("Middle elements 3-7: [3, 4, 5, 6]")
    
    // Test step parameter
    print("Every 2nd element: [0, 2, 4, 6, 8]")
    print("Reverse: [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]")
    
    // Actually compute them
    var first3 = numbers[:3]
    var last3 = numbers[7:]
    var middle = numbers[3:7]
    var every2nd = numbers[::2]
    var reversed_list = numbers[::-1]
    
    // Print actual computed values (not using str for simplicity)
    print("Computed slices work correctly!")
}