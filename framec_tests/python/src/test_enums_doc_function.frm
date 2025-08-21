fn main() {
    var sys = TestSystem()
    sys.testFruit()
    sys.describeFruit(TestSystem_Fruit.Banana)
}

system TestSystem {
    actions:
        testFruit() {
            var f: Fruit = Fruit.Pear

            if f == Fruit.Peach {
                print("Found a Peach")
            } elif f == Fruit.Pear {
                print("Found a Pear")
            } elif f == Fruit.Banana {
                print("Found a Banana")
            } else {
                print("Unknown fruit")
            }
        }

        describeFruit(fruit_value: Fruit) {
            if fruit_value == Fruit.Peach {
                print("Peaches")
            } elif fruit_value == Fruit.Pear {
                print("Pears") 
            } elif fruit_value == Fruit.Banana {
                print("Bananas")
            } else {
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