# TypeScript Frame system with interface and operations

system TypeScriptWithInterface {
    interface:
        test()
    
    operations:
        getValue(): str {
            return "interface test value"
        }
    
    machine:
        $Start {
            test() {
                print("TypeScript with interface test")
                var result = self.getValue()
                print("Result: " + result)
            }
        }
}