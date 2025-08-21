`import time`

fn main() {
    var looper = Looper(10)  // Small test with 10 loops
}

system Looper ($>(loops)) {

    machine:

    $Start {
        $>(loops) {
            print("Starting")
            -> (loops, loops, time.time()) $A
        }
    }

    $A {
        $>(total_loops, loops_left, start) {
            if loops_left == 0 {
                -> (total_loops, start) $Done
                return
            }
            -> (total_loops, loops_left, start) $B
        }
    }
    
    $B {
        $>(total_loops, loops_left, start) {
            loops_left = loops_left - 1
            -> (total_loops, loops_left, start) $A
        }
    }

    $Done {
        $>(total_loops, start) {
            print("Done. Looped " + str(total_loops) + " times in ", end = " ") 
            print(str(time.time() - start) + " seconds.")
        }
    }
}