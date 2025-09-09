// Test string slicing operations - Phase 1
// Testing comprehensive string slicing support

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

fn test_negative_indexing() {
    var text = "Python Frame"
    
    // Negative start index
    var last_5 = text[-5:]
    print("Slice [-5:]:", last_5)  // "Frame"
    
    // Negative end index
    var first_6 = text[:-6]
    print("Slice [:-6]:", first_6)  // "Python"
    
    // Both negative
    var middle = text[-11:-6]
    print("Slice [-11:-6]:", middle)  // "ython"
    
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
    
    // Reverse string
    var reversed = text[::-1]
    print("Slice [::-1]:", reversed)  // "9876543210"
    
    // Every third character
    var thirds = text[::3]
    print("Slice [::3]:", thirds)  // "0369"
    
    // Complex slice with all three
    var complex = text[2:8:2]
    print("Slice [2:8:2]:", complex)  // "246"
    
    return
}

fn test_list_slicing() {
    var numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    
    // Basic list slicing
    var first_five = numbers[:5]
    print("List [:5]:", str(first_five))  // [0, 1, 2, 3, 4]
    
    // Last five
    var last_five = numbers[-5:]
    print("List [-5:]:", str(last_five))  // [5, 6, 7, 8, 9]
    
    // Middle section
    var middle = numbers[3:7]
    print("List [3:7]:", str(middle))  // [3, 4, 5, 6]
    
    // Every other element
    var every_other = numbers[::2]
    print("List [::2]:", str(every_other))  // [0, 2, 4, 6, 8]
    
    // Reverse list
    var reversed_list = numbers[::-1]
    print("List [::-1]:", str(reversed_list))  // [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
    
    return
}

fn test_slice_assignment() {
    // Test that slices can be used in assignments
    var original = "ABCDEFGHIJ"
    
    var slice1 = original[2:5]
    var slice2 = original[-3:]
    var slice3 = original[1::3]
    
    print("Assigned slices:")
    print("  slice1:", slice1)  // "CDE"
    print("  slice2:", slice2)  // "HIJ"
    print("  slice3:", slice3)  // "BEH"
    
    return
}

fn test_expression_slicing() {
    // Test slicing with expressions as indices
    var text = "0123456789"
    var start = 2
    var end = 8
    var step = 2
    
    var result = text[start:end:step]
    print("Expression slice:", result)  // "246"
    
    // With calculations
    var calc_slice = text[start+1:end-1]
    print("Calculated slice:", calc_slice)  // "34567"
    
    return
}

fn main() {
    print("=== Basic Slicing ===")
    test_basic_slicing()
    
    print("\n=== Negative Indexing ===")
    test_negative_indexing()
    
    print("\n=== Step Slicing ===")
    test_step_slicing()
    
    print("\n=== List Slicing ===")
    test_list_slicing()
    
    print("\n=== Slice Assignment ===")
    test_slice_assignment()
    
    print("\n=== Expression Slicing ===")
    test_expression_slicing()
    
    print("\n=== All slicing tests completed ===")
}