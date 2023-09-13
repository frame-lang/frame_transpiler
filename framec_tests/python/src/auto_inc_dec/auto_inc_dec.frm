```
from framelang.framelang import FrameEvent
```
#[codegen.python.code.public_state_info:bool="true"]
#AutoIncDec

    -interface-

    pre
    post
    trans
    -machine-

    $Inc

        |pre|
            b = ++a
            print(b)^

        |post|
            c = a++
            print(c)^

        |trans|
            -> $Dec ^
    $Dec

        |pre|
            b = --a
            print(b)^

        |post|
            c = a--
            print(c)^

        |trans|
            -> $Inc ^

    -actions-
    print_it[val]  {
        print(val)
    }

    -domain-
    var a:int = 0
    var b:int = 0
    var c:int = 0
##
