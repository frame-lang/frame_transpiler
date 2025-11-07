# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    sys = TestSystem()
    sys.testFruit()
    sys.describeFruit(TestSystem_Fruit.Banana)
}

system TestSystem {
    interface:
        testFruit()
        describeFruit(fruit_value: Fruit)
        
    machine:
        $Ready {
            testFruit() {
                self.testFruit()
                return
            }
            
            describeFruit(fruit_value: Fruit) {
                self.describeFruit(fruit_value)
                return
            }
        }
        
    actions:
        testFruit() {
            f: Fruit = Fruit.Pear

            if f == Fruit.Peach:
                print("Found a Peach")
            elif f == Fruit.Pear:
                print("Found a Pear")
            elif f == Fruit.Banana:
                print("Found a Banana")
            else:
                print("Unknown fruit")
            }
        }

        describeFruit(fruit_value: Fruit) {
            if fruit_value == Fruit.Peach:
                print("Peaches")
            elif fruit_value == Fruit.Pear:
                print("Pears") 
            elif fruit_value == Fruit.Banana:
                print("Bananas")
            else:
                print("Other Fruit")
            }
        }
        
    domain:
        enum Fruit {
            Peach
            Pear
            Banana
        }
}
