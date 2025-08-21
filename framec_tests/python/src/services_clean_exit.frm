`import time`
`import signal`
`import sys`

fn main() {
    var service = CleanExitService()
}

system CleanExitService {

    operations:

    signal_handler(sig, frame) {
        sys.exit(0)
    }

    machine:

    $Init {
        $>() {
            signal.signal(signal.SIGINT, self.signal_handler)
            -> $A
        }
    }

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