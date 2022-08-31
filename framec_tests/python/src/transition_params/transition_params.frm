```
from framelang.framelang import FrameEvent
```
#[codegen.python.code.public_state_info:bool="true"]

#TransitParams
    -interface-
    Next
    Change

    -machine-
    $Init
        |Next|
            -> ("hi A") $A ^
        |Change|
            ->> $A ^

    $A
        |>| [msg:str]
            log(msg) ^

        |<|
            log("bye A") ^

        |Next|
            -> ("hi B" 42) $B ^

        |Change|
            ->> $B ^

    $B
        |>| [msg:str val:int]
            log(msg)
            log(str(val)) ^

        |<| [val:bool msg:str]
            log(str(val))
            log(msg) ^

        |Next|
            (true "bye B") -> ("hi again A") $A ^

        |Change|
            ->> $A ^

    -actions-
    log [msg:str]

    -domain-
    var tape = `[]`
##
