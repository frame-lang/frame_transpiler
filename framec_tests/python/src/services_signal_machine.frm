`import time`
`import signal`
`import sys`

fn main() {
    var service = SignalMachineService()
}

system SignalMachineService {

    operations:
        signal_handler(sig, frame) {
            self.quit()
        }

    interface:
        quit()

    machine:

    $Init {
        $>() {
            signal.signal(signal.SIGINT, self.signal_handler)
            -> $A
        }
    }

    $A => $Done {
        $>() {
            print("$A")
            time.sleep(.2)
            -> $B
        }
    }
    
    $B => $Done {
        $>() {
            print("$B")
            time.sleep(.2)
            -> $A
        }
    }
    
    $Done {
        quit() {
            print("Goodbye!")
            sys.exit(0)
        }
    }
}