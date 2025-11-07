# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    service = TestSystem()
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