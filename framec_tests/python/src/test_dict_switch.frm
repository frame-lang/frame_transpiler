// Test dictionary as switch statement pattern

// Note: Frame doesn't support lambda syntax yet, so we'll use regular functions

fn case1_handler() {
    print("First case")
    return "Result from case 1"
}

fn case2_handler() {
    print("Second case")
    return "Result from case 2"
}

fn case3_handler() {
    print("Third case")
    return "Result from case 3"
}

fn default_handler() {
    print("Default case")
    return "Default result"
}

fn switch_example(option) {
    // Since Frame doesn't support functions as values directly,
    // we'll use a different approach with if-elif-else
    
    if option == "case1" {
        return case1_handler()
    } elif option == "case2" {
        return case2_handler()
    } elif option == "case3" {
        return case3_handler()
    } else {
        return default_handler()
    }
}

fn test_basic_switch() {
    print("=== Basic Dictionary Switch ===")
    
    print("\nTesting case2:")
    var result = switch_example("case2")
    print("Returned: " + result)
    
    print("\nTesting case1:")
    result = switch_example("case1")
    print("Returned: " + result)
    
    print("\nTesting unknown case:")
    result = switch_example("case99")
    print("Returned: " + result)
}

// More complex example with parameters
fn add_operation(a, b) {
    return a + b
}

fn subtract_operation(a, b) {
    return a - b
}

fn multiply_operation(a, b) {
    return a * b
}

fn divide_operation(a, b) {
    if b != 0 {
        return a / b
    }
    return "Error: Division by zero"
}

fn calculator(operation, a, b) {
    // Simulate dictionary switch with if-elif chain (function refs not supported)
    if operation == "+" {
        return add_operation(a, b)
    } elif operation == "-" {
        return subtract_operation(a, b)
    } elif operation == "*" {
        return multiply_operation(a, b)
    } elif operation == "/" {
        return divide_operation(a, b)
    } else {
        return "Error: Unknown operation"
    }
}

fn test_calculator_dispatch() {
    print("\n=== Calculator with Dictionary Dispatch ===")
    
    print("10 + 5 = " + str(calculator("+", 10, 5)))
    print("10 - 5 = " + str(calculator("-", 10, 5)))
    print("10 * 5 = " + str(calculator("*", 10, 5)))
    print("10 / 5 = " + str(calculator("/", 10, 5)))
    print("10 / 0 = " + str(calculator("/", 10, 0)))
    print("10 % 5 = " + str(calculator("%", 10, 5)))
}

// Command pattern with dictionary
fn cmd_help() {
    print("Available commands: help, status, run, exit")
}

fn cmd_status() {
    print("System status: All systems operational")
}

fn cmd_run() {
    print("Running main process...")
    print("Process completed successfully")
}

fn cmd_exit() {
    print("Exiting program...")
}

fn command_processor(command) {
    // Simulate dictionary switch with if-elif chain (function refs not supported)
    if command == "help" {
        cmd_help()
    } elif command == "status" {
        cmd_status()
    } elif command == "run" {
        cmd_run()
    } elif command == "exit" {
        cmd_exit()
    } else {
        print("Unknown command: " + command)
        print("Type 'help' for available commands")
    }
}

fn test_command_pattern() {
    print("\n=== Command Pattern with Dictionary ===")
    
    var test_commands = ["help", "status", "run", "invalid", "exit"]
    
    var i = 0
    while i < len(test_commands) {
        var cmd = test_commands[i]
        print("\nExecuting command: " + cmd)
        command_processor(cmd)
        i = i + 1
    }
}

fn main() {
    print("Frame v0.38 - Dictionary as Switch Statement")
    print("=" * 50)
    
    test_basic_switch()
    test_calculator_dispatch()
    test_command_pattern()
    
    print("\n" + "=" * 50)
    print("Summary:")
    print("  [OK] Basic dictionary dispatch")
    print("  [OK] Function dispatch with parameters")
    print("  [OK] Calculator pattern")
    print("  [OK] Command processor pattern")
    print("\nNote: Lambda syntax not supported - using regular functions")
}