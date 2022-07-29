```
from framelang.framelang import FrameEvent
```
#[codegen.python.code.public_state_info:bool="true"]

#SimpleHandlerCalls
    -interface-
    A
    B
    C
    D
    E

    -machine-
    $Init
        |A| -> $A ^

        |B| -> $B ^

        |C| A() ^

        |D|
            B()
            -> $A ^

        |E|
            D()
            C() ^

    $A
    $B
##
