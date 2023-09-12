```
from framelang.framelang import FrameEvent
```
#[codegen.python.code.public_state_info:bool="true"]
#AutoIncDec

    -interface-

    inc
    dec

    -machine-

    $S1

        |inc|
            b = a++
            print(b)
            c = ++a
            print(c) ^

        |dec|
            b = a--
            print(b)
            c = --a
            print(c)^

    -actions-
    print_it[val]  {
        print(val)
    }

    -domain-
    var a:int = 0
    var b:int = 0
    var c:int = 0
##
