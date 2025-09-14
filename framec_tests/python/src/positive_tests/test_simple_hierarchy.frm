# Test simpler module hierarchy

module Utils {
    module Math {
        fn square(x) {
            return x * x
        }
    }
    
    fn useNested() {
        return Math::square(5)
    }
}

fn main() {
    # Direct nested access
    var result = Utils::Math::square(7)
    print("7 squared = " + str(result))
    
    # Indirect access through outer module
    var indirect = Utils::useNested()
    print("5 squared = " + str(indirect))
}

main()