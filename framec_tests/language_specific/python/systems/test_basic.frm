@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION

#[codegen.python.code.public_state_info:bool="true"]

system Basic {
    interface:
        A()
        B()

    machine:
        $S0 {
            $>() {
                entered("S0") 
                return
            }
            <$() {
                left("S0") 
                return
            }
            A() {
                -> "ooh" $S1 
            }
        }

        $S1 {
            $>() {
                entered("S1") 
                return
            }
            <$() {
                left("S1") 
                return
            }
            B() {
                -> "aah" $S0 
            }
        }

    actions:
        entered(msg:str) {
        }
        left(msg:str) {
        }

    domain:
        entry_log = []
        exit_log = []
}
