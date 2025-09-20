# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Simple v0.46 test

class Animal {
    var count = 0
    
    fn init(name) {
        self.name = name
    }
    
    @classmethod
    fn get_count(cls) {
        return cls.count
    }
}

fn test() {
    var a = Animal("Dog")
    print(str(Animal.get_count()))
}

test()