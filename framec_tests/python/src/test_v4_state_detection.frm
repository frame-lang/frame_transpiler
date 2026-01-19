@@target python_3

@@system Calculator {
    interface:
        add(a, b)
        subtract(a, b)
    
    machine:
        $Ready {
            add(a, b) {
                return a + b
            }
            
            subtract(a, b) {
                return a - b
            }
        }
}

# Test that the state name is correctly detected
def main():
    calc = Calculator()
    result = calc.add(2, 3)
    if result == 5:
        print("SUCCESS: Calculator state detection working")
    else:
        print("FAIL: Calculator state detection not working")
        # Force test failure
        var failed_tests = []
        var index = failed_tests[999]
    
main()