```
package transition_params

import (
    "strconv"
    "golang/framelang")

```
#TransitParams
    -interface-
    Next
    Change

    -machine-
    $Init
        |Next|
            -> ("hi A") $A ^
        |Change|
            // ->> $A
            ^

    $A
        |>| [msg:string]
            log(msg) ^

        |<|
            log("bye A") ^

        |Next|
            -> ("hi B", 42) $B ^

        |Change|
            // ->> $B
            ^

    $B
        |>| [msg:string, val:int]
            log(msg)
            log(strconv.Itoa(val)) ^

        |<| [val:bool, msg:string]
            log(strconv.FormatBool(val))
            log(msg) ^

        |Next|
            (true, "bye B") -> ("hi again A") $A ^

        |Change|
            // ->> $A
            ^

    -actions-
    log [msg:string]

    -domain-
    var tape:`[]string` = `[]string{}`
##
