`import time`

fn main() {
    var service = BasicService()
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