# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    # Test with max_loops=2 to allow Init->A->B->A->B then stop
    var service = TestSystem(2)
    print("Test completed - system initialized with max_loops=2")
}

system TestSystem (max_loops) {
    machine:
    
    $Init {
        $>() {
            print("Init enter")
            -> $A
        }
    }

    $A {
        $>() {
            print("A enter, total_loops=" + str(total_loops))
            if total_loops < max_loops {
                total_loops = total_loops + 1
                print("  Transitioning to B (loop " + str(total_loops) + " of " + str(max_loops) + ")")
                -> $B
            } else {
                print("  Staying in A (max_loops " + str(max_loops) + " reached)")
            }
        }
    }
    
    $B {
        $>() {
            print("B enter, total_loops=" + str(total_loops))
            if total_loops < max_loops {
                total_loops = total_loops + 1
                print("  Transitioning to A (loop " + str(total_loops) + " of " + str(max_loops) + ")")
                -> $A
            } else {
                print("  Staying in B (max_loops " + str(max_loops) + " reached)")
            }
        }
    }
    
    domain:
        max_loops = 0  # Default value, will be overridden by constructor parameter
        total_loops = 0
}