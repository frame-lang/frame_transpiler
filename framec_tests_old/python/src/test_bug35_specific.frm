fn main() {
    var sys = TestMappingTypes()
    sys.test()
}

system TestMappingTypes {
    interface:
        test()
        
    machine:
        $Start {
            test() {
                print("Hello World")     # Should be MappingType::Print
                var x = 42               # Should be MappingType::VarDecl
                x = 100                  # Should be MappingType::Assignment
                var result = test_func() # VarDecl + FunctionCall
                return                   # Should be MappingType::Return
            }
        }
}

fn test_func() {  # Should be MappingType::FunctionDef
    return "test"
}