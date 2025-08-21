system Grocery {
    interface:
        getFruitOfTheDay(): string

    machine:
        $Start {
            getFruitOfTheDay(): string {
                var f: Fruit = Fruit.Peach
                
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