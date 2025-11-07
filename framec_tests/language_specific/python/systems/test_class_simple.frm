# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Simple class test

class TestClass {
    var x:int = 0
    var y:int = 0
    
    fn init(x, y) {
        self.x = x
        self.y = y
    }
    
    fn add() {
        return self.x + self.y
    }
}

fn main() {
    obj = TestClass(5, 10)
    result = obj.add()
    print("Result: " + str(result))
    return
}
