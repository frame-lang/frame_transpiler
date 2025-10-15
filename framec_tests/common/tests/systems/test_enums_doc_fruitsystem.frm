# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
system FruitSystem {
    actions:
        getFruitOfTheDay(): Fruit {
            var fruit_of_the_day: Fruit = Fruit.Pear
            return fruit_of_the_day
        }
        
    domain:
        enum Fruit {
            Peach
            Pear
            Banana
        }
}