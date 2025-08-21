// Basic enum declaration and usage test

system CalendarSystem {
    domain:
        enum Days {
            SUNDAY
            monday
            Tuesday
            WEDNESDAY
            tHuRsDaY
            FRIDAY
            SATURDAY
        }
}

system EnumValueSystem {
    domain:
        enum Days {
            SUNDAY
            monday
            Tuesday = 1000
            WEDNESDAY
            tHuRsDaY
            FRIDAY
            SATURDAY = 1000
            SUNDAY2 = 2000
        }
}

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

fn testFruit() {
    var f: Fruit = getFruitOfTheDay()

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

fn describeFruit(fruit_value: Fruit) {
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