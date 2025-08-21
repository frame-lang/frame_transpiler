`from enum import Enum`
`import random`

fn main() {
    var grocery = Grocery()
    print("We are selling " + grocery.getFruitOfTheDay() + " today.")
    print("We sold " + grocery.getFruitOfTheDay() + " yesterday.")
    print("We are selling " + grocery.getFruitOfTheDay() + " tomorrow.")
}

system Grocery {
    interface:
        getFruitOfTheDay(): string

    machine:
        $Start {
            getFruitOfTheDay(): string {
                var f: Fruit = getRandomFruit()

                // Demonstrate boolean tests for enums and return
                if f == Fruit.Peach {
                    print("Found a Peach.")
                    return "Peaches"
                } elif f == Fruit.Pear {
                    print("Found a Pear.")
                    return "Pears"
                } elif f == Fruit.Banana {
                    print("Found a Banana.")
                    return "Bananas"
                }
                
                return "None"
            }
        }

    actions:
        getRandomFruit(): Fruit {
            var val = random.randint(1, 3)

            if val == 1 {
                return Fruit.Peach
            } elif val == 2 {
                return Fruit.Pear
            } elif val == 3 {
                return Fruit.Banana
            } else {
                return Fruit.Peach
            }
        }

    domain:
        enum Fruit {
            Peach
            Pear
            Banana
        }
}