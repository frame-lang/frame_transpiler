# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
import time

fn main() {
    service = BasicService()
}

system BasicService {

    machine:

    $A {
        $>() {
            print("$A")
            time.sleep(.2)
            -> $B
        }
    }
    
    $B {
        $>() {
            print("$B")
            time.sleep(.2)
            -> $A
        }
    }
}