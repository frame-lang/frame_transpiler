# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test current Frame v0.38 capabilities (v0.39 features not yet implemented)
# This test demonstrates what Frame can do today

fn test_basic_features() {
    print("=== Basic Frame Features Working ===")
    
    # Variables and basic operations
    x = 10
    y = 20
    sum = x + y
    print("Basic math: " + str(x) + " + " + str(y) + " = " + str(sum))
    
    # String operations
    greeting = "Hello"
    name = "Frame"
    message = greeting + " " + name
    print("String concat: " + message)
    
    return
}

fn test_collections() {
    print("\n=== Collections Working ===")
    
    # Lists
    numbers = [1, 2, 3, 4, 5]
    numbers.append(6)
    print("List operations: " + str(numbers))
    
    # Dictionaries
    person = {"name": "Alice", "age": 30}
    person["city"] = "NYC"
    print("Dict operations: " + str(person))
    
    # Dictionary comprehensions
    squares = {x: x * x for x in range(5)}
    print("Dict comprehension: " + str(squares))
    
    return
}

fn test_control_flow() {
    print("\n=== Control Flow Working ===")
    
    # if-elif-else
    score = 85
    grade = ""
    if score >= 90:
        grade = "A"
    elif score >= 80:
        grade = "B"
    else:
        grade = "C"
    }
    print("Grade for " + str(score) + ": " + grade)
    
    # Loops
    count = 0
    i = 0
    while i < 3:
        count = count + 1
        i = i + 1
    print("Loop result: " + str(count))
    
    return
}

fn helper(a, b) {
    return a * b
}

fn test_functions() {
    print("\n=== Functions Working ===")
    
    result = helper(6, 7)
    print("Function call: " + str(result))
    
    return
}

# Test system functionality
system TestSystem {
    interface:
        increment()
        getCount(): int
    
    machine:
        $Ready {
            increment() {
                counter = counter + 1
                print("Incremented to: " + str(counter))
                return
            }
            
            getCount() {
                system.return = counter
            }
        }
        
    domain:
        counter = 0
}

fn test_systems() {
    print("\n=== Systems Working ===")
    
    sys = TestSystem()
    sys.increment()
    sys.increment()
    count = sys.getCount()
    print("System counter: " + str(count))
    
    return
}

fn main() {
    print("Frame v0.38 - Current Capability Showcase")
    print("=" * 50)
    
    test_basic_features()
    test_collections()
    test_control_flow()
    test_functions()
    test_systems()
    
    print("\n" + "=" * 50)
    print("Summary - Frame v0.38 Working Features:")
    print("  [✓] Basic operations and variables")
    print("  [✓] String manipulation")
    print("  [✓] Lists, dictionaries, comprehensions")
    print("  [✓] Control flow (if/elif/else, while)")
    print("  [✓] Functions and returns")
    print("  [✓] State machine systems")
    print("  [✓] Interface methods")
    print("  [✓] Domain variables")
    print("\nFeatures for future versions:")
    print("  [○] Module imports")
    print("  [○] Method chaining")
    print("  [○] Lambda expressions")
    print("  [○] Advanced Python integration")
    
    return
}
