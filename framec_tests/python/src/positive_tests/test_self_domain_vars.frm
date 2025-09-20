# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test explicit self.domainVar syntax
fn main() {
    var sys = DomainVarTest()
    sys.test()
}

system DomainVarTest {
    interface:
        test()
        
    machine:
        $Start {
            test() {
                # Test reading domain var with self (works)
                print("Count is: " + str(self.count))
                
                # Test writing domain var (self. not supported in assignments yet)
                count = count + 1
                print("Count after increment: " + str(self.count))
                
                # Test domain var in expression with self (works)
                var doubled = self.count * 2
                print("Doubled: " + str(doubled))
                
                # Test string domain var
                message = "Hello from self"
                print("Message: " + self.message)
                
                return
            }
        }
        
    domain:
        var count: int = 10
        var message: string = "initial"
}