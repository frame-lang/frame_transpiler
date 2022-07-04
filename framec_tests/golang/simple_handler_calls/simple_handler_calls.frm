```
package simple_handler_calls

import "golang/framelang"

```
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
