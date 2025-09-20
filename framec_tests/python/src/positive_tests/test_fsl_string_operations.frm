# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test FSL string operations - Phase 3
# These should work WITHOUT backticks

fn main() {
    # Test string.upper()
    var name = "hello"
    var upper_name = name.upper()
    print("Upper:", upper_name)  # HELLO
    
    # Test string.lower()
    var loud = "WORLD"
    var quiet = loud.lower()
    print("Lower:", quiet)  # world
    
    # Test string.trim()
    var padded = "  spaces  "
    var trimmed = padded.trim()
    print("Trimmed:", "[" + trimmed + "]")  # [spaces]
    
    # Test string.contains() - NOT YET IMPLEMENTED
    # Python needs: "world" in text (not text.contains("world"))
    # var text = "hello world"
    # var has_world = text.contains("world")
    # print("Contains 'world':", has_world)  // true
    
    # Test string.replace()
    var original = "hello world"
    var replaced = original.replace("world", "Frame")
    print("Replaced:", replaced)  # hello Frame
    
    # Test string.split()
    var csv = "apple,banana,orange"
    var fruits = csv.split(",")
    print("Split result:", fruits)  # ['apple', 'banana', 'orange']
    
    # Test string.substring() - NOT YET IMPLEMENTED
    # Python needs: text[0:5] (not text.substring(0, 5))
    # var full_text = "Frame Language"
    # var sub = full_text.substring(0, 5)
    # print("Substring:", sub)  // Frame
    
    # Test string.length property
    var sample = "test"
    var length = sample.length
    print("Length:", length)  # 4
    
    return
}