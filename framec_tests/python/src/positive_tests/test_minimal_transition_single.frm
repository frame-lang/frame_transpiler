fn main() {
    var service = TestSystem()
}

system TestSystem {
    machine:
    
    $Init {
        $>() {
            -> $A
        }
    }

    $A {
        $>() {
            return
        }
    }
}