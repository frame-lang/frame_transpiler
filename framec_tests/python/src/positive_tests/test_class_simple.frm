# Simple class test

class TestClass {
    var x = 0
    var y = 0
    
    fn init(self, x, y) {
        self.x = x
        self.y = y
    }
    
    fn add(self) {
        return self.x + self.y
    }
}

fn main() {
    var obj = TestClass()
    obj.init(5, 10)
    var result = obj.add()
    print("Result: " + str(result))
    return
}