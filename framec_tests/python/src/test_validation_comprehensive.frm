// Comprehensive validation test for v0.30 multi-entity support

fn main() {
    // Test multi-system instantiation
    var sys1 = SystemA()
    var sys2 = SystemB("parameter") 
    var sys3 = SystemC()
    
    sys1.start()
    sys2.activate("test")
    sys3.run()
}

fn helper() {
    print("Helper function works")
}

system SystemA {
    interface:
        start()
        stop()
        
    machine:
        $Idle {
            start() {
                helper()
                -> $Running
            }
        }
        
        $Running {
            stop() {
                -> $Idle
            }
        }
}

system SystemB(param) {
    interface:
        activate(data)
        
    machine:
        $Start {
            activate(data) {
                print(param)
                print(data)
                -> $Active  
            }
        }
        
        $Active {
            $>() {
                print("SystemB active")
            }
        }
        
    domain:
        var value:string
}

system SystemC {
    operations:
        run()
        
    machine:
        $Begin {
        }
        
    actions:
        run() {
            print("SystemC running")
        }
}