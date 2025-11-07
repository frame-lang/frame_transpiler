# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
system Grocery {
    interface:
        getFruitOfTheDay(): string

    machine:
        $Start {
            getFruitOfTheDay(): string {
                f: Fruit = Fruit.Peach
                
                if f == Fruit.Peach {
                    print("Found a Peach")
                }
                
                return "Peaches"
            }
        }

    domain:
        enum Fruit {
            Peach
            Pear
            Banana
        }
}