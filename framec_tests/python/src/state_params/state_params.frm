
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
                return
            }
        }

        $Split(val:int) {
            Next() {
                -> $Merge(val, val+1)
                return
            }
            Prev() {
                -> $Merge(val+1, val)
                return
            }
            Log() {
                got_param("val", val)
                return
            }
        }

        $Merge(left:int, right:int) {
            Next() {
                -> $Split(left+right)
                return
            }
            Prev() {
                -> $Split(left*right)
                return
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
        var param_log = `[]`
}
