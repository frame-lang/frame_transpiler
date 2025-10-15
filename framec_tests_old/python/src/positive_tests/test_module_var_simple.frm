# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Simple module-level variable test with system only

# Module-level variable declaration  
var module_var = "I am at module level"

system TestModuleVar {
    machine:
        $Start {
            $>() {
                print("System accessing module variable:")
                print(module_var)
            }
        }
}