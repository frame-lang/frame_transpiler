@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
system Grocery {
    interface:
        getFruitOfTheDay(): string

    machine:
        $Start {
            getFruitOfTheDay(): string {
                f: Fruit = Fruit.Peach
                
                if f == Fruit.Peach:
                    return "Peaches"
                
                return "None"
            }
        }

    actions:
        getRandomFruit(): Fruit {
            return Fruit.Peach
        }

    domain:
        enum Fruit {
            Peach
            Pear
            Banana
        }
}
