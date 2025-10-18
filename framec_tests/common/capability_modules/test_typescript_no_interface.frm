# TypeScript Frame system demonstrating capability-style patterns (no interface)

system TypeScriptCapabilityDemo {
    operations:
        createOk(value): dict {
            return {
                "isOk": true,
                "isError": false,
                "value": value,
                "error": None
            }
        }
        
        getValue(): str {
            return "test value"
        }

    machine:
        $Start {
            $>() {
                print("TypeScript capability demo")
                var result = self.getValue()
                print("Result: " + result)
                
                var okResult = self.createOk("success")
                print("Created OK result")
            }
        }
}