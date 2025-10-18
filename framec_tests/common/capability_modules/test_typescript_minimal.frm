# Minimal TypeScript Frame system test

system TypeScriptTest {
    interface:
        test()
    
    machine:
        $Start {
            test() {
                print("TypeScript test")
                var result = self.getValue()
                print("Result: " + result)
            }
        }
    
    operations:
        getValue(): str {
            return "test value"
        }
}