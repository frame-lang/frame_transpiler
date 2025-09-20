# Comprehensive debugging test with Frame systems (state machines)
# Tests debugging through state machines and transitions

#TrafficLight $[start, stop]

    -interface-
    start
    stop
    tick
    
    -machine-
    
    $Begin
        |start| -> $Red ^
    
    $Red
        |>|
            print("  [TrafficLight] Entered RED state")
            count = 0
        ^
        
        |tick|
            count = count + 1
            print("  [TrafficLight] Red tick " + str(count))
            if count >= 3 {
                -> $Green ^
            }
        ^
        
        |stop| -> $End ^
    
    $Green
        |>|
            print("  [TrafficLight] Entered GREEN state")
            count = 0
        ^
        
        |tick|
            count = count + 1
            print("  [TrafficLight] Green tick " + str(count))
            if count >= 3 {
                -> $Yellow ^
            }
        ^
        
        |stop| -> $End ^
    
    $Yellow
        |>|
            print("  [TrafficLight] Entered YELLOW state")
            count = 0
        ^
        
        |tick|
            count = count + 1
            print("  [TrafficLight] Yellow tick " + str(count))
            if count >= 1 {
                -> $Red ^
            }
        ^
        
        |stop| -> $End ^
    
    $End
        |>|
            print("  [TrafficLight] System stopped")
        ^
    
    -actions-
    
    -domain-
    var count = 0
    
##

#VendingMachine $[insert_coin, select_item, refund]

    -interface-
    insert_coin [amount]
    select_item [item]
    refund
    dispense
    
    -machine-
    
    $Begin
        |insert_coin| [amount]
            balance = 0
            -> $Idle ^
    
    $Idle
        |>|
            print("  [VendingMachine] Ready. Balance: " + str(balance))
        ^
        
        |insert_coin| [amount]
            balance = balance + amount
            print("  [VendingMachine] Inserted " + str(amount) + ". New balance: " + str(balance))
            if balance >= 100 {
                -> $Ready ^
            }
        ^
        
        |refund|
            print("  [VendingMachine] Refunding " + str(balance))
            balance = 0
        ^
    
    $Ready
        |>|
            print("  [VendingMachine] Ready to vend. Balance: " + str(balance))
        ^
        
        |select_item| [item]
            print("  [VendingMachine] Selected: " + item)
            if item == "soda" {
                price = 150
            } elif item == "chips" {
                price = 100
            } else {
                price = 200
            }
            
            if balance >= price {
                balance = balance - price
                selected_item = item
                -> $Dispensing ^
            } else {
                print("  [VendingMachine] Insufficient funds. Need " + str(price))
            }
        ^
        
        |insert_coin| [amount]
            balance = balance + amount
            print("  [VendingMachine] Added " + str(amount) + ". New balance: " + str(balance))
        ^
        
        |refund|
            print("  [VendingMachine] Refunding " + str(balance))
            balance = 0
            -> $Idle ^
        ^
    
    $Dispensing
        |>|
            print("  [VendingMachine] Dispensing " + selected_item)
            dispense() -> $Complete ^
        ^
    
    $Complete
        |>|
            print("  [VendingMachine] Transaction complete. Change: " + str(balance))
            if balance > 0 {
                print("  [VendingMachine] Returning change: " + str(balance))
            }
            balance = 0
            -> $Idle ^
        ^
    
    -actions-
    
    dispense {
        print("  [VendingMachine.dispense] Physical dispense of " + selected_item)
        return
    }
    
    -domain-
    var balance = 0
    var selected_item = ""
    var price = 0
    
##

fn test_traffic_light() {
    print("=== Testing Traffic Light System ===")
    
    # Create and start traffic light
    var light = TrafficLight()
    light.start()
    
    # Test state transitions
    for cycle in range(2) {
        print("--- Cycle " + str(cycle + 1) + " ---")
        
        # Red state (3 ticks)
        for i in range(3) {
            light.tick()
        }
        
        # Green state (3 ticks)
        for i in range(3) {
            light.tick()
        }
        
        # Yellow state (1 tick)
        light.tick()
    }
    
    # Stop the system
    light.stop()
    
    print("=== Traffic Light Test Complete ===")
    return
}

fn test_vending_machine() {
    print("=== Testing Vending Machine System ===")
    
    # Create vending machine
    var vending = VendingMachine()
    
    # Test insufficient funds
    vending.insert_coin(50)
    vending.insert_coin(25)
    
    # Try to buy without enough money
    vending.select_item("soda")  # Needs 150
    
    # Add more money
    vending.insert_coin(100)
    
    # Buy chips
    vending.select_item("chips")  # Costs 100
    
    # Test refund
    vending.insert_coin(75)
    vending.refund()
    
    # Test exact change
    vending.insert_coin(150)
    vending.select_item("soda")
    
    print("=== Vending Machine Test Complete ===")
    return
}

fn test_nested_calls() {
    print("=== Testing Nested Function Calls ===")
    
    # Test deep nesting
    var result = level1(5)
    print("Final nested result: " + str(result))
    
    print("=== Nested Calls Test Complete ===")
    return result
}

fn level1(x) {
    print("  [level1] Called with x=" + str(x))
    var result = level2(x * 2)
    print("  [level1] Returning " + str(result))
    return result
}

fn level2(x) {
    print("  [level2] Called with x=" + str(x))
    var result = level3(x + 5)
    print("  [level2] Returning " + str(result))
    return result
}

fn level3(x) {
    print("  [level3] Called with x=" + str(x))
    var result = level4(x - 3)
    print("  [level3] Returning " + str(result))
    return result
}

fn level4(x) {
    print("  [level4] Called with x=" + str(x))
    # Final computation
    var result = x * 3
    print("  [level4] Final computation: " + str(result))
    return result
}

fn main() {
    print("=== Starting System Debug Test ===")
    
    # Test traffic light state machine
    test_traffic_light()
    
    # Test vending machine state machine
    test_vending_machine()
    
    # Test nested function calls
    test_nested_calls()
    
    print("=== All System Tests Completed Successfully ===")
    return
}