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
                console.log("Hello World");
                var x = 42;
                x = 100;
                let result = test_func();
                return;
            }
        }
}

fn test_func() {
    return "test"
}

