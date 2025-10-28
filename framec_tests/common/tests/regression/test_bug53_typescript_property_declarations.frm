system Bug53Test {
    interface:
        initialize(value)
        doSomething()
    
    machine:
        $Start {
            $>() {
                self.myVariable = ""
                self.myPort = 0
                self.frameFile = "test.frm"
            }
            
            initialize(value) {
                self.myVariable = value
                -> $Ready
            }
        }
        
        $Ready {
            doSomething() {
                var result = frameRuntimeCreateServer()
                self.myPort = result.port
            }
        }
    
    actions:
        someAction() {
            frameRuntimeDoSomething(self.myVariable)
        }
}