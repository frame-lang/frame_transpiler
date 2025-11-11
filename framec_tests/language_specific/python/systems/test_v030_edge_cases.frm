@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test: Edge cases for v0.30 multi-entity support
# Empty systems, minimal functions, various combinations

fn main() {
    minimal()
    empty = EmptySystem()
    simple = SimpleSystem()
}

fn minimal() {
    return "minimal"
}

system EmptySystem {
    machine:
        $S {
        }
}

system SimpleSystem {
    interface:
        test()
        
    machine:
        $Begin {
            test() {
                return
            }
        }
}

system SystemWithDomain {
    machine:
        $Init {
        }
        
    domain:
        value:string = "default"
        count:int = 0
}

system SystemWithOperations {
    interface:
        op1()
        op2(param)
        
    machine:
        $State {
            op1() {
                # empty operation
            }
            
            op2(param) {
                print(param)
            }
        }
}
