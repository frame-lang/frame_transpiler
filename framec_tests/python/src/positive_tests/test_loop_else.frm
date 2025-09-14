# Test loop else clauses - v0.51
# Expected: else blocks execute when loops complete normally (without break)

fn test_for_else_no_break() {
    print("Testing for-else without break:")
    
    for i in range(5) {
        print("  i = " + str(i))
    }
    else {
        print("  Loop completed normally - else executed")
    }
    
    return "completed"
}

fn test_for_else_with_break() {
    print("Testing for-else with break:")
    
    for i in range(5) {
        print("  i = " + str(i))
        if i == 2 {
            print("  Breaking at i=2")
            break
        }
    }
    else {
        print("  This should NOT print - loop was broken")
    }
    
    return "broken"
}

fn test_while_else_no_break() {
    print("Testing while-else without break:")
    var count = 0
    
    while count < 3 {
        print("  count = " + str(count))
        count = count + 1
    }
    else {
        print("  While loop completed normally - else executed")
    }
    
    return count
}

fn test_while_else_with_break() {
    print("Testing while-else with break:")
    var count = 0
    
    while count < 5 {
        print("  count = " + str(count))
        if count == 2 {
            print("  Breaking at count=2")
            break
        }
        count = count + 1
    }
    else {
        print("  This should NOT print - loop was broken")
    }
    
    return count
}

fn test_nested_loops_with_else() {
    print("Testing nested loops with else:")
    
    for i in range(3) {
        print("Outer loop i = " + str(i))
        
        for j in range(3) {
            print("  Inner loop j = " + str(j))
            if i == 1 and j == 1 {
                print("  Breaking inner loop")
                break
            }
        }
        else {
            print("  Inner loop completed normally")
        }
    }
    else {
        print("Outer loop completed normally")
    }
    
    return "nested"
}

fn test_for_else_search_found() {
    print("Testing for-else for search (item found):")
    var items = ["apple", "banana", "cherry", "date"]
    var target = "cherry"
    var found = false
    
    for item in items {
        print("  Checking: " + item)
        if item == target {
            print("  Found: " + item)
            found = true
            break
        }
    }
    else {
        print("  Item not found: " + target)
    }
    
    return found
}

fn test_for_else_search_not_found() {
    print("Testing for-else for search (item not found):")
    var items = ["apple", "banana", "cherry", "date"]
    var target = "grape"
    var found = false
    
    for item in items {
        print("  Checking: " + item)
        if item == target {
            print("  Found: " + item)
            found = true
            break
        }
    }
    else {
        print("  Item not found: " + target)
    }
    
    return found
}

fn test_while_else_condition_false() {
    print("Testing while-else with initially false condition:")
    var x = 10
    
    while x < 5 {
        print("  This should not execute")
        x = x + 1
    }
    else {
        print("  While condition was false - else executed")
    }
    
    return x
}

fn test_continue_with_else() {
    print("Testing continue with else:")
    
    for i in range(5) {
        if i == 2 {
            print("  Skipping i = " + str(i))
            continue
        }
        print("  Processing i = " + str(i))
    }
    else {
        print("  Loop with continue completed - else executed")
    }
    
    return "continue_test"
}

fn main() {
    print("=== Testing Loop Else Clauses ===")
    print("")
    
    # Test for-else without break
    var result1 = test_for_else_no_break()
    print("Result: " + result1)
    print("")
    
    # Test for-else with break
    var result2 = test_for_else_with_break()
    print("Result: " + result2)
    print("")
    
    # Test while-else without break
    var result3 = test_while_else_no_break()
    print("Result: " + str(result3))
    print("")
    
    # Test while-else with break
    var result4 = test_while_else_with_break()
    print("Result: " + str(result4))
    print("")
    
    # Test nested loops
    var result5 = test_nested_loops_with_else()
    print("Result: " + result5)
    print("")
    
    # Test search scenarios
    var found1 = test_for_else_search_found()
    print("Found (should be true): " + str(found1))
    print("")
    
    var found2 = test_for_else_search_not_found()
    print("Found (should be false): " + str(found2))
    print("")
    
    # Test while with false condition
    var result6 = test_while_else_condition_false()
    print("Result: " + str(result6))
    print("")
    
    # Test continue with else
    var result7 = test_continue_with_else()
    print("Result: " + result7)
    print("")
    
    print("=== All Loop Else Tests Complete ===")
}