# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION

#[codegen.python.code.public_state_info:bool="true"]

system StateParams {
    interface:
        Next()
        Prev()
        Log()

    machine:
        $Init {
            Next() {
                -> $Split(1)
            }
        }

        $Split(val:int) {
            Next() {
                -> $Merge(val, val+1)
            }
            Prev() {
                -> $Merge(val+1, val)
            }
            Log() {
                got_param("val", val)
                return
            }
        }

        $Merge(left:int, right:int) {
            Next() {
                -> $Split(left+right)
            }
            Prev() {
                -> $Split(left*right)
            }
            Log() {
                got_param("left", left)
                got_param("right", right)
                return
            }
        }

    actions:
        got_param(name:str, val:int) {
        }

    domain:
        var param_log = []
}
