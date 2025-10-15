# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test for return statements with self.method() calls in class methods

class TestClass {
    fn method1() {
        print("In method1")
        return self.method2()
    }
    
    fn method2() {
        print("In method2")
        return 42
    }
}

fn main() {
    var obj = TestClass()
    var result = obj.method1()
    print("Result: " + str(result))
}