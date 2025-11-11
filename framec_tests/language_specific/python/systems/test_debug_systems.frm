@target python

system TrafficLight {
    interface:
        start()
        stop()
        tick()

    domain:
        count: int = 0

    machine:
        $Begin {
            start() {
                -> $Red
            }
        }
        $Red {
            $>() {
                # Enter RED, reset count
                self.count = 0
            }
            tick() {
                self.count = self.count + 1
                if self.count >= 3:
                    -> $Green
                return
            }
            stop() { -> $End }
        }
        $Green {
            $>() { self.count = 0 }
            tick() {
                self.count = self.count + 1
                if self.count >= 3:
                    -> $Yellow
                return
            }
            stop() { -> $End }
        }
        $Yellow {
            $>() { self.count = 0 }
            tick() { -> $Red }
            stop() { -> $End }
        }
        $End { $>() { return } }
}

system VendingMachine {
    interface:
        insert_coin(amount: int)
        select_item(item: str)
        refund()

    domain:
        balance: int = 0
        selected_item: str = ""
        price: int = 0

    machine:
        $Begin {
            insert_coin(amount: int) {
                self.balance = 0
                -> $Idle
            }
        }
        $Idle {
            $>() {
                # Ready
            }
            insert_coin(amount: int) {
                self.balance = self.balance + amount
                if self.balance >= 100:
                    -> $Ready
                return
            }
            refund() {
                self.balance = 0
                return
            }
        }
        $Ready {
            $>() { }
            select_item(item: str) {
                if item == "soda":
                    self.price = 150
                elif item == "chips":
                    self.price = 100
                else:
                    self.price = 200
                if self.balance >= self.price:
                    self.balance = self.balance - self.price
                    self.selected_item = item
                    -> $Dispensing
                else:
                    # insufficient funds
                    return
            }
            insert_coin(amount: int) {
                self.balance = self.balance + amount
                return
            }
            refund() {
                self.balance = 0
                -> $Idle
            }
        }
        $Dispensing {
            $>() {
                -> $Complete
            }
        }
        $Complete {
            $>() {
                if self.balance > 0:
                    # return change
                    self.balance = 0
                -> $Idle
            }
        }
}

fn test_traffic_light() {
    print("=== Testing Traffic Light System ===")
    light = TrafficLight()
    light.start()
    # Simulate cycles
    for cycle in range(2):
        for i in range(3):
            light.tick()
        for i in range(3):
            light.tick()
        light.tick()
    light.stop()
    print("=== Traffic Light Test Complete ===")
    return
}

fn test_vending_machine() {
    print("=== Testing Vending Machine System ===")
    vending = VendingMachine()
    vending.insert_coin(50)
    vending.insert_coin(25)
    vending.select_item("soda")  # Needs 150
    vending.insert_coin(100)
    vending.select_item("chips")  # Costs 100
    vending.insert_coin(75)
    vending.refund()
    vending.insert_coin(150)
    vending.select_item("soda")
    print("=== Vending Machine Test Complete ===")
    return
}

fn test_nested_calls() {
    print("=== Testing Nested Function Calls ===")
    result = level1(5)
    print("Final nested result: " + str(result))
    print("=== Nested Calls Test Complete ===")
    return result
}

fn level1(x) {
    print("  [level1] Called with x=" + str(x))
    result = level2(x * 2)
    print("  [level1] Returning " + str(result))
    return result
}

fn level2(x) {
    print("  [level2] Called with x=" + str(x))
    result = level3(x + 5)
    print("  [level2] Returning " + str(result))
    return result
}

fn level3(x) {
    print("  [level3] Called with x=" + str(x))
    result = level4(x - 3)
    print("  [level3] Returning " + str(result))
    return result
}

fn level4(x) {
    print("  [level4] Called with x=" + str(x))
    result = x * 3
    print("  [level4] Final computation: " + str(result))
    return result
}

fn main() {
    print("=== Starting System Debug Test ===")
    test_traffic_light()
    test_vending_machine()
    test_nested_calls()
    print("=== All System Tests Completed Successfully ===")
    return
}
