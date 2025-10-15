fn main() {
    var sys = TestSourceMapping()
    sys.test()
}

system TestSourceMapping {
    interface:
        test()
        
    machine:
        $Start {
            test() {
                # Test various executable statements that should NOT be mapped as function_def
                var message = "Hello"          # Line 12: Should be VarDecl
                message = "Updated"            # Line 13: Should be Assignment  
                print("Debug output")          # Line 14: Should be Print or Statement
                var result = self.helper()    # Line 15: Should be VarDecl (var) + MethodCall (call)
                return                         # Line 16: Should be Return
            }
        }
    
    actions:
        helper() { 
            return "helper result" 
        }
}