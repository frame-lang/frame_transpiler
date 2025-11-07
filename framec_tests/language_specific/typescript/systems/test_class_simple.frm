# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Simple class test

class TestClass {
    var x = 0
    var y = 0
    
    fn init(x, y) {
        self.x = x
        self.y = y
    }
    
    fn add() {
        return self.x + self.y
    }
}

fn main() {
    var obj = TestClass(5, 10)
    var result = obj.add()
    print("Result: " + str(result))
    return
}