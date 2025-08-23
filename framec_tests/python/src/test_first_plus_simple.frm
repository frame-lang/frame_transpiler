// First system plus simple second system

system FirstSystem {
    interface:
        start()
        
    machine:
        $Begin {
            start() {
                -> $Running
            }
        }
        
        $Running {
            $>() {
                print("Running")
            }
        }
}

system SimpleSystem {
    machine:
        $S {
        }
}