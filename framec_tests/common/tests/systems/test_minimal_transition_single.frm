# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
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