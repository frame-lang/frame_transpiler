# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
#[codegen.python.code.public_state_info:bool="true"]

system BasicTransitionDemo {
    interface:
        next()

    machine:
        $Start {
            next() {
                -> $End
                return
            }
        }

        $End {
        }
}