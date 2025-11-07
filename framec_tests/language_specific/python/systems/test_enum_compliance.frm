# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test enum compliance with Python standards

system ColorSystem {
    domain:
        enum Color {
            RED = 1
            GREEN = 2  
            BLUE = 3
        }
        
        enum Priority : string {
            LOW = "low"
            MEDIUM = "medium"
            HIGH = "high"
        }
}

fn test_enum_usage() {
    # Test enum member access
    color = Color.RED
    priority = Priority.HIGH
    
    # Test enum comparisons
    if color == Color.RED {
        print("Color is red")
    }
    
    if priority == Priority.HIGH {
        print("Priority is high") 
    }
    
    # Test enum values
    print("Color value:", color.value)
    print("Priority value:", priority.value)
    
    # Test enum names  
    print("Color name:", color.name)
    print("Priority name:", priority.name)
    
    return
}

fn test_enum_iteration() {
    # Test iterating over enum members
    print("All colors:")
    for color in Color {
        print(" -", color.name, "=", color.value)
    }
    
    print("All priorities:")
    for priority in Priority {
        print(" -", priority.name, "=", priority.value)
    }
    
    return
}

fn main() {
    test_enum_usage()
    test_enum_iteration()
    return
}